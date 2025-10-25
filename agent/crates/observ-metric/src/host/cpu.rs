use super::{Collector, MetricError, constants::*};
use log::warn;
use observ_cpu::stat;
use observ_event::{gauge, metric::Metric};
use std::io;
use uom::si::time::second;

pub struct CpuCollector;

impl CpuCollector {
	pub(crate) const fn new() -> Result<Self, MetricError> {
		Ok(Self {})
	}
}

#[async_trait::async_trait]
impl Collector for CpuCollector {
	fn name(&self) -> &'static str {
		"host cpu collector"
	}

	async fn collect(&self, buffer: &mut Vec<Metric>) -> Result<(), MetricError> {
		match stat::stat().await {
			Ok(stat) => {
				for (cpu, time) in stat.cpu_times() {
					buffer.push(gauge!(USER_USAGE, time.get_user().get::<second>(), "cpu" => cpu));
					buffer.push(gauge!(NICE_USAGE, time.get_nice().get::<second>(), "cpu" => cpu));
					buffer.push(
						gauge!(SYSTEM_USAGE, time.get_system().get::<second>(), "cpu" => cpu),
					);
					buffer.push(gauge!(IDLE_USAGE, time.get_idle().get::<second>(), "cpu" => cpu));
					buffer.push(
						gauge!(IO_WAIT_USAGE, time.get_iowait().get::<second>(), "cpu" => cpu),
					);
					buffer.push(gauge!(IRQ_USAGE, time.get_irq().get::<second>(), "cpu" => cpu));
					buffer.push(
						gauge!(SOFT_IRQ_USAGE, time.get_softirq().get::<second>(), "cpu" => cpu),
					);
					buffer
						.push(gauge!(STEAL_USAGE, time.get_steal().get::<second>(), "cpu" => cpu));
					buffer
						.push(gauge!(GUEST_USAGE, time.get_guest().get::<second>(), "cpu" => cpu));
					buffer.push(
						gauge!(GUEST_NICE_USAGE, time.get_guest_nice().get::<second>(), "cpu" => cpu),
					);
				}
				buffer.push(gauge!(CONTEXT_SWITCHES, stat.context_switches()));
				buffer.push(gauge!(BOOT_TIME, stat.boot_time()));
				buffer.push(gauge!(PROCESSES, stat.processes()));
				buffer.push(gauge!(PROCS_RUNNING, stat.procs_running()));
				buffer.push(gauge!(PROCS_BLOCKED, stat.procs_blocked()));
			},
			Err(error) => {
				warn!("Failed to collect cpu metrics: {error}");
				return Err(io::Error::last_os_error().into());
			},
		}
		Ok(())
	}
}
