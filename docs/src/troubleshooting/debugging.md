# Debugging Guide

This comprehensive debugging guide helps you diagnose and resolve issues with DeepTrace components. It covers systematic troubleshooting approaches, diagnostic tools, and common problem resolution strategies.

## Debugging Methodology

### 1. Problem Identification

Start with these key questions:

- **What is the expected behavior?**
- **What is the actual behavior?**
- **When did the problem start?**
- **What changed recently?**
- **Is the problem consistent or intermittent?**

### 2. Information Gathering

Collect relevant information systematically:

```bash
# System information
uname -a
cat /etc/os-release
free -h
df -h

# DeepTrace version
deeptrace-agent --version
deeptrace-server --version

# Process status
ps aux | grep deeptrace
systemctl status deeptrace-agent
systemctl status deeptrace-server
```

### 3. Log Analysis

Enable comprehensive logging:

```toml
# agent.toml
[logging]
level = "debug"
format = "json"
output = "file"
file_path = "/var/log/deeptrace/agent-debug.log"

# server.toml
[logging]
level = "debug"
format = "json"
output = "file"
file_path = "/var/log/deeptrace/server-debug.log"
```

## Component-Specific Debugging

### Agent Debugging

#### eBPF Program Issues

**Check eBPF Support:**
```bash
# Verify kernel version
uname -r

# Check eBPF filesystem
ls -la /sys/fs/bpf/

# Verify BPF capabilities
grep CONFIG_BPF /boot/config-$(uname -r)
```

**Debug Program Loading:**
```bash
# Enable eBPF debug logging
echo 1 > /proc/sys/kernel/bpf_stats_enabled

# Check loaded programs
sudo bpftool prog list | grep deeptrace

# Monitor kernel messages
sudo dmesg -w | grep bpf

# Check program verification logs
journalctl -f | grep bpf
```

**Common eBPF Errors:**

1. **BTF_KIND:0 Error**
   ```bash
   # Check BTF availability
   ls -la /sys/kernel/btf/vmlinux
   
   # Verify BTF format
   bpftool btf dump file /sys/kernel/btf/vmlinux | head -20
   
   # Fallback to non-CO-RE mode
   export DEEPTRACE_EBPF_ENABLE_CO_RE=false
   ```

2. **Permission Denied**
   ```bash
   # Check capabilities
   getcap /usr/bin/deeptrace-agent
   
   # Add required capabilities
   sudo setcap cap_sys_admin,cap_bpf+ep /usr/bin/deeptrace-agent
   
   # Or run with sudo (not recommended for production)
   sudo deeptrace-agent --config agent.toml
   ```

3. **Program Too Large**
   ```bash
   # Check program size limits
   cat /proc/sys/kernel/bpf_jit_limit
   
   # Increase limit if needed
   echo 1000000000 | sudo tee /proc/sys/kernel/bpf_jit_limit
   ```

#### Span Collection Issues

**Debug Span Collection:**
```bash
# Enable span collection debugging
curl -X POST http://localhost:7899/config \
  -H "Content-Type: application/json" \
  -d '{"logging": {"level": "trace"}}'

# Monitor span collection rate
watch -n 1 'curl -s http://localhost:7899/status | jq .collection.spans_per_second'

# Check process filtering
curl http://localhost:7899/processes | jq '.processes[] | select(.status == "monitored")'
```

**Span Collection Troubleshooting:**

```python
#!/usr/bin/env python3
# debug_span_collection.py

import requests
import time
import json

def debug_span_collection():
    agent_url = "http://localhost:7899"
    
    # Get agent status
    status = requests.get(f"{agent_url}/status").json()
    print(f"Agent Status: {status['agent']['status']}")
    print(f"eBPF Programs: {status['ebpf']['programs_loaded']}")
    print(f"Spans Collected: {status['collection']['spans_collected']}")
    
    # Check monitored processes
    processes = requests.get(f"{agent_url}/processes").json()
    monitored = [p for p in processes['processes'] if p['status'] == 'monitored']
    print(f"Monitored Processes: {len(monitored)}")
    
    for proc in monitored[:5]:  # Show first 5
        print(f"  PID {proc['pid']}: {proc['name']} ({proc['spans_collected']} spans)")
    
    # Monitor span rate
    print("\nMonitoring span collection rate...")
    prev_count = status['collection']['spans_collected']
    time.sleep(10)
    
    new_status = requests.get(f"{agent_url}/status").json()
    new_count = new_status['collection']['spans_collected']
    rate = (new_count - prev_count) / 10
    
    print(f"Span Rate: {rate:.2f} spans/second")
    
    if rate == 0:
        print("WARNING: No spans being collected!")
        print("Check:")
        print("- eBPF programs are loaded")
        print("- Processes are being monitored")
        print("- Network activity is occurring")

if __name__ == "__main__":
    debug_span_collection()
```

