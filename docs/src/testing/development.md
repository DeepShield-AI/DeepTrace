# Development Setup

This guide provides comprehensive instructions for setting up a development environment for DeepTrace, including all necessary tools, dependencies, and configurations for building, testing, and debugging the system.

## Prerequisites

### System Requirements

**Operating System:**
- Ubuntu 22.04 LTS or later (recommended)
- Linux kernel 5.15+ (kernel 6.8+ strongly recommended)
- x86_64 architecture

**Hardware Requirements:**
- Minimum: 4 CPU cores, 8GB RAM, 20GB disk space
- Recommended: 8+ CPU cores, 16GB+ RAM, 50GB+ disk space
- SSD storage recommended for better performance

### Required Packages

```bash
# Update system packages
sudo apt update && sudo apt upgrade -y

# Install essential development tools
sudo apt install -y \
    build-essential \
    git \
    curl \
    wget \
    pkg-config \
    libssl-dev \
    libz-dev \
    linux-headers-$(uname -r) \
    clang \
    llvm \
    libbpf-dev \
    bpftool

# Install additional dependencies
sudo apt install -y \
    cmake \
    ninja-build \
    python3 \
    python3-pip \
    docker.io \
    docker-compose \
    jq \
    htop \
    tree
```

## Rust Development Environment

### Rust Installation

```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install required Rust components
rustup component add rustfmt clippy
rustup target add x86_64-unknown-linux-musl

# Install cargo extensions
cargo install cargo-watch cargo-edit cargo-audit
```

### Rust Configuration

Create `~/.cargo/config.toml`:

```toml
[build]
target-dir = "/tmp/cargo-target"

[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[registries.crates-io]
protocol = "sparse"

[net]
retry = 2
git-fetch-with-cli = true
```

## eBPF Development Environment

### LLVM and Clang Setup

```bash
# Install specific LLVM version for eBPF
wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh
sudo ./llvm.sh 15

# Set up alternatives
sudo update-alternatives --install /usr/bin/clang clang /usr/bin/clang-15 100
sudo update-alternatives --install /usr/bin/llc llc /usr/bin/llc-15 100
sudo update-alternatives --install /usr/bin/opt opt /usr/bin/opt-15 100

# Verify installation
clang --version
llc --version
```

### libbpf Installation

```bash
# Clone and build libbpf
git clone https://github.com/libbpf/libbpf.git
cd libbpf/src
make
sudo make install

# Update library path
echo '/usr/local/lib64' | sudo tee -a /etc/ld.so.conf.d/libbpf.conf
sudo ldconfig
```

### BPF Development Tools

```bash
# Install bpftrace
sudo apt install -y bpftrace

# Install additional BPF tools
sudo apt install -y \
    linux-tools-$(uname -r) \
    linux-tools-generic \
    bpfcc-tools

# Verify BPF functionality
sudo bpftool prog list
sudo bpftrace -e 'BEGIN { printf("BPF is working!\n"); exit(); }'
```

## Project Setup

### Repository Clone

```bash
# Clone the repository
git clone https://github.com/DeepShield-AI/DeepTrace.git
cd DeepTrace

# Set up git hooks (optional)
git config core.hooksPath .githooks
chmod +x .githooks/*
```

### Environment Configuration

Create `.env` file in project root:

```bash
# Development environment variables
export RUST_LOG=debug
export RUST_BACKTRACE=1
export DEEPTRACE_LOG_LEVEL=debug

# eBPF development
export BPF_CLANG=clang-15
export BPF_CFLAGS="-O2 -g -Wall -Werror"

# Test configuration
export TEST_ELASTICSEARCH_URL=http://localhost:9200
export TEST_TIMEOUT=300

# Development paths
export CARGO_TARGET_DIR=/tmp/cargo-target
export TMPDIR=/tmp
```

Load environment variables:

```bash
source .env
echo 'source $(pwd)/.env' >> ~/.bashrc
```

## Build System Setup

### Initial Build

```bash
# Build all components
cargo build --release

# Build specific components
cargo build -p deeptrace-agent --release
cargo build -p deeptrace-server --release
cargo build -p ebpf-common --release
```

### Development Build

```bash
# Fast development build
cargo build

# Build with specific features
cargo build --features "debug-logs,test-utils"

# Build for testing
cargo build --tests
```

### eBPF Build Verification

```bash
# Test eBPF compilation
cd crates/ebpf-common
cargo build --release

# Verify eBPF object files
ls -la target/release/build/ebpf-common-*/out/
file target/release/build/ebpf-common-*/out/*.o
```

## Testing Environment

### Unit Tests Setup

