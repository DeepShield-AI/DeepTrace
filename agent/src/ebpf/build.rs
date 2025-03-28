use which::which;

/// Building this crate has an undeclared dependency on the `bpf-linker` binary. This would be
/// better expressed by [artifact-dependencies][bindeps] but issues such as
/// https://github.com/rust-lang/cargo/issues/12385 make their use impractical for the time being.
///
/// This file implements an imperfect solution: it causes cargo to rebuild the crate whenever the
/// mtime of `which bpf-linker` changes. Note that possibility that a new bpf-linker is added to
/// $PATH ahead of the one used as the cache key still exists. Solving this in the general case
/// would require rebuild-if-changed-env=PATH *and* rebuild-if-changed={every-directory-in-PATH}
/// which would likely mean far too much cache invalidation.
///
/// [bindeps]: https://doc.rust-lang.org/nightly/cargo/reference/unstable.html?highlight=feature#artifact-dependencies
// fn get_branch() -> Result<String> {
//     if let Ok(branch) = env::var("GITHUB_REF_NAME") {
//         return Ok(branch);
//     }
//
//     let output = Command::new("git")
//         .args(["branch", "--show-current"])
//         .output()?;
//     if output.status.success() {
//         return Ok(String::from_utf8(output.stdout)?);
//     }
//
//     let output = Command::new("git")
//         .args(["rev-parse", "--abbrev-ref", "HEAD"])
//         .output()?;
//     if output.status.success() && &output.stdout != "HEAD".as_bytes() {
//         return Ok(String::from_utf8(output.stdout)?);
//     }
//
//     let output = Command::new("git")
//         .args(["log", "-n", "1", "--pretty=%D", "HEAD"])
//         .output()?;
//     if output.status.success() {
//         // output: HEAD -> master, origin/main
//         return match output.stdout.iter().position(|x| *x == ',' as u8) {
//             Some(mut position) => {
//                 while (output.stdout[position] as char).is_ascii_whitespace()
//                     && position < output.stdout.len()
//                 {
//                     position += 1;
//                 }
//                 Ok(str::from_utf8(&output.stdout[position..])?.to_owned())
//             }
//             _ => Ok(String::from_utf8(output.stdout)?),
//         };
//     }
//
//     panic!("no branch name found")
// }

// fn set_build_info() -> Result<()> {
//     // println!("cargo:rustc-env=AGENT_NAME=deepflow-agent-ce");
//     // println!("cargo:rustc-env=BRANCH={}", get_branch()?);
//     // println!(
//     //     "cargo:rustc-env=COMPILE_TIME={}",
//     //     Local::now().format("%F %T")
//     // );
//     // let entries = vec![
//     //     EnvCommand("COMMIT_ID", vec!["git", "rev-parse", "HEAD"]),
//     //     EnvCommand("REV_COUNT", vec!["git", "rev-list", "--count", "HEAD"]),
//     //     EnvCommand("RUSTC_VERSION", vec!["rustc", "--version"]),
//     // ];
//     // for e in entries {
//     //     let output = Command::new(e.1[0]).args(&e.1[1..]).output()?.stdout;
//     //     println!("cargo:rustc-env={}={}", e.0, String::from_utf8(output)?);
//     // }
//     Ok(())
// }

// libtrace scatters generated files in different folders, making it difficult to watch a single folder for changes
//
// rerun build script when one of the following file changes
// - C source files, except for
//   - generated bpf bytecode files (socket_trace_*.c / perf_profiler_*.c)
//   - java agent so files and jattach bin
// - Header files
// - `src/ebpf/mod.rs` (to exlude rust sources in `samples` folder)
// - Makefiles
// fn set_libtrace_rerun_files() -> Result<()> {
//     fn watched(path: &Path) -> bool {
//         if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
//             match ext {
//                 "c" => {
//                     if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
//                         if name.starts_with("socket_trace_") || name.starts_with("perf_profiler_") {
//                             return false;
//                         }
//                         if name.starts_with("java_agent_so_") {
//                             return false;
//                         }
//                         if name == "deepflow_jattach_bin.c" {
//                             return false;
//                         }
//                         return true;
//                     }
//                 }
//                 "h" => return true,
//                 _ => (),
//             }
//         }
//         if path == Path::new("src/ebpf/mods.rs") {
//             return true;
//         }
//         if let Some(name) = path.file_name() {
//             if name == "Makefile" {
//                 return true;
//             }
//         }
//         false
//     }
//     // let base_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
//     // for entry in WalkDir::new(base_dir.join("src/ebpf")) {
//     //     let entry = entry?;
//     //     let relative_path = entry.path().strip_prefix(&base_dir)?;
//     //     if !watched(relative_path) {
//     //         continue;
//     //     }
//     //     println!("cargo:rerun-if-changed={}", relative_path.display());
//     // }
//     Ok(())
// }

