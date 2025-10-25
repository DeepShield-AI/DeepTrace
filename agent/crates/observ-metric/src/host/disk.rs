use super::{Collector, MetricError, constants::*};
use log::warn;
use observ_disk::diskstat;
use observ_event::{gauge, metric::Metric};
use std::io;
use uom::si::{information::byte, time::millisecond};

pub struct DiskCollector;

impl DiskCollector {
	pub(crate) const fn new() -> Result<Self, MetricError> {
		Ok(Self {})
	}
}

#[async_trait::async_trait]
impl Collector for DiskCollector {
	fn name(&self) -> &'static str {
		"host disk collector"
	}

	async fn collect(&self, buffer: &mut Vec<Metric>) -> Result<(), MetricError> {
		match diskstat::diskstat().await {
			Ok(diskstats) => {
				for disk in diskstats {
					let device_name = disk.get_name().to_string();

					// Basic disk information
					buffer.push(gauge!(MAJOR, disk.get_major(), "device" => device_name.clone()));
					buffer.push(gauge!(MINOR, disk.get_minor(), "device" => device_name.clone()));

					// Read metrics
					buffer.push(
						gauge!(READ_COMPLETED, disk.get_read_completed(), "device" => device_name.clone()),
					);
					buffer.push(
						gauge!(READ_MERGED, disk.get_read_merged(), "device" => device_name.clone()),
					);
					buffer.push(
						gauge!(SECTORS_READ, disk.get_sectors_read().get::<byte>(), "device" => device_name.clone()),
					);
					buffer.push(
						gauge!(READ_TIME, disk.get_read_time().get::<millisecond>(), "device" => device_name.clone()),
					);

					// Write metrics
					buffer.push(
						gauge!(WRITE_COMPLETED, disk.get_write_completed(), "device" => device_name.clone()),
					);
					buffer.push(
						gauge!(WRITE_MERGED, disk.get_write_merged(), "device" => device_name.clone()),
					);
					buffer.push(gauge!(SECTORS_WRITTEN, disk.get_sectors_written().get::<byte>(), "device" => device_name.clone()));
					buffer.push(gauge!(WRITING_TIME, disk.get_writing_time().get::<millisecond>(), "device" => device_name.clone()));

					// I/O metrics
					buffer.push(
						gauge!(IOS_IN_PROGRESS, disk.get_ios_in_progress(), "device" => device_name.clone()),
					);
					buffer.push(
						gauge!(IO_TIME, disk.get_io_time().get::<millisecond>(), "device" => device_name.clone()),
					);
					buffer.push(gauge!(WEIGHTED_IO_TIME, disk.get_weighted_io_time().get::<millisecond>(), "device" => device_name.clone()));

					// Optional discard metrics (kernel 4.18+)
					if let Some(discard_completed) = disk.get_discard_completed() {
						buffer.push(
							gauge!(DISCARD_COMPLETED, discard_completed, "device" => device_name.clone()),
						);
					}
					if let Some(discard_merged) = disk.get_discard_merged() {
						buffer.push(
							gauge!(DISCARD_MERGED, discard_merged, "device" => device_name.clone()),
						);
					}
					if let Some(sectors_discarded) = disk.get_sectors_discarded() {
						buffer.push(
							gauge!(SECTORS_DISCARDED, sectors_discarded.get::<byte>(), "device" => device_name.clone()),
						);
					}
					if let Some(discarding_time) = disk.get_discarding_time() {
						buffer.push(gauge!(DISCARDING_TIME, discarding_time.get::<millisecond>(), "device" => device_name.clone()));
					}

					// Optional flush metrics (kernel 5.5+)
					if let Some(flush_completed) = disk.get_flush_completed() {
						buffer.push(
							gauge!(FLUSH_COMPLETED, flush_completed, "device" => device_name.clone()),
						);
					}
					if let Some(flushing_time) = disk.get_flushing_time() {
						buffer.push(
							gauge!(FLUSHING_TIME, flushing_time.get::<millisecond>(), "device" => device_name.clone()),
						);
					}
				}
			},
			Err(error) => {
				warn!("Failed to collect disk metrics: {error}");
				return Err(io::Error::last_os_error().into());
			},
		}
		Ok(())
	}
}
