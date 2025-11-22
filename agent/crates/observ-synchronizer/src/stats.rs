use crate::{
	config::sync_config,
	statistic::{LogStore, Statistic, add_log, get_statistic, sync_log},
};
use arc_swap::access::Access;
use chrono::Local;
use chrono_tz::Asia::Shanghai;
use elasticsearch::{
	Elasticsearch, GetParts, IndexParts, UpdateParts,
	auth::Credentials,
	cert::CertificateValidation,
	http::{
		Url,
		transport::{SingleNodeConnectionPool, TransportBuilder},
	},
};
use log::{debug, error, info};
use nix::{ifaddrs::getifaddrs, sys::socket::SockAddr};
use observ_config::{agent_config, elastic_sender_config, synchronizer_config};
use serde::Serialize;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
	fs::File,
	io::{BufRead, BufReader},
	net::{IpAddr, UdpSocket},
	sync::{
		Arc,
		atomic::{AtomicBool, Ordering},
	},
	time::Duration,
};
use sysinfo::System;
use tokio::time::interval;

#[derive(Serialize)]
struct Stats {
	/// 状态信息 状态 1：运行
	state: String,
	/// 采集器名称                     
	name: String,
	/// 采集器ID TODO
	lcuuid: String,
	/// 区域 TODO                   
	region_name: String,
	/// 可用区 TODO               
	az: String,
	/// 可用区名称 TODO                      
	az_name: String,
	/// 采集器组ID TODO                  
	vtap_group_lcuuid: String,
	/// 采集器组 TODO            
	vtap_group_name: String,
	/// 所属容器集群 TODO            
	pod_cluster_name: String,
	/// 软件版本 TODO            
	revision: String,
	/// 完整版本号 TODO
	complete_revision: String,
	/// K8s镜像地址 TODO
	current_k8s_image: String,
	/// 采集模式 0：本地 TODO    
	tap_mode: i32,
	/// 运行环境类型 1：容器-V TODO                    
	arch_type: i32,
	/// 体系架构                
	arch: String,
	/// 操作系统                      
	os: String,
	/// 内核版本                       
	kernel_version: String,
	/// CPU核数             
	cpu_num: u16,
	/// 总内存(GB)                       
	memory_size: f64,
	/// 运行环境IP              
	launch_server: String,
	/// 控制IP              
	ctrl_ip: String,
	/// 控制MAC                  
	ctrl_mac: String,
	/// 分配控制器IP             
	controller_ip: String,
	/// 当前控制器IP             
	cur_controller_ip: String,
	/// 分配数据节点IP            
	analyzer_ip: String,
	/// 当前数据节点IP                
	cur_analyzer_ip: String,
	/// 异常信息            
	error_info: Option<String>,
	/// 数据节点通信耗时间         
	synced_analyzer_at: Option<String>,
	/// 控制器同步耗时间  
	synced_controller_at: Option<String>,
	/// 授权类型
	license_type: String,
	/// 授权功能            
	license_functions: Option<String>,
	/// 创建时间
	create_time: String,
	/// 更新时间           
	update_time: String,
	/// 所属用户                
	user_id: Option<i32>,
}

