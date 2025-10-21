# Quick Start Guide

Get DeepTrace up and running in just 10 minutes! This guide will walk you through the fastest way to deploy DeepTrace and start collecting traces from your applications.

## Prerequisites

Before you begin, ensure you have:

- **Ubuntu 24.04 LTS** (or compatible Linux distribution)
- **Kernel version 6.8.0+** with eBPF support
- **Docker 26.1.3+** installed and running
- **40GB+ free disk space**
- **Root/sudo access**
- **Internet connectivity**

## Step 1: Clone the Repository

```bash
git clone https://github.com/DeepShield-AI/DeepTrace.git
cd DeepTrace
```

> ⚠️ **Important**: Do not clone into `/etc` directory as the agent will use this path later.

## Step 2: Quick Configuration

Copy the example configuration and update the required fields:

```bash
cp server/config/config.toml.example server/config/config.toml
```

Edit the configuration file and fill in these **required** fields:

```toml
[server]
ip = "YOUR_SERVER_IP"  # External IP of your host

[elastic]
elastic_password = "YOUR_ELASTIC_PASSWORD"  # Choose a secure password

[[agents.agent_info]]
agent_name = "agent-1"
user_name = "YOUR_USERNAME"  # SSH username for the host
host_ip = "YOUR_HOST_IP"     # Same as server.ip for all-in-one mode
ssh_port = 22
host_password = "YOUR_SSH_PASSWORD"  # SSH password
```

## Step 3: Deploy DeepTrace Server

Launch the DeepTrace server and Elasticsearch database:

```bash
sudo bash scripts/deploy_server.sh
```

This command will:
- Pull required Docker images
- Start Elasticsearch database
- Launch DeepTrace server
- Set up the web interface

**Verify deployment**:
```bash
sudo docker ps | grep deeptrace
```

You should see containers running for `deeptrace_server` and `elasticsearch`.

## Step 4: Access Web Interface

Open your browser and navigate to:
```
http://YOUR_SERVER_IP:5601
```

**Login credentials**:
- Username: `elastic`
- Password: `YOUR_ELASTIC_PASSWORD` (from Step 2)

## Step 5: Deploy Sample Application (Optional)

For testing purposes, deploy the Social Network microservice application:

```bash
sudo docker exec -it deeptrace_server python -m cli.src.cmd agent install_app
```

This will set up a complete microservices environment for trace collection.

## Step 6: Install and Start Agent

Install the DeepTrace agent on your host:

```bash
# Install agent (compiles from source)
sudo docker exec -it deeptrace_server python -m cli.src.cmd agent install

# Start collecting traces
sudo docker exec -it deeptrace_server python -m cli.src.cmd agent run
```

The agent will automatically:
- Compile eBPF programs
- Start monitoring all Docker containers
- Begin collecting network traces
- Send data to the server

## Step 7: Generate Sample Traffic

If you deployed the sample application, generate some traffic:

```bash
# Find the workload generator container
CONTAINER_ID=$(sudo docker ps | grep wrk2 | awk '{print $1}')

# Enter the container and run load test
sudo docker exec -it $CONTAINER_ID /bin/bash
cd /root
./wrk -D exp -t 6 -c 6 -d 30 -L -s ./wrk2/scripts/social-network/compose-post.lua http://nginx-web-server:8080/wrk2-api/post/compose -R 50
```

## Step 8: Build and View Traces

Correlate spans and assemble traces:

```bash
# Perform span correlation using DeepTrace algorithm
sudo docker exec -it deeptrace_server python -m cli.src.cmd asso algo deeptrace

# Assemble traces from correlated spans
sudo docker exec -it deeptrace_server python -m cli.src.cmd assemble
```

## Step 9: Explore Your Traces

1. **Web Interface**: Visit `http://YOUR_SERVER_IP:5601`
2. **Navigate to Discover**: Click on "Discover" in the left sidebar
3. **Select Index**: Choose the trace index pattern
4. **View Traces**: Explore collected traces with rich metadata

## Verification Checklist

✅ **Server Running**: `sudo docker ps | grep deeptrace_server`  
✅ **Agent Connected**: Check agent status in web interface  
✅ **Traces Collected**: Verify traces appear in Elasticsearch  
✅ **Web Interface Accessible**: Can login and view data  

## Next Steps

Congratulations! You now have DeepTrace running and collecting traces. Here's what to explore next:

- **[Configuration Guide](./configuration.md)**: Customize DeepTrace for your environment
- **[Basic Usage](../user-guide/basic-usage.md)**: Learn essential operations
- **[Architecture Overview](../architecture/overview.md)**: Understand how DeepTrace works
- **[Troubleshooting](../troubleshooting/common-issues.md)**: Resolve common issues

## Clean Up

To remove DeepTrace and all components:

```bash
sudo bash scripts/clear.sh
```

This will stop and remove all containers, networks, and temporary files.

## Need Help?

- **Common Issues**: Check our [troubleshooting guide](../troubleshooting/common-issues.md)
- **GitHub Issues**: [Report bugs or ask questions](https://github.com/DeepShield-AI/DeepTrace/issues)
- **Documentation**: Explore the full [documentation](../README.md)

---

**Estimated Time**: 10-15 minutes  
**Difficulty**: Beginner  
**Prerequisites**: Basic Docker and Linux knowledge
