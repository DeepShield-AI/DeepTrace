# Manual Compilation

This guide walks you through compiling DeepTrace from source code. Manual compilation gives you full control over the build process and is required for custom modifications or when Docker is not available.

## When to Use Manual Compilation

Choose manual compilation when you need to:
- **Customize the build process** or modify source code
- **Work in environments** where Docker is not available
- **Understand the dependencies** and build process in detail
- **Optimize for specific hardware** or kernel configurations

> 💡 **Tip**: For most users, the [Docker installation](./docker.md) method is faster and more reliable.

## Prerequisites

### System Requirements

- **Ubuntu 24.04 LTS** (recommended) or compatible Linux distribution
- **Kernel 4.7.0+** with eBPF support and BTF information
- **20GB+ free disk space**
- **8GB+ RAM**
- **Internet connectivity** for downloading dependencies

### Required Packages

The following packages must be installed before compilation:

```bash
# Update package lists
sudo apt-get update

# Install essential build tools
sudo apt-get install -y --no-install-suggests --no-install-recommends \
  build-essential \
  clang \
  llvm-18 \
  llvm-18-dev \
  llvm-18-tools \
  curl \
  ca-certificates \
  git \
  make \
  libelf-dev \
  libclang-18-dev \
  pkg-config \
  libssl-dev \
  openssl
```

## Step-by-Step Installation

### Step 1: Set Up Environment Variables

Configure LLVM environment variables for the build process:

```bash
# Set LLVM paths
export LLVM_PATH=/lib/llvm-18
export PATH=$PATH:/lib/llvm-18/bin

# Make changes persistent
echo "export LLVM_PATH=/lib/llvm-18" >> ~/.bashrc
echo "export PATH=\$PATH:/lib/llvm-18/bin" >> ~/.bashrc
source ~/.bashrc

# Verify LLVM installation
llvm-config-18 --version
clang-18 --version
```

### Step 2: Build and Install libbpf

DeepTrace requires `libbpf` for eBPF functionality:

```bash
# Clone libbpf repository
git clone https://github.com/libbpf/libbpf.git --branch libbpf-1.6.2 --depth 1
cd libbpf/src

# Build with static linking only
BUILD_STATIC_ONLY=y make -j$(nproc)

# Install system-wide
sudo make install

# Update library cache
sudo ldconfig

# Verify installation
pkg-config --modversion libbpf
```

### Step 3: Install Rust Toolchain

DeepTrace is written in Rust and requires specific toolchain components:

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain=stable

# Add Rust to PATH
echo "export PATH=\$PATH:\$HOME/.cargo/bin" >> ~/.bashrc
source ~/.bashrc

# Verify Rust installation
rustc --version
cargo --version
```

### Step 4: Configure Rust for eBPF Development

Install additional Rust components needed for eBPF compilation:

```bash
# Add Rust source code (required for eBPF)
rustup component add rust-src

# Install nightly toolchain with rust-src
rustup toolchain install nightly --component rust-src

# Add target for cross-compilation (if needed)
rustup target add aarch64-unknown-linux-gnu

# Install BPF linker
cargo install bpf-linker

# Verify BPF linker installation
bpf-linker --version
```

### Step 5: Clone DeepTrace Repository

```bash
# Clone the repository
git clone https://github.com/DeepShield-AI/DeepTrace.git
cd DeepTrace

# Check repository structure
ls -la
```

### Step 6: Compile DeepTrace

Now compile the DeepTrace agent with optimizations:

```bash
cd agent

# Compile with release profile for optimal performance
cargo xtask build --profile release

# The compilation process will:
# 1. Build eBPF programs
# 2. Compile Rust userspace components
# 3. Link everything together
```

**Expected compilation time**: 10-30 minutes depending on your hardware.

### Step 7: Verify Compilation

Check that the compilation was successful:

```bash
# Verify agent binary exists
ls -la target/x86_64-unknown-linux-gnu/release/deeptrace

