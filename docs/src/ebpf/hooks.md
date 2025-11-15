# System Hooks

DeepTrace's eBPF implementation uses tracepoint-based system call hooks to intercept and monitor network operations. Built with the Aya framework, these hooks provide non-intrusive monitoring of network I/O operations for distributed tracing.

## Hook Architecture

DeepTrace employs a **dual-phase tracepoint strategy** using Linux tracepoints:

1. **Entry Tracepoints** (`sys_enter_*`): Capture system call parameters and context
2. **Exit Tracepoints** (`sys_exit_*`): Extract actual data and build trace messages

```mermaid
graph LR
    APP[Application] --> SYSCALL[System Call]
    SYSCALL --> ENTER[sys_enter_* Tracepoint]
    ENTER --> KERNEL[Kernel Processing]
    KERNEL --> EXIT[sys_exit_* Tracepoint]
    EXIT --> USERSPACE[User Space Agent]
```

## Implementation Framework

### Aya Tracepoint Macros

DeepTrace uses Aya's tracepoint macros for hook implementation:

```rust
use aya_ebpf::{
    macros::tracepoint,
    programs::TracePointContext,
};

#[tracepoint(category = "syscalls", name = "sys_enter_read")]
fn sys_enter_read(ctx: TracePointContext) -> u32 {
    // Entry processing logic
}

#[tracepoint(category = "syscalls", name = "sys_exit_read")]
fn sys_exit_read(ctx: TracePointContext) -> u32 {
    // Exit processing logic
}
```

## Monitored System Calls

DeepTrace monitors **10 critical network system calls** divided into two categories:

### Ingress Operations (Data Receiving)

These hooks capture incoming network data and responses:

#### 1. `read()` System Call

**Purpose**: Monitor data reading from file descriptors

**Implementation Location**: `observ-trace-ebpf/src/read.rs`

**Entry Hook**:
```rust
#[tracepoint(category = "syscalls", name = "sys_enter_read")]
fn sys_enter_read(ctx: TracePointContext) -> u32 {
    if !is_filtered_pid() {
        return 0;
    }

    let timestamp = unsafe { bpf_ktime_get_ns() };
    let Ok(fd) = (unsafe { ctx.read_at::<c_ulong>(16) }) else { return 0 };
    if fd < 3 {
        return 0;  // Skip stdin, stdout, stderr
    }
    
    let buf = match unsafe { ctx.read_at::<c_ulong>(24) } {
        Ok(buf) if buf != 0 => buf as *mut u8,
        _ => return 0,
    };
    
    let count = match unsafe { ctx.read_at::<c_ulong>(32) } {
        Ok(count) if count != 0 => count as u32,
        _ => return 0,
    };
    
    let Ok(seq) = read_seq(fd) else { return 0 };
    let args = Args::from_ubuf(fd, buf, count, timestamp, seq);
    try_or_log!(&ctx, try_enter(args, Direction::Ingress))
}
```

**Exit Hook**:
```rust
#[tracepoint(category = "syscalls", name = "sys_exit_read")]
fn sys_exit_read(ctx: TracePointContext) -> u32 {
    if !is_filtered_pid() {
        return 0;
    }

    let Ok(ret) = (unsafe { ctx.read_at::<c_long>(16) }) else { return 0 };
    try_or_log!(&ctx, try_exit(&ctx, ret, Syscall::Read, Direction::Ingress))
}
```

**Captured Data**:
- File descriptor (offset 16)
- Buffer pointer (offset 24)
- Read count (offset 32)
- Return value (bytes read)
- TCP sequence number
- Timestamp information

#### 2. `recvmsg()` System Call

**Purpose**: Intercept message reception from sockets

**Implementation Location**: `observ-trace-ebpf/src/recvmsg.rs`

