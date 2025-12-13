
# Container Tagging Module

This module provides functionality for collecting and caching container metadata (Docker and Kubernetes) for processes traced by eBPF. It's part of a distributed tracing system that enriches spans with container information.

## Overview

The module maintains thread-local caches of container metadata keyed by TGID (Thread Group ID) and provides methods to retrieve this information. It supports both Docker containers and Kubernetes pods.

## Thread-Local Caches

### Docker Cache
```rust
thread_local! {
    static TGID_DOCKER_MAP: RefCell<HashMap<u32, DockerTag>> = RefCell::new(HashMap::new());
}
```
Caches Docker container metadata to avoid repeated API calls.

### Kubernetes Cache
```rust
thread_local! {
    static TGID_K8S_MAP: RefCell<HashMap<u32, K8sTag>> = RefCell::new(HashMap::new());
}
```
Caches Kubernetes pod metadata to avoid repeated CLI calls.

## Data Structures

### DockerTag
Represents Docker container metadata.

**Fields:**
- `container_id`: String - Docker container ID
- `container_name`: String - Name of the container
- `image`: String - Docker image used
- `hostname`: String - Container hostname
- `gateway`: String - Network gateway
- `tgid`: u32 - Thread Group ID
- `ip`: String - Container IP address
- `network_mode`: String - Docker network mode
- `created`: String - Creation timestamp

**Methods:**

#### `get_docker_tags(tgid: u32) -> Option<DockerTag>`
Retrieves Docker metadata for a given TGID.

**Workflow:**
1. Checks thread-local cache first
2. If not cached, connects to Docker daemon
3. Lists all containers and inspects each
4. Maps container PIDs to TGIDs using `docker top`
5. Caches results for all PIDs in the container
6. Returns cached entry if available

**Returns:** `Some(DockerTag)` if found, `None` otherwise

### K8sTag
Represents Kubernetes pod metadata.

**Fields:**
- `tgid`: u32 - Thread Group ID
- `name`: String - Pod name
- `state`: String - Pod state
- `created_at`: String - Creation timestamp
- `image`: String - Container image
- `namespace`: String - Kubernetes namespace
- `cpu_period`: String - CPU period limit
- `cpu_shares`: String - CPU shares

**Methods:**

#### `get_k8s_tags(tgid: u32) -> Option<K8sTag>`
Retrieves Kubernetes metadata for a given TGID.

**Workflow:**
1. Checks thread-local cache first
2. If not cached, uses `crictl` CLI to list containers
3. Inspects each container with `crictl inspect`
4. Extracts PID and metadata from JSON output
5. Caches the result
6. Returns cached entry if found

**Returns:** `Some(K8sTag)` if found, `None` otherwise

### EbpfTag
Represents eBPF tracing metadata from network events.

**Fields:**
- `tgid`: u32 - Thread Group ID
- `pid`: u32 - Process ID
- `component`: String - Process name (from comm field)
- `direction`: Direction - Traffic direction (Ingress/Egress)
- `protocol`: L7Protocol - Application layer protocol
- `src_ip`: String - Source IP address
- `dst_ip`: String - Destination IP address
- `src_port`: u16 - Source port
- `dst_port`: u16 - Destination port
- `req_seq`: u32 - Request sequence number
- `resp_seq`: u32 - Response sequence number

### SpanTag
Aggregate structure containing all tagging information for a span.

**Fields:**
- `ebpf_tag`: EbpfTag - eBPF tracing metadata
- `docker_tag`: Option<DockerTag> - Optional Docker metadata
- `k8s_tag`: Option<K8sTag> - Optional Kubernetes metadata
- `other_tags`: HashMap<String, String> - Additional tags (e.g., user)

**Methods:**

#### `set_tags(req: &Message, resp: &Message) -> SpanTag`
Creates a complete SpanTag from request and response messages.

**Workflow:**
1. Extracts network quintuple information based on direction
2. Creates `EbpfTag` from message data
3. Attempts to fetch Docker and Kubernetes metadata asynchronously
4. Adds additional tags from agent configuration
5. Returns populated `SpanTag`

## Helper Functions

### `u32_to_ip(ip: u32) -> String`
Converts a 32-bit integer to IPv4 address string.

## Usage Example

```rust
let span_tag = SpanTag::set_tags(&request_message, &response_message).await;

// Access different tag types
if let Some(docker_tag) = &span_tag.docker_tag {
    println!("Container: {}", docker_tag.container_name);
}

if let Some(k8s_tag) = &span_tag.k8s_tag {
    println!("Pod: {} in namespace {}", k8s_tag.name, k8s_tag.namespace);
}
```

## Dependencies

- `arc_swap`: For atomic configuration access
- `bollard`: Docker client library
- `observ_config`: Configuration management
- `observ_trace_common`: Common tracing types
- `serde`: Serialization/deserialization
- `std`: Standard library components

## Notes

- Thread-local caching improves performance but limits cache sharing across threads
- Docker integration requires access to Docker daemon
- Kubernetes integration relies on `crictl` CLI tool being available
- Network address conversion assumes IPv4 addresses
- All async methods should be awaited in async contexts
