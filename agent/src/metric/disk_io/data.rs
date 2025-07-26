use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DiskMetric {
	pub device: String,
	pub read_completed: u64,
	pub read_merged: u64,
	pub sectors_read: u64,
	pub time_spent_read: u64,
	pub write_completed: u64,
	pub write_merged: u64,
	pub sectors_written: u64,
	pub time_spent_writing: u64,
	pub io_in_progress: u64,
	pub time_spent_io: u64,
	pub weighted_time_spent_io: u64,
	pub aqu_sz: f64,     //平均IO队列长度
	pub await_time: f64, // 平均I/O等待时间
	pub svctm_time: f64, // 平均I/O服务时间
}

#[derive(Debug, Clone)]
pub struct DiskUsage {
	pub filesystem: String,
	pub size: u64,
	pub used: u64,
	pub available: u64,
	pub use_percent: f64, // 使用百分比
	pub mounted_on: String,
}

#[derive(Debug, Clone)]
pub struct Ext4CacheStats {
	pub hits: u64,
	pub misses: u64,
}
