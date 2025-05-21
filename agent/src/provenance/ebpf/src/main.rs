#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(dead_code)]
#![allow(clippy::empty_loop)]

#[cfg_attr(not(test), panic_handler)]
fn panic(_info: &core::panic::PanicInfo) -> ! {
	loop {}
}
