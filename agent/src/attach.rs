use aya::{Ebpf, programs::TracePoint};

pub(crate) fn attach_socket(ebpf: &mut Ebpf) -> anyhow::Result<()> {
	let sys_enter_close: &mut TracePoint =
		ebpf.program_mut("sys_enter_close").unwrap().try_into()?;
	sys_enter_close.load()?;
	sys_enter_close.attach("syscalls", "sys_enter_close")?;
	Ok(())
}

pub(crate) fn attach_ingress(ebpf: &mut Ebpf) -> anyhow::Result<()> {
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

pub(crate) fn attach_egress(ebpf: &mut Ebpf) -> anyhow::Result<()> {
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
