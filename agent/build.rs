use anyhow::{Context as _, anyhow};
use aya_build::cargo_metadata;

fn main() -> anyhow::Result<()> {
	let cargo_metadata::Metadata { packages, .. } = cargo_metadata::MetadataCommand::new()
		.no_deps()
		.exec()
		.context("MetadataCommand::exec")?;
	let ebpf_package = packages
		.into_iter()
		.find(|cargo_metadata::Package { name, .. }| name == "mercury-ebpf")
		.ok_or_else(|| anyhow!("mercury-ebpf package not found"))?;
	aya_build::build_ebpf([ebpf_package])
}
