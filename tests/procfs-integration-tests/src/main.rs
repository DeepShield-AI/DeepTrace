use anyhow::Result;
use chrono::Local;
use clap::Parser;
use std::{
	env, fs,
	process::{Command, exit},
};

mod run;
mod validators;

#[derive(Parser)]
#[command(
	name = "procfs_integration_tests",
	version,
	about = "A comprehensive testing framework for observ ProcFS integration",
	long_about = "This tool generates random procfs data and validates that observ can correctly parse and process it. It supports multiple test runs with configurable output directories and verbose logging.",
	styles = clap::builder::Styles::styled(),
	help_template = "{before-help}{name} {version}\n{author}\n{about}\n\n{usage-heading} {usage}\n\n{all-args}{after-help}"
)]
struct Args {
	#[arg(short, long, default_value_t = 1, help = "Number of random tests to run")]
	count: usize,
	#[arg(short, long, default_value = "output", help = "Output directory for test results")]
	output: String,
	#[arg(short, long, default_value_t = false, help = "Enable verbose output")]
	verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
	// Check if this is an internal subprocess call via environment variable
	if let Ok(test_spec) = env::var("observ_INTERNAL_TEST_SPEC") {
		let parts: Vec<&str> = test_spec.split(':').collect();
		if parts.len() == 2 {
			let test_id: usize = parts[0].parse()?;
			let session_dir = parts[1];
			return run::single_test(test_id, session_dir).await;
		}
		return Err(anyhow::anyhow!("Invalid internal test spec format"));
	}

	let args = Args::parse();

	println!("Starting observ ProcFS Random Integration Tests");
	println!("==================================================");
	println!("Running {} test{}", args.count, if args.count == 1 { "" } else { "s" });

	fs::create_dir_all(&args.output)?;

	// Create a timestamp-based directory inside output
	let timestamp = Local::now();
	let session_dir = format!("{}/{}", args.output, timestamp.format("%Y%m%d-%H%M%S"));
	fs::create_dir_all(&session_dir)?;

	println!("Test session directory: {session_dir}/");

	let mut passed_tests = 0;
	let mut failed_tests = 0;

	for test_id in 1..=args.count {
		println!("\nRunning test {test_id}/{}", args.count);

		// Run each test in a separate process to avoid static variable conflicts
		// Pass test parameters via environment variables (cleaner than command line args)
		let test_spec = format!("{test_id}:{session_dir}");
		let mut cmd = Command::new(env::current_exe()?);
		cmd.env("observ_INTERNAL_TEST_SPEC", &test_spec);

		// Pass verbose flag via environment variable if needed
		if args.verbose {
			cmd.env("observ_VERBOSE", "1");
		}

		let status = cmd.status()?;

		if status.success() {
			println!("✅ Test #{test_id} passed");
			passed_tests += 1;
		} else {
			println!("❌ Test #{test_id} failed");
			failed_tests += 1;
		}
	}

	println!("\nTest session completed!");
	println!("Results: {passed_tests} passed, {failed_tests} failed");
	println!("All test results saved in: {session_dir}/");

	if failed_tests > 0 {
		exit(1);
	}

	Ok(())
}
