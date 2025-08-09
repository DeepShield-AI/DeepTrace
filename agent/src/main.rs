use agent::App;
use clap::Parser;
use tokio::signal;

#[derive(Debug, Parser)]
struct Opts {
	/// Specify config file location
	#[clap(short = 'f', long, default_value ="agent/config/default.toml")]
	config: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let opt = Opts::parse();
	let mut deeptrace = App::new(opt.config).expect("Failed to create app");

	deeptrace.start();
	
	signal::ctrl_c().await?;
	// sys::wait_on_signal();
	println!("ctrl-c received!");

	deeptrace.stop().await;

	Ok(())
}
