use crate::{Result, TraceError};
use aya::{
	Ebpf, EbpfLoader, VerifierLogLevel, include_bytes_aligned, maps::HashMap, util::KernelVersion,
};
use ebpf_manager::{kernel, log::init_logger, program::Programs};
use log::{debug, error, info, warn};
use observ_config::ebpf_config;
use observ_trace_common::map::PIDS_MAP;

const BPF_ELF: &[u8] = {
	#[cfg(debug_assertions)]
	let d = include_bytes_aligned!("../../../target/bpfel-unknown-none/debug/observ_trace_ebpf");
	#[cfg(not(debug_assertions))]
	let d = include_bytes_aligned!("../../../target/bpfel-unknown-none/release/observ_trace_ebpf");
	d
};

pub(crate) fn prepare_ebpf() -> Result<Ebpf> {
	let ebpf_log_level = ebpf_config("trace").log_level;
	let mut ebpf = EbpfLoader::new()
		.verifier_log_level(VerifierLogLevel::from_bits_truncate(ebpf_log_level))
		.load(BPF_ELF)?;
	init_logger(&mut ebpf);
	Ok(ebpf)
}

/// Loads eBPF programs in the kernel and attach each program to its attach
pub(crate) fn load_and_attach_bpf(ebpf: &mut Ebpf) -> Result<()> {
	let current_kernel_version =
		KernelVersion::current().map_err(|_| TraceError::KernelVersionError)?;
	let enabled_probes = ebpf_config("trace").enabled_probes;

	// parse eBPF ELF to extract section names
	let mut programs = Programs::with_ebpf(ebpf).with_elf_info(BPF_ELF)?;

	configure_probes(&mut programs);

	// generic program loader
	for (_, p) in programs.sorted_by_prio() {
		// filtering probes to enable (only available in debug)
		if !enabled_probes.is_empty() &&
			enabled_probes.iter().filter(|e| p.name.contains(*e)).count() == 0
		{
			continue;
		}

		// we force enabling of selected probes
		// debug probes are disabled by default
		if !enabled_probes.is_empty() {
			p.enable();
		}

		if !p.enable {
			warn!("{} probe has been disabled", p.name);
			continue;
		}

		if !p.is_compatible(&current_kernel_version) {
			warn!(
				"{} probe is not compatible with current kernel: min={} max={} current={}",
				p.name,
				p.min_kernel_version(),
				p.max_kernel_version(),
				current_kernel_version
			);
			continue;
		}

		info!("loading: {} {:?} with priority={}", p.name, p.prog_type(), p.priority);

		p.load()?;

		// this handles the very specific case where /proc/kallsyms
		// is not available to check if syscore_resume is present
		// In such case attach will fail with a SyscallError and
		// a warning must be shown
		let r = p.attach();

		let _ = r.inspect_err(|e| {
			if let Some(attach_point) = p.attach_point.as_ref() {
				error!(
					"failed to attach probe={} to function={}: verify function exists in your kernel",
					&p.name, &attach_point
				)
			} else {
				error!("failed to attach probe={}", &p.name)
			}

			debug!("error for attach failure: {e}");
		});
	}

	Ok(())
}

pub(crate) fn configure_pids(ebpf: &mut Ebpf, pids: Vec<u32>) -> Result<()> {
	let mut pids_map: HashMap<_, u32, u32> = HashMap::try_from(ebpf.map_mut(PIDS_MAP).unwrap())?;

	for pid in pids {
		pids_map.insert(pid, 0, 0)?;
	}
	Ok(())
}

/// Function managing probe priorities and compatibilities with kernels
///
/// # Panic
///
/// If a given probe name is not found
fn configure_probes(programs: &mut Programs) {
	// socket probes
	programs.program_mut("sys_exit_socket").set_min_kernel_version(kernel!(4, 7));

	programs.program_mut("sys_enter_close").set_min_kernel_version(kernel!(4, 7));

	// ingress probes
	programs.program_mut("sys_enter_read").set_min_kernel_version(kernel!(4, 7));

	programs
		.program_mut("sys_exit_read")
		.set_min_kernel_version(kernel!(4, 7))
		.set_priority(51);

	programs.program_mut("sys_enter_readv").set_min_kernel_version(kernel!(4, 7));

	programs
		.program_mut("sys_exit_readv")
		.set_min_kernel_version(kernel!(4, 7))
		.set_priority(51);

	programs.program_mut("sys_enter_recvfrom").set_min_kernel_version(kernel!(4, 7));

	programs
		.program_mut("sys_exit_recvfrom")
		.set_min_kernel_version(kernel!(4, 7))
		.set_priority(51);

	programs.program_mut("sys_enter_recvmsg").set_min_kernel_version(kernel!(4, 7));

	programs
		.program_mut("sys_exit_recvmsg")
		.set_min_kernel_version(kernel!(4, 7))
		.set_priority(51);

	programs.program_mut("sys_enter_recvmmsg").set_min_kernel_version(kernel!(4, 7));

	programs
		.program_mut("sys_exit_recvmmsg")
		.set_min_kernel_version(kernel!(4, 7))
		.set_priority(51);

	// egress probes
	programs.program_mut("sys_enter_write").set_min_kernel_version(kernel!(4, 7));

	programs
		.program_mut("sys_exit_write")
		.set_min_kernel_version(kernel!(4, 7))
		.set_priority(51);

	programs.program_mut("sys_enter_writev").set_min_kernel_version(kernel!(4, 7));

	programs
		.program_mut("sys_exit_writev")
		.set_min_kernel_version(kernel!(4, 7))
		.set_priority(51);

	programs.program_mut("sys_enter_sendto").set_min_kernel_version(kernel!(4, 7));

	programs
		.program_mut("sys_exit_sendto")
		.set_min_kernel_version(kernel!(4, 7))
		.set_priority(51);

	programs.program_mut("sys_enter_sendmsg").set_min_kernel_version(kernel!(4, 7));

	programs
		.program_mut("sys_exit_sendmsg")
		.set_min_kernel_version(kernel!(4, 7))
		.set_priority(51);

	programs.program_mut("sys_enter_sendmmsg").set_min_kernel_version(kernel!(4, 7));

	programs
		.program_mut("sys_exit_sendmmsg")
		.set_min_kernel_version(kernel!(4, 7))
		.set_priority(51);
}
