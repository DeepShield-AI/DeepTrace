#!/bin/bash
set -euo pipefail

# Update packages and install essential tools
apt-get update && apt-get install -y --no-install-suggests --no-install-recommends \
  build-essential clang llvm-14 llvm-14-dev llvm-14-tools \
  curl ca-certificates git make libelf-dev

# Set LLVM environment variables (persist in ~/.bashrc if needed)
echo "export LLVM_PATH=/lib/llvm-14" >> ~/.bashrc
echo "export PATH=$PATH:/lib/llvm-14/bin" >> ~/.bashrc
source ~/.bashrc

# Build and Install `bpftool`
git clone --recurse-submodules https://github.com/libbpf/bpftool.git
cd bpftool/src
make -j$(nproc) && sudo make install  # Build with parallelism 
cd ../../ && rm -rf bpftool  # Cleanup

# Verify installation
bpftool version  # Should display version info 

# Install Rust non-interactively
echo "Starting Rust installation..."
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain=stable

# Set up environment variables
echo "Configuring environment variables..."
source "$HOME/.cargo/env"
echo 'source "$HOME/.cargo/env"' >> ~/.bashrc

# Install essential components
echo "Installing trace_common toolchain components..."
rustup component add rust-src
rustup toolchain install nightly --component rust-src

# Verify installation
echo "Verifying installation..."
rustc --version
cargo --version

echo "✅ Rust environment setup completed! Recommended to restart terminal or run:"
echo "source ~/.bashrc"

# Install BPF-specific tools
cargo install --features=llvm-sys/prefer-dynamic bpf-linker
cargo install bindgen-cli  # Generate Rust bindings for C code 
cargo install --git https://github.com/aya-rs/aya -- aya-tool

aya-tool generate task_struct user_msghdr mmsghdr tcp_sock socket files_struct > agent/src/trace/ebpf/src/vmlinux.rs
sed -i '2i\#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code, unnecessary_transmutes)]' agent/src/trace/ebpf/src/vmlinux.rs