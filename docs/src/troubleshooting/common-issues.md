# Common Issues

This guide covers the most frequently encountered issues when deploying and operating DeepTrace, along with step-by-step solutions and preventive measures.

## Quick Diagnosis Checklist

Before diving into specific issues, run this quick diagnostic checklist:

```bash
# 1. Check all containers are running
sudo docker ps | grep -E "(deeptrace|elasticsearch)"

# 2. Verify network connectivity
curl -f http://localhost:7901/health
curl -f http://localhost:9200/_cluster/health
curl -f http://localhost:7899/status

# 3. Check logs for errors
sudo docker logs deeptrace_server --tail 50
sudo docker logs elasticsearch --tail 50

# 4. Verify eBPF programs are loaded
sudo bpftool prog list | grep deeptrace

# 5. Check system resources
free -h
df -h
```

## Installation Issues

### 1. Docker Installation Failures

#### Problem: Docker daemon not running

**Symptoms**:
```
Cannot connect to the Docker daemon at unix:///var/run/docker.sock
```

**Solution**:
```bash
# Start Docker service
sudo systemctl start docker
sudo systemctl enable docker

# Verify Docker is running
sudo systemctl status docker

# Test Docker functionality
sudo docker run hello-world
```

#### Problem: Permission denied accessing Docker

**Symptoms**:
```
permission denied while trying to connect to the Docker daemon socket
```

**Solution**:
```bash
# Add user to docker group
sudo usermod -aG docker $USER

# Apply group changes
newgrp docker

# Verify access
docker ps
```

#### Problem: Docker registry connection issues

**Symptoms**:
```
Error response from daemon: Get https://47.97.67.233:5000/v2/: http: server gave HTTP response to HTTPS client
```

**Solution**:
```bash
# Configure insecure registry
sudo nano /etc/docker/daemon.json

# Add configuration:
{
  "insecure-registries": ["47.97.67.233:5000"]
}

# Restart Docker
sudo systemctl restart docker
```

### 2. Compilation Errors

#### Problem: Missing dependencies

**Symptoms**:
```
error: failed to run custom build command for `ebpf-common`
clang: error: no such file or directory: '/usr/include/linux/bpf.h'
```

**Solution**:
```bash
# Install required packages
sudo apt-get update
sudo apt-get install -y \
  build-essential \
  clang \
  llvm-18 \
  llvm-18-dev \
  libelf-dev \
  libclang-18-dev \
  linux-headers-$(uname -r)

# Verify installation
clang-18 --version
ls /usr/include/linux/bpf.h
```

#### Problem: BTF (BPF Type Format) issues

**Symptoms**:
```
libbpf: failed to find valid kernel BTF
libbpf: Error loading vmlinux BTF: -2
```

**Solution**:
```bash
# Check BTF availability
ls -la /sys/kernel/btf/vmlinux

# If missing, check kernel config
zgrep CONFIG_DEBUG_INFO_BTF /proc/config.gz

# For Ubuntu, install BTF-enabled kernel
sudo apt-get install linux-image-generic-hwe-22.04

# Reboot if kernel was updated
sudo reboot
```

#### Problem: Rust compilation errors

**Symptoms**:
```
error: linking with `cc` failed: exit status: 1
/usr/bin/ld: cannot find -lbpf: No such file or directory
```

**Solution**:
```bash
# Install libbpf development libraries
sudo apt-get install libbpf-dev

# Or compile libbpf from source
git clone https://github.com/libbpf/libbpf.git
cd libbpf/src
make
sudo make install
sudo ldconfig
```

## Runtime Issues

### 3. Agent Connection Problems

#### Problem: Agent fails to start

**Symptoms**:
```bash
curl http://localhost:7899/status
# curl: (7) Failed to connect to localhost port 7899: Connection refused
```

**Diagnosis**:
```bash
# Check if agent process is running
ps aux | grep deeptrace

# Check agent logs
sudo docker exec -it deeptrace_server cat /var/log/deeptrace/agent.log

# Verify eBPF programs
sudo bpftool prog list | grep deeptrace
```

