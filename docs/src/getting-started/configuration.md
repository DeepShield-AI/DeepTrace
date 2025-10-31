# Configuration Guide

This comprehensive guide covers all aspects of configuring DeepTrace for your specific environment and requirements. DeepTrace consists of two main components that require separate configuration: the **Server** and the **Agent**.

## Configuration Overview

DeepTrace uses TOML configuration files to manage settings. The configuration system is designed to be:

- **Simple**: Straightforward configuration structure
- **Flexible**: Support for multiple deployment scenarios  
- **Secure**: Sensitive information can be externalized
- **Validated**: Configuration is checked at startup

## Configuration Files

DeepTrace provides several configuration files:

- **Server**: `server/config/config.toml` - Server and agent management configuration
- **Agent**: `agent/config/deeptrace.toml` - Agent-side configuration (current)
- **Agent Template**: `agent/config/deeptrace.toml.example` - Agent configuration template
- **Prism**: `agent/config/prism.toml` - Lightweight monitoring configuration

---

# Server Configuration

The server configuration manages the DeepTrace server, Elasticsearch integration, and agent deployment settings. The main configuration file is located at `server/config/config.toml`.

## Required Server Configuration

The server configuration is simple and requires only essential fields:

### Server Settings

```toml
[server]
# External IP address of the DeepTrace server (REQUIRED)
ip = "192.168.1.100"  # Replace with your server's IP
```

### Elasticsearch Configuration

```toml
[elastic]
# Elasticsearch password (REQUIRED - choose a secure password)
elastic_password = "your_secure_password_here"
```

## Agent Management Configuration

The server manages agent deployments through SSH connections:

### Single Agent Configuration

```toml
[[agents]]
  [agents.agent_info]
  # Unique identifier for this agent (REQUIRED)
  agent_name = "agent-1"
  
  # SSH connection details (ALL REQUIRED)
  user_name = "ubuntu"              # SSH username
  host_ip = "192.168.1.101"        # Agent host IP
  ssh_port = 22                     # SSH port (usually 22)
  host_password = "ssh_password"    # SSH password (consider using SSH keys)
```

### Multiple Agents Configuration

```toml
# Agent 1 - Web servers
[[agents]]
  [agents.agent_info]
  agent_name = "agent-1"
  user_name = "ubuntu"
  host_ip = "192.168.1.101"
  ssh_port = 22
  host_password = "password1"

# Agent 2 - Database servers  
[[agents]]
  [agents.agent_info]
  agent_name = "agent-2"
  user_name = "ubuntu"
  host_ip = "192.168.1.102"
  ssh_port = 22
  host_password = "password2"

# Agent 3 - Cache servers
[[agents]]
  [agents.agent_info]
  agent_name = "agent-3"
  user_name = "ubuntu"
  host_ip = "192.168.1.103"
  ssh_port = 22
  host_password = "password3"
```

# Agent Configuration

The agent configuration defines how the DeepTrace agent operates on target systems. The main configuration file is `agent/config/deeptrace.toml`.

## Required Agent Configuration

### Basic Agent Settings

```toml
[agent]
name = "deeptrace"                # Agent identifier (required)
```

## Configuration Modules

### Metric Collection Configuration

```toml
[metric]
interval = 10                     # Metric collection interval (seconds)
sender = "metric"                 # Sender configuration name for metrics
```

### Data Sending Configuration

#### File-based Storage for Metrics

```toml
[sender.file.metric]
path = "metrics.csv"              # File path for metrics storage
rotate = true                     # Enable file rotation
max_size = 512                    # Maximum file size (MB)
max_age = 7                       # Maximum retention (days)
rotate_time = 10                  # Rotation interval (days)
data_format = "%Y%m%d"            # Timestamp format for rotation
```

#### Elasticsearch Sender for Traces

```toml
[sender.elastic.trace]
node_urls = "http://localhost:9200"      # Elasticsearch URL
username = "elastic"                     # Elasticsearch username
password = "your_password"               # Elasticsearch password
request_timeout = 10                     # Request timeout (seconds)
index_name = "agent1"                    # Index name for this agent
bulk_size = 32                           # Bulk operation size
```

### Tracing Configuration

```toml
[trace]
ebpf = "trace"                    # eBPF configuration name for tracing
sender = "trace"                  # Sender configuration name for traces

[trace.span]
cleanup_interval = 30             # Cleanup interval for expired spans (seconds)
max_sockets = 1024                # Maximum tracked socket count
```

### eBPF Configuration

```toml
[ebpf.trace]
log_level = 1                     # Log level: 0=off, 1=debug, 3=verbose, 4=stats
pids = [523094]                   # Process IDs to monitor (specific PIDs)
max_buffered_events = 128         # Maximum events processed per batch
enabled_probes = [                # List of enabled system call probes
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

## Complete Configuration Examples

### Full-Featured Agent Configuration

```toml
[agent]
name = "production-agent"

[metric]
interval = 5
sender = "metric"

[sender.file.metric]
path = "/var/log/deeptrace/metrics.csv"
rotate = true
max_size = 256
max_age = 30
rotate_time = 7
data_format = "%Y%m%d"

[sender.elastic.trace]
node_urls = "http://prod-elastic:9200"
username = "elastic"
password = "prod_password"
request_timeout = 30
index_name = "production_traces"
bulk_size = 64

[trace]
ebpf = "trace"
sender = "trace"

[trace.span]
cleanup_interval = 30
max_sockets = 10000

[ebpf.trace]
log_level = 1
enabled_probes = [
    "sys_enter_read",
    "sys_exit_read",
    "sys_enter_recvfrom",
    "sys_exit_recvfrom",
    "sys_enter_write",
    "sys_exit_write",
    "sys_enter_sendto",
    "sys_exit_sendto",
    "sys_exit_socket",
    "sys_enter_close"
]
max_buffered_events = 256
pids = []  # Monitor no processes
```

# Troubleshooting Configuration

## Common Server Issues

### Configuration File Not Found

```bash
# Check file exists and permissions
ls -la server/config/config.toml
chmod 644 server/config/config.toml
```

### Invalid TOML Syntax

```bash
# Validate TOML syntax
python3 -c "import toml; toml.load('server/config/config.toml')"
```

### Agent Connection Issues

```bash
# Test SSH connectivity to agent
ssh ubuntu@192.168.1.101 -p 22

# Test DeepTrace server port
telnet 192.168.1.100 7901
```

## Common Agent Issues

### Configuration Loading Errors

```bash
# Check agent configuration syntax
cd agent/config
python3 -c "import toml; toml.load('deeptrace.toml')"

# Validate configuration structure
cargo run --bin prism -- --config-path config/prism.toml --validate-only
```

### Network Connectivity

```bash
# Test server connectivity
telnet 192.168.1.100 7901

# Test Elasticsearch connectivity
curl http://192.168.1.100:9200/_cluster/health
```

### Permission Issues

```bash
# Check eBPF capabilities
sudo setcap cap_sys_admin,cap_net_admin,cap_bpf+ep /path/to/deeptrace

# Check file permissions
ls -la agent/config/deeptrace.toml
chmod 644 agent/config/deeptrace.toml
```

# Next Steps

After configuring DeepTrace:

1. **[All-in-One Deployment](./all-in-one.md)**: Deploy for testing
2. **[Basic Usage](../user-guide/basic-usage.md)**: Start using DeepTrace
3. **[Troubleshooting](../troubleshooting/common-issues.md)**: Resolve issues