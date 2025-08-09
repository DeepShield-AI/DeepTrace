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
		// 在初始化 writer 时，更新列标题
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
			writeln!(dw, "# iowait_usage: I/O等待使用率")?;
			writeln!(dw, "# irq_usage: 硬中断使用率")?;
			writeln!(dw, "# softirq_usage: 软中断使用率")?;
			writeln!(dw, "# steal_usage: 虚拟机偷取时间使用率")?;
			writeln!(dw, "# guest_usage: 虚拟CPU运行时间使用率")?;
			writeln!(dw, "# guest_nice_usage: 低优先级虚拟CPU运行时间使用率")?;
			writeln!(dw, "# bt_usage: ???")?; // 需要确认这个字段的含义
			writeln!(dw, "# context_switches: 上下文切换次数")?;
			writeln!(dw, "# page_faults: 页错误次数")?;
			// 新增eBPF指标说明
			writeln!(dw, "# ksoftirqd_delay: 软中断处理延迟")?;
			writeln!(dw, "# cpu_migrations: CPU迁移次数")?;
			writeln!(dw, "# nr_csw: 上下文切换次数")?;
			writeln!(
				dw,
				"timestamp,cpu_id,cpu_load,cpu_usage,user,user_usage,nice,nice_usage,system,system_usage,idle,idle_usage,iowait_usage,irq_usage,softirq_usage,steal_usage,guest_usage,guest_nice_usage,bt_usage,context_switches,page_faults,ksoftirqd_delay,cpu_migrations,nr_csw"
			)?;
		}
		Ok(Self { detail_writer })
	}

	// 在 write 方法中，更新写入的数据，包含新增的eBPF指标
	pub fn write(&self, detail: &CpuMetric) {
		let mut writer = self.detail_writer.lock().unwrap();
		let _ = writeln!(
			writer,
			"{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
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
			detail.iowait_usage,
			detail.irq_usage,
			detail.softirq_usage,
			detail.steal_usage,
			detail.guest_usage,
			detail.guest_nice_usage,
			detail.bt_usage,
			detail.context_switches,
			detail.page_faults,
			detail.ksoftirqd_delay,  // 新增eBPF指标
			detail.cpu_migrations,   // 新增eBPF指标
			detail.nr_csw            // 新增eBPF指标
		);
	}
	
	pub fn flush(&self) {
		let _ = self.detail_writer.lock().unwrap().flush();
	}

	pub fn clone(&self) -> Self {
		Self { detail_writer: self.detail_writer.clone() }
	}
}