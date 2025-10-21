# Configuration Guide

This comprehensive guide covers all aspects of configuring DeepTrace for your specific environment and requirements. Proper configuration is essential for optimal performance and accurate trace collection.

## Configuration Overview

DeepTrace uses TOML configuration files to manage settings for both server and agent components. The configuration system is designed to be:

- **Hierarchical**: Settings can be organized in logical groups
- **Flexible**: Support for multiple deployment scenarios
- **Secure**: Sensitive information can be externalized
- **Validated**: Configuration is checked at startup

## Configuration Files

### Main Configuration File

The primary configuration file is located at:
```
server/config/config.toml
```

### Example Configurations

DeepTrace provides several example configurations:

- `config.toml.example` - Basic configuration template
- `full.toml` - Complete configuration with all options
- `production.toml` - Production-ready configuration
- `development.toml` - Development environment settings

## Required Configuration

Before deploying DeepTrace, you **must** configure these essential fields:

### Server Configuration

```toml
[server]
# External IP address of the DeepTrace server
ip = "192.168.1.100"  # Replace with your server's IP

# Server port (default: 7901)
port = 7901

# WebSocket path for agent connections
path = "deeptrace/ws"
```

### Elasticsearch Configuration

```toml
[elastic]
# Elasticsearch password (choose a secure password)
elastic_password = "your_secure_password_here"

# Elasticsearch username (default: elastic)
username = "elastic"

# Elasticsearch port (default: 9200)
port = 9200

# Connection timeout in seconds
request_timeout = 10

# Bulk write size for performance optimization
bulk_size = 1024

# Index name for agent status
agent_status_index = "agent_status"
```

### Agent Configuration

```toml
[[agents.agent_info]]
# Unique identifier for this agent
agent_name = "agent-production-1"

# SSH connection details
user_name = "ubuntu"              # SSH username
host_ip = "192.168.1.101"        # Agent host IP
ssh_port = 22                     # SSH port
host_password = "ssh_password"    # SSH password (consider using SSH keys)

# Worker threads for data processing
workers = 16
```

## Optional Configuration

### Server Settings

```toml
[server]
ip = "0.0.0.0"
port = 7901
path = "deeptrace/ws"

# Additional server settings
max_connections = 1000
connection_timeout = 30
heartbeat_interval = 10
```

### DeepTrace Service

```toml
# DeepTrace agent service port
deeptrace_port = 52001

# Service configuration
[service]
log_level = "info"
log_file = "/var/log/deeptrace/agent.log"
pid_file = "/var/run/deeptrace/agent.pid"
```

### Agent Performance Settings

```toml
[agents]
# Span processing configuration
[agents.span]
batch_size = 1024
flush_interval = 5
max_queue_size = 10000

# Data sender configuration
[agents.sender]
mem_buffer_size = 16      # MB
file_buffer_size = 32     # MB
file_size_limit = 1024    # MB
batch_size = 1024
compression = true
retry_attempts = 3
retry_delay = 1000        # milliseconds

# Trace collection settings
[agents.trace]
# Specific PIDs to monitor (empty = monitor all Docker containers)
pids = []

# Process filtering
include_processes = ["nginx", "redis", "mongodb"]
exclude_processes = ["systemd", "kernel"]

# Protocol detection
auto_detect_protocols = true
supported_protocols = ["http", "grpc", "redis", "mongodb", "mysql"]
```

### API Configuration

```toml
[agents.api]
# API server settings
port = 7899
address = "0.0.0.0"
workers = 1
ident = "deeptrace"

# API security
enable_auth = false
api_key = "your_api_key_here"

# Rate limiting
rate_limit = 1000  # requests per minute
burst_limit = 100
```

## Advanced Configuration

### Multiple Agents

Configure multiple agents for distributed deployments:

```toml
# Agent 1 - Web servers
[[agents.agent_info]]
agent_name = "web-cluster-1"
user_name = "ubuntu"
host_ip = "192.168.1.101"
ssh_port = 22
host_password = "password1"
workers = 8

# Agent 2 - Database servers
[[agents.agent_info]]
agent_name = "db-cluster-1"
user_name = "ubuntu"
host_ip = "192.168.1.102"
ssh_port = 22
host_password = "password2"
workers = 16

# Agent 3 - Cache servers
[[agents.agent_info]]
agent_name = "cache-cluster-1"
user_name = "ubuntu"
host_ip = "192.168.1.103"
ssh_port = 22
host_password = "password3"
workers = 4
```

### Environment-Specific Settings

#### Production Configuration

```toml
[server]
ip = "prod-deeptrace.company.com"
port = 7901

[elastic]
elastic_password = "${ELASTIC_PASSWORD}"  # Use environment variable
port = 9200
bulk_size = 2048
request_timeout = 30

[agents.trace]
# Monitor specific production services
include_processes = ["nginx", "app-server", "redis", "postgres"]

[agents.sender]
# Optimized for production throughput
mem_buffer_size = 64
file_buffer_size = 128
batch_size = 2048
compression = true

[logging]
level = "warn"
file = "/var/log/deeptrace/production.log"
rotation = "daily"
max_files = 30
```

#### Development Configuration

