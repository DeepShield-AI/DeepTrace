use aya::{Ebpf, programs::TracePoint};

use crate::trace::attach;

pub fn attach_tracepoint(ebpf: &mut Ebpf) -> anyhow::Result<()> {
	attach_socket(ebpf)?;
	attach_ingress(ebpf)?;
	attach_egress(ebpf)?;
	attach_cpu_metrics(ebpf)?;
	attach_mem_metrics(ebpf)?;
	attach_io_metrics(ebpf)?;
	println!("Attach tracepoint done");
	Ok(())
}
fn attach_io_metrics(ebpf: &mut Ebpf) -> anyhow::Result<()> {
	let block_rq_insert: &mut TracePoint = ebpf
        .program_mut("block_rq_insert")
        .unwrap()
        .try_into()?;
    block_rq_insert.load()?;
    block_rq_insert.attach("block", "block_rq_insert")?;
    let block_rq_issue: &mut TracePoint = ebpf
        .program_mut("block_rq_issue")
        .unwrap()
        .try_into()?;
    block_rq_issue.load()?;
    block_rq_issue.attach("block", "block_rq_issue")?;

    // 附加 block_rq_complete 程序
    let block_rq_complete: &mut TracePoint = ebpf
        .program_mut("block_rq_complete")
        .unwrap()
        .try_into()?;
    block_rq_complete.load()?;
    block_rq_complete.attach("block", "block_rq_complete")?;

    // 附加 block_rq_merge 程序
    let block_rq_merge: &mut TracePoint = ebpf
        .program_mut("block_rq_merge")
        .unwrap()
        .try_into()?;
    block_rq_merge.load()?;
    block_rq_merge.attach("block", "block_rq_merge")?;

    Ok(())
}
fn attach_mem_metrics(ebpf: &mut Ebpf) -> anyhow::Result<()> {
    // 追踪 mm_vmscan_wakeup_kswapd
    let vmscan_wakeup_kswapd: &mut TracePoint = ebpf
        .program_mut("vmscan_kswapd_wake")
        .unwrap()
        .try_into()?;
    vmscan_wakeup_kswapd.load()?;
    vmscan_wakeup_kswapd.attach("vmscan", "mm_vmscan_wakeup_kswapd")?;

    // 追踪 mm_page_alloc_extfrag
    let page_alloc_extfrag: &mut TracePoint = ebpf
        .program_mut("page_alloc_extfrag")
        .unwrap()
        .try_into()?;
    page_alloc_extfrag.load()?;
    page_alloc_extfrag.attach("kmem", "mm_page_alloc_extfrag")?;

    Ok(())
}
 fn attach_cpu_metrics(ebpf: &mut Ebpf) -> anyhow::Result<()> {
	
    // 追踪软中断入口和出口以计算 ksoftirqd 延迟
    let softirq_entry: &mut TracePoint = ebpf
        .program_mut("softirq_entry")
        .unwrap()
        .try_into()?;
    softirq_entry.load()?;
    softirq_entry.attach("irq", "softirq_entry")?;

    let softirq_exit: &mut TracePoint = ebpf
        .program_mut("softirq_exit")
        .unwrap()
        .try_into()?;
    softirq_exit.load()?;
    softirq_exit.attach("irq", "softirq_exit")?;

    // 追踪 CPU 迁移
    let sched_migrate_task: &mut TracePoint = ebpf
        .program_mut("sched_migrate_task")
        .unwrap()
        .try_into()?;
    sched_migrate_task.load()?;
    sched_migrate_task.attach("sched", "sched_migrate_task")?;

    // 追踪上下文切换
    let sched_switch: &mut TracePoint = 
	ebpf.program_mut("sched_switch").unwrap().try_into()?;
    sched_switch.load()?;
    sched_switch.attach("sched", "sched_switch")?;
	println!("attach finished");
    Ok(())
}
fn attach_socket(ebpf: &mut Ebpf) -> anyhow::Result<()> {
	let sys_exit_socket: &mut TracePoint =
		ebpf.program_mut("sys_exit_socket").unwrap().try_into()?;
	sys_exit_socket.load()?;
	sys_exit_socket.attach("syscalls", "sys_exit_socket")?;

	let sys_enter_close: &mut TracePoint =
		ebpf.program_mut("sys_enter_close").unwrap().try_into()?;
	sys_enter_close.load()?;
	sys_enter_close.attach("syscalls", "sys_enter_close")?;
	Ok(())
}