impl Stats {
	async fn new() -> Self {
		let update_time = Local::now().with_timezone(&Shanghai).to_rfc3339();
		let state = "running".to_string();
		let name = agent_config().load().name.clone();
		let mut hasher = Sha256::new();
		hasher.update(name.as_bytes());
		let lcuuid = format!("{:x}", hasher.finalize());

		let sys = System::new_all();
		let os = System::name().unwrap_or_else(|| "Unknown".to_string());
		let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
		let cpu_num = System::physical_core_count().unwrap_or(0);
		let memory_size = sys.total_memory() as f64 / 1024.0 / 1024.0;

		let cur_analyzer_ip = Self::get_local_ip()
			.map(|ip| ip.to_string())
			.unwrap_or_else(|_| "0.0.0.0".to_string());
		let ctrl_mac = Self::get_mac_address().unwrap_or_else(|_| "".to_string());
		let arch = std::env::consts::ARCH.to_string();

		let _static = get_statistic().await.unwrap_or_else(|| Statistic {
			cpu_usage: 0.0,
			memory_usage: 0.0,
			name: name.clone(),
			lcuuid: lcuuid.clone(),
			span_num: 0,
			timestamp: chrono::Utc::now().to_rfc3339(),
			log_store: LogStore {
				agent_name: name.clone(),
				lcuuid: lcuuid.clone(),
				logs: Vec::new(),
			},
		});

		let create_time = _static.timestamp.clone();

		Self {
			update_time,
			state,
			name,
			lcuuid,
			region_name: "".to_string(),
			az: "".to_string(),
			az_name: "".to_string(),
			vtap_group_lcuuid: "".to_string(),
			vtap_group_name: "".to_string(),
			pod_cluster_name: "".to_string(),
			revision: "".to_string(),
			complete_revision: "".to_string(),
			current_k8s_image: "".to_string(),
			tap_mode: 0,
			arch_type: 0,
			arch,
			os,
			kernel_version,
			cpu_num: cpu_num.try_into().unwrap_or(0),
			memory_size,
			launch_server: "".to_string(),
			ctrl_ip: "".to_string(),
			ctrl_mac,
			controller_ip: "".to_string(),
			cur_controller_ip: "".to_string(),
			analyzer_ip: "".to_string(),
			cur_analyzer_ip,
			error_info: None,
			synced_analyzer_at: None,
			synced_controller_at: None,
			license_type: "".to_string(),
			license_functions: None,
			create_time,
			user_id: None,
		}
	}
	fn get_local_ip() -> Result<IpAddr, Box<dyn std::error::Error>> {
		let socket = UdpSocket::bind("0.0.0.0:0")?;
		socket.connect("8.8.8.8:80")?;
		Ok(socket.local_addr()?.ip())
	}

	fn get_mac_address() -> Result<String, Box<dyn std::error::Error>> {
		let interfaces = getifaddrs()?;
		for interface in interfaces {
			if let Some(SockAddr::Link(link_addr)) = interface.address {
				let mac = link_addr.addr(); // 直接获取 [u8; 6] 数组
				return Ok(mac.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(":"));
			}
		}
		Err("MAC 地址未找到".into())
	}
}

#[derive(Serialize)]
struct Metrics {
	name: String,
	lcuuid: String,
	cpu_usage: f64,
	memory_usage: f64,
	timestamp: String,
	span_num: u64,
}

impl Metrics {
	async fn new() -> Self {
		let timestamp = Local::now().with_timezone(&Shanghai).to_rfc3339();
		let name = agent_config().load().name.clone();
		let mut hasher = Sha256::new();
		hasher.update(name.as_bytes());
		let lcuuid = format!("{:x}", hasher.finalize());

		// 获取当前进程 CPU 使用率
		let mut sys = System::new_all();
		let pid = sysinfo::get_current_pid().expect("Failed to get current PID");
		sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[pid]), true); // 第一次刷新
		let _cpu_before = sys.process(pid).map(|p| p.cpu_usage()).unwrap_or(0.0);

		tokio::time::sleep(Duration::from_secs(1)).await; // 间隔1秒

		sys.refresh_processes(sysinfo::ProcessesToUpdate::Some(&[pid]), true); // 第二次刷新
		let cpu_usage = sys.process(pid).map(|p| p.cpu_usage() as f64).unwrap_or(0.0);
		debug!("3 Current process CPU usage: {}", cpu_usage);

		let memory_usage = Self::get_mem_usage();

		// span_num 示例赋值为 0
		let span_num = 0;

		Self { name, lcuuid, cpu_usage, memory_usage, timestamp, span_num }
	}

	fn get_mem_usage() -> f64 {
		let pid = sysinfo::get_current_pid().expect("Failed to get current PID");
		let path = format!("/proc/{}/status", pid);
		let file = File::open(path).ok();
		if let Some(file) = file {
			let reader = BufReader::new(file);
			for line in reader.lines().flatten() {
				if line.starts_with("VmRSS:") {
					let parts: Vec<&str> = line.split_whitespace().collect();
					if parts.len() >= 2 {
						if let Ok(kb) = parts[1].parse::<u64>() {
							return kb as f64 / 1024.0; // 转换为 MB
						}
					}
				}
			}
		}
		0.0
	}
}

