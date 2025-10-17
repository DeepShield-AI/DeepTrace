use super::{
	BpfTarget, BuildArgs,
	utils::{nightly_cargo, stable_cargo},
};
use anyhow::Context;

/// Build command implementation
pub(super) fn build(
	ignore_ebpf: bool,
	ebpf_dirs: Vec<String>,
	target: BpfTarget,
	linker: Option<String>,
	link_args: Vec<String>,
	args: BuildArgs,
) -> anyhow::Result<()> {
	if !ignore_ebpf {
		build_ebpf(ebpf_dirs, target, linker, link_args, &args)?;
	}
	build_user(&args)
}

/// Build the userspace project
fn build_user(args: &BuildArgs) -> anyhow::Result<()> {
	stable_cargo("build", args).context("failed to build userland application")
}

fn build_ebpf(
	dirs: Vec<String>,
	target: BpfTarget,
	linker: Option<String>,
	link_args: Vec<String>,
	args: &BuildArgs,
) -> anyhow::Result<()> {
	for dir in dirs {
		let mut cmd = nightly_cargo("build", &dir, &target, &linker, &link_args, args)
			.context("failed to build eBPF application")?;
		let status = cmd.status().context("failed to run nightly cargo. Make sure `cargo` is in PATH or set the `CARGO` environment variable to the path of the cargo executable")?;
		if !status.success() {
			return Err(anyhow::anyhow!("cargo build failed with {status}"));
		}
	}
	Ok(())
}