**Solution**:
```bash
# Restart agent
sudo docker exec -it deeptrace_server python -m cli.src.cmd agent stop
sudo docker exec -it deeptrace_server python -m cli.src.cmd agent run

# Check for permission issues
sudo dmesg | grep -i bpf

# Verify kernel version compatibility
uname -r
# Should be 6.8.0 or later
```

#### Problem: Agent loses connection to server

**Symptoms**:
- Agent status shows "disconnected"
- No new spans appearing in Elasticsearch
- Network timeouts in logs

**Diagnosis**:
```bash
# Test network connectivity
telnet localhost 7901

# Check server status
curl http://localhost:7901/health

# Monitor network traffic
sudo netstat -tuln | grep 7901
```

**Solution**:
```bash
# Check firewall settings
sudo ufw status
sudo iptables -L

# Verify server configuration
sudo docker exec -it deeptrace_server cat /app/config/config.toml

# Restart networking components
sudo docker restart deeptrace_server
```

### 4. Data Collection Issues

#### Problem: No spans being collected

**Symptoms**:
- Empty Elasticsearch indices
- Zero span count in dashboard
- No eBPF events in logs

**Diagnosis**:
```bash
# Check monitored processes
sudo docker exec -it deeptrace_server python -m cli.src.cmd agent list-processes

# Verify eBPF program attachment
sudo bpftool prog show | grep deeptrace

# Check system call activity
sudo strace -e trace=network -p $(pgrep your-app) -c
```

**Solution**:
```bash
# Add processes to monitoring
sudo docker exec -it deeptrace_server python -m cli.src.cmd agent add-process --name nginx

# Verify process filtering configuration
sudo docker exec -it deeptrace_server python -m cli.src.cmd config show agents.trace.pids

# Restart with debug logging
RUST_LOG=debug sudo docker exec -it deeptrace_server python -m cli.src.cmd agent run
```

#### Problem: Incomplete span data

**Symptoms**:
- Spans missing payload data
- Incomplete network information
- Missing timing information

**Diagnosis**:
```bash
# Check payload capture settings
curl http://localhost:7899/config | jq '.capture'

# Monitor eBPF map usage
sudo bpftool map show | grep deeptrace

# Check for buffer overflows
dmesg | grep -i "ring buffer"
```

**Solution**:
```bash
# Increase buffer sizes
sudo docker exec -it deeptrace_server python -m cli.src.cmd config update \
  --key "agents.sender.mem_buffer_size" --value 64

# Enable payload compression
sudo docker exec -it deeptrace_server python -m cli.src.cmd config update \
  --key "agents.capture.enable_compression" --value true

# Adjust payload limits
sudo docker exec -it deeptrace_server python -m cli.src.cmd config update \
  --key "agents.capture.max_payload_size" --value 2048
```

### 5. Performance Issues

#### Problem: High CPU usage

**Symptoms**:
- System CPU usage > 80%
- Application performance degradation
- High eBPF program execution time

**Diagnosis**:
```bash
# Monitor CPU usage by process
htop
top -p $(pgrep deeptrace)

# Check eBPF program performance
sudo bpftool prog show | grep run_time_ns

# Profile application performance
perf top -p $(pgrep your-app)
```

**Solution**:
```bash
# Implement sampling
sudo docker exec -it deeptrace_server python -m cli.src.cmd config update \
  --key "agents.trace.sampling_rate" --value 0.1

# Reduce payload capture
sudo docker exec -it deeptrace_server python -m cli.src.cmd config update \
  --key "agents.capture.max_payload_size" --value 512

# Optimize process filtering
sudo docker exec -it deeptrace_server python -m cli.src.cmd agent remove-process --name unnecessary-process
```

#### Problem: High memory usage

**Symptoms**:
- System memory usage > 90%
- OOM (Out of Memory) errors
- Swap usage increasing

**Diagnosis**:
```bash
# Check memory usage by component
free -h
sudo docker stats

# Monitor Elasticsearch memory
curl http://localhost:9200/_nodes/stats/jvm

# Check for memory leaks
valgrind --tool=massif --pid=$(pgrep deeptrace)
```

