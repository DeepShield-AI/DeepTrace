# Performance Testing Guide

This guide provides comprehensive instructions for testing DeepTrace's performance characteristics, including overhead measurements, throughput analysis, and scalability testing.

## Overview

DeepTrace performance testing focuses on several key areas:

- **System Overhead**: Impact on system performance
- **Data Processing Throughput**: Rate of data collection and processing
- **Memory Usage**: Memory consumption patterns
- **Scalability**: Performance under increasing load
- **Resource Utilization**: CPU, memory, and network usage

## System Overhead Testing

### eBPF Program Overhead

The eBPF overhead testing measures the performance impact of DeepTrace's kernel-level monitoring.

#### Test Setup

```bash
cd agent

# Configure the agent (deeptrace.toml already exists)
# Edit config/deeptrace.toml as needed

# Start DeepTrace with eBPF monitoring
RUST_LOG=info cargo xtask run --release -c config/deeptrace.toml
```

#### Syscall Overhead Measurement

```bash
cd tests/eBPF/overhead

# Run overhead test for specific syscall
bash run.sh <syscall>
```

Supported syscall tests:
- **Basic I/O**: `write`, `read`, `writev`, `readv`
- **Network**: `sendto`, `recvfrom`, `sendmsg`, `recvmsg`
- **Batch Operations**: `sendmmsg`, `recvmmsg`
- **SSL/TLS**: `ssl_write`, `ssl_read`, `ssl`
- **Baseline**: `empty` (no-op for baseline measurement)

#### Test Methodology

The overhead test performs:
1. **Baseline Measurement**: 10^5 syscall iterations without eBPF
2. **eBPF Measurement**: 10^5 syscall iterations with eBPF active
3. **Statistical Analysis**: Average of 100 test runs
4. **Overhead Calculation**: Percentage increase in execution time

### Application-Level Overhead

Test DeepTrace's impact using the provided workload applications:

```bash
# Deploy BookInfo application
cd tests/workload/bookinfo
sudo bash deploy.sh

# Start DeepTrace with monitoring
cd agent
RUST_LOG=info cargo xtask run --release -c config/deeptrace.toml

# Generate traffic and observe performance
cd tests/workload/bookinfo
sudo bash client.sh
```

## Performance Analysis

Analyze DeepTrace performance by:
- Comparing application metrics before and after enabling monitoring
- Querying Elasticsearch for trace collection rates
- Monitoring system resources (CPU, memory) during operation

## Resource Monitoring

Monitor resource usage during testing:

```bash
# Monitor system resources
top -p $(pgrep deeptrace)
htop

# Check memory usage
free -h

# Monitor network I/O
iftop
```

## Scalability Testing

Test scalability using Social Network workload which includes multiple interconnected services:

```bash
# Deploy complex multi-service application
cd tests/workload/socialnetwork
bash deploy.sh
bash client.sh
```


## Performance Benchmarking

Benchmark performance using the overhead testing scripts:

```bash
cd tests/eBPF/overhead
bash run.sh write
bash run.sh read
bash run.sh sendto
bash run.sh recvfrom
```

## Performance Optimization

Optimize DeepTrace configuration for better performance:

```toml
# Adjust configuration in agent/config/deeptrace.toml
[trace.span]
cleanup_interval = 60
max_sockets = 10000

[ebpf.trace]
max_buffered_events = 256

[sender.elastic.trace]
bulk_size = 64
```

## Best Practices

- Run overhead tests on target deployment hardware
- Test with realistic workload applications
- Monitor system resources during testing
- Compare baseline vs monitored performance metrics
- Document configuration changes and their impact
