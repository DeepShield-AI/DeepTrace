use super::{
	AgentConfig, ApiConfig, ConfigError, ProvenanceConfig, SenderConfig, ServerConfig, SpanConfig,
	TraceConfig,
};
use crate::constants::DEFAULE_CONFIG_PATH;
use config::{Config, File};
use log::{error, warn};
use serde::Deserialize;
use rand::{Rng, seq::SliceRandom};
use arc_swap::access::Access;
use crate::config::agent_config;
use sha2::{Sha256, Digest};
use serde::Serialize;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct AppConfig {
	pub api: ApiConfig,
	pub agent: AgentConfig,
	pub server: ServerConfig,
	pub sender: SenderConfig,
	pub trace: TraceConfig,
	pub span: SpanConfig,
	pub provenance: ProvenanceConfig,
}

impl AppConfig {
	pub fn load(path: impl AsRef<str>) -> Result<Self, ConfigError> {
		Self::load_from_file(path.as_ref())
			.inspect_err(|e| {
				warn!("Failed to load config from {}: {}, using default", path.as_ref(), e)
			})
			.or_else(|_| Self::load_default_config())
	}

	fn load_from_file(path: impl AsRef<str>) -> Result<Self, ConfigError> {
		let config = Config::builder().add_source(File::with_name(path.as_ref())).build()?;
		Ok(config.try_deserialize::<AppConfig>()?)
	}

	fn load_default_config() -> Result<Self, ConfigError> {
		Self::load_from_file(DEFAULE_CONFIG_PATH)
			.inspect_err(|e| error!("Failed to load default config: {}", e))
	}
}


#[derive(Debug, Clone, Serialize)]
pub struct AllConfig { //需要同步到数据表格中
	pub agent_name: String,             // 采集器名称
	pub agent_lcuuid: String,           // 采集器ID
    // 资源限制
    pub cpu_core_limit: u32,                  // CPU限制(Core)
    pub cpu_millicore_limit: u32,             // CPU限制(MilliCore)
    pub memory_limit_mb: u32,                 // 内存限制(M)
    pub system_idle_mem_percent: u32,         // 系统空闲内存限制(%)
    pub packet_limit_kpps: u32,               // 采集包限速(Kpps)
    pub dispatch_limit_mbps: u32,             // 分发流限速(Mbps)
    pub system_load_fuse_threshold: f64,      // 系统负载熔断阈值
    pub system_load_fuse_recover: f64,        // 系统负载熔断恢复
    pub system_load_fuse_metric: String,      // 系统负载熔断指标
    pub dispatch_fuse_threshold_mbps: u32,    // 分发熔断阈值(Mbps)
    pub dispatch_fuse_monitor_interval: u32,  // 分发熔断监控间隔(秒)
    pub log_send_rate_per_hour: u32,          // 日志发送速率(条/小时)
    pub log_level: String,                    // 日志打印等级
    pub log_file_size_mb: u32,                // 日志文件大小(M字节)
    pub thread_limit: u32,                    // 线程数限制(个)
    pub process_limit: u32,                   // 进程数限制(个)

    // 基础配置
    pub collect_nic_regex: String,            // 采集网口(正则)
    pub traffic_filter: String,               // 流量过滤
    pub collect_netns: Option<String>,        // 采集NETNS
    pub packet_length: u32,                   // 采集包长(字节)
    pub traffic_mode: u32,                    // 流量采集模式
    pub traffic_api: String,                  // 流量采集API
    pub tunnel_decap_type: String,            // 隧道解封装类型
    pub vm_mac_parse: String,                 // 虚拟机MAC解析
    pub vm_xml_folder: String,                // 虚拟机XML文件夹
    pub resource_report_interval: u32,        // 资源上报间隔(秒)
    pub config_pull_interval: u32,            // 配置拉取间隔(秒)
    pub max_escape_time: u32,                 // 最长逃逸时间(秒)
    pub udp_max_mtu: u32,                     // UDP最大MTU(字节)
    pub bare_udp_vlan: u32,                   // 裸UDP外层VLAN
    pub request_nat_ip: bool,                 // 是否请求NAT IP
    pub log_store_days: u32,                  // 日志存储时长(天)
    pub controller_port: u32,                 // 控制器通信端口
    pub analyzer_port: u32,                   // 数据节点通信端口
    pub controller_ip: Option<String>,        // 控制器IP
    pub analyzer_ip: Option<String>,          // 数据节点IP

