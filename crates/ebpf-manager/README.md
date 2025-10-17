# ebpf-loader

A lightweight, practical eBPF program loader and lifecycle manager for observability applications, built on top of [Aya](https://github.com/aya-rs/aya).

## Features

- ✅ **Simple API**: Minimal abstraction over Aya
- ✅ **Kernel Compatibility**: Check version requirements for different hook types
- ✅ **Hook Validation**: Verify tracepoint and kprobe availability
- ✅ **Capability Checking**: Ensure proper BPF permissions
- ✅ **Helper Functions**: Common operations like PID filtering
- ✅ **Logger Management**: Easy eBPF logger initialization

## Design

Inspired by [kunai-project](https://github.com/kunai-project/kunai), this loader focuses on practicality and simplicity rather than over-abstraction. It provides just enough utilities to make working with Aya easier without hiding its API.

## Usage

### Basic Loading

```rust
use aya::include_bytes_aligned;
use ebpf_loader::prelude::*;

fn main() -> Result<()> {
    // Print system information
    print_system_info();
    
    // Load eBPF program
    let mut loader = BpfLoader::new();
    let ebpf = loader.load(include_bytes_aligned!(
        "../target/bpfel-unknown-none/release/my_ebpf"
    ))?;
    
    // Initialize logger
    loader.init_logger()?;
    
    // Attach tracepoint
    helper::attach_tracepoint(
        ebpf,
        "sys_enter_read",
        "syscalls",
        "sys_enter_read"
    )?;
    
    Ok(())
}
```

### Kernel Version Checking

```rust
use ebpf_loader::kernel::{HookType, KernelVersion};

// Check current kernel version
let version = KernelVersion::current()?;
println!("Kernel: {}", version);

// Check if hook type is supported
if !HookType::Fentry.is_supported()? {
    eprintln!("Fentry not supported, falling back to kprobe");
}

// Require specific hook (returns error if not supported)
HookType::TracePoint.require_support()?;
```

### Hook Point Validation

```rust
use ebpf_loader::kernel;

// Check tracepoint availability
if kernel::check_tracepoint("syscalls", "sys_enter_read")? {
    println!("Tracepoint is available");
}

// Check kprobe symbol
if !kernel::check_kprobe_symbol("my_function")? {
    eprintln!("Symbol not found in kernel");
}
```

### Integration with Module Pattern

```rust
use aya::{Ebpf, include_bytes_aligned};
use ebpf_loader::{BpfLoader, helper};

pub struct MyObservModule {
    loader: BpfLoader,
}

impl MyObservModule {
    pub fn new() -> Result<Self, ebpf_loader::BpfError> {
        let mut loader = BpfLoader::new();
        
        loader.load(include_bytes_aligned!(
            "../target/bpfel-unknown-none/release/my_ebpf"
        ))?;
        
        loader.init_logger()?;
        
        Ok(Self { loader })
    }
    
    pub fn start(&mut self) -> Result<(), ebpf_loader::BpfError> {
        let ebpf = self.loader.ebpf_mut().unwrap();
        
        // Attach programs
        helper::attach_tracepoint(ebpf, "prog1", "syscalls", "sys_enter_read")?;
        helper::attach_kprobe(ebpf, "prog2", "do_sys_open", 0)?;
        
        // Configure PID filter
        let pids = vec![1234, 5678];
        helper::config_pid_filter(ebpf, "pids", &pids)?;
        
        Ok(())
    }
}
```

## Hook Type Requirements

| Hook Type      | Minimum Kernel |
|----------------|----------------|
| KProbe         | 4.1            |
| KRetProbe      | 4.1            |
| TracePoint     | 4.7            |
| RawTracePoint  | 4.17           |
| Fentry         | 5.5            |
| Fexit          | 5.5            |
| XDP            | 4.8            |
| TC             | 4.1            |
| LSM            | 5.7            |

## Capability Requirements

- **Kernel >= 5.8**: Requires `CAP_BPF` or `CAP_SYS_ADMIN`
- **Kernel < 5.8**: Requires `CAP_SYS_ADMIN`

## Examples

See the integration in:
- `crates/deeptrace/src/trace/` - Full trace module implementation
- `crates/observ-*/` - Various observability modules

## License

MIT OR Apache-2.0
