use tokio::sync::{Mutex, OnceCell};
use chrono::Utc;
use serde_json::json;
use log::{info, error};
use serde::Serialize;
use log::debug;

#[derive(Debug, Clone, Serialize)]
pub struct LogEntry {
    pub timestamp: String,    // 日志时间戳
    pub level: String,        // 日志等级
    pub content: String,      // 日志内容
}

#[derive(Debug, Clone, Serialize)]
pub struct LogStore {
    pub agent_name: String,       // 采集器名称
    pub lcuuid: String,           // 采集器ID
    pub logs: Vec<LogEntry>,      // 日志列表
}



#[derive(Debug, Clone)]
pub struct Statistic {
    pub name: String,
    pub lcuuid: String,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub timestamp: String,
    pub span_num: u64,
    pub log_store: LogStore,      // 新增：日志存储结构体
}

// 全局异步变量
pub static GLOBAL_STAT: OnceCell<Mutex<Statistic>> = OnceCell::const_new();

// 初始化方法（在 main 或初始化流程中调用一次）
pub fn init_statistic(stat: Statistic) {
    GLOBAL_STAT.set(Mutex::new(stat)).ok();
}

// 异步读写示例
pub async fn update_cpu_usage(new_cpu: f64) {
    if let Some(stat_mutex) = GLOBAL_STAT.get() {
        let mut stat = stat_mutex.lock().await;
        stat.cpu_usage = new_cpu;
    }
}

pub async fn get_statistic() -> Option<Statistic> {
    if let Some(stat_mutex) = GLOBAL_STAT.get() {
        let stat = stat_mutex.lock().await;
        Some(stat.clone())
    } else {
        None
    }
}

pub async fn add_log(level: &str, content: &str) {
    if let Some(stat_mutex) = GLOBAL_STAT.get() {
        let mut stat = stat_mutex.lock().await;
        let log_entry = LogEntry {
            timestamp: Utc::now().to_rfc3339(),
            level: level.to_string(),
            content: content.to_string(),
        };
        stat.log_store.logs.push(log_entry.clone());
    }
}

pub async fn sync_log(client: &elasticsearch::Elasticsearch) {
    if let Some(stat_mutex) = GLOBAL_STAT.get() {
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
                .index(elasticsearch::IndexParts::Index("agent_log"))
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
                }
                Ok(r) => {
                    error!("Failed to sync log, status: {}", r.status_code());
                }
                Err(e) => {
                    error!("Failed to sync log: {:?}", e);
                }
            }
        }

        stat.log_store.logs.clear();
    }
}