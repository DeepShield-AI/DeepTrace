# System Hooks

DeepTrace's eBPF implementation uses a comprehensive set of system call hooks to intercept and monitor network operations. This document provides detailed information about each hook, their implementation, and how they contribute to distributed tracing.

## Hook Architecture

DeepTrace employs a **dual-phase hooking strategy**:

1. **Entry Hooks** (`sys_enter_*`): Capture system call parameters and context
2. **Exit Hooks** (`sys_exit_*`): Extract actual data and calculate metrics

```mermaid
graph LR
    APP[Application] --> SYSCALL[System Call]
    SYSCALL --> ENTER[Entry Hook]
    ENTER --> KERNEL[Kernel Processing]
    KERNEL --> EXIT[Exit Hook]
    EXIT --> USERSPACE[User Space Agent]
```

## Monitored System Calls

DeepTrace monitors **10 critical network system calls** divided into two categories:

### Ingress Operations (Data Receiving)

These hooks capture incoming network data and responses:

#### 1. `read()` System Call

**Purpose**: Monitor data reading from file descriptors

**Implementation Location**: `ebpf/src/network/read.rs`

**Entry Hook**:
```c
SEC("tracepoint/syscalls/sys_enter_read")
int sys_enter_read(struct trace_event_raw_sys_enter* ctx) {
    return try_enter(ctx, args, SyscallType::Ingress);
}
```

**Exit Hook**:
```c
SEC("tracepoint/syscalls/sys_exit_read")
int sys_exit_read(struct trace_event_raw_sys_exit* ctx) {
    return try_exit(ctx, ret, SyscallName::Read, SyscallType::Ingress);
}
```

**Captured Data**:
- File descriptor
- Buffer content
- Read length
- Timestamp information

#### 2. `recvmsg()` System Call

**Purpose**: Intercept message reception from sockets

**Key Features**:
- Handles complex message structures
- Extracts socket metadata
- Supports ancillary data

**Data Extraction**:
```c
struct msghdr *msg = (struct msghdr *)args[1];
// Extract message components
// - msg_name: peer address
// - msg_iov: data buffers
// - msg_control: ancillary data
```

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

**Implementation**:
```c
SEC("tracepoint/syscalls/sys_enter_write")
int sys_enter_write(struct trace_event_raw_sys_enter* ctx) {
    return try_enter(ctx, args, SyscallType::Egress);
}
```

**Optimization**:
- Minimal data copying
- Efficient buffer handling
- Protocol-aware extraction

#### 7. `sendmsg()` System Call

**Purpose**: Intercept message transmission through sockets

**Advanced Features**:
- Message header analysis
- Destination address capture
- Control message handling

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

```c
int try_enter(struct trace_event_raw_sys_enter* ctx, 
              struct args_t* args, 
              enum SyscallType type) {
    
    // 1. Process ID filtering
    u32 tgid = bpf_get_current_pid_tgid() >> 32;
    if (!should_monitor_pid(tgid)) {
        return 0;
    }
    
    // 2. Extract system call arguments
    args->fd = (u32)ctx->args[0];
    args->timestamp = bpf_ktime_get_ns();
    
    // 3. Get socket information
    struct tcp_sock *tcp_sk = get_tcp_sock(args->fd);
    if (tcp_sk) {
        args->seq = tcp_sk->snd_nxt;  // TCP sequence number
    }
    
    // 4. Store context for exit processing
    u64 key = ((u64)tgid << 32) | pid;
    if (type == SyscallType::Ingress) {
        bpf_map_update_elem(&INGRESS, &key, args, BPF_ANY);
    } else {
        bpf_map_update_elem(&EGRESS, &key, args, BPF_ANY);
    }
    
    return 0;
}
```

### Exit Phase Processing

When a system call exits, the hook performs:

```c
int try_exit(struct trace_event_raw_sys_exit* ctx,
             long ret,
             enum SyscallName syscall,
             enum SyscallType type) {
    
    // 1. Retrieve stored context
    u64 key = bpf_get_current_pid_tgid();
    struct args_t *args = bpf_map_lookup_elem(
        type == SyscallType::Ingress ? &INGRESS : &EGRESS, &key);
    
    if (!args) {
        return 0;  // No matching entry
    }
    
    // 2. Build complete data structure
    struct Data data = {};
    data.tgid = key >> 32;
    data.pid = key & 0xFFFFFFFF;
    data.enter_seq = args->seq;
    data.timestamp_ns = bpf_ktime_get_ns();
    data.len = (u32)ret;
    data.syscall = syscall;
    data.direction = type;
    
    // 3. Extract network quintuple
    extract_quintuple(args->fd, &data.quintuple);
    
    // 4. Copy payload data
    if (ret > 0 && ret <= MAX_PAYLOAD_SIZE) {
        bpf_probe_read_user(data.buf, ret, args->buffer);
    }
    
    // 5. Send to user space
    bpf_ringbuf_output(&Message, &data, sizeof(data), 0);
    
    // 6. Cleanup
    bpf_map_delete_elem(
        type == SyscallType::Ingress ? &INGRESS : &EGRESS, &key);
    
    return 0;
}
```

