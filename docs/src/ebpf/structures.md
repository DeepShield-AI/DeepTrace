# Data Structures

DeepTrace's eBPF implementation relies on carefully designed data structures to efficiently capture, store, and transmit network trace information. This document provides comprehensive details about each structure and its role in the tracing system.

## Structure Design Principles

DeepTrace's data structures are designed with several key principles:

1. **Memory Efficiency**: Minimize memory footprint while preserving essential information
2. **Performance**: Optimize for fast access and minimal copying
3. **Compatibility**: Ensure cross-platform and kernel version compatibility
4. **Extensibility**: Allow for future protocol and feature additions

## Core Enumeration Types

### `SyscallName` Enum

Identifies the specific system call being monitored:

```rust
#[repr(u32)]
pub enum SyscallName {
    Read = 0,
    RecvMsg = 1,
    RecvMMsg = 2,
    ReadV = 3,
    RecvFrom = 4,
    Write = 5,
    SendMsg = 6,
    SendMMsg = 7,
    SendTo = 8,
    WriteV = 9,
    Unknown = 255,
}
```

**Usage**: 
- Debugging and logging
- Protocol-specific processing
- Performance analysis by syscall type

**Memory Layout**: 4 bytes (u32)

### `SyscallType` Enum

Categorizes system calls by data flow direction:

```rust
#[repr(u32)]
pub enum SyscallType {
    Ingress = 0,  // Incoming data (read operations)
    Egress = 1,   // Outgoing data (write operations)
}
```

**Purpose**:
- Distinguish request vs response processing
- Enable directional filtering
- Support correlation algorithms

**Memory Layout**: 4 bytes (u32)

### `Buffer` Enum

Represents different buffer types for system call data:

```rust
#[repr(C)]
pub enum Buffer {
    Normal(NormalBuffer),
    Vectored(VectoredBuffer),
    Msg(MsgBuffer),
}
```

**Buffer Types**:

#### `NormalBuffer`
```rust
#[repr(C)]
pub struct NormalBuffer {
    pub ptr: u64,     // Buffer pointer
    pub len: u32,     // Buffer length
    pub _padding: u32,
}
```

#### `VectoredBuffer`
```rust
#[repr(C)]
pub struct VectoredBuffer {
    pub iov_ptr: u64,    // iovec array pointer
    pub iov_count: u32,  // Number of iovec entries
    pub total_len: u32,  // Total data length
}
```

#### `MsgBuffer`
```rust
#[repr(C)]
pub struct MsgBuffer {
    pub msg_ptr: u64,       // msghdr pointer
    pub msg_namelen: u32,   // Address length
    pub msg_controllen: u32, // Control data length
}
```

## Primary Data Structures

### `Quintuple` Structure

The network flow identifier that uniquely identifies a connection:

```rust
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Quintuple {
    pub src_addr: u32,    // Source IP address (network byte order)
    pub dst_addr: u32,    // Destination IP address (network byte order)
    pub src_port: u16,    // Source port (network byte order)
    pub dst_port: u16,    // Destination port (network byte order)
    pub protocol: u16,    // Protocol family (AF_INET, etc.)
    pub _padding: u16,    // Alignment padding
}
```

**Key Features**:
- **Unique Flow Identification**: Distinguishes different network connections
- **Bidirectional Support**: Same quintuple for both directions of a flow
- **Protocol Agnostic**: Works with TCP, UDP, and other protocols
- **Hash-Friendly**: Optimized for use as hash map keys

**Memory Layout**: 16 bytes total

**Usage Example**:
```c
// Extract quintuple from socket
static inline int extract_quintuple(int fd, struct Quintuple *qt) {
    struct tcp_sock *tcp_sk = get_tcp_sock_from_fd(fd);
    if (!tcp_sk) return -1;
    
    qt->src_addr = tcp_sk->inet_conn.icsk_inet.inet_saddr;
    qt->dst_addr = tcp_sk->inet_conn.icsk_inet.inet_daddr;
    qt->src_port = tcp_sk->inet_conn.icsk_inet.inet_sport;
    qt->dst_port = tcp_sk->inet_conn.icsk_inet.inet_dport;
    qt->protocol = tcp_sk->inet_conn.icsk_inet.sk.__sk_common.skc_family;
    
    return 0;
}
```

