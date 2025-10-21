# All-in-One Deployment

The All-in-One deployment mode runs both the DeepTrace server and agent on a single host. This configuration is perfect for:

- **Testing and evaluation** of DeepTrace capabilities
- **Development environments** and proof-of-concepts
- **Small-scale deployments** with limited infrastructure
- **Learning and experimentation** with distributed tracing

## Prerequisites

### Hardware Requirements

| Component | Minimum | Recommended |
|-----------|---------|-------------|
| **CPU** | 2 cores | 4+ cores |
| **Memory** | 4GB RAM | 8GB+ RAM |
| **Storage** | 40GB free space | 100GB+ free space |
| **Network** | Internet connectivity | Stable broadband |

### Software Requirements

- **Operating System**: Ubuntu 24.04 LTS
- **Kernel Version**: 6.8.0+ with eBPF support
- **Docker**: v26.1.3 or later
- **Privileges**: Root or sudo access

### System Verification

Before proceeding, verify your system meets the requirements:

```bash
# Check OS version
lsb_release -a

# Check kernel version and eBPF support
uname -r
zgrep CONFIG_BPF /proc/config.gz

# Check available resources
free -h
df -h

# Verify Docker installation
docker --version
sudo docker ps
```

## Quick Deployment

### Step 1: Clone Repository

```bash
# Clone DeepTrace repository
git clone https://github.com/DeepShield-AI/DeepTrace.git
cd DeepTrace
```

> ⚠️ **Important**: Do not clone into `/etc` directory. The agent will automatically populate `/etc` during deployment.

### Step 2: Configure for All-in-One Mode

Create and edit the configuration file:

```bash
# Copy example configuration
cp server/config/config.toml.example server/config/config.toml

# Edit configuration
nano server/config/config.toml
```

**Required configuration for All-in-One mode**:

```toml
[server]
# Use your host's external IP address
ip = "192.168.1.100"  # Replace with your actual IP

[elastic]
# Set a secure password for Elasticsearch
elastic_password = "your_secure_password"

[[agents.agent_info]]
# Agent configuration
agent_name = "all-in-one-agent"
user_name = "ubuntu"              # Your SSH username
host_ip = "192.168.1.100"        # Same as server.ip for all-in-one
ssh_port = 22
host_password = "your_ssh_password"  # Your SSH password
```

> 💡 **Tip**: In all-in-one mode, `server.ip` and `agents.agent_info.host_ip` should be identical.

### Step 3: Deploy Server Components

Deploy the DeepTrace server and Elasticsearch database:

```bash
# Run deployment script
sudo bash scripts/deploy_server.sh
```

This script will:
- Pull required Docker images
- Start Elasticsearch container
- Launch DeepTrace server container
- Set up networking between components
- Initialize the web interface

**Verify server deployment**:

```bash
# Check running containers
sudo docker ps

# Expected containers:
# - deeptrace_server
# - elasticsearch
# - kibana (optional)

# Test server connectivity
curl http://localhost:7901/health
```

### Step 4: Access Web Interface

The web interface provides monitoring and management capabilities:

**Kibana Dashboard**: `http://YOUR_SERVER_IP:5601`

**Login credentials**:
- Username: `elastic`
- Password: `your_secure_password` (from configuration)

### Step 5: Deploy Sample Application (Optional)

For demonstration purposes, deploy a microservices application:

```bash
# Deploy Social Network microservice
sudo docker exec -it deeptrace_server python -m cli.src.cmd agent install_app
```

This command deploys a complete microservices environment including:
- Multiple interconnected services
- Load balancers and databases
- Traffic generators for testing

**Alternative applications**:
- **BookInfo**: See [BookInfo documentation](../user-guide/workloads/bookinfo.md)
- **Custom applications**: Deploy your own microservices

### Step 6: Install and Start Agent

Install the DeepTrace agent on the local host:

```bash
# Install agent (compiles from source)
sudo docker exec -it deeptrace_server python -m cli.src.cmd agent install

# Start the agent
sudo docker exec -it deeptrace_server python -m cli.src.cmd agent run
```

The agent installation process:
1. **Connects to the host** via SSH
2. **Clones the repository** to `/etc/deeptrace`
3. **Compiles eBPF programs** and userspace components
4. **Starts monitoring** all Docker containers
5. **Begins collecting traces** and sending to server

**Verify agent status**:

```bash
# Check agent process
ps aux | grep deeptrace

# Check agent logs
sudo docker exec -it deeptrace_server tail -f /var/log/deeptrace/agent.log

# Verify data collection
curl http://localhost:7899/status
```

## Generate and Analyze Traces

### Step 7: Generate Sample Traffic

If you deployed the sample application, generate traffic to create traces:

#### For Social Network Application

```bash
# Find the workload generator container
CONTAINER_ID=$(sudo docker ps | grep wrk2 | awk '{print $1}')

# Enter the container
sudo docker exec -it $CONTAINER_ID /bin/bash

# Generate load
cd /root
./wrk -D exp -t 6 -c 6 -d 30 -L \
  -s ./wrk2/scripts/social-network/compose-post.lua \
  http://nginx-web-server:8080/wrk2-api/post/compose \
  -R 50
```

#### For Custom Applications

```bash
# Generate HTTP requests to your services
curl http://your-service:port/api/endpoint

# Or use load testing tools
ab -n 1000 -c 10 http://your-service:port/
```

