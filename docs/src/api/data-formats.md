# Data Formats

This document describes the data structures used by DeepTrace for spans, messages, and related entities.

## Message Structure

Messages are the fundamental units collected by eBPF programs. Each syscall (read/write/send/recv) generates a message.

### Message Fields

```rust
pub struct Message {
    pub tgid: u32,              // Thread group ID (process ID)
    pub pid: u32,               // Thread ID
    pub enter_seq: u32,         // Syscall enter sequence number
    pub exit_seq: u32,          // Syscall exit sequence number
    pub seq: u32,               // Internal sequence number
    pub timestamp_ns: u64,      // Timestamp in nanoseconds
    pub uuid: u32,              // Internal UUID for correlation
    pub quintuple: Quintuple,   // Network 5-tuple
    pub syscall: Syscall,       // Syscall type
    pub direction: Direction,   // Ingress or Egress
    pub type_: MessageType,     // Request or Response
    pub protocol: L7Protocol,   // Detected protocol
    pub comm: Buffer<16>,       // Process command name
    pub payload: Buffer<4096>,  // Message payload data
}
```

### MessageType Enum

```rust
pub enum MessageType {
    Unknown = 0,
    Request = 1,
    Response = 2,
}
```

### JSON Serialization

When serialized to JSON (for Elasticsearch), messages have the following format:

```json
{
  "tgid": 12345,
  "pid": 12346,
  "enter_seq": 100,
  "exit_seq": 101,
  "timestamp_ns": 1705320930123456789,
  "src_addr": "192.168.1.10",
  "dst_addr": "192.168.1.20",
  "src_port": 45678,
  "dst_port": 8080,
  "protocol_family": 2,
  "syscall": "read",
  "direction": "Ingress",
  "type": "Request",
  "protocol": "HTTP",
  "comm": "nginx",
  "payload": "GET /api/users HTTP/1.1\r\nHost: example.com\r\n\r\n"
}
```

**Note**: The `seq` and `uuid` fields are skipped during serialization as they are only used internally for correlation.

## Quintuple Structure

The quintuple identifies a network connection:

```rust
pub struct Quintuple {
    pub src_addr: u32,          // Source IP address (IPv4)
    pub dst_addr: u32,          // Destination IP address (IPv4)
    pub src_port: u16,          // Source port
    pub dst_port: u16,          // Destination port
    pub protocol_family: u16,   // Protocol family (AF_INET = 2)
}
```

### JSON Serialization

Quintuples are flattened into the parent message structure:

```json
{
  "src_addr": "192.168.1.10",
  "dst_addr": "192.168.1.20",
  "src_port": 45678,
  "dst_port": 8080,
  "protocol_family": 2
}
```

## Span Structure

Spans are constructed from pairs of request and response messages:

```rust
pub struct Span {
    metric: SpanMetric,
    content: SpanContent,
}
```

### SpanMetric

```rust
pub struct SpanMetric {
    pub start_time: u64,    // Request timestamp (nanoseconds)
    pub end_time: u64,      // Response timestamp (nanoseconds)
    pub duration: u64,      // Duration in nanoseconds
    pub req_size: usize,    // Request payload size (bytes)
    pub resp_size: usize,   // Response payload size (bytes)
}
```

### SpanContent

```rust
pub struct SpanContent {
    pub req_content: Buffer<4096>,   // Request payload
    pub resp_content: Buffer<4096>,  // Response payload
}
```

### JSON Serialization

Complete span in JSON format:

```json
{
  "metric": {
    "start_time": 1705320930123456789,
    "end_time": 1705320930456789012,
    "duration": 333332223,
    "req_size": 78,
    "resp_size": 1024
  },
  "content": {
    "req_content": "GET /api/users HTTP/1.1\r\nHost: example.com\r\n\r\n",
    "resp_content": "{\"users\": [{\"id\": 1, \"name\": \"John\"}]}"
  }
}
```

## Syscall Types

Supported syscall types:

```rust
pub enum Syscall {
    Unknown,
    Read,
    Readv,
    Recvfrom,
    Recvmsg,
    Recvmmsg,
    Write,
    Writev,
    Sendto,
    Sendmsg,
    Sendmmsg,
    Socket,
    Close,
}
```

## Direction Types

Message direction:

```rust
pub enum Direction {
    Unknown = 0,
    Ingress = 1,  // Incoming data (read/recv)
    Egress = 2,   // Outgoing data (write/send)
}
```

## L7 Protocol Types

Detected application-layer protocols:

```rust
pub enum L7Protocol {
    Unknown,
    HTTP,
    MySQL,
    Redis,
    MongoDB,
    Memcached,
    PostgreSQL,
    // ... more protocols
}
```

## Buffer Structure

Buffers are fixed-size arrays with length tracking:

```rust
pub struct Buffer<const N: usize> {
    data: [u8; N],
    len: usize,
}
```

### Buffer Serialization

Buffers are serialized as UTF-8 strings (with lossy conversion):

```json
{
  "payload": "GET /api/users HTTP/1.1"
}
```

## Elasticsearch Index Schema

### Spans Index

Spans are stored in Elasticsearch with the following index pattern:

```
spans_{agent_name}
```

Example indices:
- `spans_agent1`
- `spans_agent2`
- `spans_web_server`

### Document Structure

Each span document contains:

