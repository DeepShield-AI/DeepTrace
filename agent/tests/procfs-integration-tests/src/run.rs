use crate::validators::Validator;
use anyhow::{Result, anyhow};
use prism_fixtures::generators::Generator;
use std::{env, fs, path::Path};
pub(crate) async fn single_test(test_id: usize, session_dir: &str) -> Result<()> {
	// Create test directory structure: session_dir/test-{id}/procfs/
	let test_dir_name = format!("test-{test_id:03}");
	let test_dir = Path::new(session_dir).join(&test_dir_name);
	let proc_path = test_dir.join("procfs");
	fs::create_dir_all(&proc_path)?;

	// Set procfs root using environment variable for this process
	unsafe { env::set_var("PROCFS_ROOT", &proc_path) };

	// Generate random procfs data
	let generator = Generator::new();

	println!("  Test directory: {}/", test_dir.display());
	let fake_stat = generator.generate_stat(&proc_path)?;
	let fake_meminfo = generator.generate_meminfo(&proc_path)?;
	let fake_vmstat = generator.generate_vmstat(&proc_path)?;
	let fake_diskstats = generator.generate_diskstats(&proc_path)?;
	let fake_netdev = generator.generate_netdev(&proc_path)?;
	println!("  Running observ collectors and validating results");

	// Initialize the roots to pick up the environment variable
	observ_metric_common::init_roots();

	let real_stat = observ_cpu::stat::stat()
		.await
		.map_err(|e| anyhow!("Stat parsing failed: {}", e))?;
	let real_meminfo = observ_memory::meminfo::meminfo()
		.await
		.map_err(|e| anyhow!("MemInfo parsing failed: {}", e))?;
	let real_vmstat = observ_memory::vmstat::vmstat()
		.await
		.map_err(|e| anyhow!("VmStat parsing failed: {}", e))?;
	let real_diskstats = observ_disk::diskstat::diskstat()
		.await
		.map_err(|e| anyhow!("Disk parsing failed: {}", e))?;
	let real_netdev = observ_network::netdev::interfaces()
		.await
		.map_err(|e| anyhow!("NetDev parsing failed: {}", e))?;
	// Create validator with original values and test directory
	let validator = Validator::new();
	// Test all metrics with field validation
	validator.validate_cpu(fake_stat, real_stat)?;
	validator.validate_meminfo(fake_meminfo, real_meminfo)?;
	validator.validate_vmstat(fake_vmstat, real_vmstat)?;
	validator.validate_diskstat(fake_diskstats, real_diskstats)?;
	validator.validate_netdev(fake_netdev, real_netdev)?;

	println!("  Test #{test_id} validation completed successfully");
	Ok(())
}
