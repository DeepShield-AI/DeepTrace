use crate::{network::process::try_socket, utils::is_filtered_pid};
use aya_ebpf::{cty::c_long, macros::tracepoint, programs::TracePointContext};

/// `name`: sys_exit_socket `ID`: 1569
///     
///     field:long ret;;  offset:16;      size:8; signed:1;
#[tracepoint(category = "syscalls", name = "sys_exit_socket")]
fn sys_exit_socket(ctx: TracePointContext) -> u32 {
	let Ok(ret) = (unsafe { ctx.read_at::<c_long>(16) }) else { return 0 };
	if ret < 0 {
		return 0;
	}
	if !is_filtered_pid() {
		return 0;
	}
	let fd = ret as u64;
	try_socket(fd).unwrap_or_else(|ret| ret)
}
