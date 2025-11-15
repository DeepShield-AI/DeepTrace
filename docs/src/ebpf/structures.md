# Data Structures

DeepTrace's eBPF implementation uses Rust-based data structures built with the Aya framework. These structures efficiently capture, store, and transmit network trace information between eBPF programs and user space.

## Structure Design Principles

DeepTrace's data structures are designed with several key principles:

1. **Type Safety**: Leverage Rust's type system for memory safety
2. **Performance**: Optimize for fast access and minimal copying
3. **Aya Integration**: Native integration with Aya framework features
4. **Cross-Boundary Compatibility**: Seamless data sharing between eBPF and user space
5. **Protocol Awareness**: Support for L7 protocol inference and correlation

## Core Enumeration Types

### `Syscall` Enum

Identifies the specific system call being monitored:

```rust
#[cfg_attr(feature = "user", derive(serde::Serialize))]
#[repr(u8)]
pub enum Syscall {
    // Ingress operations
    Read,
    ReadV,
    RecvFrom,
    RecvMsg,
    RecvMMsg,
    
    // Egress operations
    Write,
    WriteV,
    SendTo,
    SendMsg,
    SendMMsg,
    
    Unknown,
}
```

**Usage**: 
- System call identification in traces
- Performance analysis by syscall type
- Protocol-specific processing logic
- Serialization to JSON for user space

**Memory Layout**: 1 byte (u8)

### `Direction` Enum

Categorizes system calls by data flow direction:

```rust
#[cfg_attr(feature = "user", derive(serde::Serialize))]
#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Direction {
    Ingress,  // Incoming data (read operations)
    Egress,   // Outgoing data (write operations)
    Unknown,
}
```

**Purpose**:
- Distinguish request vs response processing
- Enable directional filtering
- Support span correlation algorithms
- Request/response matching

**Memory Layout**: 1 byte (u8)

### `Buffer` Structure

A compile-time sized buffer for safe data handling:

```rust
// From ebpf-common/src/buffer.rs
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Buffer<const N: usize> {
    buf: [u8; N],
    len: usize,
}
```

**Key Features**:
- **Compile-time Size**: Size known at compile time for safety
- **Bounds Checking**: Automatic bounds checking for all operations
- **Zero-Copy**: Efficient slice operations without copying
- **Generic Size**: Can be instantiated with any size `N`

**Common Instantiations**:
```rust
pub type TaskCommBuffer = Buffer<TASK_COMM_LEN>;     // 16 bytes
pub type PayloadBuffer = Buffer<MAX_PAYLOAD_SIZE>;   // 4096 bytes  
pub type InferBuffer = Buffer<MAX_INFER_SIZE>;       // Variable size
```

**Methods**:
```rust
impl<const N: usize> Buffer<N> {
    pub fn new() -> Self;
    pub fn as_slice(&self) -> &[u8];
    pub fn from_slice(slice: &[u8]) -> Self;
    pub fn len(&self) -> usize;
    pub fn read_user_at(&mut self, ptr: *mut u8, size: u32) -> Result<()>;
    pub fn fill_from_iovec<const IOV_MAX: usize>(&mut self, iovec: iovec, vlen: u32, max_size: Option<usize>) -> Result<()>;
    pub fn fill_from_mmsghdr<const IOVLEN_MAX: usize>(&mut self, mmsg: mmsghdr, vlen: u32, max_size: Option<usize>) -> Result<()>;
}
```

### Protocol Enumerations

#### `L7Protocol` Enum

Identifies Layer 7 application protocols:

