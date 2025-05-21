use agent::{App, utils::sys};
use clap::Parser;

#[derive(Debug, Parser)]
struct Opts {
	/// Specify config file location
	#[clap(short = 'f', long, default_value = "agent/config/default.toml")]
	config: String,
}

fn main() -> anyhow::Result<()> {
	let opt = Opts::parse();
	let mut deeptrace = App::new(opt.config).expect("Failed to create app");
	deeptrace.start();

	sys::wait_on_signal();

	deeptrace.stop();

	Ok(())
}