pub(super) async fn sync_stats(client: &Elasticsearch) {
	let es_index_name = "agent_stats";

	let stats = Metrics::new().await;
	let agent_name = stats.name.clone();
	let lcuuid = stats.lcuuid.clone();
	let timestamp = stats.timestamp;
	let cpu_usage = stats.cpu_usage;
	let memory_usage = stats.memory_usage;
	let span_num = stats.span_num;

	let resp = client
		.index(IndexParts::Index(es_index_name))
		.body(json!({
			"agent_name": agent_name,
			"lcuuid": lcuuid,
			"timestamp": timestamp,
			"cpu_usage": cpu_usage,
			"memory_usage": memory_usage,
			"span_num": span_num
		}))
		.send()
		.await;

	match resp {
		Ok(r) if r.status_code().is_success() => {
			info!("Stats synced to ES successfully, agent: {}, lcuuid: {}", agent_name, lcuuid);
		},
		Ok(r) => {
			error!("Failed to sync stats, status: {}", r.status_code());
		},
		Err(e) => {
			error!("Failed to sync stats: {:?}", e);
		},
	}
}

pub(super) async fn health_checker(running: Arc<AtomicBool>) {
	let config = synchronizer_config().load();
	let c = elastic_sender_config(&config.sender);
	let url = Url::parse(&c.node_url).expect("Invalid URL");
	let conn_pool = SingleNodeConnectionPool::new(url);

	let transport = TransportBuilder::new(conn_pool)
		.disable_proxy()
		.auth(Credentials::Basic(c.username.clone(), c.password.clone()))
		.cert_validation(CertificateValidation::None)
		.timeout(Duration::from_secs(c.request_timeout))
		.build()
		.expect("Failed to build transport");

	info!("Sync agent state to Elasticsearch at {}", c.node_url);

	let client = Elasticsearch::new(transport);

	let mut interval = interval(Duration::from_secs(10));

	let es_index_name = "agent_basic";

	while running.load(Ordering::Relaxed) {
		add_log("info", "Starting state synchronization").await;
		interval.tick().await;
		sync_stats(&client).await;
		sync_config(&client).await;
		sync_log(&client).await;
		let state = Stats::new().await;
		let state_id = state.lcuuid.clone();

		let exists = client
			.get(GetParts::IndexId(es_index_name, state_id.as_str()))
			.send()
			.await
			.map(|resp| resp.status_code().is_success())
			.unwrap_or(false);

		let response_result = if exists {
			debug!("Updating state with ID: {}", state_id);
			client
				.update(UpdateParts::IndexId(es_index_name, state_id.as_str()))
				.body(json!({
					"doc": {
						"id": state_id,
						"state": state
					},
					"doc_as_upsert": true
				}))
				.send()
				.await
		} else {
			debug!("Indexing new state with ID: {}", state_id);
			client
				.index(IndexParts::IndexId(es_index_name, state_id.as_str()))
				.body(json!({
					"id": state_id,
					"state": state
				}))
				.send()
				.await
		};

		match response_result {
			Ok(response) => {
				let status = response.status_code();
				debug!("State sync response status: {}", status);
				if !status.is_success() {
					if let Ok(error_msg) = response.text().await {
						error!("Elasticsearch error: {error_msg}");
					} else {
						error!("Elasticsearch error: failed to read response text");
					}
				}
			},
			Err(e) => {
				error!("Failed to sync state: {:?}", e);
			},
		}
	}
}