## Process Filtering

DeepTrace implements intelligent process filtering to reduce overhead:

### PID-Based Filtering

```c
static inline bool should_monitor_pid(u32 tgid) {
    // Check if PID is in monitoring list
    u32 *monitored = bpf_map_lookup_elem(&PIDS, &tgid);
    return monitored != NULL;
}
```

### Container-Aware Filtering

```c
static inline bool is_container_process(u32 tgid) {
    // Check if process runs in container namespace
    struct task_struct *task = (struct task_struct *)bpf_get_current_task();
    return task->nsproxy->pid_ns_for_children->ns.inum != INIT_PID_NS_INUM;
}
```

## Data Extraction Strategies

### Protocol-Aware Extraction

DeepTrace uses protocol-specific logic for efficient data extraction:

#### HTTP Protocol Detection

```c
static inline bool is_http_traffic(char *buf, size_t len) {
    if (len < 4) return false;
    
    // Check HTTP methods
    if (bpf_strncmp(buf, "GET ", 4) == 0 ||
        bpf_strncmp(buf, "POST", 4) == 0 ||
        bpf_strncmp(buf, "PUT ", 4) == 0 ||
        bpf_strncmp(buf, "DELETE", 6) == 0) {
        return true;
    }
    
    // Check HTTP response
    if (bpf_strncmp(buf, "HTTP/", 5) == 0) {
        return true;
    }
    
    return false;
}
```

#### gRPC Protocol Detection

```c
static inline bool is_grpc_traffic(char *buf, size_t len) {
    if (len < 5) return false;
    
    // Check gRPC frame header
    // gRPC uses HTTP/2 with specific content-type
    return (buf[0] == 0x00 && buf[1] == 0x00 && buf[2] == 0x00);
}
```

## Performance Optimizations

### 1. Efficient Buffer Handling

```c
// Minimize memory copies
static inline int copy_payload(struct args_t *args, struct Data *data, long ret) {
    size_t copy_len = ret > MAX_PAYLOAD_SIZE ? MAX_PAYLOAD_SIZE : ret;
    
    // Use efficient kernel copy functions
    if (args->buffer_type == BUFFER_USER) {
        return bpf_probe_read_user(data->buf, copy_len, args->buffer);
    } else {
        return bpf_probe_read_kernel(data->buf, copy_len, args->buffer);
    }
}
```

### 2. Map Size Optimization

```c
// Optimal map sizes based on workload analysis
struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 10240);  // Tuned for typical workloads
    __type(key, u64);
    __type(value, struct args_t);
} INGRESS SEC(".maps");
```

### 3. Per-CPU Maps for Scalability

```c
// Reduce lock contention with per-CPU maps
struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, 1);
    __type(key, u32);
    __type(value, struct tcp_sock);
} TCP_SOCK SEC(".maps");
```

## Error Handling and Edge Cases

### 1. Invalid File Descriptors

```c
static inline bool is_valid_socket_fd(int fd) {
    if (fd < 0) return false;
    
    struct file *file = get_file_from_fd(fd);
    if (!file) return false;
    
    // Check if it's a socket
    return file->f_op == &socket_file_ops;
}
```

### 2. Buffer Overflow Protection

```c
static inline size_t safe_copy_size(size_t requested, size_t max_size) {
    return requested > max_size ? max_size : requested;
}
```

### 3. Map Cleanup

```c
// Prevent map overflow by cleaning stale entries
static inline void cleanup_stale_entries(void) {
    u64 current_time = bpf_ktime_get_ns();
    u64 threshold = current_time - (5 * 1000000000ULL); // 5 seconds
    
    // Cleanup logic for old entries
}
```

## Debugging and Monitoring

### Hook Status Monitoring

```c
// Debug counters for hook performance
struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 16);
    __type(key, u32);
    __type(value, u64);
} DEBUG_COUNTERS SEC(".maps");

enum debug_counter {
    COUNTER_ENTER_CALLS = 0,
    COUNTER_EXIT_CALLS = 1,
    COUNTER_FILTERED_PIDS = 2,
    COUNTER_EXTRACTED_BYTES = 3,
};
```

### Performance Metrics

```c
static inline void update_perf_metrics(enum debug_counter counter, u64 value) {
    u32 key = counter;
    u64 *current = bpf_map_lookup_elem(&DEBUG_COUNTERS, &key);
    if (current) {
        __sync_fetch_and_add(current, value);
    }
}
```

## Troubleshooting Common Issues

### 1. Missing Hooks

**Problem**: System calls not being intercepted

**Solutions**:
```bash
# Check if tracepoints are available
ls /sys/kernel/debug/tracing/events/syscalls/

# Verify eBPF program loading
bpftool prog list | grep deeptrace

# Check hook attachment
bpftool prog show id <program_id>
```

### 2. High Overhead

**Problem**: Excessive CPU usage from hooks

**Solutions**:
- Implement more aggressive PID filtering
- Reduce payload extraction size
- Optimize map operations

### 3. Data Loss

**Problem**: Missing trace data

**Solutions**:
- Increase ring buffer size
- Implement backpressure handling
- Monitor map capacity