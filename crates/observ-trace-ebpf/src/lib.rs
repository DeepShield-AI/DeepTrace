// This file exists to enable the library target.
#![no_std]

mod maps;

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
mod socket;

/// handle
mod process;

mod types;
mod utils;
