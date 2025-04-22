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
