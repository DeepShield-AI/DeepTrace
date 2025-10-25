use crate::{CpuMetricError, utils::clock_ticks_to_seconds};
use observ_macros::ProcParser;
use observ_metric_common::procfs_root;
use observ_metric_utils::read_to_string;
use std::fmt;
use uom::si::{f64::Time, time::second};

/// CPU statistics from /proc/stat
/// Each CPU line contains: user nice system idle iowait irq softirq steal guest guest_nice
#[derive(ProcParser, Clone)]
#[fmt = "table"]
pub struct CpuTime {
	/// Time spent in user mode
	#[arg(with = clock_ticks_to_seconds)]
	user: Time,
	/// Time spent in user mode with low priority (nice)
	#[arg(with = clock_ticks_to_seconds)]
	nice: Time,
	/// Time spent in system mode
	#[arg(with = clock_ticks_to_seconds)]
	system: Time,
	/// Time spent in idle task
	#[arg(with = clock_ticks_to_seconds)]
	idle: Time,
	/// Time spent waiting for I/O to complete
	#[arg(with = clock_ticks_to_seconds)]
	iowait: Time,
	/// Time spent servicing hardware interrupts
	#[arg(with = clock_ticks_to_seconds)]
	irq: Time,
	/// Time spent servicing software interrupts
	#[arg(with = clock_ticks_to_seconds)]
	softirq: Time,
	/// Time stolen by other operating systems running in a virtual environment
	#[arg(with = clock_ticks_to_seconds)]
	steal: Time,
	/// Time spent running a virtual CPU for guest operating systems
	#[arg(with = clock_ticks_to_seconds)]
	guest: Time,
	/// Time spent running a niced guest
	#[arg(with = clock_ticks_to_seconds)]
	guest_nice: Time,
}

/// Overall system statistics from /proc/stat
pub struct Stat {
	/// Aggregate CPU statistics
	cpu_total: CpuTime,
	/// Per-CPU statistics
	cpus: Vec<CpuTime>,
	/// Total number of context switches
	context_switches: u64,
	/// Boot time in seconds since Unix epoch
	boot_time: u64,
	/// Total number of processes created
	processes: u64,
	/// Number of processes currently running
	procs_running: u64,
	/// Number of processes currently blocked
	procs_blocked: u64,
}

impl Stat {
	pub const fn cpu_total(&self) -> &CpuTime {
		&self.cpu_total
	}

	pub const fn context_switches(&self) -> u64 {
		self.context_switches
	}

	pub const fn boot_time(&self) -> u64 {
		self.boot_time
	}

	pub const fn processes(&self) -> u64 {
		self.processes
	}

	pub const fn procs_running(&self) -> u64 {
		self.procs_running
	}

	pub const fn procs_blocked(&self) -> u64 {
		self.procs_blocked
	}

	pub fn cpu_times(&self) -> impl IntoIterator<Item = (usize, &CpuTime)> {
		self.cpus.iter().enumerate()
	}