```bash
# Run all unit tests
cargo test

# Run specific test suite
cargo test --package deeptrace-agent
cargo test --package ebpf-common

# Run tests with output
cargo test -- --nocapture
```

### Integration Tests Setup

```bash
# Deploy test workloads
cd tests/workload/bookinfo
sudo bash deploy.sh

# Run tests
cd tests/workload
python3 test_span_construct.py

# Cleanup
cd bookinfo
sudo bash clear.sh
```

### eBPF Tests Setup

```bash
# Run eBPF functionality tests
cd tests/eBPF/functionality
python3 server.py &
python3 client.py

# Run performance overhead tests
cd tests/eBPF/overhead
bash run.sh write
bash run.sh read
bash run.sh sendto
```

## Development Tools

### IDE Configuration

#### Visual Studio Code

Install recommended extensions:

```bash
# Install VS Code extensions
code --install-extension rust-lang.rust-analyzer
code --install-extension vadimcn.vscode-lldb
code --install-extension ms-vscode.cpptools
code --install-extension ms-python.python
code --install-extension redhat.vscode-yaml
```

Create `.vscode/settings.json`:

```json
{
    "rust-analyzer.cargo.target": "x86_64-unknown-linux-gnu",
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.cargo.features": "all",
    "files.watcherExclude": {
        "**/target/**": true,
        "/tmp/cargo-target/**": true
    },
    "C_Cpp.default.includePath": [
        "/usr/include",
        "/usr/local/include",
        "/usr/include/x86_64-linux-gnu"
    ]
}
```

Create `.vscode/launch.json`:

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug DeepTrace Agent",
            "cargo": {
                "args": ["build", "--bin=deeptrace-agent"],
                "filter": {
                    "name": "deeptrace-agent",
                    "kind": "bin"
                }
            },
            "args": ["-f", "config/deeptrace.toml"],
            "cwd": "${workspaceFolder}",
            "environment": [
                {"name": "RUST_LOG", "value": "debug"}
            ]
        }
    ]
}
```

### Debugging Tools

#### GDB Setup

```bash
# Install GDB with Rust support
sudo apt install -y gdb

# Create .gdbinit
echo 'set auto-load safe-path /' >> ~/.gdbinit
echo 'set print pretty on' >> ~/.gdbinit
```

#### Valgrind Setup

```bash
# Install Valgrind
sudo apt install -y valgrind

# Run memory check
valgrind --tool=memcheck --leak-check=full \
    ./target/debug/deeptrace-agent -f config/deeptrace.toml
```

#### Performance Profiling

```bash
# Install perf tools
sudo apt install -y linux-perf

# Profile application
perf record -g ./target/release/deeptrace-agent -f config/deeptrace.toml
perf report

# CPU profiling with flamegraph
cargo install flamegraph
cargo flamegraph --bin deeptrace-agent -- -f config/deeptrace.toml
```

## Database Setup

### Elasticsearch Development

```bash
# Start Elasticsearch for development
docker run -d \
    --name elasticsearch-dev \
    -p 9200:9200 \
    -p 9300:9300 \
    -e "discovery.type=single-node" \
    -e "ES_JAVA_OPTS=-Xms512m -Xmx512m" \
    elasticsearch:8.11.0

# Wait for Elasticsearch to start
curl -X GET "localhost:9200/_cluster/health?wait_for_status=yellow&timeout=30s"

# Test Elasticsearch connection
curl -X GET "localhost:9200/_cluster/health"

# Spans will be automatically indexed by the agent
# Index pattern: spans_{agent_name}
```

### Test Database Setup

```bash
# Set up test-specific Elasticsearch
docker run -d \
    --name elasticsearch-test \
    -p 9201:9200 \
    -e "discovery.type=single-node" \
    -e "ES_JAVA_OPTS=-Xms256m -Xmx256m" \
    elasticsearch:8.11.0

# Configure test environment
export TEST_ELASTICSEARCH_URL=http://localhost:9201
```

## Development Workflow

### Code Style and Linting

```bash
# Format code
cargo fmt

# Run clippy lints
cargo clippy -- -D warnings

# Run additional lints
cargo clippy --all-targets --all-features -- -D warnings

# Check for security vulnerabilities
cargo audit
```

### Pre-commit Hooks

Create `.githooks/pre-commit`:

```bash
#!/bin/bash
set -e

echo "Running pre-commit checks..."

# Format check
if ! cargo fmt -- --check; then
    echo "Code formatting issues found. Run 'cargo fmt' to fix."
    exit 1
fi

# Clippy check
if ! cargo clippy --all-targets --all-features -- -D warnings; then
    echo "Clippy warnings found. Please fix them."
    exit 1
fi

# Test check
if ! cargo test --lib; then
    echo "Unit tests failed."
    exit 1