### `Args` Structure

Stores system call context during the entry phase:

```rust
#[repr(C)]
pub struct Args {
    pub fd: u32,           // File descriptor
    pub seq: u32,          // TCP sequence number at entry
    pub timestamp: u64,    // Entry timestamp (nanoseconds)
    pub buffer: Buffer,    // Buffer information
}
```

**Lifecycle**:
1. **Created**: When system call enters
2. **Stored**: In INGRESS/EGRESS eBPF maps
3. **Retrieved**: When system call exits
4. **Destroyed**: After data extraction

**Memory Layout**: 32 bytes (with padding)

**Key Fields**:
- `fd`: Links to socket information
- `seq`: Enables TCP sequence tracking
- `timestamp`: Calculates syscall latency
- `buffer`: Handles different buffer types

### `Data` Structure

The complete trace record sent to user space:

```rust
#[repr(C)]
pub struct Data {
    // Process Information
    pub tgid: u32,              // Thread Group ID (process ID)
    pub pid: u32,               // Thread ID
    
    // Timing Information
    pub enter_seq: u32,         // TCP sequence at entry
    pub exit_seq: u32,          // TCP sequence at exit
    pub timestamp_ns: u64,      // Exit timestamp (nanoseconds)
    
    // System Call Information
    pub len: u32,               // Actual data length transferred
    pub syscall: SyscallName,   // System call identifier
    pub direction: SyscallType, // Ingress/Egress direction
    
    // Network Information
    pub quintuple: Quintuple,   // Network flow identifier
    
    // Process Information
    pub comm: [u8; TASK_CMD_LEN], // Process name (16 bytes)
    
    // Payload Data
    pub buf: [u8; MAX_PAYLOAD_SIZE], // Actual network data
}
```

**Constants**:
```rust
pub const TASK_CMD_LEN: usize = 16;      // Linux task command length
pub const MAX_PAYLOAD_SIZE: usize = 4096; // Maximum captured payload
```

**Memory Layout**: ~4.2KB total

**Field Details**:

#### Process Identification
- `tgid`: Process ID for correlation
- `pid`: Thread ID for fine-grained tracking
- `comm`: Process name for debugging

#### Timing and Sequencing
- `enter_seq`/`exit_seq`: TCP sequence numbers for ordering
- `timestamp_ns`: High-resolution timing

#### Network Context
- `quintuple`: Complete flow identification
- `direction`: Request vs response classification

#### Payload Data
- `buf`: Actual network data for protocol parsing
- `len`: Actual data length (may be less than buffer size)

## Auxiliary Structures

### Kernel Structure Wrappers

DeepTrace uses eBPF maps to safely access kernel structures:

#### `TASK_STRUCT` Map
```c
struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, 1);
    __type(key, u32);
    __type(value, struct task_struct);
} TASK_STRUCT SEC(".maps");
```

**Purpose**: Safe access to process information
**Size**: 13,696 bytes (kernel-dependent)

#### `TCP_SOCK` Map
```c
struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, 1);
    __type(key, u32);
    __type(value, struct tcp_sock);
} TCP_SOCK SEC(".maps");
```

**Purpose**: Access TCP socket state
**Size**: 2,304 bytes (kernel-dependent)

#### `FILE` Map
```c
struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, 1);
    __type(key, u32);
    __type(value, struct file);
} FILE SEC(".maps");
```

**Purpose**: File descriptor information
**Size**: 232 bytes

## Memory Management

### Stack Space Optimization

eBPF has a 512-byte stack limit, so large structures use per-CPU maps:

```c
// Instead of stack allocation:
// struct tcp_sock sock; // Too large for stack!

// Use per-CPU map:
static inline struct tcp_sock* get_tcp_sock_buffer(void) {
    u32 key = 0;
    return bpf_map_lookup_elem(&TCP_SOCK, &key);
}
```

### Alignment and Padding

All structures use explicit padding for consistent memory layout:

```rust
#[repr(C)]
pub struct AlignedStruct {
    pub field1: u32,
    pub _padding1: u32,  // Explicit padding
    pub field2: u64,     // Naturally aligned
}
```

### Memory Safety

DeepTrace implements several safety mechanisms:

#### Bounds Checking
```c
static inline int safe_copy_payload(void *dst, const void *src, size_t len) {
    if (len > MAX_PAYLOAD_SIZE) {
        len = MAX_PAYLOAD_SIZE;
    }
    return bpf_probe_read_user(dst, len, src);
}
```

#### Null Pointer Checks
```c
static inline bool validate_buffer_ptr(const struct Args *args) {
    if (!args) return false;
    
    switch (args->buffer.type) {
        case BUFFER_NORMAL:
            return args->buffer.normal.ptr != 0;
        case BUFFER_VECTORED:
            return args->buffer.vectored.iov_ptr != 0;
        case BUFFER_MSG:
            return args->buffer.msg.msg_ptr != 0;
        default:
            return false;
    }
}
```

## Protocol-Specific Extensions

### HTTP Structure Extensions

For HTTP protocol parsing:

```rust
#[repr(C)]
pub struct HttpMetadata {
    pub method: [u8; 8],        // HTTP method (GET, POST, etc.)
    pub status_code: u16,       // Response status code
    pub content_length: u32,    // Content-Length header
    pub is_request: bool,       // Request vs response flag
    pub _padding: [u8; 3],      // Alignment
}
```

### gRPC Structure Extensions

For gRPC protocol support:

```rust
#[repr(C)]
pub struct GrpcMetadata {
    pub service_name: [u8; 64], // gRPC service name
    pub method_name: [u8; 64],  // gRPC method name
    pub status_code: u32,       // gRPC status code
    pub message_type: u8,       // Request/Response/Stream
    pub _padding: [u8; 3],      // Alignment
}
```

## Serialization and Wire Format

### Network Byte Order

All network-related fields use network byte order:

```c
static inline void host_to_network_quintuple(struct Quintuple *qt) {
    qt->src_addr = htonl(qt->src_addr);
    qt->dst_addr = htonl(qt->dst_addr);
    qt->src_port = htons(qt->src_port);
    qt->dst_port = htons(qt->dst_port);
}
```

### Versioning Support

Structures include version fields for backward compatibility:

```rust
#[repr(C)]
pub struct DataHeader {
    pub version: u16,           // Structure version
    pub size: u16,              // Total structure size
    pub checksum: u32,          // Data integrity check
}
```

## Performance Considerations

### Cache-Friendly Layout

Structures are organized for optimal cache performance:

```rust
// Hot fields first (frequently accessed)
#[repr(C)]
pub struct OptimizedData {
    // Hot path fields
    pub timestamp_ns: u64,
    pub len: u32,
    pub syscall: SyscallName,
    
    // Warm fields
    pub quintuple: Quintuple,
    
    // Cold fields (less frequently accessed)
    pub comm: [u8; 16],
    pub buf: [u8; 4096],
}
```

### Memory Pool Usage

For high-frequency allocations:

```c
// Pre-allocated structure pool
struct data_pool {
    struct Data entries[POOL_SIZE];
    u32 next_free;
    spinlock_t lock;
};
```

## Debugging and Introspection

### Debug Information

Structures include debug fields in development builds:

```rust
#[cfg(debug_assertions)]
#[repr(C)]
pub struct DebugInfo {
    pub allocation_time: u64,
    pub source_file: [u8; 32],
    pub source_line: u32,
}
```

### Structure Validation

Runtime validation for structure integrity:

```c
static inline bool validate_data_structure(const struct Data *data) {
    // Check magic numbers
    if (data->magic != DATA_MAGIC) return false;
    
    // Validate ranges
    if (data->len > MAX_PAYLOAD_SIZE) return false;
    
    // Check enum values
    if (data->syscall >= SyscallName_MAX) return false;
    
    return true;
}
```

---

DeepTrace's data structures provide a robust foundation for efficient network tracing, balancing performance, memory usage, and functionality while maintaining compatibility across different kernel versions and architectures.