// fn set_build_libtrace() -> Result<()> {
//     set_libtrace_rerun_files()?;
//     let output = Command::new("sh")
//         .arg("-c")
//         .arg("cd c && make clean && make --no-print-directory")
//         .output()?;
//     // let output = match env::var("CARGO_CFG_TARGET_ENV")?.as_str() {
//     //     "gnu" => Command::new("sh").arg("-c")
//     //         .arg("cd c && make clean && make --no-print-directory")
//     //         .output()?,
//     //     "musl" => Command::new("sh").arg("-c")
//     //         .arg("cd src/ebpf && make clean && CC=musl-gcc CLANG=musl-clang make --no-print-directory && CC=musl-gcc CLANG=musl-clang make tools --no-print-directory")
//     //         .output()?,
//     //     _ => panic!("Unsupported target :{}", env::var("CARGO_CFG_TARGET_ENV")?),
//     // };
//     if !output.status.success() {
//         eprintln!("{}", str::from_utf8(&output.stderr)?);
//         panic!("compile libtrace.a error!");
//     }
//     let library_name = "utils";
//     let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
//     let library_dir = PathBuf::from(root.join("c"));
//     println!("cargo:rustc-link-lib=dylib={}", library_name);
//     println!(
//         "cargo:rustc-link-search=native={}",
//         env::join_paths(&[library_dir])?.to_str().unwrap()
//     );
//     Ok(())
// }

// fn set_linkage() -> Result<()> {
// 	let target_env = env::var("CARGO_CFG_TARGET_ENV")?;
// 	if target_env.as_str() == "musl" {
// 		#[cfg(target_arch = "x86_64")]
// 		println!("cargo:rustc-link-search=native=/usr/x86_64-linux-musl/lib64");

// 		#[cfg(target_arch = "aarch64")]
// 		println!("cargo:rustc-link-search=native=/usr/aarch64-linux-musl/lib64");
// 	}
// 	println!("cargo:rustc-link-search=native=/usr/lib");
// 	println!("cargo:rustc-link-search=native=/usr/lib64");

// println!("cargo:rustc-link-lib=static=GoReSym");

// #[cfg(target_arch = "x86_64")]
// println!("cargo:rustc-link-lib=static=bddisasm");

// println!("cargo:rustc-link-lib=static=dwarf");
// println!("cargo:rustc-link-lib=static=bcc_bpf");

// println!("cargo:rustc-link-lib=static=elf");

// match target_env.as_str() {
// "gnu" => {
// println!("cargo:rustc-link-lib=static=bcc");
// println!("cargo:rustc-link-lib=dylib=pthread");
// println!("cargo:rustc-link-lib=dylib=z");
// println!("cargo:rustc-link-lib=dylib=stdc++");
// #[cfg(feature = "dylib_pcap")]
// println!("cargo:rustc-link-lib=dylib=pcap");
// #[cfg(not(feature = "dylib_pcap"))]
// println!("cargo:rustc-link-lib=static=pcap");
// }
// "musl" => {
// #[cfg(target_arch = "x86_64")]
// println!("cargo:rustc-link-lib=static=bcc");

// #[cfg(target_arch = "x86_64")]
// println!("cargo:rustc-link-lib=static=stdc++");

// println!("cargo:rustc-link-lib=static=pcap");
// println!("cargo:rustc-link-lib=static=c");
// println!("cargo:rustc-link-lib=static=elf");
// println!("cargo:rustc-link-lib=static=m");
// println!("cargo:rustc-link-lib=static=z");
// println!("cargo:rustc-link-lib=static=pthread");
// println!("cargo:rustc-link-lib=static=rt");
// println!("cargo:rustc-link-lib=static=dl");
// }
// _ => panic!("Unsupported target"),
// }
// 	Ok(())
// }

fn main() {
	// set_build_info().unwrap();
	// set_build_libtrace().unwrap();
	// set_linkage().unwrap();
	let bpf_linker = which("bpf-linker").unwrap();
	println!("cargo:rerun-if-changed={}", bpf_linker.to_str().unwrap());
	// println!("cargo:rustc-link-search=native=/home/ubuntu/smore/mercury/mercury-ebpf/c/utils");
	// println!("cargo:rustc-link-lib=static=utils");
	// // println!("cargo:rerun-if-changed=c/my_c_code.h");
	// cc::Build::new()
	//     .file("c/utils.c")
	//     .include("c/include")
	//     .compile("utils");
}
