pub struct CpuMetric {
	pub cpu_id: usize,
	/// CPU one-minute average load
	pub cpu_load: f64,
	/// CPU usage percentage (%)
	pub cpu_usage: f64,
	/// User-mode CPU time (microseconds)
	pub user: u64,
	/// User-mode CPU usage percentage (%)
	pub user_usage: f64,
	/// Low-priority user-mode CPU time (microseconds)
	pub nice: u64,
	/// Low-priority user-mode CPU usage percentage (%)
	pub nice_usage: f64,
	/// System-mode CPU time (microseconds)
	pub system: u64,
	/// System-mode CPU usage percentage (%)
	pub system_usage: f64,
	/// Idle CPU time (microseconds)
	pub idle: u64,
	/// Idle CPU usage percentage (%)
	pub idle_usage: f64,

	// pub total_time: u64,      // 总时间
	/// Timestamp in seconds since UNIX epoch
	pub timestamp: u64,
}