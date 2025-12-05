# Server Management Reference

**Note**: The DeepTrace Server is not an API server. It is a Python-based management tool for:

1. **Agent Deployment**: Deploy agents to remote hosts via SSH
2. **Agent Management**: Start, stop, and configure agents remotely  
3. **Configuration Sync**: Synchronize agent configurations
4. **Infrastructure Setup**: Deploy Elasticsearch and Kibana
5. **Trace Processing**: Span correlation and trace assembly
6. **Analytics**: Service metrics and call graph construction

## Server Architecture

The DeepTrace Server is a Python application that:
- Manages multiple agents via SSH connections
- Deploys and configures Elasticsearch/Kibana
- Synchronizes agent configurations
- Performs span correlation using various algorithms
- Assembles traces from correlated spans
- Generates service metrics and call graphs

There is no HTTP API exposed by the server. All operations are performed via:
- **CLI Scripts**: Python command-line tools (`python server/cli/src/cmd.py`)
- **Configuration Files**: TOML-based configuration
- **SSH**: Remote agent management

## Server Configuration

The server is configured via `server/config/config.toml`:

```toml
[server]
# External IP address of the DeepTrace server
ip = "192.168.1.100"

[elastic]
# Elasticsearch password
elastic_password = "your_password"

# Agent management (can have multiple agents)
[[agents]]
  [agents.agent_info]
  agent_name = "agent-1"
  user_name = "ubuntu"
  host_ip = "192.168.1.101"
  ssh_port = 22
  host_password = "ssh_password"

[[agents]]
  [agents.agent_info]
  agent_name = "agent-2"
  user_name = "ubuntu"
  host_ip = "192.168.1.102"
  ssh_port = 22
  host_password = "ssh_password"
```

## CLI Commands

### Agent Management

#### Install Agents

Deploy agent binaries and configurations to remote hosts:

```bash
python server/cli/src/cmd.py agent install
```

**Operations**:
- Copies agent binary to remote host
- Creates necessary directories
- Deploys configuration files
- Sets up permissions

#### Start Agents

Start all configured agents:

```bash
python server/cli/src/cmd.py agent run
```

**Operations**:
- Executes agent start script via SSH
- Verifies agent process is running
- Checks eBPF programs are loaded

#### Stop Agents

Stop all running agents:

```bash
python server/cli/src/cmd.py agent stop
```

**Operations**:
- Sends stop signal to agent process
- Cleans up eBPF programs
- Verifies agent has stopped

#### Test Agent Connections

Test SSH connectivity to all agents:

```bash
python server/cli/src/cmd.py agent test
```

**Operations**:
- Tests SSH connection to each agent host
- Verifies credentials
- Reports connection status

#### Sync Agent Configuration

Synchronize configuration from server to agents:

```bash
python server/cli/src/cmd.py agent sync
```

**Operations**:
- Generates agent-specific configuration
- Updates Elasticsearch connection settings
- Copies configuration to remote hosts
- Optionally restarts agents

#### Install Workload Applications

Deploy workload applications (BookInfo, Social Network) to agent hosts:

```bash
python server/cli/src/cmd.py agent install_app
```

#### Uninstall Workload Applications

Remove workload applications from agent hosts:

```bash
python server/cli/src/cmd.py agent uninstall_app
```

### Span Correlation

#### Run Correlation Algorithm

Correlate spans using a specific algorithm:

```bash
# Using FIFO algorithm
python server/cli/src/cmd.py asso algo fifo

# Using DeepTrace algorithm
python server/cli/src/cmd.py asso algo deeptrace
```

**Available Algorithms**:
- **fifo**: First-In-First-Out correlation
- **deeptrace**: Advanced transaction-based correlation

**Operations**:
- Reads spans from Elasticsearch
- Performs inter-service association
- Applies selected correlation algorithm
- Writes correlated spans back to Elasticsearch

### Trace Assembly

