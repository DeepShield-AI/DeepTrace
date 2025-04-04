# DeepTrace Usage Guide

## Environment Configuration

1. Rust Toolchain Setup

```bash
# Install Rust non-interactively
echo "Starting Rust installation..."
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain=stable

# Set up environment variables
source "$HOME/.cargo/env"
echo 'source "$HOME/.cargo/env"' >> ~/.bashrc
```
2. Essential Components

```bash
rustup component add rust-src rustfmt clippy
rustup toolchain install stable
rustup toolchain install nightly --component rust-src
```

3. BPF Support
Install required tools for eBPF development:

```bash
cargo install bpf-linker
cargo install bindgen-cli
cargo install --git https://github.com/aya-rs/aya -- aya-tool
# Generate kernel structure bindings
aya-tool generate task_struct user_msghdr mmsghdr tcp_sock socket files_struct > ../agent/src/ebpf/src/vmlinux.rs
```

## Command Line Arguments

|    Flag     |        Description        |   Type   |         Default         | 
| ----------- | ------------------------- | -------- | ----------------------- |
| --file      | eBPF output file position | String   | "tests/output/ebpf.txt" |
| --stdout    | eBPF output to stdout     | bool     | false                   |
| --no_output | eBPF has no output        | bool     | false                   |
| -p, --pids  | Process IDs to monitor    | Vec<u32> | None                    |


*Note: When `pids` are not specified, DeepTrace automatically detects running container instance PIDs via CRI and Docker.*

## Building & Compilation

### Test with Local Workload

```bash
# Navigate to test workload directory
cd tests/workload/http

# Start test server in background
python3 server.py &
SERVER_PID=$!  # Capture background process PID

# Run DeepTrace with PID monitoring
RUST_LOG=info cargo run --release \
  --config 'target."cfg(all())".runner="sudo -E"' \
  -- --pids $SERVER_PID

# Cleanup test server
kill $SERVER_PID
```

### Test with Kubernetes Pods

```bash
RUST_LOG=info cargo run --release --config 'target."cfg(all())".runner="sudo -E"'
```

Output will be generated at `tests/output/ebpf.txt`

## Deploy

Execute the deployment script:

```bash
cd deployment
bash ./deployment.sh
```

## Output Format

The output file `ebpf.txt` contains structured records with the following fields:
```plaintext
1201353, RecvFrom, python3, skc_family: IP protocol family, saddr: 127.0.0.1, daddr: 127.0.0.1, sport: 8080, dport: 1814, 707083292245311, 2953620009, 2953620073, 64, [71, 69, 84, 32, 47, 32, 72, 84, 84, 80, 47, 49, 46, 49, 13, 10, 72, 111, 115, 116, 58, 32, 49, 50, 55, 46, 48, 46, 48, 46, 49, 58, 56, 48, 56, 48, 13, 10, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 58, 32, 107, 101, 101, 112, 45, 97, 108, 105, 118, 101, 13, 10, 13, 10]
```

Field Breakdown:
1. TGID: Thread Group ID (Process ID)
2. Syscall: System call name (e.g., RecvFrom)
3. Process: Process name
4. Protocol Family: Network protocol (Ie.g., Pv4/IPv6)
5. Source Address: Connection source IP
6. Destination Address: Connection target IP
7. Source Port: Connection source port
8. Destination Port: Connection target port
9. Timestamp: Nanosecond-precision event timestamp
10. TCP Sequence Start: Initial TCP sequence number
11. TCP Sequence End: Final TCP sequence number
12. Payload Length: Message size in bytes
13. Payload Buffer: Raw message bytes (ASCII decimal values)