```toml
[server]
ip = "localhost"
port = 7901

[elastic]
elastic_password = "dev_password"
port = 9200

[agents.trace]
# Monitor all processes in development
pids = []

[logging]
level = "debug"
console = true
file = "/tmp/deeptrace-dev.log"
```

### Security Configuration

#### SSH Key Authentication

Instead of passwords, use SSH keys for better security:

```toml
[[agents.agent_info]]
agent_name = "secure-agent-1"
user_name = "deeptrace"
host_ip = "192.168.1.101"
ssh_port = 22
# Remove host_password and configure SSH keys instead
ssh_key_path = "/home/deeptrace/.ssh/id_rsa"
```

#### TLS Configuration

Enable TLS for secure communication:

```toml
[server.tls]
enabled = true
cert_file = "/etc/deeptrace/certs/server.crt"
key_file = "/etc/deeptrace/certs/server.key"
ca_file = "/etc/deeptrace/certs/ca.crt"

[elastic.tls]
enabled = true
verify_certificates = true
ca_file = "/etc/elasticsearch/certs/ca.crt"
```

## Environment Variables

DeepTrace supports environment variable substitution in configuration files:

```toml
[elastic]
elastic_password = "${ELASTIC_PASSWORD}"
username = "${ELASTIC_USER:-elastic}"  # Default value: elastic

[server]
ip = "${SERVER_IP}"
port = "${SERVER_PORT:-7901}"  # Default value: 7901
```

Set environment variables:

```bash
export ELASTIC_PASSWORD="secure_password"
export ELASTIC_USER="deeptrace_user"
export SERVER_IP="192.168.1.100"
export SERVER_PORT="7901"
```

## Configuration Validation

DeepTrace validates configuration at startup. Common validation errors:

### Missing Required Fields

```
Error: Missing required field 'server.ip'
```

**Solution**: Ensure all required fields are configured.

### Invalid Values

```
Error: Invalid port number: 99999
```

**Solution**: Use valid port numbers (1-65535).

### Network Connectivity

```
Error: Cannot connect to Elasticsearch at localhost:9200
```

**Solution**: Verify Elasticsearch is running and accessible.

## Configuration Best Practices

### 1. Security

- **Use environment variables** for sensitive information
- **Enable TLS** for production deployments
- **Use SSH keys** instead of passwords
- **Restrict network access** to DeepTrace ports

### 2. Performance

- **Tune batch sizes** based on your traffic volume
- **Adjust worker threads** based on CPU cores
- **Configure appropriate timeouts** for your network
- **Enable compression** for network efficiency

### 3. Monitoring

- **Set appropriate log levels** for your environment
- **Configure log rotation** to prevent disk space issues
- **Monitor resource usage** and adjust settings accordingly
- **Set up alerts** for configuration-related errors

### 4. Maintenance

- **Version control** your configuration files
- **Document custom settings** and their purposes
- **Test configuration changes** in development first
- **Keep backups** of working configurations

## Configuration Templates

### Small Deployment (1-5 hosts)

```toml
[server]
ip = "192.168.1.100"
port = 7901

[elastic]
elastic_password = "simple_password"
bulk_size = 512

[[agents.agent_info]]
agent_name = "small-deployment"
user_name = "ubuntu"
host_ip = "192.168.1.101"
ssh_port = 22
host_password = "password"
workers = 4

[agents.sender]
batch_size = 512
mem_buffer_size = 16
```

### Medium Deployment (5-20 hosts)

```toml
[server]
ip = "deeptrace.internal.com"
port = 7901

[elastic]
elastic_password = "${ELASTIC_PASSWORD}"
bulk_size = 1024
request_timeout = 20

# Multiple agents configuration...
[agents.sender]
batch_size = 1024
mem_buffer_size = 32
compression = true
```

### Large Deployment (20+ hosts)

```toml
[server]
ip = "deeptrace-cluster.company.com"
port = 7901
max_connections = 5000

[elastic]
elastic_password = "${ELASTIC_PASSWORD}"
bulk_size = 2048
request_timeout = 30

[agents.sender]
batch_size = 2048
mem_buffer_size = 64
file_buffer_size = 128
compression = true

[logging]
level = "warn"
file = "/var/log/deeptrace/production.log"
```

## Troubleshooting Configuration

### Common Issues

#### Configuration File Not Found

```bash
# Check file exists and permissions
ls -la server/config/config.toml
chmod 644 server/config/config.toml
```

#### Invalid TOML Syntax

```bash
# Validate TOML syntax
python3 -c "import toml; toml.load('server/config/config.toml')"
```

#### Network Connectivity

```bash
# Test server connectivity
telnet 192.168.1.100 7901

# Test Elasticsearch connectivity
curl http://localhost:9200/_cluster/health
```

## Next Steps

After configuring DeepTrace:

1. **[All-in-One Deployment](./all-in-one.md)**: Deploy for testing
2. **[Basic Usage](../user-guide/basic-usage.md)**: Start using DeepTrace
3. **[Troubleshooting](../troubleshooting/common-issues.md)**: Resolve issues

---

Proper configuration is crucial for DeepTrace's performance and reliability. Take time to understand each setting and adjust them according to your specific requirements and environment.
