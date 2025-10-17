use super::{
	BpfTarget, ClippyArgs,
	utils::{nightly_cargo, stable_cargo},
};
use anyhow::Context;
use cargo_metadata::{CompilerMessage, Message};
use std::{
	io::{BufRead as _, BufReader},
	process::{Child, Stdio},
};

/// Clippy command implementation
pub(super) fn clippy(
	ignore_ebpf: bool,
	ebpf_dirs: Vec<String>,
	target: BpfTarget,
	linker: Option<String>,
	link_args: Vec<String>,
	args: ClippyArgs,
) -> anyhow::Result<()> {
	if !ignore_ebpf {
		clippy_ebpf(ebpf_dirs, target, linker, link_args, &args)
			.context("failed to run clippy on eBPF application")?;
	}
	clippy_user(&args).context("failed to run clippy on userland application")
}

fn clippy_user(opts: &ClippyArgs) -> anyhow::Result<()> {
	stable_cargo("clippy", &opts.clone().into())
		.context("failed to run clippy on userland application")
}

fn clippy_ebpf(
	dirs: Vec<String>,
	target: BpfTarget,
	linker: Option<String>,
	link_args: Vec<String>,
	opts: &ClippyArgs,
) -> anyhow::Result<()> {
	for dir in dirs {
		let mut cmd =
			nightly_cargo("clippy", &dir, &target, &linker, &link_args, &opts.clone().into())?;
		let mut child = cmd
			.stdout(Stdio::piped())
			.stderr(Stdio::piped())
			.spawn()
			.with_context(|| format!("failed to spawn {cmd:?}"))?;
		let Child { stdout, stderr, .. } = &mut child;

		// Trampoline stdout to cargo warnings.
		let stderr = stderr.take().expect("stderr");
		let stderr = BufReader::new(stderr);

		let stderr = std::thread::spawn(move || {
			for line in stderr.lines() {
				let line = line.expect("read line");
				eprintln!("{line}");
			}
		});

		let stdout = stdout.take().expect("stdout");
		let stdout = BufReader::new(stdout);

		for message in Message::parse_stream(stdout) {
			// #[allow(clippy::collapsible_match)]
			match message.expect("valid JSON") {
				Message::CompilerMessage(CompilerMessage { message, .. }) => {
					for line in message.rendered.unwrap_or_default().split('\n') {
						println!("{line}");
					}
				},
				Message::TextLine(line) => {
					println!("{line}");
				},
				_ => {},
			}
		}

		let status = child.wait().with_context(|| format!("failed to wait for {cmd:?}"))?;
		if !status.success() {
			return Err(anyhow::anyhow!("{cmd:?} failed: {status:?}"));
		}

		match stderr.join().map_err(std::panic::resume_unwind) {
			Ok(()) => {},
			Err(err) => match err {},
		}
	}

	Ok(())
}
