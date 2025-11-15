# Quick Start Guide

Get DeepTrace up and running in just 10 minutes! This guide will walk you through the fastest way to deploy DeepTrace and start collecting traces from your applications.

## Prerequisites

Before you begin, ensure you have:

- **Ubuntu 24.04 LTS** (or compatible Linux distribution)
- **Kernel version 4.7.0+** with eBPF support
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

- **To deploy DeepTrace, you must fill in the following fields in the `server/config/config.toml` in order to run it.** These required fields are presented in the configuration file in the format of **xxx**. In all-in-one mode, the `server.ip` and `agents.agent_info.host_ip` values are identical.  

Edit the configuration file and fill in these **required** fields:

| Configuration Item | Description |
| --- | --- |
| `server.ip` | The external IP address of the host running the DeepTrace server and the Elastic database |
| `elastic.elastic_password` | Password for Elastic |
| `agents.agent_info.agent_name` | Name of the agent, which uniquely identifies each agent instance |
| `agents.agent_info.user_name` | The username for logging into the host where the agent is located via SSH |
| `agents.agent_info.host_ip` | IP address of the agent host |
| `agents.agent_info.ssh_port` | SSH port of the agent host (usually 22) |
| `agents.agent_info.host_password` | The password for logging into the host where the agent is located via SSH |

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

## Step 4: Access Elasticsearch Web Interface

Open your browser and navigate to:
```
http://YOUR_SERVER_IP:5601
```

**Login credentials**:
- Username: `elastic`
- Password: `YOUR_ELASTIC_PASSWORD` (from Step 2)

## Step 5: Deploy a Microservice Application

To generate traces, deploy a test microservice application:

**Choose an application**: See [Workload Applications](../user-guide/workloads.md) for detailed deployment instructions.

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

Generate traffic to your deployed microservice application:

**Follow the traffic generation instructions** in [Workload Applications](../user-guide/workloads.md) for your chosen application.

## Step 8: Build and View Traces

Correlate spans and assemble traces:

```bash
# Perform span correlation using DeepTrace algorithm
sudo docker exec -it deeptrace_server python -m cli.src.cmd asso algo deeptrace

# Assemble traces from correlated spans
sudo docker exec -it deeptrace_server python -m cli.src.cmd assemble
```

## Step 9: Explore Your Traces

1. **Elasticsearch Web Interface**: Visit `http://YOUR_SERVER_IP:5601`
2. **Navigate to Discover**: Click on "Discover" in the left sidebar
3. **Select Index**: Choose the trace index pattern
4. **View Traces**: Explore collected traces with rich metadata

## Verification Checklist

✅ **Server Running**: `sudo docker ps | grep deeptrace_server`  
✅ **Agent Connected**: Check agent status in web interface  
✅ **Traces Collected**: Verify traces appear in Elasticsearch  
✅ **Elasticsearch Web Interface Accessible**: Can login and view data  

## Clean Up

To remove DeepTrace and all components:

```bash
sudo bash scripts/clear.sh
```

This will stop and remove all containers, networks, and temporary files.

## Next Steps

Congratulations! You now have DeepTrace running and collecting traces. Here's what to explore next:

- **[Configuration Guide](./configuration.md)**: Customize DeepTrace for your environment
- **[Basic Usage](../user-guide/basic-usage.md)**: Learn essential operations
- **[Architecture Overview](../architecture/overview.md)**: Understand how DeepTrace works
- **[Troubleshooting](../troubleshooting/common-issues.md)**: Resolve common issues

## Need Help?

- **Common Issues**: Check our [troubleshooting guide](../troubleshooting/common-issues.md)
- **GitHub Issues**: [Report bugs or ask questions](https://github.com/DeepShield-AI/DeepTrace/issues)
- **Documentation**: Explore the full [documentation](../README.md)
