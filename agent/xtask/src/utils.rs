use super::{BpfTarget, BuildArgs, Profile};
use anyhow::Context;
use std::{
	env::{VarError, var},
	io::ErrorKind,
	path::PathBuf,
	process::Command,
};

pub(crate) fn stable_cargo(command: &str, opts: &BuildArgs) -> anyhow::Result<()> {
	let mut args = vec![command.to_string()];

	args.push(format!("--profile={}", opts.common.profile));

	args.push(format!("--target={}", opts.common.target));

	opts.build_args.iter().for_each(|build_arg| args.push(build_arg.clone()));

	let mut rustflags = rustflags()?;

	if let Some(linker) = opts.common.linker.as_ref() {
		rustflags.push(format!("-C linker={linker}"))
	}

	let mut cmd = Command::new("cargo");
	if !rustflags.is_empty() {
		cmd.env("RUSTFLAGS", rustflags.join(" "));
	}

	let status = cmd.args(&args).status().expect("failed to run stable cargo");
	if !status.success() {
		return Err(anyhow::anyhow!("cargo {command} failed with {status}"));
	}
	Ok(())
}

pub(crate) fn nightly_cargo(
	command: &str,
	dir: &str,
	target: &BpfTarget,
	linker: &Option<String>,
	link_args: &[String],
	opts: &BuildArgs,
) -> anyhow::Result<Command> {
	let mut args = vec![
		command.to_string(),
		format!("--target={}", target),
		"-Z".into(),
		"build-std=core".into(),
		// "--message-format=json".into(),
	];

	match opts.common.profile {
		Profile::Debug => args.push("--debug".into()),
		Profile::Release => args.push("--release".into()),
	}

	// opts.build_args.iter().for_each(|arg| args.push(arg.clone()));

	// Command::new creates a child process which inherits all env variables. This means env
	// vars set by the cargo xtask command are also inherited. RUSTUP_TOOLCHAIN and CARGO are removed
	// so the rust-toolchain.toml file in the xxx-ebpf folder is honored.
	let mut cmd = Command::new("cargo");
	cmd.current_dir(PathBuf::from(dir))
		.env_remove("RUSTUP_TOOLCHAIN")
		.env_remove("CARGO")
		.args(&args)
		.env(
			"RUSTFLAGS",
			ebpf_rustflags(&opts.common.profile, &opts.common.arch, target, linker, link_args)?,
		);
	Ok(cmd)
}

fn mandatory_rustflags(
	arch: &str,
	linker: &Option<String>,
	link_args: &[String],
) -> anyhow::Result<Vec<String>> {
	let mut rustflags = rustflags()?;

	if let Some(linker) = linker {
		rustflags.push(format!("-C linker={linker}"));
	}

	// setting specific config bpf_target_arch
	// do it here so that we don't have to do it in several build.rs files
	rustflags.push(format!(r#"--cfg bpf_target_arch="{arch}""#));
	rustflags.push(
		"--check-cfg=cfg(bpf_target_arch,values(\"x86_64\",\"arm\",\"aarch64\",\"riscv64\"))"
			.into(),
	);
	// add linker arguments
	link_args
		.iter()
		.for_each(|link_arg| rustflags.push(format!("-C link-arg={link_arg}")));

	// enable BTF emission
	rustflags.push("-C link-arg=--btf".into());

	Ok(rustflags)
}

fn ebpf_rustflags(
	profile: &Profile,
	arch: &str,
	target: &BpfTarget,
	linker: &Option<String>,
	link_args: &[String],
) -> anyhow::Result<String> {
	let mut rustflags = mandatory_rustflags(arch, linker, link_args)?;

	let profile = match profile {
		Profile::Release => "release",
		_ => "debug",
	};

	let linker_out_dir = {
		let t = PathBuf::from("target").join(target.to_string()).join(profile).join("linker");
		std::fs::create_dir_all(&t).context("Failed to create target directory")?;
		t.canonicalize().context("Failed to canonicalize target directory")?
	};

	// bpf-linker log file
	let log_file = linker_out_dir.join("bpf-linker.log");
	// ignore NotFound error
	if let Err(e) = std::fs::remove_file(&log_file) &&
		e.kind() != ErrorKind::NotFound
	{
		return Err(e.into());
	}
	// bpf-linker dump directory
	let dump_dir = linker_out_dir.join("dump_module");

	// do not override any previous rustflags set in command line
	for (opt, value) in [
		("-C link-arg=--log-level", "info"),
		("-C link-arg=--log-file", log_file.to_string_lossy().as_ref()),
		("-C link-arg=--dump-module", dump_dir.to_string_lossy().as_ref()),
	] {
		if !rustflags.iter().any(|s| s.contains(opt)) {
			rustflags.push(format!("{opt}={value}"))
		}
	}

	Ok(rustflags.join(" "))
}

fn rustflags() -> anyhow::Result<Vec<String>> {
	match var("RUSTFLAGS") {
		Ok(s) => Ok(vec![s]),
		Err(e) => match e {
			VarError::NotPresent => Ok(vec![]),
			_ => Err(e.into()),
		},
	}
}

pub(super) fn default_target() -> String {
	let output = Command::new("rustc").arg("-vV").output().expect("Failed to execute rustc -vV");

	if !output.status.success() {
		eprintln!("rustc -vV failed: {:?}", output);
		return "unknown-unknown-unknown".to_string()
	}

	for line in String::from_utf8(output.stdout).expect("Invalid UTF-8 from rustc").lines() {
		if line.starts_with("host: ") {
			return line.trim_start_matches("host: ").trim().to_string()
		}
	}

	eprintln!("Could not find host target in rustc output");
	"unknown-unknown-unknown".to_string()
}

pub(super) const fn runner() -> &'static str {
	if cfg!(unix) { "sudo -E" } else { "" }
}