```rust
// From observ-trace-common/src/protocols/l7.rs
#[cfg_attr(feature = "user", derive(Eq, Hash, serde::Serialize))]
#[derive(FromPrimitive, IntoPrimitive, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum L7Protocol {
    #[default]
    Unknown = 0,
    
    // HTTP
    HTTP1 = 20,
    Http2 = 21,
    
    // RPC
    Dubbo = 40,
    Grpc = 41,
    SofaRPC = 43,
    FastCGI = 44,
    Brpc = 45,
    Tars = 46,
    SomeIp = 47,
    Thrift = 48,
    
    // SQL
    MySQL = 60,
    PostgreSQL = 61,
    Oracle = 62,
    
    // NoSQL
    Redis = 80,
    MongoDB = 81,
    Memcached = 82,
    Cassandra = 83,
    
    // MQ
    Kafka = 100,
    MQTT = 101,
    AMQP = 102,
    OpenWire = 103,
    NATS = 104,
    Pulsar = 105,
    ZMTP = 106,
    RocketMQ = 107,
    
    // INFRA
    DNS = 120,
    TLS = 121,
    Ping = 122,
    
    Custom = 127,
    Max = 255,
}
```

#### `L4Protocol` Enum

Identifies Layer 4 transport protocols:

```rust
// From observ-trace-common/src/protocols/l4.rs
#[cfg_attr(feature = "user", derive(serde::Serialize, Hash, Eq))]
#[derive(Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum L4Protocol {
    IPPROTO_IP = 0,      // Dummy protocol for TCP
    IPPROTO_ICMP = 1,    // Internet Control Message Protocol
    IPPROTO_IGMP = 2,    // Internet Group Management Protocol
    IPPROTO_IPIP = 4,    // IPIP tunnels
    IPPROTO_TCP = 6,     // Transmission Control Protocol
    IPPROTO_EGP = 8,     // Exterior Gateway Protocol
    IPPROTO_PUP = 12,    // PUP protocol
    IPPROTO_UDP = 17,    // User Datagram Protocol
    // ... more protocols
    IPPROTO_RAW = 255,   // Raw IP packets
    IPPROTO_MPTCP = 262, // Multipath TCP connection
}
```

## Primary Data Structures

### `Quintuple` Structure

The network flow identifier that uniquely identifies a connection:

```rust
#[cfg_attr(feature = "user", derive(serde::Serialize, Hash, Eq, PartialEq))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Quintuple {
    pub src_addr: u32,           // Source IP address
    pub dst_addr: u32,           // Destination IP address
    pub src_port: u16,           // Source port
    pub dst_port: u16,           // Destination port
    pub l4_protocol: L4Protocol, // L4 protocol (TCP/UDP)
    #[cfg_attr(feature = "user", serde(skip))]
    padding: u16,                // Alignment padding
}
```

**Key Features**:
- **Unique Flow Identification**: Distinguishes different network connections
- **Bidirectional Support**: Same quintuple for both directions of a flow
- **Protocol Awareness**: Includes L4 protocol information
- **Serialization Support**: JSON serialization for user space
- **Hash-Friendly**: Optimized for use as hash map keys

**Memory Layout**: 16 bytes total

**Constructor**:
```rust
impl Quintuple {
    pub fn new(
        src_addr: u32,
        dst_addr: u32,
        src_port: u16,
        dst_port: u16,
        l4_protocol: u16,
    ) -> Quintuple {
        // Implementation handles protocol conversion
    }
}
```

**Usage Example**:
```rust
// From observ-trace-ebpf/src/utils.rs
#[inline(always)]
pub fn quintuple_from_sock(tcp_sock: tcp_sock) -> Result<Quintuple> {
    let src_addr = core_read_kernel!(tcp_sock, inet_conn, icsk_inet, inet_saddr)?.to_be();
    let sock_common = core_read_kernel!(tcp_sock, inet_conn, icsk_inet, sk, __sk_common)?;
    let dst_addr = sock_common.skc_daddr().ok_or(READ_SKC_DADDR_FAILED)?.to_be();
    let src_port = core_read_kernel!(tcp_sock, inet_conn, icsk_inet, inet_sport)?.to_be();
    let dst_port = sock_common.skc_dport().ok_or(READ_SKC_DPORT_FAILED)?.to_be();
    let skc_family = sock_common.skc_family().ok_or(READ_SKC_FAMILY_FAILED)?;
    Ok(Quintuple::new(src_addr, dst_addr, src_port, dst_port, skc_family))
}
```

