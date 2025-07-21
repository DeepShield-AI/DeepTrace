use super::CpuMetric;
use std::{
	fs::File,
	io::{BufWriter, Write},
	sync::{Arc, Mutex},
};
// TODO: Temporary logger for detailed CPU metrics, need to change to new compress module input
// TODO: Change to be async
pub struct CpuLogger {
	detail_writer: Arc<Mutex<BufWriter<File>>>,
}

impl CpuLogger {
	pub fn new(file_path: &str) -> Result<Self, std::io::Error> {
		let detail_file_path = format!("{}_detail.csv", file_path.trim_end_matches(".csv"));
		let detail_file = BufWriter::new(File::create(detail_file_path)?);
		let detail_writer = Arc::new(Mutex::new(detail_file));
		{
			let mut dw = detail_writer.lock().unwrap();
			writeln!(dw, "# timestamp: seconds since UNIX epoch")?;
			writeln!(dw, "# cpu_id: 0,1,2... (0-based)")?;
			writeln!(dw, "# cpu_load: CPU 1分钟平均负载")?;
			writeln!(dw, "# cpu_usage: CPU 使用率")?;
			writeln!(dw, "# user: 用户态CPU时间")?;
			writeln!(dw, "# user_usage: 用户态CPU使用率")?;
			writeln!(dw, "# nice: 低优先级用户态CPU时间")?;
			writeln!(dw, "# nice_usage: 低优先级用户态CPU使用率")?;
			writeln!(dw, "# system: 内核态CPU时间")?;
			writeln!(dw, "# system_usage: 内核态CPU使用率")?;
			writeln!(dw, "# idle: 空闲CPU时间")?;
			writeln!(dw, "# idle_usage: 空闲CPU使用率")?;
			// writeln!(dw, "# total_time: 总CPU时间")?;
			writeln!(
				dw,
				"timestamp,cpu_id,cpu_load,cpu_usage,user,user_usage,nice,nice_usage,system,system_usage,idle,idle_usage"
			)?;
		}
		Ok(Self { detail_writer })
	}

	pub fn write(&self, detail: &CpuMetric) {
		println!("CPU  write");
		let mut writer = self.detail_writer.lock().unwrap();
		let _ = writeln!(
			writer,
			"{},{},{},{},{},{},{},{},{},{},{},{}",
			detail.timestamp,
			detail.cpu_id,
			detail.cpu_load,
			detail.cpu_usage,
			detail.user,
			detail.user_usage,
			detail.nice,
			detail.nice_usage,
			detail.system,
			detail.system_usage,
			detail.idle,
			detail.idle_usage,
			// detail.total_time,
		);
	}
	pub fn flush(&self) {
		let _ = self.detail_writer.lock().unwrap().flush();
	}

	pub fn clone(&self) -> Self {
		Self { detail_writer: self.detail_writer.clone() }
	}
}