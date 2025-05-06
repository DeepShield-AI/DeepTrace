# DeepTrace Agent Compilation Guide

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

---

### Step 5: Generate Kernel Bindings
```bash
mkdir -p agent/src/ebpf/src
aya-tool generate task_struct user_msghdr mmsghdr tcp_sock socket files_struct > agent/src/ebpf/trace/src/vmlinux.rs

# Allow non-standard naming in generated code
sed -i '2i\#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unnecessary_transmutes)]' agent/src/ebpf/trace/src/vmlinux.rs

# Build the project
cargo build --release  # Compile with optimizations 
```

### Step 6: Load the Workload Images
Import our pre-built Docker image (memcached, redis, and mongo):
```bash
cd DeepTrace/
chmod +x ./scripts/load_docker_images.sh
./scripts/load_docker_images.sh
```
---

### Step 7: Output

The compiled agent will be located at `target/release/deeptrace`. You can run it with:
```bash
RUST_LOG=info cargo run --release \
  --config 'target."cfg(all())".runner="sudo -E"' \
  -- --pids <pid>
```

For more testing, check the [Testing Guide](../tests/README.md) for more details.

### References
- [Ubuntu 24.04 Installation Guide](https://ubuntu.com/download/desktop)   
- [bpftool Installation from Source](https://99rdp.com/mastering-ebpf-how-to-install-bpftool-in-linux)   
- [Aya eBPF Framework Documentation](https://github.com/aya-rs/aya)   
- [Rust BPF Toolchain Setup](https://github.com/aya-rs/bpf-linker)   

## Use Docker

The easiest way is to use our docker image:

### 1. Docker Installation by OS

You can install Docker by following the official instructions: [Docker Installation](https://docs.docker.com/get-started/get-docker/)

Check if Docker is installed correctly:
```bash
docker --version
```

### 2. Git lfs

Git LFS is required to handle large files in Git repositories. You can install it by following the official instructions: [Git LFS Installation](https://git-lfs.com/)

Then you can clone the repository:
```bash
git lfs install
git lfs clone https://github.com/DeepShield-AI/DeepTrace.git
```

---

### 3. Image Import Operations

Once Docker is installed, you can import our pre-built Docker image:
```bash
cd DeepTrace/
chmod +x ./scripts/load_docker_images.sh
./scripts/load_docker_images.sh

# Check
docker images | grep deeptrace
```
If you load the images, you don't need to compile the image again.

---  

### 4. Build & Deployment (Optional)

#### 1. **Compile Agent**
```bash
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