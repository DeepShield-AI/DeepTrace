use arc_swap::access::Access;
use chrono::Utc;
use elasticsearch::{Elasticsearch, IndexParts};
use log::{error, info};
use observ_config::agent_config;
use serde::Serialize;
use serde_json::json;
use sha2::{Digest, Sha256};
use tokio::sync::{Mutex, OnceCell};

#[derive(Debug, Clone, Serialize)]
pub struct LogEntry {
	pub timestamp: String,
	pub level: String,
	pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LogStore {
	/// 采集器名称
	pub agent_name: String,
	/// 采集器ID
	pub lcuuid: String,
	/// 日志列表    
	pub logs: Vec<LogEntry>,
}

#[derive(Debug, Clone)]
pub struct Statistic {
	pub name: String,
	pub lcuuid: String,
	pub cpu_usage: f64,
	pub memory_usage: f64,
	pub timestamp: String,
	pub span_num: u64,
	pub log_store: LogStore,
}

pub static STAT: OnceCell<Mutex<Statistic>> = OnceCell::const_new();

impl Statistic {
	pub fn new() -> Self {
		let config = agent_config().load();
		let name = config.name.clone();
		let mut hasher = Sha256::new();
		hasher.update(name.as_bytes());
		let lcuuid = format!("{:x}", hasher.finalize());
		Self {
			name: name.to_string(),
			lcuuid: lcuuid.to_string(),
			cpu_usage: 0.0,
			memory_usage: 0.0,
			timestamp: Utc::now().to_rfc3339(),
			span_num: 0,
			log_store: LogStore {
				agent_name: name.to_string(),
				lcuuid: lcuuid.to_string(),
				logs: Vec::new(),
			},
		}
	}
}

pub fn init_statistic(stat: Statistic) {
	STAT.set(Mutex::new(stat)).ok();
}

pub async fn update_cpu_usage(new_cpu: f64) {
	if let Some(stat_mutex) = STAT.get() {
		let mut stat = stat_mutex.lock().await;
		stat.cpu_usage = new_cpu;
	}
}

pub async fn get_statistic() -> Option<Statistic> {
	if let Some(stat_mutex) = STAT.get() {
		let stat = stat_mutex.lock().await;
		Some(stat.clone())
	} else {
		None
	}
}

pub async fn add_log(level: &str, content: &str) {
	if let Some(stat_mutex) = STAT.get() {
		let mut stat = stat_mutex.lock().await;
		let log_entry = LogEntry {
			timestamp: Utc::now().to_rfc3339(),
			level: level.to_string(),
			content: content.to_string(),
		};
		stat.log_store.logs.push(log_entry.clone());
	}
}

pub async fn sync_log(client: &Elasticsearch) {
	if let Some(stat_mutex) = STAT.get() {
		let mut stat = stat_mutex.lock().await;
		let lcuuid = stat.lcuuid.clone();
		let agent_name = stat.name.clone();
		let logs_to_sync = stat.log_store.logs.clone();

		if logs_to_sync.is_empty() {
			info!("No logs to sync for agent: {}", agent_name);
			return;
		}

		for log_entry in logs_to_sync.iter() {
			let resp = client
				.index(IndexParts::Index("agent_log"))
				.body(json!({
					"agent_name": agent_name,
					"lcuuid": lcuuid,
					"content": log_entry.content,
					"timestamp": log_entry.timestamp,
					"level": log_entry.level,
				}))
				.send()
				.await;

			match resp {
				Ok(r) if r.status_code().is_success() => {
					info!("Log synced to ES successfully, agent: {}, id: {}", agent_name, lcuuid);
				},
				Ok(r) => {
					error!("Failed to sync log, status: {}", r.status_code());
				},
				Err(e) => {
					error!("Failed to sync log: {:?}", e);
				},
			}
		}

		stat.log_store.logs.clear();
	}
}
