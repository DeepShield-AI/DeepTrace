# Configuration Schema

DeepTrace uses TOML configuration files for both agent and server components. This document provides the complete schema reference for all configuration options.

## Agent Configuration

The agent configuration file (`agent.toml`) controls span collection, processing, and transmission behavior.

### Complete Schema

```toml
[agents]
# Agent identification
agent_id = "agent-001"
hostname = "web-server-01"

[agents.trace]
# Span collection settings
batch_size = 1024                    # Number of spans per batch
flush_interval = 5000                # Flush interval in milliseconds
sampling_rate = 1.0                  # Sampling rate (0.0-1.0)
max_queue_size = 10000              # Maximum queued spans
enable_async = true                  # Enable asynchronous processing

# Process filtering
include_processes = ["nginx", "redis-server", "app-server"]
exclude_processes = ["systemd", "kernel", "ssh"]
include_pids = [1234, 5678]         # Specific PIDs to monitor
exclude_pids = [1, 2]               # PIDs to exclude

# Protocol filtering
protocols = ["http", "https", "tcp", "udp"]
ports = [80, 443, 8080, 3306]      # Specific ports to monitor

[agents.capture]
# Payload capture settings
max_payload_size = 1024             # Maximum payload size in bytes
enable_compression = true           # Enable payload compression
capture_request = true              # Capture request payloads
capture_response = true             # Capture response payloads
truncate_large_payloads = true      # Truncate payloads exceeding max size

# Content filtering
exclude_content_types = [
    "image/*",
    "video/*",
    "application/octet-stream"
]

[agents.sender]
# Server communication
server_url = "http://localhost:7901"
timeout = 30000                     # Request timeout in milliseconds
retry_count = 3                     # Number of retries
retry_delay = 1000                  # Delay between retries in milliseconds
keep_alive = true                   # Enable HTTP keep-alive

# Buffering
mem_buffer_size = 16                # Memory buffer size in MB
disk_buffer_size = 100              # Disk buffer size in MB (0 = disabled)
disk_buffer_path = "/tmp/deeptrace" # Disk buffer directory

# Batch processing
batch_timeout = 5000                # Batch timeout in milliseconds
max_batch_size = 2048               # Maximum batch size
compression = "gzip"                # Compression algorithm (gzip, lz4, none)

[agents.ebpf]
# eBPF program settings
ring_buffer_size = 262144           # Ring buffer size (power of 2)
map_max_entries = 10240             # Maximum map entries
program_timeout = 30000             # Program load timeout
enable_debug = false                # Enable eBPF debug mode

# Kernel compatibility
min_kernel_version = "5.15"         # Minimum required kernel version
enable_co_re = true                 # Enable CO-RE (Compile Once, Run Everywhere)
fallback_mode = "disabled"          # Fallback mode (disabled, legacy, userspace)

[logging]
# Logging configuration
level = "info"                      # Log level (trace, debug, info, warn, error)
format = "json"                     # Log format (json, text)
output = "stdout"                   # Output destination (stdout, stderr, file)
file_path = "/var/log/deeptrace/agent.log"
max_file_size = "100MB"             # Maximum log file size
max_files = 5                       # Maximum number of log files
enable_rotation = true              # Enable log rotation

[metrics]
# Metrics collection
enable = true                       # Enable metrics collection
port = 9090                         # Metrics server port
path = "/metrics"                   # Metrics endpoint path
interval = 30                       # Metrics collection interval in seconds

# Custom metrics
custom_labels = { environment = "production", region = "us-west-2" }
```

### Configuration Sections

#### [agents.trace]

Controls span collection behavior:

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `batch_size` | integer | 1024 | Number of spans to batch before sending |
| `flush_interval` | integer | 5000 | Maximum time (ms) to wait before flushing |
| `sampling_rate` | float | 1.0 | Fraction of requests to trace (0.0-1.0) |
| `max_queue_size` | integer | 10000 | Maximum spans to queue in memory |
| `enable_async` | boolean | true | Enable asynchronous span processing |

#### [agents.capture]

Controls payload capture:

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `max_payload_size` | integer | 1024 | Maximum payload size to capture (bytes) |
| `enable_compression` | boolean | true | Compress captured payloads |
| `capture_request` | boolean | true | Capture request payloads |
| `capture_response` | boolean | true | Capture response payloads |

#### [agents.sender]

Controls server communication:

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `server_url` | string | "http://localhost:7901" | DeepTrace server URL |
| `timeout` | integer | 30000 | Request timeout (milliseconds) |
| `retry_count` | integer | 3 | Number of retry attempts |
| `mem_buffer_size` | integer | 16 | Memory buffer size (MB) |

## Server Configuration

The server configuration file (`server.toml`) controls trace processing, correlation, and storage.

### Complete Schema

