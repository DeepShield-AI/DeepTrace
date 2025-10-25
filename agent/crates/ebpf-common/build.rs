use bindgen::builder;
use std::{env, fs, path::Path, process::Command};

fn bindgen<P: AsRef<Path>, Q: AsRef<Path>>(file: P, out_dir: Q) {
	let out_file = out_dir.as_ref().join("generate.rs");
	// TODO: check clang args
	let bindings = builder()
		.header(file.as_ref().to_string_lossy())
		.layout_tests(false) // --no-layout-tests
		.use_core() // --use-core
		.allowlist_function("shim_.*")
		.size_t_is_usize(false) // --no-size_t-is-usize
		.clang_arg("-target")
		.clang_arg("bpf")
		.clang_arg("-I")
		.clang_arg("src/shim")
		.clang_arg("-I")
		.clang_arg("src/shim/include")
		.clang_arg("-I")
		.clang_arg("src/shim/include/linux")
		.disable_header_comment()
		.generate()
		.expect("failed at generating bindings");

	fs::create_dir_all(out_dir).expect("failed to create Rust shim output directory");

	bindings.write_to_file(out_file).expect("failed at writing generated bindings");
}

fn main() {
	let out_dir = env::var("OUT_DIR").unwrap();
	let shim_file = Path::new("src/shim/shim.c");
	bindgen(shim_file, "src/co_re");

	if env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "bpf" {
		// TODO: check clang args
		let status = Command::new("clang")
			.arg("-I")
			.arg("src/shim")
			.arg("-I")
			.arg("src/shim/include")
			.arg("-I")
			.arg("src/shim/include/linux")
			.arg("-O2")
			.arg("-emit-llvm")
			.arg("-target")
			.arg("bpf")
			.arg("-c")
			.arg("-g") // Enable debug info for CO-RE
			.arg("-fno-debug-info-for-profiling") // Disable problematic debug info
			.arg("-mno-relax") // Disable relaxation that might cause BTF issues
			.arg(shim_file)
			.arg("-o")
			.arg(format!("{out_dir}/shim.o"))
			.status()
			.expect("failed to execute clang");

		if !status.success() {
			panic!("failed to compile C-shim")
		}

		println!("cargo:rustc-link-search=native={out_dir}");
		println!("cargo:rustc-link-lib=link-arg={out_dir}/shim.o");
	}

	println!("cargo:rerun-if-changed={}", shim_file.to_string_lossy());
}