### Step 8: Build Traces

Process collected spans into complete traces:

```bash
# Perform span correlation using DeepTrace algorithm
sudo docker exec -it deeptrace_server python -m cli.src.cmd asso algo deeptrace

# Assemble correlated spans into traces
sudo docker exec -it deeptrace_server python -m cli.src.cmd assemble
```

**Available correlation algorithms**:
- `deeptrace` - Advanced transaction-based correlation (recommended)
- `fifo` - Simple first-in-first-out correlation
- `vpath` - Virtual path-based correlation
- `wap5` - WAP5 algorithm
- `traceweaver_v1` - TraceWeaver v1 algorithm
- `deepflow` - DeepFlow algorithm

### Step 9: Explore Traces

Access the web interface to explore collected traces:

1. **Open Kibana**: Navigate to `http://YOUR_SERVER_IP:5601`
2. **Login**: Use elastic credentials
3. **Discover Data**: Click "Discover" in the sidebar
4. **Select Index**: Choose the trace index pattern
5. **Analyze Traces**: Explore trace data with filters and visualizations

**Key metrics to explore**:
- Request latency and throughput
- Service dependencies and call patterns
- Error rates and failure points
- Resource utilization across services

## Monitoring and Management

### Health Checks

Monitor the health of all components:

```bash
# Server health
curl http://localhost:7901/health

# Elasticsearch health
curl http://localhost:9200/_cluster/health

# Agent status
curl http://localhost:7899/status

# Container status
sudo docker ps --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"
```

### Log Analysis

Access logs for troubleshooting:

```bash
# Server logs
sudo docker logs deeptrace_server

# Elasticsearch logs
sudo docker logs elasticsearch

# Agent logs (if available)
sudo docker exec -it deeptrace_server cat /var/log/deeptrace/agent.log
```

### Performance Monitoring

Monitor resource usage:

```bash
# Container resource usage
sudo docker stats

# System resources
htop
iotop
```

## Troubleshooting

### Common Issues

#### Port Conflicts

```bash
# Check port usage
sudo netstat -tuln | grep -E ':(5601|7901|9200|52001)'

# Kill conflicting processes
sudo fuser -k PORT_NUMBER/tcp
```

#### Insufficient Resources

```bash
# Check memory usage
free -h

# Check disk space
df -h

# Clean up Docker resources
sudo docker system prune -a
```

#### Agent Connection Issues

```bash
# Test SSH connectivity
ssh username@localhost

# Check SSH service
sudo systemctl status ssh

# Verify SSH credentials in configuration
```

#### Elasticsearch Issues

```bash
# Check Elasticsearch status
curl http://localhost:9200/_cat/health

# Restart Elasticsearch
sudo docker restart elasticsearch

# Check Elasticsearch logs
sudo docker logs elasticsearch
```

### Performance Optimization

#### Memory Optimization

```bash
# Adjust Elasticsearch memory
# Edit docker-compose.yml or deployment script
ES_JAVA_OPTS="-Xms2g -Xmx2g"
```

#### Storage Optimization

```bash
# Clean old indices
curl -X DELETE "localhost:9200/old-index-*"

# Configure index lifecycle management
curl -X PUT "localhost:9200/_ilm/policy/deeptrace-policy" -H 'Content-Type: application/json' -d'
{
  "policy": {
    "phases": {
      "delete": {
        "min_age": "7d"
      }
    }
  }
}'
```

## Scaling Considerations

### When to Move Beyond All-in-One

Consider distributed deployment when you experience:

- **High resource usage** on the single host
- **Need for high availability** and redundancy
- **Multiple hosts** requiring monitoring
- **Production workloads** with strict SLAs

### Migration Path

To migrate from all-in-one to distributed:

1. **Export configuration** and data
2. **Set up dedicated server cluster**
3. **Deploy agents** on target hosts
4. **Migrate data** and update configurations
5. **Update client connections**

## Cleanup

### Stop All Services

```bash
# Stop DeepTrace components
sudo bash scripts/clear.sh
```

### Manual Cleanup

If the script fails, manually clean up:

```bash
# Stop containers
sudo docker stop $(sudo docker ps -q --filter "name=deeptrace")
sudo docker stop $(sudo docker ps -q --filter "name=elasticsearch")

# Remove containers
sudo docker rm $(sudo docker ps -aq --filter "name=deeptrace")
sudo docker rm $(sudo docker ps -aq --filter "name=elasticsearch")

# Remove volumes
sudo docker volume prune -f

# Remove networks
sudo docker network prune -f
```

## Next Steps

After successful all-in-one deployment:

1. **[Basic Usage Guide](../user-guide/basic-usage.md)**: Learn essential operations
2. **[Trace Analysis](../user-guide/trace-analysis.md)**: Analyze collected traces
3. **[Architecture Overview](../architecture/overview.md)**: Understand DeepTrace internals
4. **[Distributed Deployment](../user-guide/deployment-modes/distributed.md)**: Scale to multiple hosts

## Support

For issues with all-in-one deployment:

- **Check Prerequisites**: Verify system requirements
- **Review Logs**: Examine container and application logs
- **Consult Documentation**: Check specific component guides
- **Community Support**: Visit [GitHub Issues](https://github.com/DeepShield-AI/DeepTrace/issues)

---

The all-in-one deployment provides the fastest way to experience DeepTrace's capabilities. Once familiar with the system, consider distributed deployment for production use cases.
