pub struct MemMetric {
	pub mem_used: u64,
	pub mem_used_app: u64,
	pub mem_vsz: u64,
	pub mem_free: u64,
	pub numa_miss: u64,
	pub dirty: u64,
	pub writeback: u64,
	pub buffers: u64,
	pub timestamp: u64,
}
