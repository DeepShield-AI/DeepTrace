use crate::{network::process::try_close, utils::is_filtered_pid};
use aya_ebpf::{cty::c_ulong, macros::tracepoint, programs::TracePointContext};

/// `name`: sys_enter_close `ID`: 700
///     
///     field:unsigned int fd;  offset:16;      size:8; signed:0;
#[tracepoint(category = "syscalls", name = "sys_enter_close")]
fn sys_enter_close(ctx: TracePointContext) -> u32 {
	if !is_filtered_pid() {
		return 0;
	}

	let Ok(fd) = (unsafe { ctx.read_at::<c_ulong>(16) }) else { return 0 };
	if fd < 3 {
		return 0;
	}
	try_close(ctx, fd).unwrap_or_else(|ret| ret)
}
