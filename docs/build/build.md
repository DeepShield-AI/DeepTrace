# DeepTrace Agent Compilation Guide

## Use Docker

The easiest way is to use our docker image:

### 1. Docker Installation by OS

You can install Docker by following the official instructions: [Docker Installation](https://docs.docker.com/get-started/get-docker/)

Check if Docker is installed correctly:
```bash
docker --version
```

---

### 2. Docker Configuration

### Edit Docker Daemon Configuration
Modify `/etc/docker/daemon.json`:
```json
{
  "insecure-registries": ["47.97.67.233:5000"]
}
```
> Required for HTTP connections to private registry (https://medium.com/@dataq/membuat-docker-private-registry-6efe534df4d5) (https://gist.github.com/paulgwebster-oe/b3eab23c5c369d659bf0f66f124f2715)

### Restart Docker Service
```bash
sudo systemctl daemon-reload
sudo systemctl restart docker
```

---

### 3. Pull Images from Private Registry
```bash
docker pull 47.97.67.233:5000/deepshield/deeptrace:latest
```
> Verify with: `docker images | grep deeptrace`

---

### 4. Compile Agent

```bash
cd DeepTrace

docker run --privileged --rm -it -v $(pwd):/DeepTrace 47.97.67.233:5000/deepshield/deeptrace bash -c \
'cd /DeepTrace/agent &&
aya-tool generate task_struct user_msghdr mmsghdr tcp_sock socket files_struct > src/trace/ebpf/src/vmlinux.rs &&
sed -i '"'"'2i\#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unnecessary_transmutes)]'"'"' src/trace/ebpf/src/vmlinux.rs &&
cargo build --release'

# binary file directory: ./agent/target/release/agent
# You can also run the agent with:
cp agent/config/default.toml.example agent/config/default.toml
# modify the config file
RUST_LOG=info sudo ./agent/target/release/agent -f agent/config/default.toml
```

## Manually compilation

### Prerequisites
#### 1. Set Up Ubuntu 24.04 LTS
- **Download ISO**:  
  Visit [Ubuntu 24.04 LTS Desktop](https://ubuntu.com/download/desktop)  to download the ISO image.  
- **Create a VM**:  
  Use **VMware**, **Parallels**, or **Multipass** to create a Ubuntu 24.04 VM. Allocate at least **20GB disk space** and **4GB RAM**.  

---

### Step 1: Install Base Dependencies
```bash
# Update packages and install essential tools
sudo apt-get update && sudo apt-get install -y --no-install-suggests --no-install-recommends \
  build-essential clang llvm-18 llvm-18-dev llvm-18-tools \
  curl ca-certificates git make libelf-dev

# Set LLVM environment variables (persist in ~/.bashrc if needed)

echo "export LLVM_PATH=/lib/llvm-18" >> ~/.bashrc
echo "export PATH=$PATH:/lib/llvm-18/bin" >> ~/.bashrc
source ~/.bashrc
```

---

### Step 2: Build and Install `bpftool`
```bash
git clone --recurse-submodules https://github.com/libbpf/bpftool.git
cd bpftool/src
make -j$(nproc) && sudo make install  # Build with parallelism 
cd ../../ && rm -rf bpftool  # Cleanup
# Alternatively, you can use the package manager
sudo apt-get install bpftool
#  choose 6.5.0-45.45.1~22.04.1 linux-hwe-6.5-tools-common
sudo apt-get install linux-hwe-6.5-tools-common
# Verify installation
bpftool version  # Should display version info 

# Mount the tracefs filesystem
sudo mkdir -p /sys/kernel/tracing
sudo mount -t tracefs nodev /sys/kernel/tracing
```

---

### Step 3: Set Up Rust and BPF Toolchain
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain=stable

echo "export PATH=$PATH:$HOME/.cargo/bin" >> ~/.bashrc
source ~/.bashrc

# Add components and toolchains
rustup component add rust-src
rustup toolchain install nightly --component rust-src

# Install BPF-specific tools
cargo install --features=llvm-sys/prefer-dynamic bpf-linker
cargo install bindgen-cli  # Generate Rust bindings for C code 
cargo install --git https://github.com/aya-rs/aya -- aya-tool
```

---

### Step 4: Clone the Repository
```bash
git clone https://github.com/DeepShield-AI/DeepTrace.git
cd DeepTrace
```
#
---

### Step 5: Generate Kernel Bindings
```bash
mkdir -p agent/src/trace/ebpf/src
aya-tool generate task_struct user_msghdr mmsghdr tcp_sock socket files_struct > agent/src/trace/ebpf/src/vmlinux.rs

# Allow non-standard naming in generated code
sed -i '2i\#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unnecessary_transmutes)]' agent/src/trace/ebpf/src/vmlinux.rs

# Build the project
cargo build --release  # Compile with optimizations 
```

#

### References
- [Ubuntu 24.04 Installation Guide](https://ubuntu.com/download/desktop)   
- [bpftool Installation from Source](https://99rdp.com/mastering-ebpf-how-to-install-bpftool-in-linux)   
- [Aya eBPF Framework Documentation](https://github.com/aya-rs/aya)   
- [Rust BPF Toolchain Setup](https://github.com/aya-rs/bpf-linker)   
Protocol: Redis, Count: 4032
Protocol: Memcached, Count: 3990
Syscall: Read, Count: 4012
Syscall: Write, Count: 2016
Syscall: SendMsg, Count: 1994