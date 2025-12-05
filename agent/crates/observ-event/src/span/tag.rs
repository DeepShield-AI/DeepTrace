use arc_swap::access::Access;
use bollard::{
	Docker,
	query_parameters::{InspectContainerOptions, ListContainersOptions},
};
use observ_config::agent_config;
use observ_trace_common::{Direction, L7Protocol, Message};
use serde::Serialize;
use std::{cell::RefCell, collections::HashMap, ffi::CStr, process::Command};

thread_local! {
	static TGID_DOCKER_MAP: RefCell<HashMap<u32, DockerTag>> = RefCell::new(HashMap::new());
}

thread_local! {
    static TGID_K8S_MAP: RefCell<HashMap<u32, K8sTag>> = RefCell::new(HashMap::new());
}

#[derive(Serialize, Clone)]
pub(in crate::span) struct DockerTag {
	pub container_id: String,
	pub container_name: String,
	pub image: String,
	pub hostname: String,
	pub gateway: String,
	pub tgid: u32,
	pub ip: String,
	pub network_mode: String,
	pub created: String,
}

impl DockerTag {
	pub async fn get_docker_tags(tgid: u32) -> Option<DockerTag> {
		if let Some(tag) = TGID_DOCKER_MAP.with(|map_cell| map_cell.borrow().get(&tgid).cloned()) {
			return Some(tag);
		}

		let docker = Docker::connect_with_local_defaults().ok()?;
		let containers = docker
			.list_containers(Some(ListContainersOptions { all: true, ..Default::default() }))
			.await
			.ok()?;

		for container in containers {
			let id = container.id.clone().unwrap_or_default();
			let inspect =
				docker.inspect_container(&id, None::<InspectContainerOptions>).await.ok()?;
			let ip_address = inspect
				.network_settings
				.as_ref()
				.and_then(|ns| ns.networks.as_ref())
				.and_then(|networks| {
					networks.values().next().and_then(|net| net.ip_address.clone())
				})
				.unwrap_or_default();

			let network_mode = inspect
				.host_config
				.as_ref()
				.and_then(|hc| hc.network_mode.clone())
				.unwrap_or_default();

			let created = inspect.created.clone().unwrap_or_default();

			let output = Command::new("docker")
				.arg("top")
				.arg(&id)
				.output()
				.expect("failed to execute docker top");
			let stdout = String::from_utf8_lossy(&output.stdout);
			let mut pids = Vec::new();
			for line in stdout.lines().skip(1) {
				let fields: Vec<&str> = line.split_whitespace().collect();
				if fields.len() > 1 {
					if let Ok(pid) = fields[1].parse::<i64>() {
						pids.push(pid);
					}
				}
			}

			let mut tag = DockerTag {
				tgid,
				container_id: id,
				container_name: container
					.names
					.clone()
					.unwrap_or_default()
					.get(0)
					.cloned()
					.map_or(String::new(), |n| n),
				image: container.image.unwrap_or_default(),
				hostname: inspect
					.config
					.as_ref()
					.and_then(|c| c.hostname.clone())
					.unwrap_or_default(),
				gateway: inspect
					.network_settings
					.as_ref()
					.and_then(|ns| ns.networks.as_ref())
					.and_then(|networks| {
						networks.values().next().and_then(|net| net.gateway.clone())
					})
					.unwrap_or_default(),
				ip: ip_address,
				network_mode,
				created,
			};

			for pid in pids {
				tag.tgid = pid as u32;
				TGID_DOCKER_MAP.with(|map_cell| {
					map_cell.borrow_mut().insert(pid as u32, tag.clone());
				});
			}
		}

		if let Some(tag) = TGID_DOCKER_MAP.with(|map_cell| map_cell.borrow().get(&tgid).cloned()) {
			return Some(tag);
		}
		None
	}
}

#[derive(Serialize, Clone, Debug)]
pub struct K8sTag {
    pub tgid: u32,
    pub name: String,
    pub state: String,
    pub created_at: String,
    pub image: String,
    pub namespace: String,
    pub cpu_period: String,
    pub cpu_shares: String,
}

