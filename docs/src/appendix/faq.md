# Frequently Asked Questions (FAQ)

This FAQ addresses common questions about DeepTrace, covering installation, configuration, usage, and troubleshooting.

## General Questions

### What is DeepTrace?

**Q: What makes DeepTrace different from other distributed tracing solutions?**

A: DeepTrace is unique in several ways:
- **Non-intrusive**: No code changes required in your applications
- **eBPF-based**: Uses kernel-level instrumentation for comprehensive monitoring
- **Transaction-aware**: Uses intelligent correlation based on application semantics
- **Protocol-agnostic**: Supports 20+ protocols out of the box
- **High accuracy**: Achieves >95% tracing accuracy even under high concurrency

### What are the system requirements?

**Q: What operating systems and kernel versions does DeepTrace support?**

A: DeepTrace requires:
- **OS**: Ubuntu 24.04 LTS (or compatible Linux distribution)
- **Kernel**: 6.8.0+ with eBPF and BTF support
- **Memory**: 4GB minimum, 8GB recommended
- **Storage**: 40GB+ free space
- **CPU**: 2+ cores recommended

### How does DeepTrace compare to Jaeger, Zipkin, or other solutions?

**Q: Should I use DeepTrace instead of Jaeger/Zipkin?**

A: DeepTrace complements traditional tracing solutions:

| Feature | DeepTrace | Jaeger/Zipkin |
|---------|-----------|---------------|
| **Code Changes** | None required | Manual instrumentation |
| **Protocol Support** | 20+ protocols | Application-dependent |
| **Correlation** | AI-based semantic correlation | Manual span linking |
| **Overhead** | 2-5% | 1-3% |
| **Accuracy** | >95% | Depends on instrumentation |

Use DeepTrace when you need comprehensive tracing without code changes, or alongside existing solutions for enhanced visibility.

## Installation and Setup

### Can I install DeepTrace without Docker?

**Q: Is Docker required for DeepTrace installation?**

A: While Docker is the recommended installation method, you can compile DeepTrace manually:
- Follow the [Manual Compilation Guide](../getting-started/installation/manual.md)
- Requires Rust toolchain, LLVM, and libbpf
- More complex but provides full control over the build process

### Why do I need privileged access?

**Q: Why does DeepTrace require root/sudo privileges?**

A: DeepTrace needs elevated privileges for:
- **eBPF program loading**: Requires `CAP_BPF` and `CAP_SYS_ADMIN` capabilities
- **System call monitoring**: Needs access to kernel tracepoints
- **Network interface access**: Monitors network traffic at kernel level
- **Process monitoring**: Accesses process information and file descriptors

### Can I run DeepTrace in Kubernetes?

**Q: How do I deploy DeepTrace in a Kubernetes cluster?**

A: Yes, DeepTrace supports Kubernetes deployment:
- Deploy agents as DaemonSet on each node
- Run server as Deployment with multiple replicas
- Use ConfigMaps for configuration management
- Refer to the [Kubernetes deployment examples](../user-guide/deployment-modes/distributed.md)

## Configuration and Usage

### How do I monitor specific applications?

**Q: Can I choose which applications to monitor?**

A: Yes, DeepTrace provides flexible filtering options:

```toml
[agents.trace]
# Monitor specific processes by PID
pids = [1234, 5678]

# Monitor by process name
include_processes = ["nginx", "redis-server", "app-server"]
exclude_processes = ["systemd", "kernel"]

# Monitor all Docker containers (default)
monitor_containers = true
```

### What protocols does DeepTrace support?

**Q: Which application protocols can DeepTrace trace?**

A: DeepTrace currently supports:
- **Web**: HTTP/1.1, HTTP/2, gRPC
- **Databases**: MySQL, PostgreSQL, MongoDB, Redis
- **Message Queues**: RabbitMQ, Apache Kafka (planned)
- **Cache**: Redis, Memcached
- **Custom**: Extensible protocol detection

