use bollard::{Docker, container::ListContainersOptions};
use serde::Serialize;
use std::{cell::RefCell, collections::HashMap, ffi::CStr};
use trace_common::{
	protocols::L7Protocol,
	structs::{Data, Direction},
};

thread_local! {
	static TGID_DOCKER_MAP: RefCell<HashMap<u32, DockerTag>> = RefCell::new(HashMap::new());
}

#[derive(Serialize, Clone, Debug)]
pub struct DockerTag {
	pub container_id: String,
	pub container_name: Vec<String>,
	pub image: String,
	pub hostname: String,
	pub gateway: String,
	pub tgid: u32,
	pub ip: String,
	pub network_mode: String,
	pub created: String,
}

impl DockerTag {
	/// 通过 tgid 获取 DockerTag，未命中则查 docker 并插入，再查一次缓存
	pub async fn get_docker_tags(tgid: u32) -> Option<DockerTag> {
		// 第一次查缓存
		if let Some(tag) = TGID_DOCKER_MAP.with(|map_cell| map_cell.borrow().get(&tgid).cloned()) {
			return Some(tag);
		}

		// 缓存未命中，查 docker
		let docker = Docker::connect_with_local_defaults().ok()?;
		let containers = docker
			.list_containers(Some(ListContainersOptions::<String> {
				all: true,
				..Default::default()
			}))
			.await
			.ok()?;

		for container in containers {
			let id = container.id.clone().unwrap_or_default();
			let inspect = docker.inspect_container(&id, None).await.ok()?;
			let docker_tgid = inspect.state.as_ref().and_then(|s| s.pid).unwrap_or(0) as u32;
			// 获取 IPAddress
			let ip_address = inspect
				.network_settings
				.as_ref()
				.and_then(|ns| ns.networks.as_ref())
				.and_then(|networks| {
					networks.values().next().and_then(|net| net.ip_address.clone())
				})
				.unwrap_or_default();

			// 获取 NetworkMode
			let network_mode = inspect
				.host_config
				.as_ref()
				.and_then(|hc| hc.network_mode.clone())
				.unwrap_or_default();

			// 获取 Created 时间
			let created = inspect.created.clone().unwrap_or_default();

			let tag = DockerTag {
				tgid: docker_tgid,
				container_id: id,
				container_name: container.names.unwrap_or_default(),
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
			TGID_DOCKER_MAP.with(|map_cell| {
				map_cell.borrow_mut().insert(docker_tgid, tag.clone());
			});
		}

		// 插入后再查一次缓存
		if let Some(tag) = TGID_DOCKER_MAP.with(|map_cell| map_cell.borrow().get(&tgid).cloned()) {
			return Some(tag);
		}
		None
	}
}

#[derive(Serialize, Clone, Debug)]
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
pub struct SpanTag {
	pub ebpf_tag: EbpfTag,
	// docker tags
	pub docker_tag: Option<DockerTag>,
}

pub fn u32_to_ip(ip: u32) -> String {
	use std::net::Ipv4Addr;
	Ipv4Addr::from(ip).to_string()
}

impl SpanTag {
	pub async fn set_tags(req: &Data, resp: &Data) -> Self {
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
			component: CStr::from_bytes_until_nul(&req.comm)
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

		SpanTag { ebpf_tag, docker_tag }
	}
}
