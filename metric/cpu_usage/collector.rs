// cpu_usage/collector.rs
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::thread;
use std::io::{BufRead, BufReader};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use log::{info, warn};
use crate::metric::cpu_usage::CpuUsageDetail;

pub struct CpuUsageCollector {
    cpu_id: usize,
    interval: Duration, // 新增字段：采集频率
}

fn read_load_avg() -> f64 {
    let file = match File::open("/proc/loadavg") {
        Ok(f) => f,
        Err(e) => {
            warn!("Failed to open /proc/loadavg: {}", e);
            return 0.0;
        }
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

impl CpuUsageCollector {
    pub fn new(cpu_id: usize) -> Self {
        Self {
            cpu_id,
            interval: Duration::from_secs(1), // 默认每秒采集一次
        }
    }

    // 可选：设置采集间隔
    pub fn with_interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    // 采集方法
        pub fn collect(&self) -> Vec<CpuUsageDetail> {
        let file = match File::open("/proc/stat") {
            Ok(f) => f,
            Err(e) => {
                warn!("Failed to open /proc/stat: {}", e);
                return vec![];
            }
        };
        let reader = BufReader::new(file);
        let mut results = vec![];
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let load_avg_1min = read_load_avg();
        for line in reader.lines().filter_map(|r| r.ok()) {
            if line.starts_with("cpu") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 5 && parts[0] != "cpu" {
                    if let Ok(cpu_id) = parts[0][3..].parse::<usize>() {
                        let user = parts[1].parse::<u64>().unwrap_or(0);
                        let nice = parts[2].parse::<u64>().unwrap_or(0);
                        let system = parts[3].parse::<u64>().unwrap_or(0);
                        let idle = parts[4].parse::<u64>().unwrap_or(0);

                        let total = user + nice + system + idle;
                        let usage = if total > 0 {
                            (user + nice + system) as f64 / total as f64 * 100.0
                        } else {
                            0.0
                        };

                        // 计算各部分的百分比
                        let user_percentage = if total > 0 { user as f64 / total as f64 * 100.0 } else { 0.0 };
                        let nice_percentage = if total > 0 { nice as f64 / total as f64 * 100.0 } else { 0.0 };
                        let system_percentage = if total > 0 { system as f64 / total as f64 * 100.0 } else { 0.0 };
                        let idle_percentage = if total > 0 { idle as f64 / total as f64 * 100.0 } else { 0.0 };

                        results.push(CpuUsageDetail {
                            cpu_id,
                            user_time: user,
                            user_percentage,
                            nice_time: nice,
                            nice_percentage,
                            system_time: system,
                            system_percentage,
                            idle_time: idle,
                            idle_percentage,
                            total_time: total,
                            timestamp: current_time,
                            usage,  // 使用计算好的总使用率
                            load_avg_1min,
                        });
                    }
                }
            }
        }

        results
    }

    // 采集后休眠
    pub fn sleep_duration(&self) {
        thread::sleep(self.interval);
    }
}