**Entry Hook**:
```rust
#[tracepoint(category = "syscalls", name = "sys_enter_recvmsg")]
fn sys_enter_recvmsg(ctx: TracePointContext) -> u32 {
    if !is_filtered_pid() {
        return 0;
    }

    let timestamp = unsafe { bpf_ktime_get_ns() };
    let Ok(fd) = (unsafe { ctx.read_at::<c_ulong>(16) }) else { return 0 };
    
    // Extract msghdr structure using CO-RE
    let (vec, vlen) = match unsafe { ctx.read_at::<c_ulong>(24) } {
        Ok(msg) if msg != 0 => {
            let msg = user_msghdr::from_ptr(msg as *const _);
            match (msg.msg_iov(), msg.msg_iovlen()) {
                (Some(vec), Some(vlen)) if !vec.is_null() && vlen != 0 => 
                    (vec, vlen as u32),
                _ => return 0,
            }
        },
        _ => return 0,
    };
    
    let Ok(seq) = read_seq(fd) else { return 0 };
    let args = Args::from_msg(fd, vec, vlen, timestamp, seq);
    try_or_log!(&ctx, try_enter(args, Direction::Ingress))
}
```

**Key Features**:
- **CO-RE Support**: Uses `user_msghdr` for kernel compatibility
- **iovec Extraction**: Extracts `msg_iov` and `msg_iovlen` fields
- **Type Safety**: Rust-based implementation with error handling
- **Memory Safety**: Safe pointer handling with null checks

**Data Extraction**:
- **fd** (offset 16): File descriptor
- **msg** (offset 24): Pointer to `user_msghdr` structure
- **msg_iov**: Vector of I/O buffers (`iovec` array)
- **msg_iovlen**: Number of `iovec` entries

#### 3. `recvmmsg()` System Call

**Purpose**: Monitor multiple message reception

**Advantages**:
- Batch processing efficiency
- Reduced system call overhead
- Better performance for high-throughput applications

#### 4. `readv()` System Call

**Purpose**: Vectored read operations

**Special Handling**:
- Multiple buffer support
- Scatter-gather I/O
- Complex buffer reconstruction

#### 5. `recvfrom()` System Call

**Purpose**: Receive data with source address information

**Additional Data**:
- Source address extraction
- UDP packet handling
- Connectionless protocol support

### Egress Operations (Data Sending)

These hooks capture outgoing network data and requests:

#### 6. `write()` System Call

**Purpose**: Monitor data writing to file descriptors

**Implementation Location**: `observ-trace-ebpf/src/write.rs`

**Entry Hook**:
```rust
#[tracepoint(category = "syscalls", name = "sys_enter_write")]
fn sys_enter_write(ctx: TracePointContext) -> u32 {
    if !is_filtered_pid() {
        return 0;
    }

    let timestamp = unsafe { bpf_ktime_get_ns() };
    let Ok(fd) = (unsafe { ctx.read_at::<c_ulong>(16) }) else { return 0 };
    if fd < 3 {
        return 0;  // Skip stdin, stdout, stderr
    }
    
    let buf = match unsafe { ctx.read_at::<c_ulong>(24) } {
        Ok(buf) if buf != 0 => buf as *mut u8,
        _ => return 0,
    };
    
    let count = match unsafe { ctx.read_at::<c_ulong>(32) } {
        Ok(count) if count != 0 => count as u32,
        _ => return 0,
    };
    
    let Ok(seq) = write_seq(fd) else { return 0 };
    let args = Args::from_ubuf(fd, buf, count, timestamp, seq);
    try_or_log!(&ctx, try_enter(args, Direction::Egress))
}
```

**Exit Hook**:
```rust
#[tracepoint(category = "syscalls", name = "sys_exit_write")]
fn sys_exit_write(ctx: TracePointContext) -> u32 {
    if !is_filtered_pid() {
        return 0;
    }

    let Ok(ret) = (unsafe { ctx.read_at::<c_long>(16) }) else { return 0 };
    try_or_log!(&ctx, try_exit(&ctx, ret, Syscall::Write, Direction::Egress))
}
```

