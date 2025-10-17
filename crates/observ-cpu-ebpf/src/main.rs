#![cfg_attr(not(test), no_std, no_main)]

pub use observ_cpu_ebpf;

#[cfg_attr(not(test), panic_handler)]
fn panic(_info: &core::panic::PanicInfo) -> ! {
	loop {}
}
