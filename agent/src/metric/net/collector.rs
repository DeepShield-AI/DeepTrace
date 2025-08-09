// net/collector.rs

use super::NetMetric;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::time;

pub struct NetCollector {
    interval: Duration,
}

impl NetCollector {
    pub fn new() -> Self {
		Self {
			// Default sampling interval of 1 second
			interval: Duration::from_secs(1),
		}
	}

    pub fn collect(&self) -> Vec<NetMetric> {
        let mut results = Vec::new();

        // Collect basic metrics from /proc/net/dev
        if let Ok(file) = File::open("/proc/net/dev") {
            let reader = BufReader::new(file);

            for line in reader.lines().filter_map(|r| r.ok()).skip(2) { // Skip the header lines
                let parts: Vec<&str> = line.trim().split_whitespace().collect();
                if parts.len() >= 16 {
                    let interface = parts[0].trim_end_matches(':').to_string();
                    let rx_bytes: u64 = parts[1].parse().unwrap_or(0);
                    let tx_bytes: u64 = parts[9].parse().unwrap_or(0);
                    let rx_packets: u64 = parts[2].parse().unwrap_or(0);
                    let tx_packets: u64 = parts[10].parse().unwrap_or(0);
                    let rx_dropped: u64 = parts[4].parse().unwrap_or(0);
                    let tx_dropped: u64 = parts[12].parse().unwrap_or(0);

                    // Initialize other metrics to 0 (will be updated later)
                    let mut metric = NetMetric {
                        interface,
                        rx_bytes,
                        tx_bytes,
                        rx_packets,
                        tx_packets,
                        rx_dropped,
                        tx_dropped,
                        active_opens: 0,
                        in_segs: 0,
                        out_segs: 0,
                        retrans_segs: 0,
                        in_errs: 0,
                        out_rsts: 0,
                        curr_estab: 0,
                        passive_opens: 0,
                        in_datagrams: 0,
                        out_datagrams: 0,
                        timestamp: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    };

                    // Update other metrics from /proc/net/tcp and /proc/net/udp
                    self.update_tcp_metrics(&mut metric);
                    self.update_udp_metrics(&mut metric);

                    results.push(metric);
                }
            }
        }

        results
    }

    fn update_tcp_metrics(&self, metric: &mut NetMetric) {

    if let Ok(file) = File::open("/proc/net/snmp") {
        let reader = BufReader::new(file);

        for line in reader.lines().filter_map(|r| r.ok()) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 16 && parts[0] == "Tcp:" {
                metric.in_segs = parts[10].parse().unwrap_or(0);
                metric.out_segs = parts[11].parse().unwrap_or(0);
                metric.retrans_segs = parts[12].parse().unwrap_or(0);
                metric.in_errs = parts[13].parse().unwrap_or(0);
                metric.out_rsts = parts[14].parse().unwrap_or(0);
                metric.active_opens = parts[5].parse().unwrap_or(0); // ActiveOpens
                metric.passive_opens = parts[6].parse().unwrap_or(0); // PassiveOpens
            }
        }
    }
}

    fn update_udp_metrics(&self, metric: &mut NetMetric) {
        if let Ok(file) = File::open("/proc/net/snmp") {
            let reader = BufReader::new(file);

            for line in reader.lines().filter_map(|r| r.ok()) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 7 && parts[0] == "Udp:" {
                    metric.in_datagrams = parts[1].parse().unwrap_or(0);
                    metric.out_datagrams = parts[4].parse().unwrap_or(0);
                }
            }
        }
    }

    pub async fn sleep_duration(&self){
        time::sleep(self.interval).await;
    }
}