**Solution**:
```bash
# Reduce Elasticsearch heap size
sudo docker exec -it elasticsearch bash -c 'export ES_JAVA_OPTS="-Xms1g -Xmx2g"'

# Implement data retention
curl -X PUT "localhost:9200/_ilm/policy/deeptrace-policy" -H 'Content-Type: application/json' -d'
{
  "policy": {
    "phases": {
      "delete": {
        "min_age": "7d"
      }
    }
  }
}'

# Clean old indices
curl -X DELETE "localhost:9200/traces-$(date -d '7 days ago' +%Y.%m.%d)"
```

### 6. Elasticsearch Issues

#### Problem: Elasticsearch cluster health is red

**Symptoms**:
```bash
curl http://localhost:9200/_cluster/health
# {"status":"red","timed_out":false}
```

**Diagnosis**:
```bash
# Check cluster status details
curl http://localhost:9200/_cluster/health?pretty

# Check node status
curl http://localhost:9200/_cat/nodes?v

# Check shard allocation
curl http://localhost:9200/_cat/shards?v
```

**Solution**:
```bash
# Restart Elasticsearch
sudo docker restart elasticsearch

# Check disk space
df -h

# Reallocate unassigned shards
curl -X POST "localhost:9200/_cluster/reroute?retry_failed=true"

# If disk space is low, clean old data
curl -X DELETE "localhost:9200/traces-*" -H 'Content-Type: application/json' -d'
{
  "query": {
    "range": {
      "@timestamp": {
        "lt": "now-7d"
      }
    }
  }
}'
```

#### Problem: Slow query performance

**Symptoms**:
- Dashboard loading slowly
- Query timeouts
- High Elasticsearch CPU usage

**Diagnosis**:
```bash
# Check slow queries
curl http://localhost:9200/_nodes/stats/indices/search

# Monitor query performance
curl http://localhost:9200/_cat/thread_pool/search?v

# Check index statistics
curl http://localhost:9200/_cat/indices?v&s=store.size:desc
```

**Solution**:
```bash
# Optimize indices
curl -X POST "localhost:9200/traces-*/_forcemerge?max_num_segments=1"

# Add more replicas for read performance
curl -X PUT "localhost:9200/traces-*/_settings" -H 'Content-Type: application/json' -d'
{
  "index": {
    "number_of_replicas": 1
  }
}'

# Create index templates with optimized mappings
curl -X PUT "localhost:9200/_index_template/traces" -H 'Content-Type: application/json' -d'
{
  "index_patterns": ["traces-*"],
  "template": {
    "settings": {
      "number_of_shards": 1,
      "number_of_replicas": 0,
      "refresh_interval": "30s"
    }
  }
}'
```

## Correlation and Assembly Issues

### 7. Poor Correlation Results

#### Problem: Low correlation accuracy

**Symptoms**:
- Traces with missing spans
- Incorrect parent-child relationships
- Fragmented traces

**Diagnosis**:
```bash
# Check correlation statistics
sudo docker exec -it deeptrace_server python -m cli.src.cmd asso stats

# Analyze correlation parameters
sudo docker exec -it deeptrace_server python -m cli.src.cmd asso config show

# Review sample traces
curl "http://localhost:9200/traces/_search?size=10&pretty"
```

**Solution**:
```bash
# Adjust correlation window
sudo docker exec -it deeptrace_server python -m cli.src.cmd asso config --window 2000

# Lower similarity threshold
sudo docker exec -it deeptrace_server python -m cli.src.cmd asso config --threshold 0.6

# Try different algorithm
sudo docker exec -it deeptrace_server python -m cli.src.cmd asso algo fifo

# Enable debug mode
sudo docker exec -it deeptrace_server python -m cli.src.cmd asso config --debug
```

#### Problem: Correlation timeouts

**Symptoms**:
- Correlation process hangs
- High CPU usage during correlation
- Memory exhaustion

**Diagnosis**:
```bash
# Monitor correlation process
ps aux | grep correlation
htop -p $(pgrep correlation)

# Check memory usage
free -h
sudo docker stats deeptrace_server
```

**Solution**:
```bash
# Increase timeout values
sudo docker exec -it deeptrace_server python -m cli.src.cmd asso config --timeout 300

# Process in smaller batches
sudo docker exec -it deeptrace_server python -m cli.src.cmd asso config --batch-size 1000

# Add more memory to container
sudo docker update --memory 4g deeptrace_server
```

