use aya_ebpf::helpers::bpf_get_current_comm;

#[inline(always)]
pub fn is_filtered_comm() -> bool {
	bpf_get_current_comm().is_ok_and(|comm| {
		&comm[..4] == b"ssh\0" || &comm[..4] == b"scp\0" || &comm[..5] == b"sshd\0"
	})
}