**Key Features**:
- **Process Filtering**: Only monitors filtered PIDs
- **FD Validation**: Skips standard I/O file descriptors (0, 1, 2)
- **Write Sequence**: Tracks TCP write sequence numbers
- **Type Safety**: Rust-based implementation with error handling
- **Memory Safety**: Safe pointer handling and validation

**Captured Data**:
- **fd** (offset 16): File descriptor
- **buf** (offset 24): Buffer pointer
- **count** (offset 32): Write count
- **Return value**: Bytes written
- **TCP sequence number**: For correlation

#### 7. `sendmsg()` System Call

**Purpose**: Intercept message transmission through sockets

**Implementation Location**: `observ-trace-ebpf/src/sendmsg.rs`

**Entry Hook**:
```rust
#[tracepoint(category = "syscalls", name = "sys_enter_sendmsg")]
fn sys_enter_sendmsg(ctx: TracePointContext) -> u32 {
    if !is_filtered_pid() {
        return 0;
    }

    let timestamp = unsafe { bpf_ktime_get_ns() };
    let Ok(fd) = (unsafe { ctx.read_at::<c_ulong>(16) }) else { return 0 };

    // Extract msghdr structure using CO-RE
    let (vec, vlen) = match unsafe { ctx.read_at::<c_ulong>(24) } {
        Ok(msg) if msg != 0 => {
            let msg = user_msghdr::from_ptr(msg as *const _);
            match (msg.msg_iov(), msg.msg_iovlen()) {
                (Some(vec), Some(vlen)) if !vec.is_null() && vlen != 0 => 
                    (vec, vlen as u32),
                _ => return 0,
            }
        },
        _ => return 0,
    };
    
    let Ok(seq) = write_seq(fd) else { return 0 };
    let args = Args::from_msg(fd, vec, vlen, timestamp, seq);
    try_or_log!(&ctx, try_enter(args, Direction::Egress))
}
```

**Key Features**:
- **CO-RE Support**: Uses `user_msghdr` for kernel compatibility
- **iovec Processing**: Handles vectored I/O operations
- **Write Sequence**: Tracks TCP write sequence numbers
- **Type Safety**: Rust-based implementation with error handling

#### 8. `sendmmsg()` System Call

**Purpose**: Monitor multiple message transmission

**Benefits**:
- Batch operation support
- High-performance scenarios
- Reduced kernel transitions

#### 9. `writev()` System Call

**Purpose**: Vectored write operations

**Complexity**:
- Multiple buffer aggregation
- Efficient data reconstruction
- Memory-efficient processing

#### 10. `sendto()` System Call

**Purpose**: Send data to specific destinations

**Use Cases**:
- UDP communication
- Connectionless protocols
- Direct addressing

## Hook Implementation Details

### Entry Phase Processing

When a system call enters, the hook performs:

```rust
// From process.rs
#[inline(always)]
pub fn try_enter(args: Args, direction: Direction) -> Result<u32> {
    let id = bpf_get_current_pid_tgid();

    // 1. Select appropriate map based on direction
    let map = match direction {
        Direction::Ingress => unsafe { &INGRESS },
        Direction::Egress => unsafe { &EGRESS },
        Direction::Unknown => return Err(INVALID_DIRECTION),
    };

    // 2. Store context for exit processing
    map.insert(&id, &args, 0).map_err(|_| MAP_INSERT_FAILED)?;
    Ok(0)
}
```

**Entry Processing Steps**:
1. **Process Filtering**: Check `is_filtered_pid()` before processing
2. **Timestamp Capture**: Record entry time with `bpf_ktime_get_ns()`
3. **Parameter Extraction**: Extract fd, buffer, and count from tracepoint context
4. **Sequence Number**: Get TCP sequence number for correlation
5. **Args Construction**: Build `Args` structure with all context
6. **Map Storage**: Store in INGRESS or EGRESS map for exit processing

