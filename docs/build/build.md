# DeepTrace Agent Compilation Guide

## Use Docker

The easiest way is to use our docker image:

### Ⅰ. Docker Installation by OS

#### 1. **Windows Installation** 
**Prerequisites**:
- Windows 10/11 64-bit (Build 19045+)
- WSL 2 enabled (`wsl --install`)
- Hyper-V & Virtualization enabled in BIOS

**Steps**:
```bash
# 1. Install WSL 2
wsl --install -d Ubuntu

# 2. Download Docker Desktop
https://desktop.docker.com/win/main/amd64/Docker%20Desktop%20Installer.exe

# 3. Configure WSL integration
Docker Desktop → Settings → Resources → WSL Integration → Enable Ubuntu

# 4. Verify installation
docker --version
```

#### 2. **macOS Installation** 
**Requirements**:
- macOS 10.15+ (Catalina+) for Intel
- macOS 11+ (Big Sur+) for Apple Silicon

**Methods**:
```bash
# Option 1: Homebrew installation
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install --cask docker

# Option 2: Manual installation
# Download from https://docs.docker.com/desktop/install/mac-install/

# Verify with:
docker --version
```

#### 3. **Linux Installation** 
**For Ubuntu/Debian**:
```bash
# 1. Uninstall old versions
sudo apt-get remove docker docker-engine docker.io containerd runc

# 2. Set up repository
sudo apt-get update
sudo apt-get install ca-certificates curl gnupg
sudo install -m 0755 -d /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
echo "deb [arch=$(dpkg --print-architecture)] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

# 3. Install Docker
sudo apt-get update
sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

# 4. Verify
docker --version
```

**For CentOS/RHEL**:
```bash
# 1. Remove legacy
sudo yum remove docker docker-client docker-client-latest docker-common docker-latest docker-latest-logrotate docker-logrotate docker-engine

# 2. Add repo
sudo yum install -y yum-utils
sudo yum-config-manager --add-repo https://download.docker.com/linux/centos/docker-ce.repo

# 3. Install
sudo yum install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
sudo systemctl start docker
```

---

### Ⅱ. Image Import Operations (Optional)

#### 1. **Host Architecture Detection**  
```bash  
# Linux/Unix  
uname -m  # Output: x86_64 (AMD64), aarch64 (ARM64), armv7l (ARMv7), etc.  

# Windows (PowerShell)  
[System.Environment]::GetEnvironmentVariable("PROCESSOR_ARCHITECTURE")  
```  

---  

#### 2. Single-Architecture Image Import  

First, assuming you have already cloned the repository and are currently in the `DeepTrace` directory:
```bash
# Navigate to the image directory
cd deployment/docker/images
```

**x86_64/AMD64**  
```bash
cd x86_64
# Import local .tar image  
docker load -i memcached.tar
docker load -i redis.tar
docker load -i mongo.tar
docker load -i ubuntu.tar
```  

**ARM64**  
```bash
cd aarch64
# Import local image  
docker load -i memcached.tar
docker load -i redis.tar
docker load -i mongo.tar
docker load -i ubuntu.tar
```  

---  

### Ⅲ. Build & Deployment

#### 1. **Compile Agent**
```bash
git clone https://github.com/DeepShield-AI/DeepTrace.git
cd DeepTrace

docker build \
  --build-arg APP_NAME=deeptrace \
  --network=host \
  -t deeptrace \
  -f deployment/docker/Dockerfile .
```

_Note: The Docker image build process can be time-consuming. Please be patient during this period and ensure stable network connectivity to both github.com and Docker Hub (hub.docker.com), especially if relying on remote image repositories rather than local cached images._

#### 2. **Check Image**
```bash
docker images | grep deeptrace
# Output:
deeptrace            latest      04ed8cf89494   now    5.89GB
```
The output of `docker images | grep deeptrace` should show the image you just built. If it doesn't, you may need to check your Docker build process or network connectivity.

## Manually compilation

TODO