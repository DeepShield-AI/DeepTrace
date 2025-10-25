use super::{BpfTarget, RunArgs, build};
use std::{os::unix::process::CommandExt, path::PathBuf, process::Command};

/// Run command implementation
pub(super) fn run(
	ignore_ebpf: bool,
	ebpf_dirs: Vec<String>,
	target: BpfTarget,
	linker: Option<String>,
	link_args: Vec<String>,
	opts: RunArgs,
) -> anyhow::Result<()> {
	build::build(ignore_ebpf, ebpf_dirs, target, linker, link_args, opts.common.clone().into())?;

	// the binary path
	let binary = PathBuf::from("target")
		.join(opts.common.target)
		.join(opts.common.profile.to_string())
		.join(opts.bin);

	// configure args
	let mut args: Vec<_> = opts.runner.trim().split_terminator(' ').collect();
	let binary = binary.to_string_lossy();
	args.push(binary.as_ref());
	// arguments to pass to the application
	args.append(&mut opts.run_args.iter().map(String::as_str).collect::<Vec<&str>>());

	// spawn the command
	let error = Command::new(args.first().expect("No first argument"))
		.args(args.iter().skip(1))
		.exec();

	// we shouldn't get here unless the command failed to spawn
	Err(anyhow::Error::from(error).context(format!("Failed to run `{}`", args.join(" "))))
}
