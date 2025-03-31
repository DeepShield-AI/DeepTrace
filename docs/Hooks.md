# Overview of Instrumentation Hooks

The eBPF program utilizes a series of hooks to intercept network operations at different stages of data transmission (sending and receiving) and during socket operations. The implementation of these hooks can be found at `agent/src/ebpf/src/network/{syscall_name}.rs`. Below is the categorization of these hooks:

## Receiving Hooks & Sending Hooks

Receiving hooks and sending hooks are implemented to conduct **information collection**.

### Receiving Hooks

Hooks in this category are triggered during the entry and exit points of system calls related to receiving network data. They allow the eBPF program to monitor and act upon data being received by the system. The following hooks are implemented:

- **Read Operations**
  - Entry: `sys_enter_read` in file `ebpf/src/network/read.rs#20`
    - Action: `try_enter(ctx, args, SyscallType::Ingress)`
  - Exit: `sys_exit_read` in file `ebpf/src/network/read.rs#50`
    - Action: `try_exit(ctx, ret, SyscallName::Read, SyscallType::Ingress)`
- **Message Receiving Operations**
  - For `recvmsg`, `recvmmsg`, `readv`, and `recvfrom` operations, both entry and exit points are hooked similarly to read operations, with specific actions tied to each syscall type, allowing for detailed monitoring and control over these receiving paths.

### Sending Hooks

These hooks are activated at the system call entry points related to sending network data. They enable the observation and manipulation of outbound network traffic. The hooks include:

- **Write Operations**
  - Entry: `sys_enter_write` in file `ebpf/src/network/write.rs#21`
    - Action: `try_enter(ctx, args, SyscallType::Egress)`
- **Message Sending Operations**
  - For `sendmsg`, `sendmmsg`, `writev`, and `sendto` operations, entry points are hooked to invoke actions specific to each type of sending operation, facilitating precise intervention in data sending processes.