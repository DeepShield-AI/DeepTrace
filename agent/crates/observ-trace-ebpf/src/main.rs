#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

pub use observ_trace_ebpf;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
	unsafe { core::hint::unreachable_unchecked() }
}
