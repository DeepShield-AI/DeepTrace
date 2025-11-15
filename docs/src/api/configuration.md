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
  [agents.agent]
  # Agent identification and SSH connection (ALL REQUIRED)
  name = "agent-1"                  # Unique agent identifier
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
| `name` | string | Yes | Unique agent identifier |
| `user_name` | string | Yes | SSH username for agent host |
| `host_ip` | string | Yes | IP address of agent host |
| `ssh_port` | integer | Yes | SSH port (usually 22) |
| `host_password` | string | Yes | SSH password for agent host |

## Environment Variables

**Note**: DeepTrace currently does not support environment variable overrides. All configuration must be specified in the TOML configuration files.

## Configuration Validation

### Agent Configuration Validation

```bash
# Check TOML syntax
toml-check agent/config/deeptrace.toml

# Or use Python
python -c "import toml; toml.load('agent/config/deeptrace.toml')"
```

### Server Configuration Validation

```bash
# Check TOML syntax
toml-check server/config/config.toml

# Or use Python
python -c "import toml; toml.load('server/config/config.toml')"

# Test agent connections
python server/cli/src/cmd.py agent test
```

## Configuration Examples

### High-Throughput Agent

```toml
[agent]
name = "high-throughput-agent"

[ebpf.trace]
log_level = 0  # Disable debug logging for performance
max_buffered_events = 256  # Larger buffer
pids = []  # Monitor all processes

[sender.elastic.trace]
node_url = "http://localhost:9200"
username = "elastic"
password = "password"
index_name = "spans_production"
bulk_size = 128  # Larger bulk size
request_timeout = 30

[trace.span]
cleanup_interval = 60  # Less frequent cleanup
max_sockets = 4096  # More sockets
```

### Development Environment

```toml
[agent]
name = "dev-agent"

[ebpf.trace]
log_level = 3  # Verbose logging
max_buffered_events = 32
pids = [12345]  # Monitor specific process

[sender.elastic.trace]
node_url = "http://localhost:9200"
username = "elastic"
password = "dev_password"
index_name = "spans_dev"
bulk_size = 16

[trace.span]
cleanup_interval = 10  # Frequent cleanup for testing
max_sockets = 256
```

### Multi-Agent Server

```toml
[server]
ip = "192.168.1.100"

[elastic]
elastic_password = "production_password"

[[agents]]
  [agents.agent]
  name = "web-server-1"
  user_name = "ubuntu"
  host_ip = "192.168.1.101"
  ssh_port = 22
  host_password = "ssh_pass_1"

[[agents]]
  [agents.agent]
  name = "web-server-2"
  user_name = "ubuntu"
  host_ip = "192.168.1.102"
  ssh_port = 22
  host_password = "ssh_pass_2"

[[agents]]
  [agents.agent]
  name = "database-server"
  user_name = "ubuntu"
  host_ip = "192.168.1.103"
  ssh_port = 22
  host_password = "ssh_pass_3"
```

## Configuration Tips

## Best Practices

### Performance Optimization

1. **Batch Size Tuning**: Increase `bulk_size` for high-throughput environments
2. **Buffer Sizing**: Adjust `max_buffered_events` based on workload
3. **Socket Tracking**: Set `max_sockets` based on application connection count
4. **Log Level**: Use `log_level=0` in production to minimize overhead
5. **Process Filtering**: Use `pids` array to monitor only relevant processes

### Security

1. **Credential Protection**: Store passwords securely, avoid committing to version control
2. **SSH Keys**: Consider using SSH keys instead of passwords for agent management
3. **Network Security**: Use firewall rules to restrict Elasticsearch access
4. **Elasticsearch Security**: Enable Elasticsearch security features in production

### Monitoring

1. **Metrics Collection**: Enable file-based metrics collection for agent monitoring
2. **Log Levels**: Use `log_level=1` for debugging, `log_level=0` for production
3. **Cleanup Intervals**: Adjust `cleanup_interval` based on span lifetime
4. **Index Management**: Use separate indices per agent for better organization

## Next Steps

- **[Agent API](./agent.md)**: Agent management and control
- **[Server API](./server.md)**: Server management and CLI tools
- **[Data Formats](./data-formats.md)**: Span and trace data structures