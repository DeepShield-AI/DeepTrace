// net/data.rs

pub struct NetMetric {
	pub interface: String,
	pub rx_bytes: u64,
	pub tx_bytes: u64,
	pub rx_packets: u64,
	pub tx_packets: u64,
	pub rx_dropped: u64,
	pub tx_dropped: u64,
	pub active_opens: u64,
	pub in_segs: u64,
	pub out_segs: u64,
	pub retrans_segs: u64,
	pub in_errs: u64,
	pub out_rsts: u64,
	pub curr_estab: u64,
	pub passive_opens: u64,
	pub in_datagrams: u64,
	pub out_datagrams: u64,
	pub timestamp: u64,
}
