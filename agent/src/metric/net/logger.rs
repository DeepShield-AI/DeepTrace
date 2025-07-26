// net/logger.rs

use super::NetMetric;
use std::{
	fs::File,
	io::{BufWriter, Write},
	sync::{Arc, Mutex},
};

pub struct NetLogger {
	writer: Arc<Mutex<BufWriter<File>>>,
}

impl NetLogger {
	pub fn new(file_path: &str) -> std::io::Result<Self> {
		let file = BufWriter::new(File::create(file_path)?);
		let writer = Arc::new(Mutex::new(file));

		// 初始化 writer 时，写入列标题和说明
		{
			let mut w = writer.lock().unwrap();
			writeln!(w, "# timestamp: seconds since UNIX epoch")?;
			writeln!(w, "# interface: Network interface name")?;
			writeln!(w, "# rx_bytes: Received bytes")?;
			writeln!(w, "# tx_bytes: Transmitted bytes")?;
			writeln!(w, "# rx_packets: Received packets")?;
			writeln!(w, "# tx_packets: Transmitted packets")?;
			writeln!(w, "# rx_dropped: Dropped received packets")?;
			writeln!(w, "# tx_dropped: Dropped transmitted packets")?;
			writeln!(w, "# active_opens: Active TCP connections opened")?;
			writeln!(w, "# in_segs: Incoming TCP segments")?;
			writeln!(w, "# out_segs: Outgoing TCP segments")?;
			writeln!(w, "# retrans_segs: Retransmitted TCP segments")?;
			writeln!(w, "# in_errs: Incoming errors")?;
			writeln!(w, "# out_rsts: Outgoing resets")?;
			writeln!(w, "# curr_estab: Currently established connections")?;
			writeln!(w, "# passive_opens: Passive TCP connections opened")?;
			writeln!(w, "# in_datagrams: Incoming UDP datagrams")?;
			writeln!(w, "# out_datagrams: Outgoing UDP datagrams")?;
			writeln!(
				w,
				"interface,rx_bytes,tx_bytes,rx_packets,tx_packets,rx_dropped,tx_dropped,active_opens,in_segs,out_segs,retrans_segs,in_errs,out_rsts,curr_estab,passive_opens,in_datagrams,out_datagrams,timestamp"
			)?;
		}

		Ok(NetLogger { writer })
	}

	pub fn write(&self, metrics: &[NetMetric]) {
		let mut writer = self.writer.lock().unwrap();
		for metric in metrics {
			let log_line = format!(
				"{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
				metric.interface,
				metric.rx_bytes,
				metric.tx_bytes,
				metric.rx_packets,
				metric.tx_packets,
				metric.rx_dropped,
				metric.tx_dropped,
				metric.active_opens,
				metric.in_segs,
				metric.out_segs,
				metric.retrans_segs,
				metric.in_errs,
				metric.out_rsts,
				metric.curr_estab,
				metric.passive_opens,
				metric.in_datagrams,
				metric.out_datagrams,
				metric.timestamp
			);
			if let Err(e) = writer.write_all(log_line.as_bytes()) {
				eprintln!("Failed to write to file: {}", e);
			}
		}
	}

	pub fn flush(&self) {
		let mut writer = self.writer.lock().unwrap();
		if let Err(e) = writer.flush() {
			eprintln!("Failed to flush file: {}", e);
		}
	}

	pub fn clone(&self) -> Self {
		Self { writer: self.writer.clone() }
	}
}
