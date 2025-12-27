use clap::{Args, Parser, Subcommand, ValueEnum, builder::OsStr};
use std::fmt::Display;
use utils::{default_target, runner};

mod build;
mod clippy;
mod run;
mod utils;

#[derive(Clone, ValueEnum)]
pub(crate) enum Profile {
	Debug,
	Release,
}

impl From<Profile> for OsStr {
	fn from(value: Profile) -> Self {
		match value {
			Profile::Debug => "debug".into(),
			Profile::Release => "release".into(),
		}
	}
}

impl Display for Profile {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Profile::Debug => write!(f, "debug"),
			Profile::Release => write!(f, "release"),
		}
	}
}

#[derive(Clone, ValueEnum)]
enum BpfTarget {
	BpfelUnknownNone,
	BpfebUnknownNone,
}

impl From<BpfTarget> for OsStr {
	fn from(value: BpfTarget) -> Self {
		match value {
			BpfTarget::BpfelUnknownNone => "bpfel-unknown-none".into(),
			BpfTarget::BpfebUnknownNone => "bpfeb-unknown-none".into(),
		}
	}
}

impl Display for BpfTarget {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			BpfTarget::BpfelUnknownNone => write!(f, "bpfel-unknown-none"),
			BpfTarget::BpfebUnknownNone => write!(f, "bpfeb-unknown-none"),
		}
	}
}

#[derive(Parser)]
#[command(
    name = "xtask",
    version,
    about = "xtask for deepshield program",
    long_about = "xtask for deepshield program",
    styles = clap::builder::Styles::styled(),
    propagate_version = true,
    help_template = "{before-help}{name} {version}\n{author}\n{about}\n\n{usage-heading} {usage}\n\n{all-args}{after-help}"
)]
struct Opts {
	#[arg(
		short = 'i',
		long,
		global = true,
		default_value_t = false,
		help = "Ignore eBPF related tasks"
	)]
	pub ignore_ebpf: bool,
	#[arg(
		long = "ebpf-dirs",
		value_delimiter = ',',
		default_values = [
			"crates/observ-trace-ebpf",
			"crates/observ-cpu-ebpf",
			"crates/observ-memory-ebpf",
			"crates/observ-disk-ebpf",
			"crates/observ-network-ebpf"
		],
		requires_if("false", "ignore-ebpf"),
		help = "Path to the directories containing eBPF programs"
	)]
	pub ebpf_dirs: Vec<String>,
	#[arg(
		long = "bpf-target",
		value_enum,
		default_value = BpfTarget::BpfelUnknownNone,
		requires_if("false", "ignore-ebpf"),
        help = "Set the endianness of the BPF target",
	)]
	pub target: BpfTarget,
	#[arg(
		long = "bpf-linker",
		requires_if("false", "ignore-ebpf"),
		help = "Path to custom bpf-linker"
	)]
	pub linker: Option<String>,
	#[arg(
		long = "bpf-link-args",
		requires_if("false", "ignore-ebpf"),
		help = "Additional linker arguments to pass to the bpf-linker"
	)]
	pub link_args: Vec<String>,
	#[clap(subcommand)]
	pub command: Command,
}

#[derive(Subcommand)]
pub(crate) enum Command {
	#[command(name = "build", alias = "b", version, about = "Build the project")]
	Build(BuildArgs),
	#[command(name = "run", alias = "r", version, about = "Run the project")]
	Run(RunArgs),
	#[command(name = "clippy", alias = "c", version, about = "Run clippy on the project")]
	Clippy(ClippyArgs),
	#[command(name = "test", alias = "t", version, about = "Run tests on the project")]
	Test(Arguments),
}

#[derive(Args, Clone)]
pub(crate) struct Arguments {
	#[arg(
        short = 'p',
        long = "profile",
        value_enum,
        default_value = Profile::Release,
        help = "Set the build profile"
    )]
	profile: Profile,
	#[arg(
        long,
        default_value_t = default_target(),
        help = "Specify the building target for userland"
    )]
	pub target: String,
	#[arg(
		long,
		default_value_t = std::env::consts::ARCH.to_string(),
		help = "CPU arch used to set cfg(bpf_target_arch) for eBPF compilation"
	)]
	pub arch: String,
	#[arg(
		long,
		help = "Set the linker to use to when building userland application this option is useful when cross-compiling"
	)]
	pub linker: Option<String>,
}

#[derive(Args)]
struct BuildArgs {
	#[clap(flatten)]
	common: Arguments,
	#[arg(
		name = "args",
		last = true,
		help = "Additional build arguments to build userland application (it will not be propagated to eBPF build command)"
	)]
	pub build_args: Vec<String>,
}

#[derive(Args)]
struct RunArgs {
	#[clap(flatten)]
	common: Arguments,
	#[arg(short = 'b', long, help = "Binary to run", default_value = "deeptrace")]
	pub bin: String,
	#[arg(
		long,
		default_value = runner(),
		help = "Runner command to use to run the application"
	)]
	pub runner: String,
	#[arg(name = "args", last = true, help = "Arguments to pass to the application")]
	pub run_args: Vec<String>,
}

impl From<Arguments> for BuildArgs {
	fn from(value: Arguments) -> Self {
		BuildArgs { common: value, build_args: vec![] }
	}
}

#[derive(Args, Clone)]
struct ClippyArgs {
	#[clap(flatten)]
	common: Arguments,
	#[arg(
		name = "args",
		last = true,
		help = "Additional clippy arguments to run on userland application"
	)]
	pub clippy_args: Vec<String>,
}

impl From<ClippyArgs> for BuildArgs {
	fn from(value: ClippyArgs) -> Self {
		BuildArgs { common: value.common.clone(), build_args: value.clippy_args }
	}
}

pub fn run() -> anyhow::Result<()> {
	let opts = Opts::parse();

	match opts.command {
		Command::Build(args) => build::build(
			opts.ignore_ebpf,
			opts.ebpf_dirs,
			opts.target,
			opts.linker,
			opts.link_args,
			args,
		),
		Command::Run(args) => run::run(
			opts.ignore_ebpf,
			opts.ebpf_dirs,
			opts.target,
			opts.linker,
			opts.link_args,
			args,
		),
		Command::Clippy(args) => clippy::clippy(
			opts.ignore_ebpf,
			opts.ebpf_dirs,
			opts.target,
			opts.linker,
			opts.link_args,
			args,
		),
		Command::Test(_args) => Ok(()),
	}
}