    // 全景图配置
    pub data_socket: String,                  // 数据套接字
    pub pcap_socket: String,                  // PCAP套接字
    pub http_log_proxy_client: String,        // HTTP日志代理客户端
    pub http_log_xrequestid: String,          // HTTP日志XRequestID
    pub app_flow_log_traceid: String,         // 应用流日志TraceID
    pub app_flow_log_spanid: String,          // 应用流日志SpanID
    pub app_log_parse_length: u32,            // 应用日志解析包长(字节)
    pub flow_log_collect_rate: u32,           // 流日志采集速率(每秒)
    pub app_log_collect_rate: u32,            // 应用日志采集速率(每秒)
    pub data_integration_service: bool,       // 数据集成服务
    pub data_integration_port: u32,           // 数据集成端口

    // 包分发配置
    pub dispatch_socket: String,              // 分发套接字
    pub inner_additional_header: String,      // 内层附加头

    // 基础功能开关
    pub sync_resource_info: bool,             // 同步资源信息
    pub log_send: bool,                       // 日志发送
    pub clock_sync: bool,                     // 时钟同步
    pub cloud_resource_down: String,          // 云平台资源信息下发
    pub container_cluster_inner_ip_down: String, // 容器集群内部IP下发

    // 全景图功能开关
    pub metric_data: bool,                    // 指标数据
    pub inactive_port_metric_data: bool,      // 非活跃端口指标数据
    pub inactive_ip_metric_data: bool,        // 非活跃IP指标数据
    pub net_perf_metric_data: bool,           // 网络性能指标数据
    pub app_perf_metric_data: bool,           // 应用性能指标数据
    pub second_metric_data: bool,             // 秒粒度指标数据
    pub flow_log_enable_net_pos: String,      // 流日志开启网络位置
    pub app_log_enable_net_pos: String,       // 应用日志开启网络位置
    pub call_log_ignore_point: String,        // 调用日志忽略观测点
    pub flow_log_ignore_point: String,        // 流日志忽略观测点

    // 插件
    pub wasm_plugins: Vec<String>,            // Wasm 插件
    pub so_plugins: Vec<String>,              // so 插件

    // 包分发功能开关
    pub global_dedup: bool,                   // 全局去重
}


impl AllConfig {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
		let config = agent_config();
		let app_config = config.load(); // 这里 config 应该是 Arc<ArcSwap<AppConfig>>
		let agent_name = app_config.name.clone();
		let mut hasher = Sha256::new();
        hasher.update(agent_name.as_bytes());
        let agent_lcuuid = format!("{:x}", hasher.finalize());

