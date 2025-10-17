#[macro_export]
macro_rules! bpf {
    ($($tokens:tt)*) => {
        cfg_if::cfg_if!{
            if #[cfg(any(target_arch = "bpf"))] {
                $($tokens)*
            }
        }
    };
}

#[macro_export]
macro_rules! not_bpf {
    ($($tokens:tt)*) => {
        cfg_if::cfg_if!{
            // negating target_arch = "bpf" causes IDE macro analysis not working properly (no autocomplete/help)
            if #[cfg(any(target_arch = "x86_64", target_arch="x86", target_arch="mips", target_arch="powerpc", target_arch="powerpc64", target_arch="arm", target_arch="aarch64"))] {
                // identity
                $($tokens)*
            }
        }
    };
}
