use k8s_openapi::api::core::v1::Pod;
use kube::{Api, Client, api::ListParams};
use log::warn;
use std::{error::Error, process::Command};

const FILTERED: &[&str] = &["redis", "mongodb", "memcached", "jaeger"];
pub async fn get_pids() -> Result<Vec<u32>, Box<dyn Error>> {
	let mut pids = Vec::with_capacity(32);

	let namespace = "default";
	let _label_selector = "app in (service2,service3)";

	let client = Client::try_default().await?;
	let pods: Api<Pod> = Api::namespaced(client.clone(), namespace);

	// let lp = ListParams::default().labels(label_selector);
	let lp = ListParams::default();
	let pod_list = pods.list(&lp).await?;

	for pod in pod_list.items {
		if let Some(pod_name) = pod.metadata.name.clone() {
			if FILTERED.iter().any(|black_list| pod_name.contains(black_list)) {
				continue;
			}
			println!("Processing Pod: {}", pod_name);

			if let Some(status) = pod.status {
				if let Some(container_statuses) = status.container_statuses {
					for container_status in container_statuses {
						let container_name = container_status.name;
						let container_id =
							container_status.container_id.unwrap_or_else(|| "Unknown".to_string());

						// println!("  Container: {}", container_name);
						// println!("  Container ID: {}", container_id);

						if let Some(pid) = get_pid_from_container_runtime(&container_id) {
							pids.push(
								u32::from_str_radix(&pid, 10)
									.expect("Parse pid error: invalid pid number"),
							);
							println!("{} 0", pid);
						} else {
							warn!("Failed to retrieve PID for container: {}", container_name);
						}
					}
				}
			}
		}
	}

	Ok(pids)
}
fn get_pid_from_container_runtime(container_id: &str) -> Option<String> {
	if container_id.starts_with("containerd://") {
		let runtime_id = container_id.strip_prefix("containerd://").unwrap();
		return run_crictl_command(runtime_id);
	} else if container_id.starts_with("docker://") {
		let runtime_id = container_id.strip_prefix("docker://").unwrap();
		return run_docker_command(runtime_id);
	}
	None
}
fn run_crictl_command(runtime_id: &str) -> Option<String> {
	let output = Command::new("crictl")
		.arg("inspect")
		.arg("--output")
		.arg("go-template")
		.arg("--template")
		.arg("{{.info.pid}}")
		.arg(runtime_id)
		.output();

	if let Ok(output) = output {
		if output.status.success() {
			let pid = String::from_utf8_lossy(&output.stdout).trim().to_string();
			return Some(pid);
		}
	}
	None
}
fn run_docker_command(runtime_id: &str) -> Option<String> {
	let output = Command::new("docker")
		.arg("inspect")
		.arg("--format")
		.arg("{{.State.Pid}}")
		.arg(runtime_id)
		.output();

	if let Ok(output) = output {
		if output.status.success() {
			let pid = String::from_utf8_lossy(&output.stdout).trim().to_string();
			return Some(pid);
		}
	}
	None
}