#### Assemble Traces

Assemble complete traces from correlated spans:

```bash
python server/cli/src/cmd.py assemble
```

**Operations**:
- Reads correlated spans from Elasticsearch
- Groups spans by trace ID
- Constructs trace hierarchy
- Stores assembled traces

### Database Management

#### Clear Database

Clear all data from Elasticsearch:

```bash
python server/cli/src/cmd.py db clear
```

**Warning**: This operation deletes all spans and traces from Elasticsearch.

### Trace Testing

#### Test Correlation and Assembly

Test the complete trace processing pipeline with a specific algorithm:

```bash
# Test with FIFO
python server/cli/src/cmd.py trace test fifo

# Test with DeepTrace
python server/cli/src/cmd.py trace test deeptrace

# Test with other algorithms
python server/cli/src/cmd.py trace test vpath
python server/cli/src/cmd.py trace test wap5
python server/cli/src/cmd.py trace test traceweaver_v1
python server/cli/src/cmd.py trace test traceweaver_v2
```

**Available Algorithms**:
- **fifo**: First-In-First-Out
- **deeptrace**: Transaction-based correlation
- **vpath**: Virtual path-based correlation
- **wap5**: WAP5 algorithm
- **traceweaver_v1**: TraceWeaver version 1
- **traceweaver_v2**: TraceWeaver version 2

### Service Analytics

#### Get Service Metrics

Calculate and display service-level metrics:

```bash
python server/cli/src/cmd.py service metrics
```

**Metrics Provided**:
- Request count per service
- Error rates
- Average latency
- P95/P99 latencies
- Throughput

### Call Graph Construction

#### Construct Service Call Graph

Build a call graph from traces:

```bash
python server/cli/src/cmd.py graph construct
```

**Operations**:
- Analyzes service dependencies from traces
- Constructs directed graph of service calls
- Calculates edge weights (call frequency)
- Exports graph visualization

## Python API

### Agent Management API

```python
from controller.src.utils import *
from config.parse_config import load_agents

# Load agent configurations
agents = load_agents()

# Install agents
install_agents(agents)

# Start agents
start_agents(agents)

# Stop agents
stop_agents(agents)

# Test agent connections
test_agents(agents)

# Sync configuration
sync_agent_config(agents)

# Install workload
install_workload(agents)

# Uninstall workload
uninstall_workload(agents)
```

### Span Correlation API

```python
from database.src.utils import es_read_agent_span_list
from trace.association.src.cross import inter_association
from trace.association.src import fifo, deeptrace

# Load agents
agents = load_agents()

# Read spans from Elasticsearch
spans = es_read_agent_span_list(agents)

# Perform inter-service association
spans = inter_association(
    spans, 
    client_ingress='ComposePost',
    tuple_used=True,
    clock_skew=False
)

# Apply correlation algorithm
span_dict = fifo.fifo(spans)
# or
span_dict = deeptrace.deeptrace(spans)
```

### Trace Assembly API

```python
from trace.assemble.src.utils import assemble_trace_from_db

# Assemble traces from database
traces = assemble_trace_from_db()
```

### Service Metrics API

```python
from service.src.metric import service_metrics

# Calculate service metrics
metrics = service_metrics()
```

### Call Graph API

```python
from callgraph.src.graph import construct_graph

# Construct service call graph
graph = construct_graph()
```

## Data Access

### Elasticsearch Queries

Spans and traces are stored in Elasticsearch. Access them via:

```bash
# Query spans from specific agent
curl -X GET "http://localhost:9200/spans_agent1/_search" \
  -H 'Content-Type: application/json' \
  -d '{
    "query": {
      "match_all": {}
    },
    "size": 10
  }'

# Query all spans
curl -X GET "http://localhost:9200/spans_*/_search" \
  -H 'Content-Type: application/json' \
  -d '{
    "query": {
      "range": {
        "timestamp": {
          "gte": "now-1h"
        }
      }
    }
  }'

# Query correlated spans
curl -X GET "http://localhost:9200/correlated_spans/_search"

# Query assembled traces
curl -X GET "http://localhost:9200/traces/_search"
```

