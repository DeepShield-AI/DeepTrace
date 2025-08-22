use crate::{
	app::state,
	config::{agent_config, elastic_config, sync_config},
    app::statistic::{Statistic, get_statistic, LogStore, sync_log, add_log},
};
use arc_swap::access::Access;
use chrono::Local;
use chrono_tz::Asia::Shanghai;
use elasticsearch::{
	BulkParts, Elasticsearch,
	auth::Credentials,
	http::{
		request::JsonBody,
		transport::{SingleNodeConnectionPool, TransportBuilder},
	},
};
use log::{debug, error, info};
use serde::Serialize;
use serde_json::json;
use std::{sync::atomic::Ordering, time::Duration};
use tokio::time::interval;
use url::Url;
use sha2::{Sha256, Digest};
use sysinfo::{System, SystemExt, ProcessExt};
use std::net::{IpAddr, UdpSocket};
use nix::ifaddrs::getifaddrs;
use nix::sys::socket::SockAddr;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Serialize)]
struct Basic {
    state: String, // 状态信息 状态 1：运行
    name: String, // 采集器名称
    lcuuid: String, // 采集器ID TODO
    region_name: String, // 区域 TODO
    az: String, // 可用区 TODO
    az_name: String, // 可用区名称 TODO
    vtap_group_lcuuid: String, // 采集器组ID TODO
    vtap_group_name: String, // 采集器组 TODO
    pod_cluster_name: String, // 所属容器集群 TODO
    revision: String, // 软件版本 TODO
    complete_revision: String, // 完整版本号 TODO
    current_k8s_image: String, // K8s镜像地址 TODO
    tap_mode: i32, // 采集模式 0：本地 TODO
    arch_type: i32, // 运行环境类型 1：容器-V TODO
    arch: String, // 体系架构
    os: String, // 操作系统
    kernel_version: String, // 内核版本
    cpu_num: u16, // CPU核数
    memory_size: f64, // 总内存(GB)
    launch_server: String, // 运行环境IP
    ctrl_ip: String, // 控制IP
    ctrl_mac: String, // 控制MAC
    controller_ip: String, // 分配控制器IP
    cur_controller_ip: String, // 当前控制器IP
    analyzer_ip: String, // 分配数据节点IP
    cur_analyzer_ip: String, // 当前数据节点IP
    error_info: Option<String>, // 异常信息
    synced_analyzer_at: Option<String>, // 数据节点通信耗时间
    synced_controller_at: Option<String>, // 控制器同步耗时间
    license_type: String, // 授权类型
    license_functions: Option<String>, // 授权功能
    create_time: String, // 创建时间
    update_time: String, // 更新时间
    user_id: Option<i32>, // 所属用户
}



