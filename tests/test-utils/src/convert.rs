use std::{io, sync::OnceLock};

static CLOCK_TICKS: OnceLock<f64> = OnceLock::new();

pub fn clock_ticks() -> f64 {
	*CLOCK_TICKS.get_or_init(|| get_clock_ticks().expect("Failed to get clock ticks"))
}

fn get_clock_ticks() -> io::Result<f64> {
	let clock_ticks = unsafe { libc::sysconf(libc::_SC_CLK_TCK) };
	if clock_ticks > 0 { Ok(clock_ticks as f64) } else { Err(io::Error::last_os_error()) }
}
