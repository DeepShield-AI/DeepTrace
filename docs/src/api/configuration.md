# Configuration Schema

DeepTrace uses TOML configuration files for both agent and server components. This document provides the complete schema reference for all configuration options.

## Agent Configuration

The agent configuration file (`deeptrace.toml`) controls span collection, processing, and transmission behavior.

### Complete Schema

```toml
[agent]
# Agent identification (REQUIRED)
name = "deeptrace"

[metric]
# Metrics collection settings
interval = 10                       # Collection interval in seconds
sender = "metric"                   # Sender configuration name

[sender.file.metric]
# File-based metrics storage
path = "metrics.csv"                # Output file path
rotate = true                       # Enable file rotation
max_size = 512                      # Max file size in MB
max_age = 7                         # Retention period in days
rotate_time = 10                    # Rotation interval in days
data_format = "%Y%m%d"              # Date format for rotation

[sender.elastic.trace]
# Elasticsearch trace storage
node_urls = "http://localhost:9200" # Elasticsearch URL
username = "elastic"                # Username
password = "password"               # Password
request_timeout = 10                # Request timeout in seconds
index_name = "agent1"               # Index name
bulk_size = 32                      # Bulk operation size

[trace]
# Tracing configuration
ebpf = "trace"                      # eBPF configuration name
sender = "trace"                    # Sender configuration name

[trace.span]
# Span management settings
cleanup_interval = 30               # Cleanup interval in seconds
max_sockets = 1024                  # Maximum tracked sockets

[ebpf.trace]
# eBPF program settings
log_level = 1                       # Log level (0=off, 1=debug, 3=verbose, 4=stats)
pids = [523094]                     # Process IDs to monitor
max_buffered_events = 128           # Max events per batch
enabled_probes = [                  # Enabled system call probes
    "sys_enter_read",
    "sys_exit_read",
    "sys_enter_readv",
    "sys_exit_readv",
    "sys_enter_recvfrom",
    "sys_exit_recvfrom",
    "sys_enter_recvmsg",
    "sys_exit_recvmsg",
    "sys_enter_recvmmsg",
    "sys_exit_recvmmsg",
    "sys_enter_write",
    "sys_exit_write",
    "sys_enter_writev",
    "sys_exit_writev",
    "sys_enter_sendto",
    "sys_exit_sendto",
    "sys_enter_sendmsg",
    "sys_exit_sendmsg",
    "sys_enter_sendmmsg",
    "sys_exit_sendmmsg",
    "sys_exit_socket",
    "sys_enter_close"
]
```

### Configuration Sections

#### [agent]

Basic agent identification:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `name` | string | Yes | Unique agent identifier |

#### [metric]

Metrics collection settings:

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `interval` | integer | 10 | Collection interval in seconds |
| `sender` | string | - | Reference to sender configuration |

#### [sender.file.*]

File-based data storage:

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `path` | string | - | Output file path |
| `rotate` | boolean | true | Enable file rotation |
| `max_size` | integer | 512 | Maximum file size (MB) |
| `max_age` | integer | 7 | Retention period (days) |
| `rotate_time` | integer | 10 | Rotation interval (days) |
| `data_format` | string | "%Y%m%d" | Date format for rotation |

#### [sender.elastic.*]

Elasticsearch data storage:

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `node_urls` | string | - | Elasticsearch URL |
| `username` | string | - | Authentication username |
| `password` | string | - | Authentication password |
| `request_timeout` | integer | 10 | Request timeout (seconds) |
| `index_name` | string | - | Target index name |
| `bulk_size` | integer | 32 | Bulk operation size |

#### [trace]

Tracing configuration:

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `ebpf` | string | - | Reference to eBPF configuration |
| `sender` | string | - | Reference to sender configuration |

#### [trace.span]

Span management settings:

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `cleanup_interval` | integer | 30 | Cleanup interval (seconds) |
| `max_sockets` | integer | 1024 | Maximum tracked sockets |

#### [ebpf.*]

eBPF program settings:

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `log_level` | integer | 1 | Log level (0-4) |
| `pids` | array | [] | Process IDs to monitor |
| `max_buffered_events` | integer | 128 | Max events per batch |
| `enabled_probes` | array | [] | List of enabled probes |

## Server Configuration

The server configuration file (`config.toml`) controls server deployment and agent management.

### Complete Schema

```toml
[server]
# Server settings (REQUIRED)
ip = "192.168.1.100"                # External IP address of the DeepTrace server

[elastic]
# Elasticsearch settings (REQUIRED)
elastic_password = "your_password"  # Elasticsearch password

# Agent management configuration
[[agents]]
  [agents.agent_info]
  # Agent identification and SSH connection (ALL REQUIRED)
  agent_name = "agent-1"            # Unique agent identifier
  user_name = "ubuntu"              # SSH username
  host_ip = "192.168.1.101"         # Agent host IP address
  ssh_port = 22                     # SSH port (usually 22)
  host_password = "ssh_password"    # SSH password
```

### Configuration Sections

#### [server]

Basic server settings:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `ip` | string | Yes | External IP address of the DeepTrace server |

#### [elastic]

Elasticsearch configuration:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `elastic_password` | string | Yes | Elasticsearch authentication password |

