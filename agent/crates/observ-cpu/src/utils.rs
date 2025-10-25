use crate::CpuMetricError;
use std::{io, sync::OnceLock};
use uom::si::{f64::Time, time};

static CLOCK_TICKS: OnceLock<f64> = OnceLock::new();

pub fn clock_ticks() -> f64 {
	*CLOCK_TICKS.get_or_init(|| get_clock_ticks().expect("Failed to get clock ticks"))
}

fn get_clock_ticks() -> Result<f64, CpuMetricError> {
	let clock_ticks = unsafe { libc::sysconf(libc::_SC_CLK_TCK) };
	if clock_ticks > 0 {
		Ok(clock_ticks as f64)
	} else {
		Err(CpuMetricError::IOError(io::Error::last_os_error()))
	}
}

pub(crate) fn clock_ticks_to_seconds(ticks: u64) -> Time {
	Time::new::<time::second>(ticks as f64 / clock_ticks())
}