impl K8sTag {
    pub async fn get_k8s_tags(tgid: u32) -> Option<K8sTag> {
        // 先查缓存
        if let Some(tag) = TGID_K8S_MAP.with(|map_cell| map_cell.borrow().get(&tgid).cloned()) {
            return Some(tag);
        }

        // 获取所有容器ID
        let output = Command::new("crictl")
            .arg("ps")
            .arg("-q")
            .output()
            .ok()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let container_ids: Vec<&str> = stdout.lines().collect();

        for container_id in container_ids {
            let inspect_output = Command::new("crictl")
                .arg("inspect")
                .arg(container_id)
                .output()
                .ok()?;
            let inspect_stdout = String::from_utf8_lossy(&inspect_output.stdout);
            let inspect_json: serde_json::Value = serde_json::from_str(&inspect_stdout).ok()?;

            let pid = inspect_json.get("info")
                .and_then(|info| info.get("pid"))
                .and_then(|pid| pid.as_u64());

            if let Some(pid) = pid {
                if pid as u32 != tgid {
                    continue;
                }

                let name = inspect_json.get("status")
                    .and_then(|s| s.get("metadata"))
                    .and_then(|m| m.get("name"))
                    .and_then(|n| n.as_str())
                    .unwrap_or_default()
                    .to_string();

                let state = inspect_json.get("status")
                    .and_then(|s| s.get("state"))
                    .and_then(|state| state.as_str())
                    .unwrap_or_default()
                    .to_string();

                let created_at = inspect_json.get("status")
                    .and_then(|s| s.get("createdAt"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_default()
                    .to_string();

                let image = inspect_json.get("status")
                    .and_then(|s| s.get("image"))
                    .and_then(|img| img.get("image"))
                    .and_then(|i| i.as_str())
                    .unwrap_or_default()
                    .to_string();

                let namespace = inspect_json.get("status")
                    .and_then(|s| s.get("labels"))
                    .and_then(|labels| labels.get("io.kubernetes.pod.namespace"))
                    .and_then(|ns| ns.as_str())
                    .unwrap_or_default()
                    .to_string();

                let cpu_period = inspect_json.get("info")
                    .and_then(|info| info.get("config"))
                    .and_then(|config| config.get("linux"))
                    .and_then(|linux| linux.get("resources"))
                    .and_then(|res| res.get("cpu_period"))
                    .map(|v| v.to_string())
                    .unwrap_or_default();

                let cpu_shares = inspect_json.get("info")
                    .and_then(|info| info.get("config"))
                    .and_then(|config| config.get("linux"))
                    .and_then(|linux| linux.get("resources"))
                    .and_then(|res| res.get("cpu_shares"))
                    .map(|v| v.to_string())
                    .unwrap_or_default();

                let tag = K8sTag {
                    tgid: pid as u32,
                    name,
                    state,
                    created_at,
                    image,
                    namespace,
                    cpu_period,
                    cpu_shares,
                };

                TGID_K8S_MAP.with(|map_cell| {
                    map_cell.borrow_mut().insert(pid as u32, tag.clone());
                });

                return Some(tag);
            }
        }

        None
    }
}

#[derive(Serialize, Clone)]
pub struct EbpfTag {
	pub tgid: u32,
	pub pid: u32,
	pub component: String,
	pub direction: Direction,
	pub protocol: L7Protocol,
	pub src_ip: String,
	pub dst_ip: String,
	pub src_port: u16,
	pub dst_port: u16,
	pub req_seq: u32,
	pub resp_seq: u32,
}

#[derive(Serialize)]
pub(in crate::span) struct SpanTag {
	pub ebpf_tag: EbpfTag,
	// docker tags
	pub docker_tag: Option<DockerTag>,
	// k8s tags
	pub k8s_tag: Option<K8sTag>,
	// other tags
	pub other_tags: HashMap<String, String>,
}

fn u32_to_ip(ip: u32) -> String {
	use std::net::Ipv4Addr;
	Ipv4Addr::from(ip).to_string()
}

impl SpanTag {
	pub async fn set_tags(req: &Message, resp: &Message) -> Self {
		let (src_ip, dst_ip, src_port, dst_port) = match req.direction {
			Direction::Egress => (
				req.quintuple.dst_addr,
				req.quintuple.src_addr,
				req.quintuple.dst_port,
				req.quintuple.src_port,
			),
			_ => (
				req.quintuple.src_addr,
				req.quintuple.dst_addr,
				req.quintuple.src_port,
				req.quintuple.dst_port,
			),
		};

		let ebpf_tag = EbpfTag {
			tgid: req.tgid,
			pid: req.pid,
			component: CStr::from_bytes_until_nul(req.comm.as_slice())
				.unwrap()
				.to_string_lossy()
				.into_owned(),
			direction: req.direction,
			protocol: req.protocol,
			src_ip: u32_to_ip(src_ip),
			dst_ip: u32_to_ip(dst_ip),
			src_port,
			dst_port,
			req_seq: req.seq,
			resp_seq: resp.seq,
		};

		let docker_tag = DockerTag::get_docker_tags(req.tgid).await;
		let k8s_tag = K8sTag::get_k8s_tags(req.tgid).await;
		let mut other_tags = HashMap::new();
		let agent_config = agent_config().load();
		other_tags.insert("user".to_string(), agent_config.user.clone());
		SpanTag { ebpf_tag, docker_tag, k8s_tag, other_tags }
	}
}