# Check binary size and permissions
file target/x86_64-unknown-linux-gnu/release/deeptrace

# Test help output
./target/x86_64-unknown-linux-gnu/release/deeptrace --help
```

### Step 8: Set Up Configuration

```bash
# Copy example configuration
cp config/deeptrace.toml.example config/deeptrace.toml

# Edit configuration as needed
nano config/deeptrace.toml
```

### Step 9: Test the Agent

Run a basic test to ensure the agent works correctly:

```bash
# Test with info logging
RUST_LOG=info cargo xtask run -c config/deeptrace.toml

# Or run the binary directly
sudo RUST_LOG=info ./target/x86_64-unknown-linux-gnu/release/deeptrace -c config/deeptrace.toml
```

## Advanced Build Options

### Debug Build

For development and debugging:

```bash
# Build with debug symbols
cargo xtask build --profile debug

# Run with debug logging
RUST_LOG=debug cargo xtask run -c config/deeptrace.toml
```

### Custom Features

Enable or disable specific features:

```bash
# Build with specific features
# todo: feature is currently not supported
cargo xtask build --profile release --features "feature1,feature2"

# Build without default features
cargo xtask build --profile release --no-default-features
```

### Cross-Compilation

For different architectures:

```bash
# Add target architecture
rustup target add aarch64-unknown-linux-gnu

# Cross-compile for ARM64
cargo xtask build --profile release --target aarch64-unknown-linux-gnu
```

## Troubleshooting Compilation Issues

### Common Build Errors

#### LLVM/Clang Issues
```bash
# Verify LLVM installation
which clang-18
llvm-config-18 --version

# Reinstall if necessary
sudo apt-get install --reinstall llvm-18 clang-18
```

#### libbpf Linking Errors
```bash
# Check libbpf installation
pkg-config --libs libbpf

# Rebuild libbpf if necessary
cd libbpf/src
make clean
BUILD_STATIC_ONLY=y make -j$(nproc)
sudo make install
sudo ldconfig
```

#### Rust Compilation Errors
```bash
# Update Rust toolchain
rustup update

# Clean build cache
cargo clean

# Rebuild with verbose output
cargo xtask build --profile release -- --verbose
```

#### eBPF Compilation Errors
```bash
# Check kernel headers
ls /usr/src/linux-headers-$(uname -r)/

# Install kernel headers if missing
sudo apt-get install linux-headers-$(uname -r)

# Verify BTF support
ls /sys/kernel/btf/vmlinux
```

### Memory Issues During Compilation

If compilation fails due to insufficient memory:

```bash
# Check available memory
free -h

# Reduce parallel jobs
cargo xtask build --profile release -j 1

# Or increase swap space
sudo fallocate -l 4G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
```

### Disk Space Issues

```bash
# Check available space
df -h

# Clean Rust cache
cargo clean

# Remove target directory
rm -rf target/

# Clean package cache
sudo apt-get clean
```

## Development Setup

For ongoing development work:

```bash
# Install development tools
cargo install cargo-watch cargo-expand

# Set up pre-commit hooks
git config core.hooksPath .githooks

# Run tests
cargo test

# Format code
cargo +nightly fmt

# Run linter
cargo xtask clippy
```

## Next Steps

After successful manual compilation:

1. **[Configuration Guide](../configuration.md)**: Set up your deployment
2. **[Testing Guide](../../testing/overview.md)**: Verify your build
3. **[Development Setup](../../testing/development.md)**: Set up for development

## References

- **[Ubuntu Installation Guide](https://ubuntu.com/download/desktop)**
- **[LLVM Documentation](https://llvm.org/docs/)**
- **[libbpf GitHub Repository](https://github.com/libbpf/libbpf)**
- **[Rust Installation Guide](https://rustup.rs/)**
- **[Aya eBPF Framework](https://github.com/aya-rs/aya)**
- **[BPF Linker Documentation](https://github.com/aya-rs/bpf-linker)**