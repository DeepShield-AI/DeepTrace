use anyhow::Context as _;
use aya_build::cargo_metadata::{Metadata, MetadataCommand, Package};

fn main() -> anyhow::Result<()> {
	let Metadata { packages, .. } =
		MetadataCommand::new().no_deps().exec().context("MetadataCommand::exec")?;
	let ebpf: Vec<Package> = packages
		.into_iter()
		.filter(|Package { name, .. }| name.contains("ebpf"))
		.collect();
	aya_build::build_ebpf(ebpf)
}
