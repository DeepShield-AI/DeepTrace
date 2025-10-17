pub use diskstat::{FakeDiskStat, FakeDiskStats};
pub use meminfo::FakeMemInfo;
pub use netdev::{FakeInterface, FakeInterfaces};
pub use stat::FakeStat;
use std::{fs, io, path::Path};
pub use vmstat::FakeVmStat;

mod diskstat;
mod meminfo;
mod netdev;
mod stat;
mod vmstat;

#[derive(Default)]
pub struct Generator;

impl Generator {
	pub fn new() -> Self {
		Default::default()
	}

	pub fn generate_stat(&self, proc_dir: &Path) -> io::Result<FakeStat> {
		let stat = FakeStat::generate();

		fs::write(proc_dir.join("stat"), stat.to_string())?;
		Ok(stat)
	}

	pub fn generate_meminfo(&self, proc_dir: &Path) -> io::Result<FakeMemInfo> {
		let meminfo = FakeMemInfo::generate();

		fs::write(proc_dir.join("meminfo"), meminfo.to_string())?;
		Ok(meminfo)
	}

	pub fn generate_vmstat(&self, proc_dir: &Path) -> io::Result<FakeVmStat> {
		let vmstat = FakeVmStat::generate();

		fs::write(proc_dir.join("vmstat"), vmstat.to_string())?;
		Ok(vmstat)
	}

	pub fn generate_diskstats(&self, proc_dir: &Path) -> io::Result<FakeDiskStats> {
		let diskstats = FakeDiskStats::generate();

		fs::write(proc_dir.join("diskstats"), diskstats.to_string())?;
		Ok(diskstats)
	}

	pub fn generate_netdev(&self, proc_dir: &Path) -> io::Result<FakeInterfaces> {
		let netdev = FakeInterfaces::generate();

		// Create net directory if it doesn't exist
		let net_dir = proc_dir.join("net");
		fs::create_dir_all(&net_dir)?;

		fs::write(net_dir.join("dev"), netdev.to_string())?;
		Ok(netdev)
	}
}
