# Agent API Reference

**Note**: The DeepTrace Agent currently does not expose a RESTful API. Agent configuration and control are performed through:

1. **Configuration File**: `agent/config/deeptrace.toml`
2. **Command Line**: Direct execution and control via shell scripts
3. **Server Management**: Remote management via the DeepTrace Server

## Agent Architecture

The DeepTrace Agent is a standalone process that:
- Loads eBPF programs into the kernel
- Collects spans from eBPF programs
- Performs span correlation locally
- Sends correlated spans directly to Elasticsearch

There is no HTTP API exposed by the agent itself.

## Agent Management

### Configuration Management

Agent configuration is managed through the `deeptrace.toml` file:

```toml
[agent]
name = "agent-1"

[ebpf.trace]
log_level = 1
pids = [12345, 67890]
max_buffered_events = 128

[sender.elastic.trace]
node_url = "http://localhost:9200"
username = "elastic"
password = "password"
index_name = "spans_agent1"
```

**Configuration Updates**:
- Edit `agent/config/deeptrace.toml`
- Restart the agent for changes to take effect

### Process Control

#### Start Agent

```bash
# Using the run script
cd /path/to/DeepTrace
./scripts/run_agent.sh

# Or directly
cd agent
cargo run --release
```

#### Stop Agent

```bash
# Using the stop script
./scripts/stop_agent.sh

# Or manually
pkill -f deeptrace
```

#### Check Agent Status

```bash
# Check if agent is running
ps aux | grep deeptrace

# Check eBPF programs loaded
sudo bpftool prog list | grep observ

# Check eBPF maps
sudo bpftool map list
```

### Monitoring

#### eBPF Program Status

```bash
# List loaded eBPF programs
sudo bpftool prog show

# Show program statistics
sudo bpftool prog show id <ID> --json | jq '.run_time_ns, .run_cnt'
```

#### Map Inspection

```bash
# List all maps
sudo bpftool map list

# Dump map contents
sudo bpftool map dump name PIDS
sudo bpftool map dump name EVENTS
```

#### Resource Usage

```bash
# CPU and memory usage
top -p $(pgrep deeptrace)

# Detailed process information
cat /proc/$(pgrep deeptrace)/status
```

### Log Files

Agent logs are written to:
- **Standard Output**: Real-time logging
- **eBPF Logs**: Kernel ring buffer (via `aya-log`)

```bash
# View eBPF logs
sudo cat /sys/kernel/debug/tracing/trace_pipe

# Or using bpftrace
sudo bpftrace -e 'tracepoint:raw_syscalls:sys_enter { printf("%s\\n", comm); }'
```

## Server-Based Management

The DeepTrace Server can manage agents remotely via SSH:

### Server Configuration

```toml
# server/config/config.toml
[[agents]]
  [agents.agent_info]
  agent_name = "agent-1"
  user_name = "ubuntu"
  host_ip = "192.168.1.101"
  ssh_port = 22
  host_password = "password"
```

### Remote Operations

The server can:
1. **Deploy Agent**: Copy agent binary and configuration
2. **Start/Stop Agent**: Execute control commands via SSH
3. **Sync Configuration**: Update agent configuration remotely
4. **Monitor Status**: Check agent health and resource usage

### Server CLI

```bash
# Deploy agent to remote host
python server/cli/deploy_agent.py --agent agent-1

# Start agent
python server/cli/start_agent.py --agent agent-1

# Stop agent
python server/cli/stop_agent.py --agent agent-1

# Check agent status
python server/cli/check_agent.py --agent agent-1
```

## Data Access

### Elasticsearch Queries

Spans are stored directly in Elasticsearch. Access them via:

```bash
# Query spans
curl -X GET "http://localhost:9200/spans_agent1/_search" \
  -H 'Content-Type: application/json' \
  -d '{
    "query": {
      "match_all": {}
    },
    "size": 10
  }'

# Query by service
curl -X GET "http://localhost:9200/spans_*/_search" \
  -H 'Content-Type: application/json' \
  -d '{
    "query": {
      "match": {
        "service_name": "user-service"
      }
    }
  }'
```

### Kibana Visualization

Access spans through Kibana:
1. Navigate to `http://localhost:5601`
2. Create index pattern: `spans_*`
3. Use Discover to explore spans
4. Create visualizations and dashboards

## Troubleshooting

### Agent Won't Start

```bash
# Check eBPF support
uname -r  # Kernel version should be >= 5.10

# Check BTF support
ls /sys/kernel/btf/vmlinux

# Check permissions
sudo -v  # Agent needs sudo for eBPF operations
```

### No Spans Collected

```bash
# Check if PIDs are configured
grep pids agent/config/deeptrace.toml

# Verify processes are running
ps aux | grep <process_name>

# Check eBPF programs are attached
sudo bpftool prog list | grep sys_enter
```

### Elasticsearch Connection Issues

```bash
# Test Elasticsearch connection
curl http://localhost:9200/_cluster/health

# Check agent configuration
grep node_url agent/config/deeptrace.toml

# Verify credentials
curl -u elastic:password http://localhost:9200
```

## Next Steps

- **[Configuration Schema](./configuration.md)**: Detailed configuration options
- **[Data Formats](./data-formats.md)**: Span data structure
- **[Server API](./server.md)**: Server management interface