### Kibana Visualization

Access data through Kibana:
1. Navigate to `http://localhost:5601`
2. Create index patterns:
   - `spans_*` for raw spans
   - `correlated_spans` for correlated spans
   - `traces` for assembled traces
3. Use Discover to explore data
4. Create visualizations and dashboards

## Workflow Examples

### Complete Deployment Workflow

```bash
# 1. Configure server
vim server/config/config.toml

# 2. Install agents
python server/cli/src/cmd.py agent install

# 3. Sync configuration
python server/cli/src/cmd.py agent sync

# 4. Start agents
python server/cli/src/cmd.py agent run

# 5. Install workload application
python server/cli/src/cmd.py agent install_app

# 6. Wait for spans to be collected...

# 7. Run correlation
python server/cli/src/cmd.py asso algo deeptrace

# 8. Assemble traces
python server/cli/src/cmd.py assemble

# 9. Generate metrics
python server/cli/src/cmd.py service metrics

# 10. Construct call graph
python server/cli/src/cmd.py graph construct
```

### Testing Workflow

```bash
# 1. Clear existing data
python server/cli/src/cmd.py db clear

# 2. Start agents
python server/cli/src/cmd.py agent run

# 3. Generate test traffic
# (application-specific)

# 4. Test correlation and assembly
python server/cli/src/cmd.py trace test deeptrace

# 5. View results in Kibana
# Navigate to http://localhost:5601
```

## Troubleshooting

### Agent Connection Issues

```bash
# Test SSH connectivity
python server/cli/src/cmd.py agent test

# Check SSH credentials in config
vim server/config/config.toml

# Verify network connectivity
ping <agent_host_ip>
```

### Correlation Issues

```bash
# Check if spans exist in Elasticsearch
curl http://localhost:9200/spans_*/_count

# Verify span structure
curl http://localhost:9200/spans_agent1/_search?size=1

# Check correlation algorithm logs
# (logs are printed to stdout)
```

### Database Issues

```bash
# Check Elasticsearch health
curl http://localhost:9200/_cluster/health

# Verify indices exist
curl http://localhost:9200/_cat/indices

# Clear and restart
python server/cli/src/cmd.py db clear
```

## Next Steps

- **[Agent API](./agent.md)**: Agent management and monitoring
- **[Configuration Schema](./configuration.md)**: Detailed configuration options
- **[Data Formats](./data-formats.md)**: Span and trace data structures



# Server API Documentation

This service is implemented with Flask and provides management interfaces for Agent registration, start, stop, deletion, configuration delivery, and configuration query.

## Basic Information
- Service address: `http://<server_ip>:59002`
- All endpoints use the `POST` method and accept `application/json` data

---

## 1. Register Agent
- **Endpoint**: `/register_agent`
- **Method**: POST
- **Request Body**:
  ```json
  {
    "host_ip": "string",
    "user_name": "string",
    "host_password": "string",
    "user_id": "string",
    "ssh_port": int,
    "agent_name": "string"
  }
  ```
- **Response Example**:
  ```json
  {"message": "Agent agent1 registered successfully"}
  ```

---

## 2. Start Agent
- **Endpoint**: `/start_agent`
- **Method**: POST
- **Request Body**: Same as Register Agent
- **Response Example**:
  ```json
  {"message": "Agent agent1 started successfully"}
  ```

---

## 3. Stop Agent
- **Endpoint**: `/stop_agent`
- **Method**: POST
- **Request Body**: Same as Register Agent
- **Response Example**:
  ```json
  {"message": "Agent agent1 stopped successfully"}
  ```

---

