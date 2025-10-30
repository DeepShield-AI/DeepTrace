# Installation Guide

This guide provides comprehensive installation instructions for DeepTrace. Choose the installation method that best fits your environment and requirements.

## Installation Methods

DeepTrace can be installed using two primary methods:

1. **[Docker Installation](./installation/docker.md)** *(Recommended)*
   - Fastest and most reliable method
   - Pre-built environment with all dependencies
   - Ideal for production deployments

2. **[Manual Compilation](./installation/manual.md)**
   - Build from source code
   - Full control over compilation process
   - Required for custom modifications

## System Requirements

### Minimum Requirements

| Component | Requirement |
|-----------|-------------|
| **Operating System** | Ubuntu 24.04 LTS (or compatible) |
| **Kernel Version** | 4.7.0+ with eBPF support |
| **Memory** | 8GB recommended |
| **Storage** | 40GB free disk space |
| **CPU** | 2 cores minimum, 4+ recommended |
| **Network** | Internet connectivity for downloads |

### Software Dependencies

- **Docker**: v26.1.3 or later
- **Container Runtime**: Docker Engine or compatible
- **Shell**: Bash 4.0+
- **Privileges**: Root or sudo access

### Kernel Requirements

DeepTrace requires specific kernel features:

```bash
# Check kernel version
uname -r

# Verify eBPF support
zgrep CONFIG_BPF /proc/config.gz
zgrep CONFIG_BPF_SYSCALL /proc/config.gz
zgrep CONFIG_BPF_JIT /proc/config.gz
```

All should return `=y` or `=m`.

## Pre-Installation Checklist

Before installing DeepTrace, verify your system meets all requirements:

### 1. System Compatibility
```bash
# Check OS version
lsb_release -a

# Check available disk space
df -h

# Check memory
free -h

# Verify Docker installation
sudo docker --version
```

### 2. Network Configuration
```bash
# Test internet connectivity
ping -c 3 github.com

# Check if required ports are available
netstat -tuln | grep -E ':(5601|7901|9200|52001)'
```

### 3. Permissions
```bash
# Verify sudo access
sudo whoami

# Check Docker permissions
sudo docker ps
```

## Installation Overview

The installation process involves several key steps:

1. **Environment Setup**: Prepare the host system
2. **Repository Clone**: Download DeepTrace source code
3. **Configuration**: Set up configuration files
4. **Server Deployment**: Install server components
5. **Agent Installation**: Deploy monitoring agents
6. **Verification**: Confirm successful installation

## Quick Installation

For users who want to get started immediately:

```bash
# Clone repository
git clone https://github.com/DeepShield-AI/DeepTrace.git
cd DeepTrace

# Quick setup with Docker (recommended)
sudo bash scripts/install_agent.sh
```

This script will:
- Pull necessary Docker images
- Set up basic configuration
- Deploy agent component

## Deployment Modes

DeepTrace supports multiple deployment configurations:

### Single Host (All-in-One)
- Server and agent on the same machine
- Ideal for testing and small deployments
- Simplified configuration and management

### Distributed Deployment
- Server cluster with multiple agents
- Production-ready scalability
- Advanced configuration options

## Post-Installation

After successful installation:

1. **Verify Services**: Ensure all components are running
2. **Access Web Interface**: Connect to the management dashboard
3. **Configure Monitoring**: Set up application monitoring
4. **Test Functionality**: Generate sample traces

## Troubleshooting Installation

Common installation issues and solutions:

### Docker Issues
```bash
# Fix Docker permissions
sudo usermod -aG docker $USER
newgrp docker

# Restart Docker service
sudo systemctl restart docker
```

### Port Conflicts
```bash
# Check port usage
sudo netstat -tuln | grep :PORT_NUMBER

# Kill conflicting processes
sudo fuser -k PORT_NUMBER/tcp
```

### Insufficient Resources
```bash
# Check system resources
htop
df -h
free -h

# Clean up disk space
docker system prune -a
```

## Next Steps

After installation, proceed to:

- **[Configuration Guide](./configuration.md)**: Customize your deployment
- **[Quick Start](./quick-start.md)**: Begin collecting traces
- **[Basic Usage](../user-guide/basic-usage.md)**: Learn essential operations

## Support

If you encounter issues during installation:

- **Check Prerequisites**: Verify all requirements are met
- **Review Logs**: Examine installation logs for errors
- **Consult Documentation**: Check specific installation method guides
- **Community Support**: Visit our [GitHub Issues](https://github.com/DeepShield-AI/DeepTrace/issues)
