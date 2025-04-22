#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(dead_code)]
#![allow(clippy::empty_loop)]
mod maps;
mod network;
mod protocols;
mod structs;
mod utils;
mod vmlinux;

#[cfg_attr(not(test), panic_handler)]
fn panic(_info: &core::panic::PanicInfo) -> ! {
	loop {}
}