fn attach_ingress(ebpf: &mut Ebpf) -> anyhow::Result<()> {
	let sys_enter_read: &mut TracePoint = ebpf.program_mut("sys_enter_read").unwrap().try_into()?;
	sys_enter_read.load()?;
	sys_enter_read.attach("syscalls", "sys_enter_read")?;

	let sys_exit_read: &mut TracePoint = ebpf.program_mut("sys_exit_read").unwrap().try_into()?;
	sys_exit_read.load()?;
	sys_exit_read.attach("syscalls", "sys_exit_read")?;

	let sys_enter_readv: &mut TracePoint =
		ebpf.program_mut("sys_enter_readv").unwrap().try_into()?;
	sys_enter_readv.load()?;
	sys_enter_readv.attach("syscalls", "sys_enter_readv")?;

	let sys_exit_readv: &mut TracePoint = ebpf.program_mut("sys_exit_readv").unwrap().try_into()?;
	sys_exit_readv.load()?;
	sys_exit_readv.attach("syscalls", "sys_exit_readv")?;

	let sys_enter_recvfrom: &mut TracePoint =
		ebpf.program_mut("sys_enter_recvfrom").unwrap().try_into()?;
	sys_enter_recvfrom.load()?;
	sys_enter_recvfrom.attach("syscalls", "sys_enter_recvfrom")?;

	let sys_exit_recvfrom: &mut TracePoint =
		ebpf.program_mut("sys_exit_recvfrom").unwrap().try_into()?;
	sys_exit_recvfrom.load()?;
	sys_exit_recvfrom.attach("syscalls", "sys_exit_recvfrom")?;

	let sys_enter_recvmsg: &mut TracePoint =
		ebpf.program_mut("sys_enter_recvmsg").unwrap().try_into()?;
	sys_enter_recvmsg.load()?;
	sys_enter_recvmsg.attach("syscalls", "sys_enter_recvmsg")?;

	let sys_exit_recvmsg: &mut TracePoint =
		ebpf.program_mut("sys_exit_recvmsg").unwrap().try_into()?;
	sys_exit_recvmsg.load()?;
	sys_exit_recvmsg.attach("syscalls", "sys_exit_recvmsg")?;

	let sys_enter_recvmmsg: &mut TracePoint =
		ebpf.program_mut("sys_enter_recvmmsg").unwrap().try_into()?;
	sys_enter_recvmmsg.load()?;
	sys_enter_recvmmsg.attach("syscalls", "sys_enter_recvmmsg")?;

	let sys_exit_recvmmsg: &mut TracePoint =
		ebpf.program_mut("sys_exit_recvmmsg").unwrap().try_into()?;
	sys_exit_recvmmsg.load()?;
	sys_exit_recvmmsg.attach("syscalls", "sys_exit_recvmmsg")?;

	Ok(())
}

fn attach_egress(ebpf: &mut Ebpf) -> anyhow::Result<()> {
	let sys_enter_write: &mut TracePoint =
		ebpf.program_mut("sys_enter_write").unwrap().try_into()?;
	sys_enter_write.load()?;
	sys_enter_write.attach("syscalls", "sys_enter_write")?;

	let sys_exit_write: &mut TracePoint = ebpf.program_mut("sys_exit_write").unwrap().try_into()?;
	sys_exit_write.load()?;
	sys_exit_write.attach("syscalls", "sys_exit_write")?;

	let sys_enter_writev: &mut TracePoint =
		ebpf.program_mut("sys_enter_writev").unwrap().try_into()?;
	sys_enter_writev.load()?;
	sys_enter_writev.attach("syscalls", "sys_enter_writev")?;

	let sys_exit_writev: &mut TracePoint =
		ebpf.program_mut("sys_exit_writev").unwrap().try_into()?;
	sys_exit_writev.load()?;
	sys_exit_writev.attach("syscalls", "sys_exit_writev")?;

	let sys_enter_sendto: &mut TracePoint =
		ebpf.program_mut("sys_enter_sendto").unwrap().try_into()?;
	sys_enter_sendto.load()?;
	sys_enter_sendto.attach("syscalls", "sys_enter_sendto")?;

	let sys_exit_sendto: &mut TracePoint =
		ebpf.program_mut("sys_exit_sendto").unwrap().try_into()?;
	sys_exit_sendto.load()?;
	sys_exit_sendto.attach("syscalls", "sys_exit_sendto")?;

	let sys_enter_sendmsg: &mut TracePoint =
		ebpf.program_mut("sys_enter_sendmsg").unwrap().try_into()?;
	sys_enter_sendmsg.load()?;
	sys_enter_sendmsg.attach("syscalls", "sys_enter_sendmsg")?;

	let sys_exit_sendmsg: &mut TracePoint =
		ebpf.program_mut("sys_exit_sendmsg").unwrap().try_into()?;
	sys_exit_sendmsg.load()?;
	sys_exit_sendmsg.attach("syscalls", "sys_exit_sendmsg")?;

	let sys_enter_sendmmsg: &mut TracePoint =
		ebpf.program_mut("sys_enter_sendmmsg").unwrap().try_into()?;
	sys_enter_sendmmsg.load()?;
	sys_enter_sendmmsg.attach("syscalls", "sys_enter_sendmmsg")?;

	let sys_exit_sendmmsg: &mut TracePoint =
		ebpf.program_mut("sys_exit_sendmmsg").unwrap().try_into()?;
	sys_exit_sendmmsg.load()?;
	sys_exit_sendmmsg.attach("syscalls", "sys_exit_sendmmsg")?;

	Ok(())
}