        Self {
            // 资源限制
			agent_name,
			agent_lcuuid,
            cpu_core_limit: rng.gen_range(1..=16),
            cpu_millicore_limit: rng.gen_range(100..=16000),
            memory_limit_mb: rng.gen_range(512..=32768),
            system_idle_mem_percent: rng.gen_range(0..=100),
            packet_limit_kpps: rng.gen_range(10..=10000),
            dispatch_limit_mbps: rng.gen_range(100..=10000),
            system_load_fuse_threshold: rng.gen_range(0.5..=5.0),
            system_load_fuse_recover: rng.gen_range(0.1..=4.0),
            system_load_fuse_metric: ["load1", "load5", "load15"].choose(&mut rng).unwrap().to_string(),
            dispatch_fuse_threshold_mbps: rng.gen_range(0..=10000),
            dispatch_fuse_monitor_interval: rng.gen_range(1..=60),
            log_send_rate_per_hour: rng.gen_range(10..=10000),
            log_level: ["INFO", "DEBUG", "WARN", "ERROR"].choose(&mut rng).unwrap().to_string(),
            log_file_size_mb: rng.gen_range(10..=5000),
            thread_limit: rng.gen_range(1..=1000),
            process_limit: rng.gen_range(1..=100),

            // 基础配置
            collect_nic_regex: "^eth.*$".to_string(),
            traffic_filter: ["全采集", "过滤"].choose(&mut rng).unwrap().to_string(),
            collect_netns: None,
            packet_length: rng.gen_range(64..=65535),
            traffic_mode: rng.gen_range(0..=2),
            traffic_api: ["自适应", "固定"].choose(&mut rng).unwrap().to_string(),
            tunnel_decap_type: ["VXLAN", "IPIP"].choose(&mut rng).unwrap().to_string(),
            vm_mac_parse: ["接口MAC", "虚拟MAC"].choose(&mut rng).unwrap().to_string(),
            vm_xml_folder: "/etc/libvirt/qemu/".to_string(),
            resource_report_interval: rng.gen_range(1..=60),
            config_pull_interval: rng.gen_range(10..=600),
            max_escape_time: rng.gen_range(60..=7200),
            udp_max_mtu: rng.gen_range(512..=9000),
            bare_udp_vlan: rng.gen_range(0..=4095),
            request_nat_ip: rng.gen_bool(0.5),
            log_store_days: rng.gen_range(1..=365),
            controller_port: rng.gen_range(1024..=65535),
            analyzer_port: rng.gen_range(1024..=65535),
            controller_ip: Some("192.168.1.1".to_string()),
            analyzer_ip: Some("192.168.1.2".to_string()),

            // 全景图配置
            data_socket: ["TCP", "UDP"].choose(&mut rng).unwrap().to_string(),
            pcap_socket: ["TCP", "UDP"].choose(&mut rng).unwrap().to_string(),
            http_log_proxy_client: "X-Forwarded-For".to_string(),
            http_log_xrequestid: "X-Request-ID".to_string(),
            app_flow_log_traceid: "traceparent".to_string(),
            app_flow_log_spanid: "sw8".to_string(),
            app_log_parse_length: rng.gen_range(1024..=65535),
            flow_log_collect_rate: rng.gen_range(100..=100000),
            app_log_collect_rate: rng.gen_range(100..=100000),
            data_integration_service: rng.gen_bool(0.5),
            data_integration_port: rng.gen_range(1024..=65535),

            // 包分发配置
            dispatch_socket: ["裸UDP", "TCP"].choose(&mut rng).unwrap().to_string(),
            inner_additional_header: ["无", "有"].choose(&mut rng).unwrap().to_string(),

            // 基础功能开关
            sync_resource_info: rng.gen_bool(0.5),
            log_send: rng.gen_bool(0.5),
            clock_sync: rng.gen_bool(0.5),
            cloud_resource_down: ["全部", "部分"].choose(&mut rng).unwrap().to_string(),
            container_cluster_inner_ip_down: ["所有集群", "部分集群"].choose(&mut rng).unwrap().to_string(),

            // 全景图功能开关
            metric_data: rng.gen_bool(0.5),
            inactive_port_metric_data: rng.gen_bool(0.5),
            inactive_ip_metric_data: rng.gen_bool(0.5),
            net_perf_metric_data: rng.gen_bool(0.5),
            app_perf_metric_data: rng.gen_bool(0.5),
            second_metric_data: rng.gen_bool(0.5),
            flow_log_enable_net_pos: ["全部", "部分"].choose(&mut rng).unwrap().to_string(),
            app_log_enable_net_pos: ["全部", "部分"].choose(&mut rng).unwrap().to_string(),
            call_log_ignore_point: ["其他网卡", "无"].choose(&mut rng).unwrap().to_string(),
            flow_log_ignore_point: ["其他网卡", "无"].choose(&mut rng).unwrap().to_string(),

            // 插件
            wasm_plugins: vec!["plugin1.wasm".to_string(), "plugin2.wasm".to_string()],
            so_plugins: vec!["plugin1.so".to_string(), "plugin2.so".to_string()],

            // 包分发功能开关
            global_dedup: rng.gen_bool(0.5),
        }
    }
}

pub async fn sync_config(client: &elasticsearch::Elasticsearch) {
    let es_index_name = "agent_config";

    // 随机生成配置
    let config = AllConfig::random();
    let config_id = config.agent_lcuuid.clone(); // 用 agent_lcuuid 作为索引

    let resp = client
        .index(
            elasticsearch::IndexParts::IndexId(es_index_name, &config_id),
        )
        .body(serde_json::json!(config))
        .send()
        .await;

    match resp {
        Ok(r) if r.status_code().is_success() => {
            log::info!("Config synced to ES successfully, id: {}", config_id);
        }
        Ok(r) => {
            log::error!("Failed to sync config, status: {}", r.status_code());
        }
        Err(e) => {
            log::error!("Failed to sync config: {:?}", e);
        }
    }
}