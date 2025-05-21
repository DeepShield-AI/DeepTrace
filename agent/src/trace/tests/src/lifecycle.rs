use anyhow::anyhow;
use std::{env, time::Duration};
use tokio::{
	process::{Child, Command},
	time,
};
pub async fn start() -> anyhow::Result<Child> {
	let output = Command::new("docker")
		.args(&["ps", "--quiet", "--filter", "name=test_workload"])
		.output()
		.await?;

	if !output.status.success() {
		let error_message = String::from_utf8_lossy(&output.stderr);
		eprintln!("Failed to execute 'docker ps': {}", error_message);
		return Err(anyhow!("Failed to execute 'docker ps'"));
	}

	let container_ids = String::from_utf8(output.stdout)?
		.lines()
		.map(|s| s.trim().to_string())
		.filter(|s| !s.is_empty())
		.collect::<Vec<String>>();

	let mut pids = Vec::new();
	for id in container_ids {
		let output = Command::new("docker")
			.args(&["inspect", "--format", "{{.State.Pid}}", &id])
			.output()
			.await?;

		if !output.status.success() {
			let error_message = String::from_utf8_lossy(&output.stderr);
			eprintln!("Failed to inspect container {}: {}", id, error_message);
			continue;
		}

		let pid_str = String::from_utf8(output.stdout)?;
		let pid = pid_str.trim();
		if pid.is_empty() {
			eprintln!("No PID found for container {}", id);
			continue;
		}

		pids.push(pid.to_string());
	}

	let manifest = env::current_dir()?
		.parent()
		.ok_or(anyhow!("Can't get trace directory"))?
		.parent()
		.ok_or(anyhow!("Can't get agent directory"))?
		.parent()
		.ok_or(anyhow!("Can't get DeepTrace directory"))?
		.display()
		.to_string();
	println!("Manifest directory: {}", manifest);
	let deeptrace = Command::new("bash")
		.current_dir(&manifest)
		.env("RUST_LOG", "info")
		.args(&[
			"-c",
			"./target/release/agent",
			// &format!("{}/Cargo.toml", manifest),
			// "--release",
			// "--config",
			// r#"target."cfg(all())".runner="sudo -E""#,
			"--",
			"--pids",
			&pids.join(","),
		])
		.spawn()?;
	println!("Program started with PID: {:?}", deeptrace.id());
	time::sleep(Duration::from_secs(10)).await;

	Ok(deeptrace)
}

pub async fn stop(mut runner: Child) -> anyhow::Result<()> {
	runner.kill().await?;
	time::sleep(Duration::from_secs(1)).await;
	runner.wait().await?;
	Ok(())
}