### Exit Phase Processing

When a system call exits, the hook performs:

```rust
// From process.rs
#[inline(always)]
pub fn try_exit(
    ctx: &TracePointContext,
    ret: c_long,
    syscall: Syscall,
    direction: Direction,
) -> Result<u32> {
    let id = bpf_get_current_pid_tgid();
    let map = match direction {
        Direction::Ingress => unsafe { &INGRESS },
        Direction::Egress => unsafe { &EGRESS },
        Direction::Unknown => return Err(INVALID_DIRECTION),
    };

    // 1. Validate return value
    if !(0 < ret && ret <= MAX_PAYLOAD_SIZE as i64) {
        debug!(ctx, "invalid ret: {}", ret);
        map.remove(&id).map_err(|_| MAP_DELETE_FAILED)?;
        return Err(SYSCALL_PAYLOAD_LENGTH_INVALID);
    }

    // 2. Retrieve stored context
    let args = match unsafe { map.get(&id) } {
        Some(a) => a,
        None => return Err(MAP_GET_FAILED),
    };

    // 3. Allocate and build Message structure
    alloc::init()?;
    let data = alloc::alloc_zero::<Message>()?;
    let sock = tcp_sock_from_fd(args.fd)?;
    let key = gen_connect_key(bpf_get_current_pid_tgid(), args.fd);

    // 4. Extract network information
    let quintuple = quintuple_from_sock(sock)?;
    data.quintuple = quintuple;
    data.quintuple.l4_protocol = is_tcp_udp(sock)?;

    // 5. Fill message fields
    data.tgid = ctx.tgid();
    data.pid = ctx.pid();
    data.comm = Buffer::from_slice(&ctx.command().map_err(|_| FAILED_TO_GET_COMM)?);
    data.enter_seq = args.enter_seq;
    data.exit_seq = match direction {
        Direction::Ingress => sock.copied_seq().ok_or(READ_TCP_SOCK_COPIED_SEQ_FAILED)?,
        Direction::Egress => sock.write_seq().ok_or(READ_TCP_SOCK_WRITE_SEQ_FAILED)?,
        _ => return Err(INVALID_DIRECTION),
    };

    // 6. Protocol inference and correlation
    let infer_payload = alloc::alloc_zero::<Buffer<MAX_INFER_SIZE>>()?;
    args.extract(infer_payload, ret as u32)?;

    let result = protocol_infer(
        ctx,
        &quintuple,
        direction,
        infer_payload,
        key,
        args.enter_seq,
        data.exit_seq,
    )?;
    
    data.timestamp_ns = unsafe { bpf_ktime_get_ns() };
    data.syscall = syscall;
    data.direction = direction;
    data.type_ = result.type_;
    data.protocol = result.protocol;
    data.seq = result.seq;
    data.uuid = result.uuid;
    
    // 7. Extract full payload
    args.extract(&mut data.payload, ret as u32)?;

    // 8. Cleanup and send
    map.remove(&id).map_err(|_| MAP_DELETE_FAILED)?;
    unsafe { EVENTS.output(ctx, data.encode(), 0) };

    Ok(0)
}
```

**Exit Processing Steps**:
1. **Return Value Validation**: Check if return value is valid (0 < ret <= MAX_PAYLOAD_SIZE)
2. **Context Retrieval**: Get stored Args from INGRESS/EGRESS map
3. **Memory Allocation**: Allocate Message structure using eBPF-safe allocator
4. **Socket Information**: Extract TCP socket and network quintuple
5. **Process Information**: Get PID, TGID, and command name
6. **TCP Sequence Numbers**: Get entry and exit sequence numbers for correlation
7. **Protocol Inference**: Analyze payload for L7 protocol detection
8. **Payload Extraction**: Copy actual network data to message
9. **Data Transmission**: Send complete message to user space via PerfEvent
10. **Cleanup**: Remove entry from map to prevent memory leaks