fi

echo "Pre-commit checks passed!"
```

Make it executable:

```bash
chmod +x .githooks/pre-commit
```

### Development Scripts

Create `scripts/dev-setup.sh`:

```bash
#!/bin/bash
# Set up test environment
# Install required Python packages
pip3 install requests elasticsearch

echo "Setting up DeepTrace development environment..."

# Check prerequisites
check_prerequisites() {
    echo "Checking prerequisites..."
    
    # Check kernel version
    KERNEL_VERSION=$(uname -r | cut -d. -f1,2)
    if (( $(echo "$KERNEL_VERSION < 5.15" | bc -l) )); then
        echo "Warning: Kernel version $KERNEL_VERSION is below recommended 5.15"
    fi
    
    # Check required commands
    for cmd in cargo clang llc bpftool docker; do
        if ! command -v $cmd &> /dev/null; then
            echo "Error: $cmd is not installed"
            exit 1
        fi
    done
    
    echo "Prerequisites check passed!"
}

# Set up development database
setup_database() {
    echo "Setting up development database..."
    
    if ! docker ps | grep -q elasticsearch-dev; then
        docker run -d \
            --name elasticsearch-dev \
            -p 9200:9200 \
            -e "discovery.type=single-node" \
            -e "ES_JAVA_OPTS=-Xms512m -Xmx512m" \
            elasticsearch:8.11.0
        
        echo "Waiting for Elasticsearch to start..."
        sleep 30
    fi
    
    # Test connection
    if curl -s http://localhost:9200/_cluster/health > /dev/null; then
        echo "Elasticsearch is running!"
    else
        echo "Error: Could not connect to Elasticsearch"
        exit 1
    fi
}

# Build project
build_project() {
    echo "Building project..."
    
    # Clean build
    cargo clean
    
    # Build all components
    cargo build --all
    
    # Run basic tests
    cargo test --lib
    
    echo "Build completed successfully!"
}

# Main execution
main() {
    check_prerequisites
    setup_database
    build_project
    
    echo "Development environment setup complete!"
    echo "You can now run:"
    echo "  cd agent && cargo xtask run --release -c config/deeptrace.toml"
    echo "  cd server && python cli/src/cmd.py agent run"
}

main "$@"
```

Make it executable:

```bash
chmod +x scripts/dev-setup.sh
```

## Troubleshooting

### Common Issues

#### eBPF Compilation Errors

```bash
# Check clang version
clang --version

# Verify BPF target support
echo 'int main() { return 0; }' | clang -target bpf -c -x c - -o /tmp/test.o
file /tmp/test.o

# Check kernel headers
ls -la /usr/src/linux-headers-$(uname -r)/
```

#### Permission Issues

```bash
# Add user to required groups
sudo usermod -a -G docker $USER
sudo usermod -a -G bpf $USER

# Set up BPF permissions
echo 'kernel.unprivileged_bpf_disabled=0' | sudo tee -a /etc/sysctl.conf
sudo sysctl -p
```

#### Build Issues

```bash
# Clear cargo cache
cargo clean
rm -rf /tmp/cargo-target

# Update dependencies
cargo update

# Check disk space
df -h
```

### Debug Logging

Enable comprehensive debug logging:

```bash
# Set environment variables
export RUST_LOG=trace
export RUST_BACKTRACE=full
export DEEPTRACE_LOG_LEVEL=trace

# Run with debug output
cargo run --bin deeptrace-agent -- -f config/deeptrace.toml 2>&1 | tee debug.log
```

### Performance Debugging

```bash
# Profile build performance
cargo build --timings

# Check compilation bottlenecks
time cargo build --release

# Monitor system resources
htop
iotop
```

## Best Practices

### Development Guidelines

- **Code Organization**: Keep modules focused and well-documented
- **Error Handling**: Use proper error types and propagation
- **Testing**: Write comprehensive unit and integration tests
- **Documentation**: Document public APIs and complex logic
- **Performance**: Profile critical paths and optimize bottlenecks

### Git Workflow

```bash
# Create feature branch
git checkout -b feature/new-feature

# Make changes and commit
git add .
git commit -m "feat: add new feature"

# Push and create PR
git push origin feature/new-feature
```

### Code Review Checklist

- [ ] Code follows Rust style guidelines
- [ ] All tests pass
- [ ] Documentation is updated
- [ ] Performance impact is considered
- [ ] Security implications are reviewed
- [ ] eBPF programs are verified for safety

### Release Process

```bash
# Update version
cargo edit --version 0.2.0

# Build release
cargo build --release

# Run full test suite
cargo test --release

# Create release tag
git tag -a v0.2.0 -m "Release version 0.2.0"
git push origin v0.2.0
```