### How accurate is the correlation?

**Q: How reliable are the trace correlations?**

A: DeepTrace achieves high correlation accuracy:
- **>95% accuracy** in typical microservices environments
- **Transaction-based correlation** using API semantics
- **Multiple algorithms** available for different scenarios
- **Confidence scoring** for each correlation decision

You can tune correlation parameters based on your specific environment.

## Performance and Overhead

### What is the performance impact?

**Q: How much overhead does DeepTrace add to my applications?**

A: DeepTrace is designed for minimal impact:
- **CPU Overhead**: 2-5% under normal load
- **Memory Usage**: 50-200MB per agent
- **Network Latency**: <1μs additional latency
- **Throughput Impact**: <3% reduction in peak throughput

See the [Performance Analysis](../ebpf/performance.md) for detailed measurements.

### Can I reduce the overhead further?

**Q: How can I minimize DeepTrace's performance impact?**

A: Several optimization strategies:

1. **Implement sampling**:
   ```toml
   [agents.trace]
   sampling_rate = 0.1  # Sample 10% of requests
   ```

2. **Reduce payload capture**:
   ```toml
   [agents.capture]
   max_payload_size = 512
   enable_compression = true
   ```

3. **Filter processes**:
   ```toml
   [agents.trace]
   include_processes = ["critical-service-only"]
   ```

### Does DeepTrace affect application startup time?

**Q: Will DeepTrace slow down application startup?**

A: No, DeepTrace has minimal impact on application startup:
- eBPF programs load independently of applications
- No application code modification required
- Monitoring begins after applications are already running

## Troubleshooting

### Why am I not seeing any traces?

**Q: DeepTrace is running but no traces appear in the dashboard.**

A: Check these common issues:

1. **Verify agent is collecting data**:
   ```bash
   curl http://localhost:7899/status
   ```

2. **Check process filtering**:
   ```bash
   sudo docker exec -it deeptrace_server python -m cli.src.cmd agent list-processes
   ```

3. **Verify eBPF programs are loaded**:
   ```bash
   sudo bpftool prog list | grep deeptrace
   ```

4. **Check Elasticsearch connectivity**:
   ```bash
   curl http://localhost:9200/_cluster/health
   ```

### Why are my traces incomplete?

**Q: I see spans but traces are fragmented or missing spans.**

A: This usually indicates correlation issues:

1. **Adjust correlation parameters**:
   ```bash
   sudo docker exec -it deeptrace_server python -m cli.src.cmd asso config --window 2000
   ```

2. **Try different correlation algorithm**:
   ```bash
   sudo docker exec -it deeptrace_server python -m cli.src.cmd asso algo fifo
   ```

3. **Check for high load conditions**:
   - High CPU usage can cause span drops
   - Network issues can cause transmission delays

### How do I debug eBPF issues?

**Q: My eBPF programs aren't loading or working correctly.**

A: Debug eBPF issues systematically:

1. **Check kernel compatibility**:
   ```bash
   uname -r  # Should be 6.8.0+
   ls /sys/kernel/btf/vmlinux  # BTF should exist
   ```

2. **Verify eBPF support**:
   ```bash
   zgrep CONFIG_BPF /proc/config.gz
   zgrep CONFIG_BPF_SYSCALL /proc/config.gz
   ```

3. **Check for errors in kernel logs**:
   ```bash
   dmesg | grep -i bpf
   ```

4. **Use bpftool for debugging**:
   ```bash
   sudo bpftool prog list
   sudo bpftool map list
   ```

## Data Management

### How long is trace data retained?

**Q: How long does DeepTrace keep trace data?**

A: Data retention is configurable:
- **Default**: 7 days
- **Configurable**: Set retention policies in Elasticsearch
- **Automatic cleanup**: Old indices are automatically deleted
- **Manual cleanup**: Use provided cleanup scripts

### Can I export trace data?

