use crate::vmlinux::task_struct;
use aya_ebpf::{macros::lsm, programs::LsmContext};
use aya_log_ebpf::info;

#[lsm(hook = "task_free")]
pub fn task_free(ctx: LsmContext) -> i32 {
	match unsafe { try_task_free(ctx) } {
		Ok(ret) => ret,
		Err(ret) => ret,
	}
}

unsafe fn try_task_free(ctx: LsmContext) -> Result<i32, i32> {
	let task: *const task_struct = unsafe { ctx.arg(0) };
	let pid = (unsafe { &*task }).pid;
	info!(&ctx, "free pid {}", pid);

	Ok(0)
}
