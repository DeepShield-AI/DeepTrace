#![no_std]
#![no_main]

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