#### [[agents]]

Agent management configuration (array of agents):

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `agent_name` | string | Yes | Unique agent identifier |
| `user_name` | string | Yes | SSH username for agent host |
| `host_ip` | string | Yes | IP address of agent host |
| `ssh_port` | integer | Yes | SSH port (usually 22) |
| `host_password` | string | Yes | SSH password for agent host |

## Environment Variables

Configuration values can be overridden using environment variables:

### Agent Environment Variables

```bash
# Agent identification
DEEPTRACE_AGENT_ID="agent-001"
DEEPTRACE_HOSTNAME="web-server-01"

# Trace collection
DEEPTRACE_TRACE_BATCH_SIZE=1024
DEEPTRACE_TRACE_SAMPLING_RATE=1.0
DEEPTRACE_TRACE_FLUSH_INTERVAL=5000

# Server communication
DEEPTRACE_SERVER_URL="http://localhost:7901"
DEEPTRACE_SENDER_TIMEOUT=30000
DEEPTRACE_SENDER_RETRY_COUNT=3

# eBPF settings
DEEPTRACE_EBPF_RING_BUFFER_SIZE=262144
DEEPTRACE_EBPF_ENABLE_DEBUG=false

# Logging
DEEPTRACE_LOG_LEVEL="info"
DEEPTRACE_LOG_FORMAT="json"
```

### Server Environment Variables

```bash
# Server settings
DEEPTRACE_SERVER_HOST="0.0.0.0"
DEEPTRACE_SERVER_PORT=7901
DEEPTRACE_SERVER_WORKERS=4

# Elasticsearch
DEEPTRACE_ELASTICSEARCH_HOSTS="http://localhost:9200"
DEEPTRACE_ELASTICSEARCH_USERNAME="elastic"
DEEPTRACE_ELASTICSEARCH_PASSWORD="changeme"
DEEPTRACE_ELASTICSEARCH_INDEX_PREFIX="deeptrace"

# Correlation
DEEPTRACE_CORRELATION_ALGORITHM="deeptrace"
DEEPTRACE_CORRELATION_AUTO=true
DEEPTRACE_CORRELATION_INTERVAL=60

# API
DEEPTRACE_API_ENABLE_AUTH=false
DEEPTRACE_API_RATE_LIMIT=100
```

## Configuration Validation

### Agent Configuration Validation

```bash
# Validate agent configuration
deeptrace-agent --config agent.toml --validate

# Check configuration syntax
deeptrace-agent --config agent.toml --check-syntax

# Show effective configuration
deeptrace-agent --config agent.toml --show-config
```

### Server Configuration Validation

```bash
# Validate server configuration
deeptrace-server --config server.toml --validate

# Test Elasticsearch connection
deeptrace-server --config server.toml --test-elasticsearch

# Validate correlation settings
deeptrace-server --config server.toml --test-correlation
```

## Configuration Examples

### High-Throughput Agent

```toml
[agents.trace]
batch_size = 4096
flush_interval = 1000
sampling_rate = 0.1
max_queue_size = 50000

[agents.sender]
mem_buffer_size = 64
batch_timeout = 1000
max_batch_size = 4096
compression = "lz4"

[agents.ebpf]
ring_buffer_size = 1048576
map_max_entries = 65536
```

### Production Server

```toml
[server]
workers = 8
max_connections = 5000

[elasticsearch]
hosts = [
    "http://es-node-1:9200",
    "http://es-node-2:9200",
    "http://es-node-3:9200"
]
batch_size = 5000
shards = 3
replicas = 1

[correlation]
batch_size = 10000
correlation_interval = 30

[api]
enable_auth = true
api_keys = ["prod-api-key-1", "prod-api-key-2"]
rate_limit = 1000
```

### Development Environment

```toml
[agents.trace]
sampling_rate = 1.0
enable_async = false

[logging]
level = "debug"
format = "text"
output = "stdout"

[correlation]
default_algorithm = "fifo"
auto_correlation = true

[api]
enable_auth = false
enable_cors = true
```

## Configuration Migration

### Version Compatibility

| Version | Config Version | Migration Required |
|---------|----------------|-------------------|
| 0.1.x   | v1            | No                |
| 0.2.x   | v2            | Yes               |

### Migration Tools

```bash
# Migrate configuration from v1 to v2
deeptrace-migrate --from v1 --to v2 --config old-config.toml --output new-config.toml

# Validate migrated configuration
deeptrace-server --config new-config.toml --validate
```

## Best Practices

### Performance Optimization

1. **Batch Size Tuning**: Increase batch sizes for high-throughput environments
2. **Sampling**: Use sampling in production to reduce overhead
3. **Buffer Sizing**: Adjust buffer sizes based on memory availability
4. **Compression**: Enable compression for network efficiency

### Security

1. **API Authentication**: Enable API authentication in production
2. **Network Security**: Use HTTPS for server communication
3. **Access Control**: Restrict API access using network policies
4. **Credential Management**: Use environment variables for sensitive data

### Monitoring

1. **Health Checks**: Enable health check endpoints
2. **Metrics Collection**: Enable metrics for monitoring
3. **Log Levels**: Use appropriate log levels for different environments
4. **Alerting**: Configure alerting for critical issues