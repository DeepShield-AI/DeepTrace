# Basic Usage

This guide covers essential operations for using DeepTrace after completing the initial setup. It focuses on day-to-day operations and advanced usage patterns.

> **Prerequisites**: Complete the [Quick Start Guide](../getting-started/quick-start.md) before using this guide.

## Core Workflow

DeepTrace follows a simple workflow for distributed tracing:

```
1. Agent Collection → 2. Span Correlation → 3. Trace Assembly → 4. Analysis
```

## Advanced Agent Operations

### Agent Status Management

```bash
# Check agent status
sudo docker exec -it deeptrace_server python -m cli.src.cmd agent status

# Restart agent with new configuration
sudo docker exec -it deeptrace_server python -m cli.src.cmd agent restart

# View agent logs
sudo docker exec -it deeptrace_server python -m cli.src.cmd agent logs
```

## Span Correlation

### Available Algorithms

| Algorithm | Description | Use Case |
|-----------|-------------|----------|
| `deeptrace` | Advanced transaction-based correlation | **Recommended** for most scenarios |
| `fifo` | Simple first-in-first-out correlation | Testing and simple applications |

### Run Correlation

```bash
# Use DeepTrace algorithm (recommended)
sudo docker exec -it deeptrace_server python -m cli.src.cmd asso algo deeptrace

# Alternative: Use FIFO algorithm
sudo docker exec -it deeptrace_server python -m cli.src.cmd asso algo fifo
```

## Trace Assembly

After correlation, assemble traces from correlated spans:

```bash
# Assemble traces
sudo docker exec -it deeptrace_server python -m cli.src.cmd assemble
```

## Advanced Data Analysis

For basic trace viewing, see the [Quick Start Guide](../getting-started/quick-start.md). This section covers advanced analysis techniques.

### Advanced Kibana Operations

```bash
# Create custom index patterns
# Set up advanced visualizations  
# Configure dashboards for monitoring
```

For detailed analysis techniques, see [Trace Analysis](./trace-analysis.md).

## System Monitoring

### Health Checks

```bash
# Check Elasticsearch cluster health
curl http://localhost:9200/_cluster/health

# Monitor container resource usage
sudo docker stats

# Verify DeepTrace containers
sudo docker ps | grep deeptrace
```

### Data Management

```bash
# Clear all collected data
sudo docker exec -it deeptrace_server python -m cli.src.cmd db clear

# Delete specific Elasticsearch index
curl -X DELETE "localhost:9200/traces"
```

## Troubleshooting

### No Traces Collected

**Common causes and solutions:**

1. **Agent not running**: Verify agent status and restart if needed
2. **No traffic**: Ensure microservice applications are receiving requests
3. **Network issues**: Check connectivity between agent and server
4. **Elasticsearch issues**: Verify Elasticsearch is accessible and healthy

### Poor Correlation Results

**Optimization strategies:**

1. **Try different algorithms**: Switch between `deeptrace` and `fifo`
2. **Increase data collection**: Ensure sufficient spans before correlation
3. **Check application traffic**: Verify microservices are generating network activity
4. **Review configuration**: Ensure proper agent and server configuration

## Cleanup

### Remove DeepTrace

To completely remove DeepTrace and all components:

```bash
sudo bash scripts/clear.sh
```

This will:
- Stop all containers
- Remove Docker images
- Clean up temporary files
- Reset the environment

## Next Steps

- **[Web UI](./web-ui.md)**: Explore the web-based monitoring interface
- **[Database Setup](./database.md)**: Advanced Elasticsearch configuration
- **[Workload Applications](./workloads.md)**: Deploy additional test applications
- **[Configuration Guide](../getting-started/configuration.md)**: Advanced configuration options