#### Network Communication Issues

**Debug Server Communication:**
```bash
# Test server connectivity
curl -v http://localhost:7901/health

# Check agent-server communication
tcpdump -i any -n port 7901

# Monitor failed requests
curl http://localhost:7899/metrics | grep failed_requests

# Check retry queue
curl http://localhost:7899/status | jq .sender.retry_queue_size
```

### Server Debugging

#### Elasticsearch Issues

**Debug Elasticsearch Connection:**
```bash
# Check Elasticsearch health
curl http://localhost:9200/_cluster/health?pretty

# Verify indices
curl http://localhost:9200/_cat/indices/deeptrace*

# Check index mappings
curl http://localhost:9200/deeptrace-spans/_mapping?pretty

# Monitor indexing performance
curl http://localhost:9200/_cat/thread_pool/write?v
```

**Elasticsearch Troubleshooting Script:**
```python
#!/usr/bin/env python3
# debug_elasticsearch.py

import requests
import json
from datetime import datetime, timedelta

def debug_elasticsearch():
    es_url = "http://localhost:9200"
    
    try:
        # Check cluster health
        health = requests.get(f"{es_url}/_cluster/health").json()
        print(f"Cluster Status: {health['status']}")
        print(f"Active Shards: {health['active_shards']}")
        print(f"Unassigned Shards: {health['unassigned_shards']}")
        
        # Check indices
        indices = requests.get(f"{es_url}/_cat/indices/deeptrace*?format=json").json()
        print(f"\nDeepTrace Indices: {len(indices)}")
        
        for idx in indices:
            print(f"  {idx['index']}: {idx['docs.count']} docs, {idx['store.size']}")
        
        # Check recent documents
        query = {
            "query": {
                "range": {
                    "timestamp": {
                        "gte": "now-1h"
                    }
                }
            },
            "size": 0
        }
        
        result = requests.get(
            f"{es_url}/deeptrace-spans/_search",
            json=query
        ).json()
        
        recent_docs = result['hits']['total']['value']
        print(f"\nRecent Documents (1h): {recent_docs}")
        
        if recent_docs == 0:
            print("WARNING: No recent documents found!")
            print("Check:")
            print("- Agent is sending data")
            print("- Index template is correct")
            print("- No indexing errors")
            
    except Exception as e:
        print(f"ERROR: Cannot connect to Elasticsearch: {e}")
        print("Check:")
        print("- Elasticsearch is running")
        print("- Network connectivity")
        print("- Authentication credentials")

if __name__ == "__main__":
    debug_elasticsearch()
```

#### Correlation Engine Issues

**Debug Correlation Process:**
```bash
# Check correlation status
curl http://localhost:7901/status | jq .correlation

# Monitor correlation jobs
curl http://localhost:7901/correlation/jobs

# Check algorithm performance
curl http://localhost:7901/analytics/services | jq '.services[] | {name, request_count, error_rate}'
```

**Correlation Debugging:**
```python
#!/usr/bin/env python3
# debug_correlation.py

import requests
import json
from datetime import datetime, timedelta

def debug_correlation():
    server_url = "http://localhost:7901"
    
    # Get server status
    status = requests.get(f"{server_url}/status").json()
    correlation = status['correlation']
    
    print(f"Correlation Algorithm: {correlation['algorithm']}")
    print(f"Spans Processed: {correlation['spans_processed']}")
    print(f"Traces Generated: {correlation['traces_generated']}")
    print(f"Correlation Rate: {correlation['correlation_rate']:.2f}%")
    
    # Check for recent traces
    traces = requests.get(f"{server_url}/traces?limit=10").json()
    print(f"\nRecent Traces: {len(traces['traces'])}")
    
    if len(traces['traces']) == 0:
        print("WARNING: No traces found!")
        print("Check:")
        print("- Spans are being received")
        print("- Correlation is running")
        print("- Algorithm parameters")
    
    # Analyze trace quality
    for trace in traces['traces'][:3]:
        print(f"\nTrace {trace['trace_id']}:")
        print(f"  Spans: {trace['span_count']}")
        print(f"  Services: {trace['service_count']}")
        print(f"  Duration: {trace['duration']}ms")
        print(f"  Has Errors: {trace['has_errors']}")

if __name__ == "__main__":
    debug_correlation()
```