impl Basic {    
	async fn new() -> Self {
        let update_time = Local::now().with_timezone(&Shanghai).to_rfc3339();
        let state = match state().load(Ordering::Relaxed) {
            true => "terminate".to_string(),
            false => "running".to_string(),
        };
        let name = agent_config().load().name.clone();
        let mut hasher = Sha256::new();
        hasher.update(name.as_bytes());
        let lcuuid = format!("{:x}", hasher.finalize());

        let sys = System::new_all();
        let os = sys.name().unwrap_or_else(|| "Unknown".to_string());
        let kernel_version = sys.kernel_version().unwrap_or_else(|| "Unknown".to_string());
        let cpu_num = sys.physical_core_count().unwrap_or(0);
        let memory_size = sys.total_memory() as f64 / 1024.0 / 1024.0;

        // 获取运行环境 IP
        let cur_analyzer_ip = Self::get_local_ip()
			.map(|ip| ip.to_string())
			.unwrap_or_else(|_| "0.0.0.0".to_string());
        // 获取 MAC 地址
        let ctrl_mac = Self::get_mac_address().unwrap_or_else(|_| "".to_string());
        // 获取体系架构
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

	// 获取 MAC 地址
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
struct Stats {
    name: String,
    lcuuid: String,
    cpu_usage: f64,
    memory_usage: f64,
    timestamp: String,
    span_num: u64,

}

impl Stats {
    async fn new() -> Self {
        let timestamp = Local::now().with_timezone(&Shanghai).to_rfc3339();
        let name = agent_config().load().name.clone();
        let mut hasher = Sha256::new();
        hasher.update(name.as_bytes());
        let lcuuid = format!("{:x}", hasher.finalize());

        // 获取当前进程 CPU 使用率
        let mut sys = System::new_all();
        let pid = sysinfo::get_current_pid().expect("Failed to get current PID");
		sys.refresh_process(pid); // 第一次刷新
		let _cpu_before = sys.process(pid).map(|p| p.cpu_usage()).unwrap_or(0.0);

		tokio::time::sleep(Duration::from_secs(1)).await; // 间隔1秒

		sys.refresh_process(pid); // 第二次刷新
		let cpu_usage = sys.process(pid).map(|p| p.cpu_usage()).unwrap_or(0.0);
		debug!("3 Current process CPU usage: {}", cpu_usage);

        // 获取当前进程内存占用（MB）
        let memory_usage = Self::get_mem_usage();

        // span_num 示例赋值为 0
        let span_num = 0;

        Self {
            name,
            lcuuid,
            cpu_usage: cpu_usage.into(),
            memory_usage,
            timestamp,
            span_num,
        }
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

pub(super) async fn sync_stats(client: &elasticsearch::Elasticsearch) {
    let es_index_name = "agent_stats";

    let stats = Stats::new().await;
    let agent_name = stats.name.clone();
    let lcuuid = stats.lcuuid.clone();
    let timestamp = stats.timestamp;
    let cpu_usage = stats.cpu_usage;
    let memory_usage = stats.memory_usage;
    let span_num = stats.span_num;

    // 每次采集直接写入一行
    let resp = client
        .index(elasticsearch::IndexParts::Index(es_index_name))
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
        }
        Ok(r) => {
            error!("Failed to sync stats, status: {}", r.status_code());
        }
        Err(e) => {
            error!("Failed to sync stats: {:?}", e);
        }
    }
}


pub(super) async fn health_checker() {
	let config = elastic_config();
	let c = config.load();
	let url = Url::parse(&c.node_url).expect("Invalid URL");
	let conn_pool = SingleNodeConnectionPool::new(url);

	let transport = TransportBuilder::new(conn_pool)
		.disable_proxy()
		.auth(Credentials::Basic(c.username.clone(), c.password.clone()))
		.timeout(Duration::from_secs(c.request_timeout))
		.build()
		.expect("Failed to build transport");

	info!("Sync agent state to Elasticsearch at {}", c.node_url);

	let client = Elasticsearch::new(transport);

	let mut interval = interval(Duration::from_secs(10));

	let es_index_name = "agent_basic";


	loop {
        add_log("info", "Starting state synchronization").await;
		interval.tick().await;
		sync_stats(&client).await;
        sync_config(&client).await;
        sync_log(&client).await;
		let state = Basic::new().await;
		let state_id = state.lcuuid.clone(); // 使用哈希后的id

		// 先检查是否存在该文档
		let exists = client
			.get(
				elasticsearch::GetParts::IndexId(
					es_index_name,
					state_id.as_str(),
				),
			)
			.send()
			.await
			.map(|resp| resp.status_code().is_success())
			.unwrap_or(false);

		let response = if exists {
			// 存在则更新
			debug!("Updating state with ID: {}", state_id);
			client
				.update(
					elasticsearch::UpdateParts::IndexId(
						es_index_name,
						state_id.as_str(),
					),
				)
				.body(json!({
					"doc": state,
					"doc_as_upsert": true
				}))
				.send()
				.await
				.expect("Failed to send update request")
		} else {
			// 不存在则插入
			debug!("Indexing new state with ID: {}", state_id);
			client
				.index(
					elasticsearch::IndexParts::IndexId(
						es_index_name,
						state_id.as_str(),
					),
				)
				.body(json!(state))
				.send()
				.await
				.expect("Failed to send index request")
		};

		let status = response.status_code();
		debug!("State sync response status: {}", status);

		if !status.is_success() {
			let error_msg = response.text().await.expect("Failed to read response");
			error!("Elasticsearch error: {error_msg}");
		}


		// interval.tick().await;

		// let mut bulk_body: Vec<JsonBody<serde_json::Value>> = Vec::with_capacity(2);
		// bulk_body.push(
		// 	json!({
		// 		"index": {
		// 			"_index": agent_config().load().state_index,
		// 		}
		// 	})
		// 	.into(),
		// );
		// bulk_body.push(json!(State::new()).into());

		// let response = client
		// 	.bulk(BulkParts::None)
		// 	.body(bulk_body)
		// 	.send()
		// 	.await
		// 	.expect("Failed to send request");

		// let status = response.status_code();
		// debug!("State sync response status: {}", status);

		// if !status.is_success() {
		// 	let error_msg = response.text().await.expect("Failed to read response");
		// 	error!("Elasticsearch error: {error_msg}");
		// }
	}
}
