use super::CpuMetric;
use log::{info, warn};
use std::{
	fs::File,
	io::{BufRead, BufReader},
	sync::{
		Arc,
		atomic::{AtomicBool, Ordering},
	},
	time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::time;

pub struct CpuCollector {
	/// Sampling rate
	interval: Duration,
}

fn read_load_avg() -> f64 {
	let file = match File::open("/proc/loadavg") {
		Ok(f) => f,
		Err(e) => {
			warn!("Failed to open /proc/loadavg: {}", e);
			return 0.0;
		},
	};
	let reader = BufReader::new(file);
	for line in reader.lines().filter_map(|r| r.ok()) {
		let parts: Vec<&str> = line.split_whitespace().collect();
		if parts.len() >= 3 {
			if let Ok(load) = parts[0].parse::<f64>() {
				return load;
			}
		}
	}
	0.0
}

impl CpuCollector {
	pub fn new() -> Self {
		Self {
			// Default sampling interval of 1 second
			interval: Duration::from_secs(1),
		}
	}

	/// Optional: Set sampling interval
	pub fn with_interval(mut self, interval: Duration) -> Self {
		self.interval = interval;
		self
	}
	// TODO: can this be async?
	/// Collect CPU usage
	pub fn collect(&self) -> Vec<CpuMetric> {
		let file = match File::open("/proc/stat") {
			Ok(f) => f,
			Err(e) => {
				warn!("Failed to open /proc/stat: {}", e);
				return vec![];
			},
		};
		let reader = BufReader::new(file);
		let mut results = vec![];
		let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
		let cpu_load = read_load_avg();
		for line in reader.lines().filter_map(|r| r.ok()) {
			if line.starts_with("cpu") {
				let parts: Vec<&str> = line.split_whitespace().collect();
				// TODO: why 5? what does it mean?
				// TODO: rewrite use nom, define a struct for cpu usage detail like CPUStats
				if parts.len() >= 5 && parts[0] != "cpu" {
					if let Ok(cpu_id) = parts[0][3..].parse::<usize>() {
						let user = parts[1].parse::<u64>().unwrap_or(0);
						let nice = parts[2].parse::<u64>().unwrap_or(0);
						let system = parts[3].parse::<u64>().unwrap_or(0);
						let idle = parts[4].parse::<u64>().unwrap_or(0);

						let total = user + nice + system + idle;
						let cpu_usage = if total > 0 {
							(user + nice + system) as f64 / total as f64 * 100.0
						} else {
							0.0
						};

						let user_usage =
							if total > 0 { user as f64 / total as f64 * 100.0 } else { 0.0 };
						let nice_usage =
							if total > 0 { nice as f64 / total as f64 * 100.0 } else { 0.0 };
						let system_usage =
							if total > 0 { system as f64 / total as f64 * 100.0 } else { 0.0 };
						let idle_usage =
							if total > 0 { idle as f64 / total as f64 * 100.0 } else { 0.0 };

						results.push(CpuMetric {
							cpu_id,
							cpu_load,
							cpu_usage,
							user,
							user_usage,
							nice,
							nice_usage,
							system,
							system_usage,
							idle,
							idle_usage,
							// total_time: total,
							timestamp: current_time,
						});
					}
				}
			}
		}

		results
	}

	// Sleep for the specified interval
	pub async fn sleep_duration(&self) {
		time::sleep(self.interval).await;
	}
}