```json
{
  "@timestamp": "2024-01-15T10:15:30.123Z",
  "agent_name": "agent1",
  "tgid": 12345,
  "pid": 12346,
  "comm": "nginx",
  "src_addr": "192.168.1.10",
  "dst_addr": "192.168.1.20",
  "src_port": 45678,
  "dst_port": 8080,
  "protocol": "HTTP",
  "direction": "Ingress",
  "type": "Request",
  "syscall": "read",
  "metric": {
    "start_time": 1705320930123456789,
    "end_time": 1705320930456789012,
    "duration": 333332223,
    "req_size": 78,
    "resp_size": 1024
  },
  "content": {
    "req_content": "GET /api/users HTTP/1.1",
    "resp_content": "{\"users\": []}"
  }
}
```

## Data Types Reference

### Numeric Types

| Field | Type | Range | Description |
|-------|------|-------|-------------|
| `tgid` | u32 | 0 - 4,294,967,295 | Process ID |
| `pid` | u32 | 0 - 4,294,967,295 | Thread ID |
| `timestamp_ns` | u64 | 0 - 18,446,744,073,709,551,615 | Nanosecond timestamp |
| `src_addr` | u32 | 0 - 4,294,967,295 | IPv4 address (network byte order) |
| `dst_addr` | u32 | 0 - 4,294,967,295 | IPv4 address (network byte order) |
| `src_port` | u16 | 0 - 65,535 | Port number |
| `dst_port` | u16 | 0 - 65,535 | Port number |

### String Types

| Field | Max Length | Description |
|-------|------------|-------------|
| `comm` | 16 bytes | Process command name (null-terminated) |
| `payload` | 4096 bytes | Message payload data |

### Timestamp Format

Timestamps are stored as nanoseconds since Unix epoch:

```
timestamp_ns = 1705320930123456789
```

To convert to human-readable format:

```python
from datetime import datetime
timestamp_sec = 1705320930123456789 / 1_000_000_000
dt = datetime.fromtimestamp(timestamp_sec)
print(dt)  # 2024-01-15 10:15:30.123456
```

## Query Examples

### Query Spans by Service

```bash
curl -X GET "http://localhost:9200/spans_*/_search" \
  -H 'Content-Type: application/json' \
  -d '{
    "query": {
      "term": {
        "comm": "nginx"
      }
    }
  }'
```

### Query by Protocol

```bash
curl -X GET "http://localhost:9200/spans_*/_search" \
  -H 'Content-Type: application/json' \
  -d '{
    "query": {
      "term": {
        "protocol": "HTTP"
      }
    }
  }'
```

### Query by Time Range

```bash
curl -X GET "http://localhost:9200/spans_*/_search" \
  -H 'Content-Type: application/json' \
  -d '{
    "query": {
      "range": {
        "metric.start_time": {
          "gte": 1705320930000000000,
          "lte": 1705321000000000000
        }
      }
    }
  }'
```

### Query by Connection

```bash
curl -X GET "http://localhost:9200/spans_*/_search" \
  -H 'Content-Type: application/json' \
  -d '{
    "query": {
      "bool": {
        "must": [
          { "term": { "src_addr": "192.168.1.10" } },
          { "term": { "dst_port": 8080 } }
        ]
      }
    }
  }'
```

### Aggregate by Protocol

```bash
curl -X GET "http://localhost:9200/spans_*/_search" \
  -H 'Content-Type: application/json' \
  -d '{
    "size": 0,
    "aggs": {
      "protocols": {
        "terms": {
          "field": "protocol"
        }
      }
    }
  }'
```

## Data Size Considerations

### Memory Usage

Each message structure occupies approximately:
- Fixed fields: ~100 bytes
- Payload buffer: 4096 bytes
- **Total per message**: ~4.2 KB

### Storage Requirements

Estimated storage per span:
- Raw span data: ~8.4 KB (request + response messages)
- Elasticsearch overhead: ~2 KB (indexing, metadata)
- **Total per span**: ~10-12 KB

For 1 million spans:
- Raw data: ~8.4 GB
- With Elasticsearch: ~10-12 GB

### Performance Implications

- **Payload Size**: Limited to 4096 bytes per message
- **Truncation**: Payloads larger than 4096 bytes are truncated
- **Buffer Overhead**: Fixed-size buffers ensure predictable memory usage
- **Serialization**: Zero-copy serialization for eBPF to userspace

## Best Practices

### Data Collection

1. **Payload Size**: Monitor payload truncation for large messages
2. **Sampling**: Use process filtering (`pids`) to reduce data volume
3. **Index Management**: Use separate indices per agent for better organization
4. **Retention**: Configure Elasticsearch ILM policies for data retention

### Data Analysis

1. **Time-based Queries**: Use `metric.start_time` for time range queries
2. **Connection Tracking**: Combine quintuple fields for connection analysis
3. **Protocol Detection**: Filter by `protocol` for protocol-specific analysis
4. **Performance Analysis**: Use `metric.duration` for latency analysis

### Data Export

1. **Kibana**: Use Discover for interactive exploration
2. **Elasticsearch API**: Use scroll API for large exports
3. **Python**: Use `elasticsearch-py` library for programmatic access
4. **Logstash**: Use Logstash for data transformation and export

## Next Steps

- **[Agent API](./agent.md)**: Agent management and control
- **[Server API](./server.md)**: Server management and CLI tools
- **[Configuration Schema](./configuration.md)**: Configuration options