```toml
[server]
# Server settings
host = "0.0.0.0"                    # Bind address
port = 7901                         # Listen port
workers = 4                         # Number of worker threads
max_connections = 1000              # Maximum concurrent connections
request_timeout = 30000             # Request timeout in milliseconds

[elasticsearch]
# Elasticsearch connection
hosts = ["http://localhost:9200"]   # Elasticsearch hosts
username = "elastic"                # Username (optional)
password = "changeme"               # Password (optional)
index_prefix = "deeptrace"          # Index prefix
batch_size = 1000                   # Bulk indexing batch size
flush_interval = 5000               # Flush interval in milliseconds
max_retries = 3                     # Maximum retry attempts

# Index settings
shards = 1                          # Number of shards per index
replicas = 0                        # Number of replicas
refresh_interval = "1s"             # Index refresh interval
mapping_total_fields_limit = 10000  # Maximum number of fields

# Index lifecycle
enable_ilm = true                   # Enable Index Lifecycle Management
hot_phase_duration = "7d"           # Hot phase duration
warm_phase_duration = "30d"         # Warm phase duration
delete_phase_duration = "90d"       # Delete phase duration

[correlation]
# Correlation engine settings
default_algorithm = "deeptrace"     # Default correlation algorithm
auto_correlation = true             # Enable automatic correlation
correlation_interval = 60          # Correlation interval in seconds
batch_size = 5000                   # Spans per correlation batch
max_trace_duration = 300000         # Maximum trace duration (ms)
max_trace_spans = 1000              # Maximum spans per trace

# Algorithm-specific settings
[correlation.deeptrace]
window_size = 1000                  # Correlation window (ms)
similarity_threshold = 0.8          # Minimum similarity score
max_iterations = 100                # Maximum correlation iterations
enable_caching = true               # Enable correlation caching

[correlation.fifo]
batch_size = 1000                   # FIFO batch size
timeout = 5000                      # Processing timeout (ms)

[api]
# API server settings
enable_auth = false                 # Enable API authentication
api_keys = []                       # Valid API keys
rate_limit = 100                    # Requests per minute per IP
burst_limit = 20                    # Burst request limit
enable_cors = true                  # Enable CORS headers
cors_origins = ["*"]                # Allowed CORS origins

# WebSocket settings
enable_websocket = true             # Enable WebSocket endpoints
websocket_timeout = 300000          # WebSocket timeout (ms)
max_websocket_connections = 100     # Maximum WebSocket connections

[storage]
# Data retention
span_retention_days = 30            # Span retention period
trace_retention_days = 90           # Trace retention period
enable_compression = true           # Enable storage compression
compression_algorithm = "gzip"      # Compression algorithm

# Backup settings
enable_backup = false               # Enable automatic backups
backup_interval = "24h"             # Backup interval
backup_location = "/backup/deeptrace" # Backup directory
max_backups = 7                     # Maximum backup files

[monitoring]
# Health checks
health_check_interval = 30          # Health check interval (seconds)
component_timeout = 5000            # Component health timeout (ms)

# Metrics
enable_metrics = true               # Enable metrics collection
metrics_port = 9091                 # Metrics server port
metrics_path = "/metrics"           # Metrics endpoint

# Alerting
enable_alerts = false               # Enable alerting
alert_webhook = ""                  # Webhook URL for alerts
alert_thresholds = { error_rate = 0.05, latency_p99 = 5000 }

[logging]
# Logging configuration
level = "info"                      # Log level
format = "json"                     # Log format
output = "stdout"                   # Output destination
file_path = "/var/log/deeptrace/server.log"
max_file_size = "100MB"
max_files = 10
enable_rotation = true

# Component-specific logging
[logging.levels]
correlation = "debug"               # Correlation engine log level
elasticsearch = "warn"              # Elasticsearch client log level
api = "info"                        # API server log level
```

### Configuration Sections

#### [server]

Basic server settings:

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `host` | string | "0.0.0.0" | Server bind address |
| `port` | integer | 7901 | Server listen port |
| `workers` | integer | 4 | Number of worker threads |
| `max_connections` | integer | 1000 | Maximum concurrent connections |

#### [elasticsearch]

Elasticsearch configuration:

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `hosts` | array | ["http://localhost:9200"] | Elasticsearch cluster hosts |
| `index_prefix` | string | "deeptrace" | Index name prefix |
| `batch_size` | integer | 1000 | Bulk indexing batch size |
| `shards` | integer | 1 | Number of shards per index |

#### [correlation]

Correlation engine settings:

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `default_algorithm` | string | "deeptrace" | Default correlation algorithm |
| `auto_correlation` | boolean | true | Enable automatic correlation |
| `correlation_interval` | integer | 60 | Correlation interval (seconds) |
| `batch_size` | integer | 5000 | Spans per correlation batch |

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
