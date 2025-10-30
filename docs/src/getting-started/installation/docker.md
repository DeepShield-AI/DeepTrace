# Docker Installation

The Docker installation method is the **recommended approach** for deploying DeepTrace. It provides a pre-configured environment with all dependencies, ensuring consistent and reliable deployments across different systems.

## Prerequisites

### System Requirements
- **Ubuntu 24.04 LTS** (or compatible Linux distribution)
- **Kernel 4.7.0+** with eBPF support
- **40GB+ free disk space**
- **8GB+ RAM**
- **Internet connectivity**

### Docker Installation

If Docker is not already installed, you can install Docker by following the official instructions: [Docker Installation](https://docs.docker.com/get-started/get-docker/)


#### 1. Verify Docker Installation

```bash
# Check Docker version
sudo docker --version

# Test Docker installation
sudo docker run hello-world
```

## DeepTrace Docker Installation

### Step 1: Clone Repository

```bash
git clone https://github.com/DeepShield-AI/DeepTrace.git
cd DeepTrace
```

> ⚠️ **Important**: Do not clone into `/etc` directory as the agent will use this path during deployment.

### Step 2: Configure Docker Registry

DeepTrace uses a private Docker registry for pre-built images. Configure Docker to access it:

#### Edit Docker Daemon Configuration

```bash
sudo nano /etc/docker/daemon.json
```

Add the following configuration:

```json
{
  "insecure-registries": ["47.97.67.233:5000"]
}
```

> **Note**: This configuration allows HTTP connections to the private registry.

#### Restart Docker Service

```bash
sudo systemctl daemon-reload
sudo systemctl restart docker
```

### Step 3: Pull DeepTrace Images

```bash
# Pull the main DeepTrace image
sudo docker pull 47.97.67.233:5000/deepshield/deeptrace:latest

# Verify image download
sudo docker images | grep deeptrace
```

### Step 4: Compile Agent

Use the Docker container to compile the DeepTrace agent:

```bash
# Navigate to DeepTrace directory
cd DeepTrace

# Compile using Docker container
sudo docker run --privileged --rm -it \
  -v $(pwd):/DeepTrace \
  47.97.67.233:5000/deepshield/deeptrace:latest \
  bash -c 'cd /DeepTrace/agent && cargo xtask build --profile release'
```

This command will:
- Mount your local DeepTrace directory into the container
- Compile the agent with release optimizations
- Generate the binary at `agent/target/x86_64-unknown-linux-gnu/release/deeptrace`

### Step 5: Configure DeepTrace

```bash
# Copy example configuration
cd agent
cp config/deeptrace.toml.example config/deeptrace.toml

# Edit configuration file
nano config/deeptrace.toml
```

Update the configuration with your specific settings. See the [Configuration Guide](../configuration.md) for detailed options.

### Step 6: Test Agent

```bash
# Test the compiled agent
sudo RUST_LOG=info ./target/x86_64-unknown-linux-gnu/release/deeptrace -c config/deeptrace.toml
```

## Verification

### 1. Verify Agent Compilation

```bash
# Check if agent binary exists
ls -la target/x86_64-unknown-linux-gnu/release/deeptrace

# Test agent help
./target/x86_64-unknown-linux-gnu/release/deeptrace --help
```

## Troubleshooting

### Common Docker Issues

#### Permission Denied
```bash
# Add user to docker group
sudo usermod -aG docker $USER
newgrp docker
```

#### Port Already in Use
```bash
# Check what's using the port
sudo netstat -tuln | grep :5601

# Kill the process
sudo fuser -k 5601/tcp
```

#### Image Pull Failures
```bash
# Check Docker daemon configuration
sudo systemctl status docker

# Restart Docker
sudo systemctl restart docker

# Try pulling again
docker pull 47.97.67.233:5000/deepshield/deeptrace:latest
```

#### Compilation Errors
```bash
# Check available disk space
df -h

# Clean Docker cache
docker system prune -a

# Retry compilation with verbose output
sudo docker run --privileged --rm -it \
  -v $(pwd):/DeepTrace \
  47.97.67.233:5000/deepshield/deeptrace:latest \
  bash -c 'cd /DeepTrace/agent && RUST_LOG=debug cargo xtask build --profile release'
```

### Resource Issues

#### Insufficient Memory
```bash
# Check memory usage
free -h

# Increase Docker memory limit (if using Docker Desktop)
# Go to Docker Desktop Settings > Resources > Memory
```

#### Disk Space
```bash
# Clean up Docker resources
sudo docker system prune -a --volumes

# Remove unused images
sudo docker image prune -a
```

## Next Steps

After successful Docker installation:

1. **[Configuration](../configuration.md)**: Customize your deployment
2. **[All-in-One Deployment](../all-in-one.md)**: Quick setup for testing
3. **[Basic Usage](../../user-guide/basic-usage.md)**: Start using DeepTrace

## Alternative: Manual Compilation

If you prefer to compile from source without Docker, see the [Manual Compilation Guide](./manual.md).
