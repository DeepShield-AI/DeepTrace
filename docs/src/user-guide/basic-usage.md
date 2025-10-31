# Basic Usage

This guide covers the essential operations for using DeepTrace. After completing the installation and initial setup, follow these instructions to start collecting and analyzing distributed traces.

## Prerequisites

- ✅ **Completed installation** ([Installation Guide](../getting-started/installation.md))
- ✅ **Configured the system** ([Configuration Guide](../getting-started/configuration.md))
- ✅ **Deployed server and agent** ([All-in-One Guide](../getting-started/all-in-one.md))
- ✅ **Deployed microservice applications** to trace

## Core Workflow

1. Deploy and start agents
2. Generate traffic to microservice applications
3. Correlate spans using chosen algorithm
4. Assemble traces from correlated spans
5. Analyze results in Kibana

## 1. Agent Management

### Installing the Agent

```bash
# Install agent on configured hosts
sudo docker exec -it deeptrace_server python -m cli.src.cmd agent install
```

### Starting the Agent

```bash
# Start the agent on target hosts
sudo docker exec -it deeptrace_server python -m cli.src.cmd agent run
```

### Stopping the Agent

```bash
# Stop the agent
sudo docker exec -it deeptrace_server python -m cli.src.cmd agent stop
```

## 2. Span Collection

By default, DeepTrace automatically collects spans from all Docker containers on the monitored hosts.

### Configure Specific Processes

Edit the configuration file to monitor specific PIDs:

```toml
# In server/config/config.toml
[ebpf.trace]
pids = [1234, 5678, 9012]  # Specific process IDs to monitor
```

## 3. Span Correlation

### Available Algorithms

DeepTrace supports two correlation algorithms:

| Algorithm | Description |
|-----------| -------------|
| `deeptrace` | Advanced transaction-based correlation (Recommended) |
| `fifo` | Simple first-in-first-out correlation |

### Running Correlation

```bash
# Use DeepTrace algorithm (recommended)
sudo docker exec -it deeptrace_server python -m cli.src.cmd asso algo deeptrace

# Use FIFO algorithm
sudo docker exec -it deeptrace_server python -m cli.src.cmd asso algo fifo
```

## 4. Trace Assembly

```bash
# Assemble traces from correlated spans
sudo docker exec -it deeptrace_server python -m cli.src.cmd assemble
```

## 5. Data Analysis

### Web Interface Access

Access Kibana to view and analyze traces:

```
URL: http://YOUR_SERVER_IP:5601
Username: elastic
Password: YOUR_ELASTIC_PASSWORD
```

### Viewing Traces

1. Navigate to **Discover** in Kibana
2. Select the `traces` index pattern
3. Set the time range
4. View and analyze collected traces

## 6. Common Operations

### Monitoring System Health

```bash
# Check Elasticsearch health
curl http://localhost:9200/_cluster/health

# Monitor container resource usage
sudo docker stats
```

### Data Management

```bash
# Clear all database tables
sudo docker exec -it deeptrace_server python -m cli.src.cmd db clear

# Delete specific index
curl -X DELETE "localhost:9200/traces"
```

## 7. Troubleshooting

### No Traces Collected

1. Verify agent is running
2. Check that traffic is being sent to the microservice application
3. Ensure PIDs are correctly configured in the configuration file
4. Check Elasticsearch is accessible

### Poor Correlation Results

1. Try different correlation algorithms (deeptrace vs fifo)
2. Ensure sufficient spans are collected before correlation
3. Verify microservice application is generating proper network traffic

## 8. Cleanup

Remove DeepTrace agents and server:

```bash
sudo bash scripts/clear.sh
```

## Additional Resources

- **[Workloads](./workloads.md)**: Deploy test microservice applications
- **[Configuration](../getting-started/configuration.md)**: Detailed configuration options
