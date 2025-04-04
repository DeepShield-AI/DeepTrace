#![no_std]
#![no_main]
#![allow(unused_imports, dead_code, unused_imports)]
mod consts;
mod maps;
mod network;
mod structs;
mod utils;
mod vmlinux;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
	loop {}
}
