use clap::Parser;
use log::info;
use prism::{Agent, Module};
use tokio::signal;

#[derive(Debug, Parser)]
struct Opts {
	#[arg(
		short = 'c',
		long,
		default_value = "config/prism.toml",
		help = "Specify config file location"
	)]
	config: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let opt = Opts::parse();
	let mut agent = Agent::new(opt.config)?;

	agent.start()?;

	signal::ctrl_c().await?;
	info!("ctrl-c received!");

	agent.stop().await?;
	Ok(())
}
