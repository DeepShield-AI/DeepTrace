# Docker Installation

The Docker installation method is the **recommended approach** for deploying DeepTrace. It provides a pre-configured environment with all dependencies, ensuring consistent and reliable deployments across different systems.

## Prerequisites

### System Requirements
- **Ubuntu 24.04 LTS** (or compatible Linux distribution)
- **Kernel 6.8.0+** with eBPF support
- **40GB+ free disk space**
- **4GB+ RAM** (8GB recommended)
- **Internet connectivity**

### Docker Installation

If Docker is not already installed, follow these steps:

#### 1. Install Docker Engine

```bash
# Update package index
sudo apt-get update

# Install required packages
sudo apt-get install -y \
    ca-certificates \
    curl \
    gnupg \
    lsb-release

# Add Docker's official GPG key
sudo mkdir -p /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg

# Set up Docker repository
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

# Install Docker Engine
sudo apt-get update
sudo apt-get install -y docker-ce docker-ce-cli containerd.io docker-compose-plugin
```

#### 2. Verify Docker Installation

```bash
# Check Docker version
sudo docker --version

# Test Docker installation
sudo docker run hello-world
```

#### 3. Configure Docker (Optional)

Add your user to the Docker group to run Docker without sudo:

```bash
sudo usermod -aG docker $USER
newgrp docker

# Test without sudo
docker --version
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
docker images | grep deeptrace
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
  bash -c 'cd /DeepTrace && cargo xtask build --profile release'
```

This command will:
- Mount your local DeepTrace directory into the container
- Compile the agent with release optimizations
- Generate the binary at `target/x86_64-unknown-linux-gnu/release/deeptrace`

### Step 5: Configure DeepTrace

```bash
# Copy example configuration
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

## Docker Compose Deployment

For easier management, you can use Docker Compose to deploy DeepTrace components:

### Create docker-compose.yml

```yaml
version: '3.8'

services:
  elasticsearch:
    image: docker.elastic.co/elasticsearch/elasticsearch:8.11.0
    container_name: deeptrace_elasticsearch
    environment:
      - discovery.type=single-node
      - xpack.security.enabled=false
      - "ES_JAVA_OPTS=-Xms2g -Xmx2g"
    ports:
      - "9200:9200"
    volumes:
      - elasticsearch_data:/usr/share/elasticsearch/data
    networks:
      - deeptrace

  kibana:
    image: docker.elastic.co/kibana/kibana:8.11.0
    container_name: deeptrace_kibana
    environment:
      - ELASTICSEARCH_HOSTS=http://elasticsearch:9200
    ports:
      - "5601:5601"
    depends_on:
      - elasticsearch
    networks:
      - deeptrace

  deeptrace-server:
    image: 47.97.67.233:5000/deepshield/deeptrace:latest
    container_name: deeptrace_server
    ports:
      - "7901:7901"
      - "52001:52001"
    volumes:
      - ./config:/app/config
      - ./logs:/app/logs
    depends_on:
      - elasticsearch
    networks:
      - deeptrace
    command: ["python", "-m", "server.main"]

volumes:
  elasticsearch_data:

networks:
  deeptrace:
    driver: bridge
```

### Deploy with Docker Compose

```bash
# Start all services
docker-compose up -d

# Check service status
docker-compose ps

# View logs
docker-compose logs -f deeptrace-server
```

## Verification

### 1. Check Running Containers

```bash
docker ps
```

You should see containers for:
- `deeptrace_server`
- `deeptrace_elasticsearch`
- `deeptrace_kibana`

### 2. Test Web Interface

Open your browser and navigate to:
- **Kibana**: `http://localhost:5601`
- **DeepTrace API**: `http://localhost:7901/health`

### 3. Verify Agent Compilation

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
  bash -c 'cd /DeepTrace && RUST_LOG=debug cargo xtask build --profile release'
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
docker system prune -a --volumes

# Remove unused images
docker image prune -a
```

## Next Steps

After successful Docker installation:

1. **[Configuration](../configuration.md)**: Customize your deployment
2. **[All-in-One Deployment](../all-in-one.md)**: Quick setup for testing
3. **[Basic Usage](../../user-guide/basic-usage.md)**: Start using DeepTrace

## Alternative: Manual Compilation

If you prefer to compile from source without Docker, see the [Manual Compilation Guide](./manual.md).

---

The Docker installation provides the fastest and most reliable way to get DeepTrace running. For production deployments, consider using the Docker Compose configuration for better service management.
