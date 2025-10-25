use aya::Ebpf;
use log::warn;

pub fn init_logger(ebpf: &mut Ebpf) {
	if let Err(e) = aya_log::EbpfLogger::init(ebpf) {
		// This can happen if you remove all log statements from your eBPF program.
		warn!("Failed to initialize eBPF logger: {}", e);
	}
}