## Network and Connectivity Issues

### 8. Port Conflicts

#### Problem: Port already in use

**Symptoms**:
```
Error starting userland proxy: listen tcp 0.0.0.0:7901: bind: address already in use
```

**Diagnosis**:
```bash
# Check what's using the port
sudo netstat -tuln | grep 7901
sudo lsof -i :7901

# Find the process
sudo fuser 7901/tcp
```

**Solution**:
```bash
# Kill conflicting process
sudo fuser -k 7901/tcp

# Or change DeepTrace port
sudo docker exec -it deeptrace_server python -m cli.src.cmd config update \
  --key "server.port" --value 7902

# Restart with new configuration
sudo docker restart deeptrace_server
```

### 9. SSL/TLS Issues

#### Problem: Certificate validation errors

**Symptoms**:
```
SSL certificate problem: self signed certificate
```

**Solution**:
```bash
# For development, disable SSL verification
curl -k https://localhost:7901/health

# For production, install proper certificates
sudo docker exec -it deeptrace_server python -m cli.src.cmd cert install \
  --cert /path/to/cert.pem \
  --key /path/to/key.pem
```

## Monitoring and Alerting

### 10. Setting Up Health Checks

Create monitoring scripts to detect issues early:

```bash
#!/bin/bash
# health-check.sh

# Check all services
services=("deeptrace_server:7901" "elasticsearch:9200" "agent:7899")

for service in "${services[@]}"; do
    name=$(echo $service | cut -d: -f1)
    port=$(echo $service | cut -d: -f2)
    
    if ! curl -f -s http://localhost:$port/health > /dev/null; then
        echo "ALERT: $name is not responding on port $port"
        # Send alert (email, Slack, etc.)
    fi
done

# Check disk space
disk_usage=$(df / | tail -1 | awk '{print $5}' | sed 's/%//')
if [ $disk_usage -gt 80 ]; then
    echo "ALERT: Disk usage is ${disk_usage}%"
fi

# Check memory usage
mem_usage=$(free | grep Mem | awk '{printf "%.0f", $3/$2 * 100.0}')
if [ $mem_usage -gt 80 ]; then
    echo "ALERT: Memory usage is ${mem_usage}%"
fi
```

### 11. Log Analysis

Set up centralized logging for better troubleshooting:

```bash
# Collect all logs
sudo docker logs deeptrace_server > deeptrace-server.log 2>&1
sudo docker logs elasticsearch > elasticsearch.log 2>&1
dmesg | grep -i bpf > kernel-bpf.log

# Analyze error patterns
grep -i error *.log
grep -i "failed\|timeout\|exception" *.log

# Monitor real-time logs
sudo docker logs -f deeptrace_server | grep -E "(ERROR|WARN|FATAL)"
```

## Prevention Strategies

### 1. Regular Maintenance

```bash
#!/bin/bash
# maintenance.sh - Run weekly

# Clean old data
curl -X DELETE "localhost:9200/traces-$(date -d '30 days ago' +%Y.%m.%d)"

# Optimize indices
curl -X POST "localhost:9200/traces-*/_forcemerge?max_num_segments=1"

# Update system packages
sudo apt-get update && sudo apt-get upgrade -y

# Restart services
sudo docker restart deeptrace_server elasticsearch
```

### 2. Capacity Planning

Monitor these metrics regularly:

- **CPU Usage**: Keep below 70% average
- **Memory Usage**: Keep below 80% average  
- **Disk Usage**: Keep below 75% average
- **Network Bandwidth**: Monitor for saturation
- **Elasticsearch Heap**: Keep below 75% of allocated memory

### 3. Backup Strategy

```bash
#!/bin/bash
# backup.sh - Run daily

# Backup Elasticsearch data
curl -X PUT "localhost:9200/_snapshot/backup/snapshot_$(date +%Y%m%d)" -H 'Content-Type: application/json' -d'
{
  "indices": "traces-*",
  "ignore_unavailable": true,
  "include_global_state": false
}'

# Backup configuration
sudo docker exec deeptrace_server tar -czf /backup/config-$(date +%Y%m%d).tar.gz /app/config/
```