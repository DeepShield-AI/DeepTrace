/// Ingress syscalls
mod read;
mod readv;
mod recvfrom;
mod recvmmsg;
mod recvmsg;

/// Egress syscalls
mod sendmmsg;
mod sendmsg;
mod sendto;
mod write;
mod writev;

/// Socket
mod close;

/// handle
mod process;