**Key Features**:
- **CO-RE Support**: Uses `core_read_kernel!` macro for safe kernel memory access
- **Error Handling**: Returns `Result<Quintuple>` with specific error codes
- **Byte Order**: Converts to big-endian (network byte order) with `.to_be()`
- **Type Safety**: Uses Rust's type system and Option types for safety
- **Memory Safety**: Safe kernel structure field access through CO-RE

### `Args` Structure

Stores system call context during the entry phase:

```rust
#[repr(C)]
pub struct Args {
    pub fd: u64,           // File descriptor
    pub enter_time: u64,   // Entry timestamp (nanoseconds)
    pub buffer: SysBufPtr, // Buffer information
    pub enter_seq: u32,    // TCP sequence number at entry
    pub padding: u32,      // Alignment padding
}
```

**Constructors**:
```rust
impl Args {
    pub fn from_ubuf(fd: u64, buf: *mut u8, count: u32, timestamp: u64, enter_seq: u32) -> Self;
    pub fn from_msg(fd: u64, vec: iovec, vlen: u32, timestamp: u64, enter_seq: u32) -> Self;
    pub fn from_mmsg(fd: u64, mmsg: mmsghdr, vlen: u32, timestamp: u64, enter_seq: u32) -> Self;
}
```

**Buffer Types**:
```rust
pub enum SysBufPtr {
    Ubuf(*mut u8, u32),    // User buffer
    Msg(iovec, u32),       // Message vector
    MMsg(mmsghdr, u32),    // Multiple messages
}
```

**Lifecycle**:
1. **Created**: When system call enters
2. **Stored**: In INGRESS/EGRESS eBPF maps
3. **Retrieved**: When system call exits
4. **Destroyed**: After data extraction

**Memory Layout**: 32 bytes total

**Key Fields**:
- `fd`: Links to socket information
- `seq`: Enables TCP sequence tracking
- `timestamp`: Calculates syscall latency
- `buffer`: Handles different buffer types

### `Message` Structure

The complete trace record sent to user space:

```rust
#[cfg_attr(feature = "user", derive(serde::Serialize))]
#[repr(C)]
pub struct Message {
    // Process Information
    pub tgid: u32,                    // Thread Group ID (process ID)
    pub pid: u32,                     // Thread ID
    
    // Timing Information
    pub enter_seq: u32,               // TCP sequence at entry
    pub exit_seq: u32,                // TCP sequence at exit
    pub timestamp_ns: u64,            // Exit timestamp (nanoseconds)
    
    // Correlation Information
    #[cfg_attr(feature = "user", serde(skip))]
    pub seq: u32,                     // Sequence for correlation
    #[cfg_attr(feature = "user", serde(skip))]
    pub uuid: u32,                    // Unique identifier for correlation
    
    // Network Information
    #[cfg_attr(feature = "user", serde(flatten))]
    pub quintuple: Quintuple,         // Network flow identifier
    
    // System Call Information
    pub syscall: Syscall,             // System call identifier
    pub direction: Direction,         // Ingress/Egress direction
    
    // Protocol Information
    #[cfg_attr(feature = "user", serde(rename(serialize = "type")))]
    pub type_: MessageType,           // Request/Response type
    pub protocol: L7Protocol,         // L7 protocol (HTTP, gRPC, etc.)
    
    // Process Information
    #[cfg_attr(feature = "user", serde(serialize_with = "serialize_comm"))]
    pub comm: Buffer<TASK_COMM_LEN>,  // Process name (16 bytes)
    
    // Payload Data
    #[cfg_attr(feature = "user", serde(serialize_with = "serialize_buffer"))]
    pub payload: Buffer<MAX_PAYLOAD_SIZE>, // Actual network data
}
```

### `MessageType` Enum

Classifies message types for correlation:

```rust
// From observ-trace-common/src/message.rs
#[cfg_attr(feature = "user", derive(serde::Serialize))]
#[derive(Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum MessageType {
    Unknown = 0,
    Request = 1,
    Response = 2,
}
```

### `SocketInfo` Structure

Socket metadata for correlation and protocol inference:

```rust
// From observ-trace-common/src/socket.rs
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SocketInfo {
    pub uuid: u32,
    pub exit_seq: u32,
    pub seq: u32,
    pub direction: Direction,
    pub pre_direction: Direction,
    pub l7protocol: L7Protocol,
    padding: u8,
    pub prev_buf: Buffer<MAX_INFER_SIZE>,
}
```

**Key Fields**:
- **uuid**: Unique identifier for correlation
- **exit_seq**: TCP sequence number at exit
- **seq**: Current sequence number
- **direction**: Current data flow direction
- **pre_direction**: Previous data flow direction
- **l7protocol**: Detected Layer 7 protocol
- **prev_buf**: Buffer for protocol inference

**Usage**:
- Protocol detection and caching
- TCP sequence tracking
- Request/response correlation
- Multi-message protocol handling

## Constants and Configuration

### Buffer Sizes

```rust
// From observ-trace-common/src/constants.rs
pub const MAX_PID_NUMBERS: u32 = 256;        // Maximum monitored PIDs
pub const MAX_INFER_SIZE: usize = 1024;      // Protocol inference buffer
pub const MAX_PAYLOAD_SIZE: usize = 4096;    // Maximum captured payload
pub const TASK_COMM_LEN: usize = 16;         // Linux task command length
```

### Memory Layout Summary

| Structure | Size | Purpose |
|-----------|------|---------|
| `Syscall` | 1 byte | System call identification |
| `Direction` | 1 byte | Data flow direction |
| `MessageType` | 1 byte | Request/Response classification |
| `L7Protocol` | 1 byte | Layer 7 protocol |
| `L4Protocol` | 2 bytes | Layer 4 protocol |
| `Quintuple` | 16 bytes | Network flow identifier |
| `Args` | 32 bytes | System call context |
| `Message` | ~4.2KB | Complete trace record |
| `SocketInfo` | Variable | Socket metadata |
| `Buffer<N>` | N + 8 bytes | Generic buffer |

## Type Safety and Validation

### Rust Type System Benefits

DeepTrace leverages Rust's type system for safety:

```rust
// Compile-time size validation
const _: () = assert!(core::mem::size_of::<Message>() <= 8192);

// Type-safe protocol handling
impl L7Protocol {
    pub fn is_http(&self) -> bool {
        matches!(self, L7Protocol::HTTP1 | L7Protocol::Http2)
    }
    
    pub fn is_rpc(&self) -> bool {
        matches!(self, L7Protocol::Grpc | L7Protocol::Dubbo | L7Protocol::Thrift)
    }
}
```

### Memory Safety Features

- **Bounds Checking**: Automatic array bounds checking
- **Null Safety**: Option types prevent null pointer dereferences
- **Lifetime Management**: RAII ensures proper cleanup
- **Type Safety**: Strong typing prevents type confusion

### Serialization Support

User space structures support JSON serialization:

```rust
// Automatic JSON serialization
#[cfg_attr(feature = "user", derive(serde::Serialize))]
pub struct Message {
    // Fields with custom serialization
    #[cfg_attr(feature = "user", serde(serialize_with = "serialize_comm"))]
    pub comm: Buffer<TASK_COMM_LEN>,
    
    // Fields excluded from serialization
    #[cfg_attr(feature = "user", serde(skip))]
    pub uuid: u32,
}
```

## Performance Optimizations

### Memory Layout

- **Cache-Friendly**: Hot fields placed first
- **Alignment**: Proper alignment for optimal access
- **Padding**: Explicit padding for consistent layout
- **Size Optimization**: Minimal memory footprint

### Zero-Copy Operations

```rust
// Zero-copy slice access
impl<const N: usize> Buffer<N> {
    pub fn as_slice(&self) -> &[u8] {
        &self.buf[..min(self.len(), N)]
    }
}

// Direct encoding without copying
impl Message {
    pub fn encode(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                core::mem::size_of::<Message>(),
            )
        }
    }
}
```

## Next Steps

- **[System Hooks](./hooks.md)**: Learn about eBPF program implementation
- **[Memory Maps](./maps.md)**: Understand eBPF map usage
- **[Performance Analysis](./performance.md)**: Optimize eBPF performance
