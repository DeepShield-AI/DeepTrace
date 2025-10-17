use fake::{Dummy, Fake, Faker};
use std::fmt;

#[derive(Debug, Dummy)]
pub struct FakeCpuTime {
	/// Time spent in user mode
	pub user: u64,
	/// Time spent in user mode with low priority (nice)
	pub nice: u64,
	/// Time spent in system mode
	pub system: u64,
	/// Time spent in idle task
	pub idle: u64,
	/// Time spent waiting for I/O to complete
	pub iowait: u64,
	/// Time spent servicing hardware interrupts
	pub irq: u64,
	/// Time spent servicing software interrupts
	pub softirq: u64,
	/// Time stolen by other operating systems running in a virtual environment
	pub steal: u64,
	/// Time spent running a virtual CPU for guest operating systems
	pub guest: u64,
	/// Time spent running a niced guest
	pub guest_nice: u64,
}

pub struct FakeStat {
	/// Aggregate CPU statistics
	pub cpu_total: FakeCpuTime,
	/// Per-CPU statistics
	pub cpus: Vec<FakeCpuTime>,
	/// Total number of context switches
	pub context_switches: u64,
	/// Boot time in seconds since Unix epoch
	pub boot_time: u64,
	/// Total number of processes created
	pub processes: u64,
	/// Number of processes currently running
	pub procs_running: u64,
	/// Number of processes currently blocked
	pub procs_blocked: u64,
}

impl FakeStat {
	pub fn generate() -> Self {
		Self {
			cpu_total: Faker.fake(),
			cpus: (Faker, 1..16).fake(),
			context_switches: Faker.fake(),
			boot_time: Faker.fake(),
			processes: Faker.fake(),
			procs_running: Faker.fake(),
			procs_blocked: Faker.fake(),
		}
	}
}

impl fmt::Display for FakeStat {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(
			f,
			"cpu  {} {} {} {} {} {} {} {} {} {}",
			self.cpu_total.user,
			self.cpu_total.nice,
			self.cpu_total.system,
			self.cpu_total.idle,
			self.cpu_total.iowait,
			self.cpu_total.irq,
			self.cpu_total.softirq,
			self.cpu_total.steal,
			self.cpu_total.guest,
			self.cpu_total.guest_nice
		)?;
		<Vec<FakeCpuTime> as AsRef<[FakeCpuTime]>>::as_ref(&self.cpus)
			.iter()
			.enumerate()
			.try_for_each(|(i, fake)| {
				writeln!(
					f,
					"cpu{}  {} {} {} {} {} {} {} {} {} {}",
					i,
					fake.user,
					fake.nice,
					fake.system,
					fake.idle,
					fake.iowait,
					fake.irq,
					fake.softirq,
					fake.steal,
					fake.guest,
					fake.guest_nice
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
