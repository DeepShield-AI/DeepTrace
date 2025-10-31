# Functional Testing Guide

This guide provides comprehensive instructions for testing DeepTrace's core functionality, including eBPF data collection, protocol inference, and span construction.

## Overview

DeepTrace functional testing covers several key areas:

- **eBPF Functionality**: Testing kernel-level data collection
- **Protocol Inference**: Validating automatic protocol detection
- **Span Construction**: Testing distributed trace span creation
- **Performance Overhead**: Measuring system impact

## eBPF Functionality Testing

### Test Environment Setup

The eBPF functionality tests validate DeepTrace's ability to collect network data at the kernel level.

#### Prerequisites

- Root privileges (for eBPF program loading)
- Python 3.6+ with required packages
- Network connectivity for test traffic generation

#### Test Execution

```bash
cd DeepTrace/tests/eBPF/functionality

# Start test server in background
python3 server.py &
SERVER_PID=$!  # Capture background process PID

# modify deeptrace.toml to include PID monitoring
vim agent/config/deeptrace.toml
# add pids = [SERVER_PID] to the ebpf section

# In another terminal, run the client to send requests
cd DeepTrace/tests/eBPF/functionality
python3 client.py

# Cleanup test server
kill $SERVER_PID
```

#### Expected Output Format

The output file contains structured records (location may vary based on configuration):

```plaintext
1201353, RecvFrom, python3, skc_family: IP protocol family, saddr: 127.0.0.1, daddr: 127.0.0.1, sport: 8080, dport: 1814, 707083292245311, 2953620009, 2953620073, 64, [71, 69, 84, 32, 47, 32, 72, 84, 84, 80, 47, 49, 46, 49, 13, 10, 72, 111, 115, 116, 58, 32, 49, 50, 55, 46, 48, 46, 48, 46, 49, 58, 56, 48, 56, 48, 13, 10, 67, 111, 110, 110, 101, 99, 116, 105, 111, 110, 58, 32, 107, 101, 101, 112, 45, 97, 108, 105, 118, 101, 13, 10, 13, 10]
```

#### Field Breakdown

1. **TGID**: Thread Group ID (Process ID)
2. **Syscall**: System call name (e.g., RecvFrom)
3. **Process**: Process name
4. **Protocol Family**: Network protocol (IPv4/IPv6)
5. **Source Address**: Connection source IP
6. **Destination Address**: Connection target IP
7. **Source Port**: Connection source port
8. **Destination Port**: Connection target port
9. **Timestamp**: Nanosecond-precision event timestamp
10. **TCP Sequence Start**: Initial TCP sequence number
11. **TCP Sequence End**: Final TCP sequence number
12. **Payload Length**: Message size in bytes
13. **Payload Buffer**: Raw message bytes (ASCII decimal values)

#### Validation Steps

1. **Data Completeness**: Verify all expected fields are present
2. **Timestamp Accuracy**: Check timestamp ordering and precision
3. **Payload Integrity**: Validate payload data matches expected content
4. **Process Tracking**: Confirm correct PID association

## Protocol Inference Testing

### Supported Protocols

DeepTrace currently supports automatic inference for:
- **MongoDB**: Document database protocol
- **Redis**: Key-value store protocol  
- **Memcached**: Distributed memory caching protocol

### Test Setup

#### Deploy Workload Server

You can deploy test servers using Docker or custom Python scripts:

```bash
# Using Docker (recommended)
docker run -d --name redis-test -p 6379:6379 redis:6.2.4
docker run -d --name mongo-test -p 27017:27017 mongo:5.0.15
docker run -d --name memcached-test -p 11211:11211 memcached:1.6.7
```

#### Obtain Container Process PID

```bash
# Retrieve container ID
docker ps

# Get PID based on container runtime
docker inspect <container-id> -f "{{.State.Pid}}"
```

### Test Execution

#### Start eBPF Monitoring

In one terminal:
```bash
cd agent
RUST_LOG=info cargo xtask build --profile release -c config/deeptrace.toml
```

#### Generate Workload Traffic

In another terminal:
```bash
cd tests/workload/protocols

# Create Python virtual environment
python3 -m venv env
source env/bin/activate

# Install required packages
pip install redis python-binary-memcached pymongo aiohttp

# Generate traffic for specific protocol
cd {protocol}  # redis, mongodb, or memcached
python3 client.py
```

#### Terminate and Analyze