## Advanced Debugging Techniques

### Performance Profiling

#### CPU Profiling
```bash
# Profile agent CPU usage
perf record -g -p $(pgrep deeptrace-agent) -- sleep 30
perf report

# Profile server CPU usage
perf record -g -p $(pgrep deeptrace-server) -- sleep 30
perf report

# Generate flame graphs
git clone https://github.com/brendangregg/FlameGraph
perf record -g -p $(pgrep deeptrace-agent) -- sleep 30
perf script | ./FlameGraph/stackcollapse-perf.pl | ./FlameGraph/flamegraph.pl > agent-flamegraph.svg
```

#### Memory Profiling
```bash
# Profile memory usage with Valgrind
valgrind --tool=massif --massif-out-file=agent.massif ./deeptrace-agent --config agent.toml
ms_print agent.massif > agent-memory-profile.txt

# Monitor memory usage over time
while true; do
    echo "$(date): $(ps -p $(pgrep deeptrace-agent) -o rss= | awk '{print $1/1024 " MB"}')"
    sleep 60
done
```

#### Network Profiling
```bash
# Monitor network traffic
sudo tcpdump -i any -w deeptrace-traffic.pcap host localhost and port 7901

# Analyze with Wireshark
wireshark deeptrace-traffic.pcap

# Monitor bandwidth usage
iftop -i any -f "port 7901"
```

### eBPF Debugging

#### BPF Program Analysis
```bash
# Dump loaded programs
sudo bpftool prog dump xlated id $(sudo bpftool prog list | grep deeptrace | awk '{print $1}' | head -1)

# Show program statistics
sudo bpftool prog show --json | jq '.[] | select(.name | contains("deeptrace"))'

# Monitor map usage
sudo bpftool map show --json | jq '.[] | select(.name | contains("deeptrace"))'
```

#### Custom eBPF Debugging
```c
// debug_ebpf.c - Custom debugging program
#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>

struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 256 * 1024);
} debug_events SEC(".maps");

struct debug_event {
    __u64 timestamp;
    __u32 pid;
    __u32 event_type;
    char comm[16];
};

SEC("kprobe/tcp_sendmsg")
int debug_tcp_sendmsg(struct pt_regs *ctx) {
    struct debug_event *event;
    
    event = bpf_ringbuf_reserve(&debug_events, sizeof(*event), 0);
    if (!event)
        return 0;
    
    event->timestamp = bpf_ktime_get_ns();
    event->pid = bpf_get_current_pid_tgid() >> 32;
    event->event_type = 1; // TCP_SENDMSG
    bpf_get_current_comm(&event->comm, sizeof(event->comm));
    
    bpf_ringbuf_submit(event, 0);
    return 0;
}

char LICENSE[] SEC("license") = "GPL";
```

### Log Analysis Tools

#### Structured Log Analysis
```python
#!/usr/bin/env python3
# analyze_logs.py

import json
import sys
from collections import defaultdict, Counter
from datetime import datetime

def analyze_logs(log_file):
    errors = []
    warnings = []
    events = defaultdict(int)
    
    with open(log_file, 'r') as f:
        for line in f:
            try:
                log_entry = json.loads(line.strip())
                level = log_entry.get('level', '').upper()
                message = log_entry.get('message', '')
                timestamp = log_entry.get('timestamp', '')
                
                if level == 'ERROR':
                    errors.append((timestamp, message))
                elif level == 'WARN':
                    warnings.append((timestamp, message))
                
                # Count events by type
                if 'event_type' in log_entry:
                    events[log_entry['event_type']] += 1
                    
            except json.JSONDecodeError:
                continue
    
    print(f"Log Analysis Results:")
    print(f"Errors: {len(errors)}")
    print(f"Warnings: {len(warnings)}")
    print(f"Event Types: {dict(events)}")
    
    if errors:
        print("\nRecent Errors:")
        for timestamp, message in errors[-5:]:
            print(f"  {timestamp}: {message}")
    
    if warnings:
        print("\nRecent Warnings:")
        for timestamp, message in warnings[-5:]:
            print(f"  {timestamp}: {message}")

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python3 analyze_logs.py <log_file>")
        sys.exit(1)
    
    analyze_logs(sys.argv[1])
```