	fn parse(input: &str) -> Result<Self, Box<dyn std::error::Error>> {
		use nom::{
			IResult, Parser,
			bytes::complete::take_while1,
			character::complete::{digit1, multispace1},
			combinator::map_res,
			sequence::preceded,
		};

		let mut cpu_total = None;
		let mut cpus = Vec::new();
		let mut context_switches = 0;
		let mut boot_time = 0;
		let mut processes = 0;
		let mut procs_running = 0;
		let mut procs_blocked = 0;

		fn parse_key_value(input: &str) -> IResult<&str, (&str, u64)> {
			(
				take_while1(|c: char| c.is_alphanumeric() || c == '_'),
				preceded(multispace1, map_res(digit1, |s: &str| s.parse::<u64>())),
			)
				.parse(input)
		}

		for line in input.lines() {
			let line = line.trim();
			if line.is_empty() {
				continue;
			}

			if line.starts_with("cpu") {
				let parts: Vec<&str> = line.split_whitespace().collect();
				if parts.len() >= 11 {
					let cpu_data = parts[1..].join(" ");
					if parts[0] == "cpu" {
						if let Ok(mut parsed_cpus) = CpuTime::parse_all(&cpu_data) &&
							let Some(cpu) = parsed_cpus.pop()
						{
							cpu_total = Some(cpu);
						}
					} else if parts[0].len() > 3 &&
						parts[0].starts_with("cpu") &&
						let Ok(mut parsed_cpus) = CpuTime::parse_all(&cpu_data) &&
						let Some(cpu) = parsed_cpus.pop()
					{
						cpus.push(cpu);
					}
				}
			} else if let Ok((_, (key, value))) = parse_key_value(line) {
				match key {
					"ctxt" => context_switches = value,
					"btime" => boot_time = value,
					"processes" => processes = value,
					"procs_running" => procs_running = value,
					"procs_blocked" => procs_blocked = value,
					_ => {},
				}
			}
		}

		let cpu_total = cpu_total.ok_or("Missing aggregate CPU statistics")?;

		Ok(Stat {
			cpu_total,
			cpus,
			context_switches,
			boot_time,
			processes,
			procs_running,
			procs_blocked,
		})
	}
}

impl fmt::Display for Stat {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		writeln!(
			f,
			"cpu  {} {} {} {} {} {} {} {} {} {}\n",
			self.cpu_total.user.get::<second>(),
			self.cpu_total.nice.get::<second>(),
			self.cpu_total.system.get::<second>(),
			self.cpu_total.idle.get::<second>(),
			self.cpu_total.iowait.get::<second>(),
			self.cpu_total.irq.get::<second>(),
			self.cpu_total.softirq.get::<second>(),
			self.cpu_total.steal.get::<second>(),
			self.cpu_total.guest.get::<second>(),
			self.cpu_total.guest_nice.get::<second>()
		)?;
		<Vec<CpuTime> as AsRef<[CpuTime]>>::as_ref(&self.cpus)
			.iter()
			.enumerate()
			.try_for_each(|(i, cpu)| {
				writeln!(
					f,
					"cpu{}  {} {} {} {} {} {} {} {} {} {}",
					i,
					cpu.user.get::<second>(),
					cpu.nice.get::<second>(),
					cpu.system.get::<second>(),
					cpu.idle.get::<second>(),
					cpu.iowait.get::<second>(),
					cpu.irq.get::<second>(),
					cpu.softirq.get::<second>(),
					cpu.steal.get::<second>(),
					cpu.guest.get::<second>(),
					cpu.guest_nice.get::<second>()
				)
			})?;
		writeln!(
			f,
			"intr 5626640454 54 0 0 0 0 0 0 0 0 3 913 0 0 0 0 0 0 0 3337290 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 341 783056 837883 845154 873105 873631 890524 799746 857240 817852 864770 832325 867967 819385 911580 878128 949096 888977 879773 829688 791239 850308 855681 840074 828653 419310 0 0 0 0 209610 0 0 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 0 0 0 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 0 0 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 209610 0 204094 1 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 209619 0 0 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 209633 0 0 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 209646 0 157088808 204000321 5167367 5035222 204034507 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0"
		)?;
		writeln!(f, "ctxt {}", self.context_switches)?;
		writeln!(f, "btime {}", self.boot_time)?;
		writeln!(f, "processes {}", self.processes)?;
		writeln!(f, "procs_running {}", self.procs_running)?;
		writeln!(f, "procs_blocked {}", self.procs_blocked)?;
		write!(
			f,
			"softirq 4679053537 3 217228708 171684 2852529223 217514 0 1920121 752158546 1247 854826491"
		)?;
		Ok(())
	}
}

pub async fn stat() -> Result<Stat, CpuMetricError> {
	let content = read_to_string(procfs_root().join("stat")).await?;
	Stat::parse(&content).map_err(Into::into)
}
