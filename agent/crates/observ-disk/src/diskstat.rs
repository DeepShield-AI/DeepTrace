use crate::DiskMetricError;
use observ_macros::ProcParser;
use observ_metric_common::procfs_root;
use observ_metric_utils::read_to_string;
use uom::si::{
	f64::{Information, Time},
	information, time,
};

pub const DISK_SECTOR_SIZE: u64 = 512;

// Helper functions for field processing
fn sectors_to_bytes(sectors: u64) -> Information {
	// Use checked multiplication to avoid overflow, fallback to f64 arithmetic
	let bytes = sectors
		.checked_mul(DISK_SECTOR_SIZE)
		.map(|b| b as f64)
		.unwrap_or_else(|| sectors as f64 * DISK_SECTOR_SIZE as f64);
	Information::new::<information::byte>(bytes)
}

fn millis_to_time(millis: u64) -> Time {
	Time::new::<time::millisecond>(millis as f64)
}

#[derive(ProcParser)]
#[fmt = "table"]
pub struct DiskStat {
	/// major number
	major: u64,
	/// minor number
	minor: u64,
	/// device name
	name: String,
	/// reads completed successfully
	read_completed: u64,
	/// reads merged
	read_merged: u64,
	/// sectors read (converted to bytes)
	#[arg(with = sectors_to_bytes)]
	sectors_read: Information,
	/// time spent reading (ms, converted to Time)
	#[arg(with = millis_to_time)]
	read_time: Time,
	/// writes completed
	write_completed: u64,
	/// writes merged
	write_merged: u64,
	/// sectors written (converted to bytes)
	#[arg(with = sectors_to_bytes)]
	sectors_written: Information,
	/// time spent writing (ms, converted to Time)
	#[arg(with = millis_to_time)]
	writing_time: Time,
	/// I/Os currently in progress
	ios_in_progress: u64,
	/// time spent doing I/Os (ms, converted to Time)
	#[arg(with = millis_to_time)]
	io_time: Time,
	/// weighted time spent doing I/Os (ms, converted to Time)
	#[arg(with = millis_to_time)]
	weighted_io_time: Time,
	/// discards completed successfully (kernel 4.18+)
	discard_completed: Option<u64>,
	/// discards merged (kernel 4.18+)
	discard_merged: Option<u64>,
	/// sectors discarded (kernel 4.18+)
	#[arg(with = sectors_to_bytes, optional)]
	sectors_discarded: Option<Information>,
	/// time spent discarding (kernel 4.18+, converted to Time)
	#[arg(with = millis_to_time, optional)]
	discarding_time: Option<Time>,
	/// flush requests completed successfully (kernel 5.5+)
	#[arg(optional)]
	flush_completed: Option<u64>,
	/// time spent flushing (kernel 5.5+, converted to Time)
	#[arg(with = millis_to_time, optional)]
	flushing_time: Option<Time>,
}

pub async fn diskstat() -> Result<Vec<DiskStat>, DiskMetricError> {
	let content = read_to_string(procfs_root().join("diskstats")).await?;
	DiskStat::parse_all(&content).map_err(Into::into)
}