// pub fn handle_data(data: Data) -> anyhow::Result<Option<()>> {
//     // print!("data: {}", data);
//     return Ok(Some(()));
//     let len = std::cmp::min(MAX_BUF_SIZE, data.len) as usize;
//     let start = match data.direction {
//         SyscallType::Ingress => 0,
//         SyscallType::Egress => 4,
//     };
//     let mut buf = &data.buf[start..len];
//     // let mut buf = &[128, 1, 0, 2, 0, 0, 0, 11, 67, 111, 109, 112, 111, 115, 101, 84, 101, 120, 116, 0, 0, 0, 0, 12, 0, 0, 11, 0, 1, 0, 0, 1, 126, 103, 99, 57, 84, 102, 72, 88, 71, 77, 49, 90, 115, 100, 99, 108, 122, 48, 54, 65, 116, 76, 82, 50, 81, 83, 89, 109, 120, 102, 106, 120, 112, 75, 81, 75, 90, 80, 65, 118, 112, 114, 119, 107, 115, 119, 51, 72, 70, 108, 77, 50, 99, 121, 98, 72, 82, 112, 109, 85, 117, 52, 112, 90, 69, 105, 121, 100, 114, 51, 99, 77, 107, 110, 112, 78, 51, 49, 75, 104, 120, 79, 99, 102, 104, 76, 97, 78, 114, 54, 73, 110, 72, 105, 99, 99, 102, 84, 105, 49, 87, 102, 107, 68, 109, 120, 66, 97, 116, 72, 116, 88, 88, 52, 111, 116, 69, 108, 103, 57, 88, 100, 75, 73, 69, 57, 113, 89, 85, 114, 65, 82, 104, 72, 50, 69, 72, 83, 56, 75, 107, 56, 108, 55, 77, 68, 74, 70, 121, 82, 57, 70, 102, 110, 65, 77, 79, 102, 98, 50, 56, 81, 69, 121, 78, 116, 75, 55, 50, 118, 75, 49, 105, 84, 101, 101, 49, 101, 106, 76, 71, 77, 84, 114, 79, 113, 80, 51, 98, 115, 78, 71, 55, 110, 110, 57, 100, 50, 104, 51, 102, 99, 122, 73, 101, 118, 116, 104, 85, 77, 82, 54, 66, 65, 118, 82, 114, 115, 107, 108, 99, 108, 87, 81, 88, 97, 70, 81, 105, 66, 106, 107, 75, 98, 97, 52, 116, 111, 90, 110, 79, 74, 73, 57, 87, 110, 115, 105, 101, 107, 49, 86, 121, 116, 72, 72, 80, 32, 64, 117, 115, 101, 114, 110, 97, 109, 101, 95, 51, 56, 56, 32, 64, 117, 115, 101, 114, 110, 97, 109, 101, 95, 51, 53, 57, 32, 64, 117, 115, 101, 114, 110, 97, 109, 101, 95, 54, 50, 48, 32, 64, 117, 115, 101, 114, 110, 97, 109, 101, 95, 52, 54, 53, 32, 64, 117, 115, 101, 114, 110, 97, 109, 101, 95, 55, 54, 51, 32, 104, 116, 116, 112, 58, 47, 47, 115, 104, 111, 114, 116, 45, 117, 114, 108, 47, 68, 107, 72, 111, 55, 103, 56, 83, 69, 81, 32, 104, 116, 116, 112, 58, 47, 47, 115, 104, 111, 114, 116, 45, 117, 114, 108, 47, 98, 88, 53, 84, 56, 81, 78, 83, 82, 76, 15, 0, 2, 12, 0, 0, 0, 0, 15, 0, 3, 12, 0, 0, 0, 2, 11, 0, 1, 0, 0, 0, 27, 104, 116, 116, 112, 58, 47, 47, 115, 104, 111, 114, 116, 45, 117, 114, 108, 47, 68, 107, 72, 111, 55, 103, 56, 83, 69, 81, 11, 0, 2, 0, 0, 0, 71, 104, 116, 116, 112, 58, 47, 47, 71, 98, 57, 85, 118, 68, 89, 100, 85, 119, 65, 105, 49, 69, 73, 113, 106, 118, 69, 77, 116, 51, 120, 90, 57, 109, 108, 120, 53, 52, 48, 118, 84, 108, 55, 70, 66, 74, 105, 102, 76, 105, 97, 116, 56, 112, 70, 52, 85, 49, 73, 50, 51, 105, 114, 53, 115, 75, 56, 70, 85, 84, 81, 51, 0, 11, 0, 1, 0, 0, 0, 27, 104, 116, 116, 112, 58, 47, 47, 115, 104, 111, 114, 116, 45, 117, 114, 108, 47, 98, 88, 53, 84, 56, 81, 78, 83, 82, 76, 11, 0, 2, 0, 0, 0, 71, 104, 116, 116, 112, 58, 47, 47, 98, 104, 116, 89, 89, 50, 68, 102, 90, 113, 49, 117, 54, 66, 57, 65, 122, 57, 89, 50, 107, 86, 116, 107, 97, 117, 122, 117, 72, 102, 50, 56, 115, 120, 71, 90, 98, 75, 112, 110, 112, 104, 83, 81, 115, 72, 77, 85, 55, 66, 75, 50, 78, 57, 116, 121, 107, 77, 66, 71, 78, 97, 78, 107, 0, 0, 0][..];
//     if buf[0] != 128 {
//         return Ok(None);
//     }
//     let decoded_message = thrift_codec::message::Message::binary_decode(&mut buf)?;
//     if decoded_message.method_name() != "ComposePost" {
//         return Ok(None);
//     }
//     println!(
//         "tgid: {:?}, Length: {}, Time: {:?}, Command: {:?}, Direction: {:?}, Data: {:?}",
//         data.tgid,
//         data.len,
//         data.timestamp_ns,
//         // Convert 'cmd' and 'buf' fields to strings for display.
//         // 'String::from_utf8_lossy' will replace invalid UTF-8 sequences with U+FFFD REPLACEMENT CHARACTER.
//         String::from_utf8_lossy(&data.comm),
//         data.direction,
//         // String::from_utf8_lossy(&data.buf[..len]),
//         decoded_message,
//     );
//     // println!("data: {}", buf.iter().map(|x| format!("{}, ", x)).collect::<String>());
//     Ok(Some(()))
// }
// // `#[warn(static_mut_refs)]`