## Process Filtering

DeepTrace implements intelligent process filtering to reduce overhead:

### PID-Based Filtering

```rust
// From utils.rs
/// Check if the pid is in pid_map, which is generated by agent at user space
#[inline(always)]
pub(crate) fn is_filtered_pid() -> bool {
    let tgid = (bpf_get_current_pid_tgid() >> 32) as u32;
    unsafe { PIDS.get_ptr(&tgid) }.is_some()
}
```

**Key Features**:
- **User Space Control**: PID list managed by DeepTrace agent
- **Fast Lookup**: O(1) hash map lookup for PID filtering
- **Thread Group ID**: Uses TGID (process ID) rather than individual thread IDs
- **Memory Efficient**: Only stores PIDs that need monitoring

### Socket Management

DeepTrace also provides socket lifecycle management:

```rust
// From process.rs
#[inline(always)]
pub fn try_socket(fd: u64) -> Result<u32> {
    let key = gen_connect_key(bpf_get_current_pid_tgid(), fd);
    let map = unsafe { &SOCKET_INFO };
    alloc::init()?;
    let socket_info = alloc::alloc_zero::<SocketInfo>()?;
    map.insert(&key, socket_info, 0).map_err(|_| MAP_INSERT_FAILED)?;
    Ok(0)
}

#[inline(always)]
pub fn try_close(fd: u64) -> Result<u32> {
    let key = gen_connect_key(bpf_get_current_pid_tgid(), fd);
    let map = unsafe { &SOCKET_INFO };
    if unsafe { map.get(&key) }.is_some() {
        map.remove(&key).map_err(|_| MAP_DELETE_FAILED)?;
    }
    Ok(0)
}
```

## Protocol Inference and Correlation

DeepTrace integrates with `l7-parser` for protocol detection and correlation:

```rust
// From process.rs - Protocol inference
let result = protocol_infer(
    ctx,
    &quintuple,
    direction,
    infer_payload,
    key,
    args.enter_seq,
    data.exit_seq,
)?;

data.type_ = result.type_;      // Request/Response
data.protocol = result.protocol; // L7 protocol (HTTP, gRPC, etc.)
data.seq = result.seq;          // Sequence for correlation
data.uuid = result.uuid;        // Unique identifier
```

**Supported Protocols**:
- HTTP/HTTPS
- gRPC
- Redis
- MongoDB
- MySQL
- PostgreSQL
- And more...

## Performance Characteristics

### Hook Overhead

| Operation | Overhead | Impact |
|-----------|----------|--------|
| **Process Filtering** | 50ns | Per syscall |
| **Entry Processing** | 200ns | Per syscall |
| **Exit Processing** | 2-5μs | Per syscall |
| **Protocol Inference** | 0.5-1μs | Per message |

### Optimization Features

- **Early Filtering**: Skip non-monitored processes immediately
- **FD Validation**: Skip standard I/O file descriptors
- **Type Safety**: Rust prevents runtime errors
- **Memory Safety**: Automatic bounds checking
- **Zero-Copy**: Efficient data handling where possible

## Error Handling

DeepTrace uses comprehensive error handling with specific error codes:

```rust
// From ebpf-common/src/error/code.rs
pub const MAP_INSERT_FAILED: u32 = 1;
pub const MAP_DELETE_FAILED: u32 = 2;
pub const MAP_GET_FAILED: u32 = 3;
pub const INVALID_DIRECTION: u32 = 4;
pub const SYSCALL_PAYLOAD_LENGTH_INVALID: u32 = 5;
```

## Next Steps

- **[Data Structures](./structures.md)**: Learn about eBPF data structures
- **[Memory Maps](./maps.md)**: Understand eBPF map usage
- **[Performance Analysis](./performance.md)**: Optimize eBPF performance