**Q: How do I export traces for analysis or backup?**

A: Yes, multiple export options are available:

```bash
# Export to JSON
sudo docker exec -it deeptrace_server python -m cli.src.cmd export \
  --format json --output traces.json

# Export specific time range
sudo docker exec -it deeptrace_server python -m cli.src.cmd export \
  --start "2024-01-01T00:00:00Z" --end "2024-01-02T00:00:00Z"

# Elasticsearch snapshot
curl -X PUT "localhost:9200/_snapshot/backup/snapshot_1"
```

### How do I backup DeepTrace data?

**Q: What's the recommended backup strategy?**

A: Implement a comprehensive backup strategy:

1. **Configuration backup**:
   ```bash
   tar -czf config-backup.tar.gz /app/config/
   ```

2. **Elasticsearch snapshots**:
   ```bash
   curl -X PUT "localhost:9200/_snapshot/backup/daily_$(date +%Y%m%d)"
   ```

3. **Automated backup script**:
   ```bash
   # Run daily via cron
   0 2 * * * /path/to/backup-deeptrace.sh
   ```

## Security and Privacy

### Is trace data encrypted?

**Q: How does DeepTrace protect sensitive data?**

A: DeepTrace implements multiple security layers:
- **Encryption in transit**: TLS for all communications
- **Encryption at rest**: Elasticsearch encryption support
- **Access control**: Role-based access control (RBAC)
- **Data sanitization**: Configurable payload filtering

### Can I filter sensitive data?

**Q: How do I prevent sensitive information from being captured?**

A: Configure data filtering:

```toml
[agents.capture]
# Disable payload capture for sensitive services
exclude_payloads = ["payment-service", "auth-service"]

# Filter sensitive headers
filter_headers = ["Authorization", "X-API-Key"]

# Mask sensitive fields
mask_patterns = ["password", "ssn", "credit_card"]
```

### Does DeepTrace comply with privacy regulations?

**Q: Is DeepTrace GDPR/CCPA compliant?**

A: DeepTrace provides tools for compliance:
- **Data minimization**: Capture only necessary data
- **Right to erasure**: Delete specific user data
- **Data portability**: Export user-specific traces
- **Audit logging**: Track all data access

Consult with your legal team for specific compliance requirements.

## Advanced Usage

### Can I extend DeepTrace with custom protocols?

**Q: How do I add support for a custom protocol?**

A: Yes, DeepTrace is extensible:

1. **Implement protocol detector**:
   ```rust
   pub fn detect_custom_protocol(payload: &[u8]) -> bool {
       // Custom protocol detection logic
   }
   ```

2. **Add protocol parser**:
   ```rust
   pub fn parse_custom_protocol(payload: &[u8]) -> ProtocolMetadata {
       // Custom parsing logic
   }
   ```

3. **Register with DeepTrace**:
   ```rust
   register_protocol_handler("custom", detect_custom_protocol, parse_custom_protocol);
   ```

### Can I integrate DeepTrace with other monitoring tools?

**Q: How do I integrate DeepTrace with Prometheus, Grafana, etc.?**

A: DeepTrace supports multiple integration methods:
- **Metrics export**: Prometheus-compatible metrics endpoint
- **Grafana dashboards**: Pre-built dashboard templates
- **API integration**: REST API for custom integrations
- **Webhook notifications**: Real-time alerts and notifications

### How do I contribute to DeepTrace?

**Q: I want to contribute code or report bugs. How do I get involved?**

A: We welcome contributions:
- **GitHub Repository**: [DeepShield-AI/DeepTrace](https://github.com/DeepShield-AI/DeepTrace)
- **Issue Reporting**: Use GitHub Issues for bugs and feature requests
- **Development Guide**: See [Contributing Guide](./contributing.md)
- **Community**: Join our discussions and community channels

---

If your question isn't answered here, please check the detailed documentation sections or reach out to the community through our GitHub repository.