## 4. Delete Agent
- **Endpoint**: `/delete_agent`
- **Method**: POST
- **Request Body**: Same as Register Agent
- **Response Example**:
  ```json
  {"message": "Agent agent1 deleted successfully"}
  ```

---

## 5. Sync Agent Configuration
- **Endpoint**: `/sync_agent_config`
- **Method**: POST
- **Request Body**:
  ```json
  {
    "agent_info": {
      "agent_name": "string",
      "host_password": "string",
      "host_ip": "string",
      "user_name": "string",
      "ssh_port": int,
      "user_id": "string"
    },
    "metric": { ... },
    "sender": { ... },
    "trace": { ... },
    "ebpf": { ... }
  }
  ```
- **Response Example**:
  ```json
  {"message": "Configuration updated for agent agent1"}
  ```

---

## 6. Query Agent Configuration
- **Endpoint**: `/query_agent_config`
- **Method**: POST
- **Request Body**: Same as Register Agent
- **Response Example**:
  ```json
  {
    "agent_info": {
      "agent_name": "agent1",
      "host_password": "xxx",
      "host_ip": "118.229.43.254",
      "user_name": "ubuntu",
      "ssh_port": 6114
    },
    "metric": { ... },
    "sender": { ... },
    "trace": { ... },
    "ebpf": { ... }
  }
  ```

---

## 7. Example curl Commands
  "user_id": "user123",
  "ssh_port": 6114,
  "agent_name": "agent1"
}'

curl -X POST http://127.0.0.1:59002/start_agent -H "Content-Type: application/json" -d '{
  "host_ip": "118.229.43.254",
  "user_name": "ubuntu",
  "host_password": "netsys204",
  "user_id": "user123",
  "ssh_port": 6114,
  "agent_name": "agent1"
}'

curl -X POST http://127.0.0.1:59002/stop_agent -H "Content-Type: application/json" -d '{
  "host_ip": "118.229.43.254",
  "user_name": "ubuntu",
  "host_password": "netsys204",
  "user_id": "user123",
  "ssh_port": 6114,
  "agent_name": "agent1"
}'

curl -X POST http://127.0.0.1:59002/delete_agent -H "Content-Type: application/json" -d '{
  "host_ip": "118.229.43.254",
  "user_name": "ubuntu",
  "host_password": "netsys204",
  "user_id": "user123",
  "ssh_port": 6114,
  "agent_name": "agent1"
}'

curl -X POST http://127.0.0.1:59002/query_agent_config -H "Content-Type: application/json" -d '{
  "host_ip": "118.229.43.254",
  "user_name": "ubuntu",
  "host_password": "netsys204",
  "user_id": "user123",
  "ssh_port": 6114,
  "agent_name": "agent1"
}'

curl -X POST http://127.0.0.1:59002/sync_agent_config -H "Content-Type: application/json" -d '{
  "agent_info": {
    "agent_name": "agent1",
    "host_password": "netsys204",
    "host_ip": "118.229.43.254",
    "user_name": "ubuntu",
    "ssh_port": 6114,
    "user_id": "user123"
  },
  "metric": {
    "interval": 10,
    "sender": "metric"
  },
  "sender": {
    "elastic": {
      "trace": {
        "node_url": "http://localhost:9200",
        "username": "elastic",
        "password": "new_password",
        "request_timeout": 10,
        "index_name": "agent1",
        "bulk_size": 64
      }
    },
    "file": {
      "metric": {
        "path": "metrics.csv",
        "rotate": true,
        "max_size": 512,
        "max_age": 6,
        "rotate_time": 11,
        "data_format": "%Y%m%d"
      }
    }
  },
  "trace": {
    "ebpf": "trace",
    "sender": "trace",
    "span": {
      "cleanup_interval": 30,
      "max_sockets": 1024
    }
  },
  "ebpf": {
    "trace": {
      "log_level": 1,
      "pids": [523094],
      "max_buffered_events": 128,
      "enabled_probes": [
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
    }
  }
}'
```
