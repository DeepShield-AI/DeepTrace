#!/bin/bash
set -euo pipefail


# Install system dependencies
echo "Installing system dependencies..."
sudo apt-get update
sudo apt-get install -y curl build-essential gcc

# Install Rust non-interactively
echo "Starting Rust installation..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain=stable

# Set up environment variables
echo "Configuring environment variables..."
source "$HOME/.cargo/env"
echo 'source "$HOME/.cargo/env"' >> ~/.bashrc

# Install essential components
echo "Installing trace_common toolchain components..."
rustup component add rust-src rustfmt clippy

# Verify installation
echo "Verifying installation..."
rustc --version
cargo --version

echo "✅ Rust environment setup completed! Recommended to restart terminal or run:"
echo "source ~/.bashrc"

rustup toolchain install stable
rustup toolchain install nightly --component rust-src

cargo install bpf-linker