#### Real-time Log Monitoring
```bash
#!/bin/bash
# monitor_logs.sh

LOG_FILE="/var/log/deeptrace/agent.log"
ERROR_COUNT=0
WARNING_COUNT=0

tail -f "$LOG_FILE" | while read line; do
    if echo "$line" | grep -q '"level":"ERROR"'; then
        ERROR_COUNT=$((ERROR_COUNT + 1))
        echo "ERROR [$ERROR_COUNT]: $line" | jq -r '.message'
        
        # Alert on high error rate
        if [ $ERROR_COUNT -gt 10 ]; then
            echo "ALERT: High error rate detected!"
            # Send notification
            curl -X POST "$SLACK_WEBHOOK" -d "{\"text\":\"DeepTrace high error rate: $ERROR_COUNT errors\"}"
        fi
    elif echo "$line" | grep -q '"level":"WARN"'; then
        WARNING_COUNT=$((WARNING_COUNT + 1))
        echo "WARNING [$WARNING_COUNT]: $line" | jq -r '.message'
    fi
done
```

## Debugging Checklists

### Agent Not Collecting Spans

- [ ] eBPF programs loaded successfully
- [ ] Processes are being monitored
- [ ] Network activity is occurring
- [ ] Ring buffer is not full
- [ ] Process filters are correct
- [ ] Protocol filters are appropriate
- [ ] Sufficient privileges (CAP_BPF or root)
- [ ] Kernel version compatibility

### Server Not Receiving Spans

- [ ] Agent can connect to server
- [ ] Server is listening on correct port
- [ ] Network connectivity between agent and server
- [ ] Authentication is configured correctly
- [ ] Server has sufficient resources
- [ ] Elasticsearch is accessible
- [ ] No firewall blocking traffic

### Correlation Not Working

- [ ] Spans are being received by server
- [ ] Correlation algorithm is running
- [ ] Algorithm parameters are appropriate
- [ ] Sufficient spans for correlation
- [ ] Time synchronization between hosts
- [ ] Elasticsearch indices are healthy
- [ ] No correlation engine errors

### Poor Performance

- [ ] Resource usage within limits
- [ ] eBPF programs are optimized
- [ ] Batch sizes are appropriate
- [ ] Network latency is acceptable
- [ ] Elasticsearch is tuned properly
- [ ] Sampling is configured if needed
- [ ] No memory leaks detected

## Emergency Procedures

### Service Recovery

```bash
#!/bin/bash
# emergency_recovery.sh

echo "DeepTrace Emergency Recovery"
echo "=========================="

# Stop services
echo "Stopping services..."
systemctl stop deeptrace-agent
systemctl stop deeptrace-server

# Clear problematic state
echo "Clearing state..."
rm -f /tmp/deeptrace-agent.pid
rm -f /tmp/deeptrace-server.pid
rm -rf /tmp/deeptrace-buffers/*

# Reset eBPF state
echo "Resetting eBPF state..."
for prog in $(bpftool prog list | grep deeptrace | awk '{print $1}'); do
    bpftool prog detach id $prog
done

# Restart with minimal configuration
echo "Starting with minimal config..."
cp /etc/deeptrace/agent.toml /etc/deeptrace/agent.toml.backup
cp /etc/deeptrace/minimal-agent.toml /etc/deeptrace/agent.toml

systemctl start deeptrace-server
sleep 5
systemctl start deeptrace-agent

echo "Recovery complete. Check status with:"
echo "  systemctl status deeptrace-agent"
echo "  systemctl status deeptrace-server"
```

### Data Recovery

```bash
#!/bin/bash
# recover_data.sh

BACKUP_DIR="/backup/deeptrace"
ES_URL="http://localhost:9200"

echo "DeepTrace Data Recovery"
echo "====================="

# Check Elasticsearch status
if ! curl -s "$ES_URL/_cluster/health" > /dev/null; then
    echo "ERROR: Elasticsearch not accessible"
    exit 1
fi

# List available backups
echo "Available backups:"
ls -la "$BACKUP_DIR"

read -p "Enter backup date (YYYY-MM-DD): " BACKUP_DATE

if [ -f "$BACKUP_DIR/deeptrace-$BACKUP_DATE.json" ]; then
    echo "Restoring data from $BACKUP_DATE..."
    
    # Restore indices
    curl -X POST "$ES_URL/_bulk" \
        -H "Content-Type: application/json" \
        --data-binary "@$BACKUP_DIR/deeptrace-$BACKUP_DATE.json"
    
    echo "Data recovery complete"
else
    echo "ERROR: Backup file not found"
    exit 1
fi
```

This debugging guide provides comprehensive tools and procedures for diagnosing and resolving DeepTrace issues. Use it systematically to identify root causes and implement effective solutions.
