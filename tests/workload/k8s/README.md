# Kubernetes Installation Guide (Sealos)

This document provides step-by-step instructions for installing a Kubernetes cluster using [Sealos](https://github.com/labring/sealos).

## Prerequisites

1. **Passwordless SSH** is configured between all nodes (master and workers).
2. **Docker is uninstalled** on all nodes:
   ```bash
   sudo apt-get autoremove docker docker-ce docker-engine docker.io docker-ce-*
   ```

## Installation Steps

### 1. Download and Install Sealos

Download the Sealos binary and install it to `/usr/bin`:

```bash
wget https://github.com/labring/sealos/releases/download/v4.3.7/sealos_4.3.7_linux_amd64.tar.gz

# Extract the binary
 tar zxvf sealos_4.3.7_linux_amd64.tar.gz sealos

# Make it executable
chmod +x sealos

# Move to /usr/bin
sudo mv sealos /usr/bin
```

### 2. Deploy Kubernetes Cluster

Replace the IP addresses and credentials as needed:

```bash
sudo sealos run \
  registry.cn-shanghai.aliyuncs.com/labring/kubernetes:v1.27.7 \
  registry.cn-shanghai.aliyuncs.com/labring/helm:v3.9.4 \
  registry.cn-shanghai.aliyuncs.com/labring/calico:v3.24.1 \
  --masters <master_ip> \
  --nodes <node1_ip>,<node2_ip> \
  -p <ssh_password> \
  --user <ssh_user>
```

- `<master_ip>`: The IP address of your master node.
- `<node1_ip>,<node2_ip>`: Comma-separated IPs of worker nodes (optional if single node).
- `<ssh_password>`: SSH password for the user.
- `<ssh_user>`: Username for SSH (e.g., `ubuntu`).

### Example

```bash
sudo sealos run \
  registry.cn-shanghai.aliyuncs.com/labring/kubernetes:v1.27.7 \
  registry.cn-shanghai.aliyuncs.com/labring/helm:v3.9.4 \
  registry.cn-shanghai.aliyuncs.com/labring/calico:v3.24.1 \
  --masters 10.10.10.114 \
  --nodes 10.10.10.115,10.10.10.116 \
  -p netsys204 \
  --user ubuntu
```

## References
- [Sealos Documentation](https://github.com/labring/sealos)
- [Kubernetes Official Site](https://kubernetes.io/)