1. Terminate the eBPF program after ~5 seconds of traffic generation
2. Output file will be generated based on your configuration (check the configured file path in `sender.file.*` sections)

### Result Validation

```bash
cd tests/eBPF/protocols

# Process raw data
python3 parse_ebpf.py --protocol {protocol}

# Calculate accuracy metrics
python3 check.py --protocol {protocol}
```

Expected output shows accuracy metrics:
```plaintext
Protocol: Redis
Accuracy: 95.2%
Total Requests: 1000
Correctly Identified: 952
```

## Span Construction Testing

Span construction testing validates DeepTrace's ability to correlate network transactions into distributed trace spans.

### Test Environment Setup

#### Start Workload Services

```bash
cd DeepTrace
docker-compose -f deployment/docker/Workload.yaml up -d

# Verify services are running
docker ps
```

Expected output:
```plaintext
a69c7abafa85   memcached:1.6.7   "docker-entrypoint.s…"   7 hours ago   Up 7 hours   0.0.0.0:11211->11211/tcp   memcached-workload
a27b67cd869a   mongo:5.0.15      "docker-entrypoint.s…"   7 hours ago   Up 7 hours   0.0.0.0:27017->27017/tcp   mongo-workload
631c9e145055   redis:6.2.4       "docker-entrypoint.s…"   7 hours ago   Up 7 hours   0.0.0.0:6379->6379/tcp     redis-workload
```

#### Initialize DeepTrace

```bash
chmod +x ./scripts/run_agent.sh
./scripts/run.sh
```

### Test Execution

#### Generate Test Spans

```bash
cd tests

# Setup Python environment
python3 -m venv workload/env
source workload/env/bin/activate
pip install redis python-binary-memcached

# Generate synthetic workload patterns
python3 -m workload.prepare_spans
```

Expected output:
```plaintext
redis workload completed successfully.
memcached workload completed successfully.
```

#### Stop Collection

Use Ctrl+C in DeepTrace container to:
- Finalize trace data
- Generate span artifacts
- Clean up tracing resources

### Span Validation

```bash
cd tests/workload
python3 test_span_construct.py
```

Expected output:
```plaintext
Protocol: Redis
Total:  1000
Correct:  1000
Accuracy:  1.0

Protocol: Memcached
Total:  1000
Correct:  1000
Accuracy:  1.0

No spans found for HTTP1 protocol.
```

### Span Quality Metrics

The validation script checks:

1. **Request-Response Correlation**: Matching requests with responses
2. **Timing Accuracy**: Span duration calculations
3. **Metadata Completeness**: Protocol-specific span attributes
4. **Trace Continuity**: Parent-child span relationships

## Performance Overhead Testing

### System Impact Measurement

#### Inject eBPF Program

```bash
cd agent
# Configure deeptrace.toml with appropriate settings
RUST_LOG=info cargo xtask run --release -c config/deeptrace.toml
```

#### Measure Syscall Overhead

```bash
cd tests/eBPF/overhead
bash run.sh <syscall>
```

Supported syscalls:
- `write` | `read` | `sendto` | `recvfrom`
- `sendmsg` | `sendmmsg` | `recvmsg` | `recvmmsg`
- `writev` | `readv` | `ssl_write` | `ssl_read`
- `ssl` | `empty`

#### Test Methodology

The overhead test:
1. Repeatedly calls a syscall 10^5 times
2. Takes the average of 100 iterations
3. Compares performance with and without eBPF

#### Expected Results

Typical overhead measurements:
```plaintext
Syscall: sendto
Without eBPF: 1.2μs average
With eBPF: 1.4μs average
Overhead: 16.7%
```

*Note: For bidirectional syscalls (recvfrom, sendto, recvmsg, sendmsg, recvmmsg, sendmmsg), you need to call both sending and receiving syscalls together.*

## Troubleshooting Test Issues

### Common Problems

1. **Permission Denied (eBPF)**:
   ```bash
   sudo setcap cap_sys_admin,cap_bpf+ep target/release/deeptrace
   ```

2. **Missing Dependencies**:
   ```bash
   # Install required packages
   sudo apt-get install linux-headers-$(uname -r)
   pip install -r tests/requirements.txt
   ```

3. **Port Conflicts**:
   ```bash
   # Check port usage
   netstat -tulpn | grep :8080
   
   # Kill conflicting processes
   sudo fuser -k 8080/tcp
   ```