use clap::Parser;
use deeptrace::{Agent, Module};
use tokio::signal;

#[derive(Debug, Parser)]
struct Opts {
	#[clap(
		short = 'c',
		long,
		default_value = "config/deeptrace.toml",
		help = "Specify config file location"
	)]
	config: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let opt = Opts::parse();
	let mut deeptrace = Agent::new(opt.config).expect("Failed to create app");

	deeptrace.start()?;

	signal::ctrl_c().await?;
	// sys::wait_on_signal();
	println!("ctrl-c received!");

	deeptrace.stop().await?;

	Ok(())
}
