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
fn read_page_faults() -> u64 {
	let file = match File::open("/proc/vmstat") {
		Ok(f) => f,
		Err(e) => {
			warn!("Failed to open /proc/vmstat: {}", e);
			return 0;
		},
	};
	let reader = BufReader::new(file);

	for line in reader.lines().filter_map(|r| r.ok()) {
		let parts: Vec<&str> = line.split_whitespace().collect();
		if parts.len() >= 2 && parts[0] == "pgfault" {
			if let Ok(page_faults) = parts[1].parse::<u64>() {
				return page_faults;
			}
		}
	}

	0
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
		println!("Collecting CPU usage...");
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
		let mut context_switches = 0u64;
		let mut page_faults = 0u64;
		let lines: Vec<String> = reader.lines().filter_map(|r| r.ok()).collect();
		for line in &lines {
			if line.starts_with("ctxt") {
				let parts: Vec<&str> = line.split_whitespace().collect();
				if parts.len() >= 2 {
					if let Ok(ctxt) = parts[1].parse::<u64>() {
						context_switches = ctxt;
					}
				}
			}
		}
		let page_faults = read_page_faults();

		for line in &lines {
			if line.starts_with("cpu") {
				let parts: Vec<&str> = line.split_whitespace().collect();
				if parts.len() >= 11 && parts[0] != "cpu" {
					// TODO: rewrite use nom, define a struct for cpu usage detail like CPUStats
					if let Ok(cpu_id) = parts[0][3..].parse::<usize>() {
						let user = parts[1].parse::<u64>().unwrap_or(0);
						let nice = parts[2].parse::<u64>().unwrap_or(0);
						let system = parts[3].parse::<u64>().unwrap_or(0);
						let idle = parts[4].parse::<u64>().unwrap_or(0);
						let iowait = parts[5].parse::<u64>().unwrap_or(0);
						let irq = parts[6].parse::<u64>().unwrap_or(0);
						let softirq = parts[7].parse::<u64>().unwrap_or(0);
						let steal = parts[8].parse::<u64>().unwrap_or(0);
						let guest = parts[9].parse::<u64>().unwrap_or(0);
						let guest_nice = parts[10].parse::<u64>().unwrap_or(0);

						let total = user + nice + system + idle + iowait + irq + softirq + steal;
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
						let iowait_usage =
							if total > 0 { iowait as f64 / total as f64 * 100.0 } else { 0.0 };
						let irq_usage =
							if total > 0 { irq as f64 / total as f64 * 100.0 } else { 0.0 };
						let softirq_usage =
							if total > 0 { softirq as f64 / total as f64 * 100.0 } else { 0.0 };
						let steal_usage =
							if total > 0 { steal as f64 / total as f64 * 100.0 } else { 0.0 };
						let guest_usage =
							if total > 0 { guest as f64 / total as f64 * 100.0 } else { 0.0 };
						let guest_nice_usage =
							if total > 0 { guest_nice as f64 / total as f64 * 100.0 } else { 0.0 };
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
							iowait_usage,
							irq_usage,
							softirq_usage,
							steal_usage,
							guest_usage,
							guest_nice_usage,
							bt_usage: 0.0,    // 这个字段在/proc/stat中没有直接对应项
							context_switches, // 这个字段需要从/proc/stat以外获取
							page_faults,      // 这个字段需要从/proc/stat以外获取
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
