# Testing Guide

This comprehensive testing guide covers all aspects of testing DeepTrace, from unit tests to end-to-end integration testing. Proper testing ensures reliability, performance, and correctness of the distributed tracing system.

## Testing Philosophy

DeepTrace's testing strategy is built on several key principles:

### 1. Multi-Layer Testing
- **Unit Tests**: Individual component functionality
- **Integration Tests**: Component interaction testing
- **System Tests**: End-to-end workflow validation
- **Performance Tests**: Scalability and overhead measurement

### 2. Realistic Test Environments
- **Production-Like Setup**: Mirror production configurations
- **Real Workloads**: Use actual microservices applications
- **Network Conditions**: Test under various network scenarios
- **Load Patterns**: Validate under different traffic patterns

### 3. Automated Testing
- **Continuous Integration**: Automated test execution
- **Regression Testing**: Prevent functionality breakage
- **Performance Regression**: Monitor performance changes
- **Compatibility Testing**: Ensure cross-platform compatibility

## Test Environment Setup

### 1. Development Environment

#### Prerequisites
```bash
# Install testing dependencies
sudo apt-get install -y \
  docker-compose \
  python3-pytest \
  python3-requests \
  curl \
  jq

# Install Rust testing tools
cargo install cargo-nextest
cargo install cargo-tarpaulin  # For coverage
```

#### Test Infrastructure
```bash
# Clone test repository
git clone https://github.com/DeepShield-AI/DeepTrace.git
cd DeepTrace

# Set up test environment
make test-setup

# Start test infrastructure
docker-compose -f tests/docker-compose.test.yml up -d
```

### 2. Test Data Generation

#### Sample Applications
```bash
# Deploy test microservices
cd tests/workload/bookinfo
docker-compose up -d

# Deploy social network application
cd tests/workload/socialnetwork
docker-compose up -d

# Verify applications are running
curl http://localhost:9080/productpage
curl http://localhost:8080/wrk2-api/home-timeline/read
```

#### Traffic Generation
```bash
# Generate HTTP traffic
cd tests/traffic-generators
./generate-http-traffic.sh --duration 300 --rps 100

# Generate database traffic
./generate-db-traffic.sh --connections 10 --duration 300

# Generate mixed workload
./generate-mixed-workload.sh --profile production
```

## Unit Testing

### 1. eBPF Program Testing

#### Test Structure
```rust
// tests/ebpf/test_hooks.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_syscall_filtering() {
        let pid = 1234u32;
        let result = should_monitor_pid(pid);
        assert_eq!(result, true);
    }
    
    #[test]
    fn test_quintuple_extraction() {
        let mock_socket = create_mock_tcp_socket();
        let mut quintuple = Quintuple::default();
        
        let result = extract_quintuple_from_socket(&mock_socket, &mut quintuple);
        
        assert_eq!(result, 0);
        assert_eq!(quintuple.src_addr, 0x7f000001); // 127.0.0.1
        assert_eq!(quintuple.dst_addr, 0x7f000001);
        assert_eq!(quintuple.src_port, 8080);
        assert_eq!(quintuple.dst_port, 80);
    }
}
```

#### Running eBPF Tests
```bash
# Run eBPF unit tests
cd crates/ebpf-common
cargo test

# Run with coverage
cargo tarpaulin --out Html --output-dir coverage/

# Test specific modules
cargo test test_hooks
cargo test test_maps
cargo test test_structures
```

### 2. Agent Component Testing

#### Span Processing Tests
```rust
// tests/agent/test_span_processor.rs
#[tokio::test]
async fn test_span_creation() {
    let processor = SpanProcessor::new();
    let raw_data = create_test_syscall_data();
    
    let span = processor.process_syscall_data(raw_data).await;
    
    assert!(span.is_ok());
    let span = span.unwrap();
    assert_eq!(span.operation_name, "GET /api/users");
    assert!(span.duration_ms > 0);
}

#[tokio::test]
async fn test_protocol_detection() {
    let detector = ProtocolDetector::new();
    let http_payload = b"GET /api/users HTTP/1.1\r\nHost: example.com\r\n\r\n";
    
    let protocol = detector.detect_protocol(http_payload);
    
    assert_eq!(protocol, Protocol::Http);
}
```

#### Configuration Tests
```rust
// tests/agent/test_config.rs
#[test]
fn test_config_loading() {
    let config_path = "tests/fixtures/test-config.toml";
    let config = AgentConfig::from_file(config_path);
    
    assert!(config.is_ok());
    let config = config.unwrap();
    assert_eq!(config.server.port, 7901);
    assert_eq!(config.trace.batch_size, 1024);
}

#[test]
fn test_config_validation() {
    let mut config = AgentConfig::default();
    config.server.ip = "invalid-ip".to_string();
    
    let result = config.validate();
    assert!(result.is_err());
}
```

### 3. Server Component Testing

#### Correlation Algorithm Tests
```rust
// tests/server/test_correlation.rs
#[tokio::test]
async fn test_deeptrace_correlation() {
    let correlator = DeepTraceCorrelator::new();
    let spans = create_test_span_set();
    
    let correlations = correlator.correlate_spans(spans).await;
    
    assert!(correlations.is_ok());
    let correlations = correlations.unwrap();
    assert_eq!(correlations.len(), 3); // Expected parent-child relationships
}

#[tokio::test]
async fn test_transaction_inference() {
    let inferrer = TransactionInferrer::new();
    let spans = create_transaction_spans();
    
    let transactions = inferrer.infer_transactions(spans).await;
    
    assert_eq!(transactions.len(), 1);
    assert_eq!(transactions[0].spans.len(), 4);
}
```

## Integration Testing

### 1. End-to-End Workflow Tests

#### Complete Tracing Pipeline
```python
# tests/integration/test_e2e_tracing.py
import pytest
import requests
import time
from elasticsearch import Elasticsearch

class TestE2ETracing:
    def setup_method(self):
        self.es = Elasticsearch([{'host': 'localhost', 'port': 9200}])
        self.agent_url = "http://localhost:7899"
        self.server_url = "http://localhost:7901"
    
    def test_complete_tracing_workflow(self):
        # 1. Start agent
        response = requests.post(f"{self.server_url}/agent/start")
        assert response.status_code == 200
        
        # 2. Generate test traffic
        self.generate_test_requests()
        
        # 3. Wait for span collection
        time.sleep(10)
        
        # 4. Trigger correlation
        response = requests.post(f"{self.server_url}/correlate")
        assert response.status_code == 200
        
        # 5. Trigger assembly
        response = requests.post(f"{self.server_url}/assemble")
        assert response.status_code == 200
        
        # 6. Verify traces in Elasticsearch
        traces = self.es.search(index="traces-*", body={"query": {"match_all": {}}})
        assert traces['hits']['total']['value'] > 0
    
    def generate_test_requests(self):
        # Generate HTTP requests to test application
        for i in range(10):
            requests.get("http://localhost:9080/productpage")
            time.sleep(0.1)
```

#### Multi-Service Communication
```python
# tests/integration/test_multi_service.py
def test_microservices_tracing(self):
    # Test complex microservices interaction
    services = [
        "http://localhost:9080/productpage",
        "http://localhost:9080/api/v1/products/1",
        "http://localhost:9080/api/v1/reviews/1"
    ]
    
    # Generate cross-service requests
    for service in services:
        response = requests.get(service)
        assert response.status_code == 200
    
    # Wait for trace assembly
    time.sleep(15)
    
    # Verify complete traces
    traces = self.query_traces_by_operation("GET /productpage")
    assert len(traces) > 0
    
    # Verify trace completeness
    for trace in traces:
        assert len(trace['spans']) >= 3  # At least 3 services involved
        assert self.verify_trace_structure(trace)
```

### 2. Performance Integration Tests

#### Overhead Measurement
```python
# tests/integration/test_performance.py
import psutil
import time

class TestPerformanceImpact:
    def test_cpu_overhead(self):
        # Measure baseline CPU usage
        baseline_cpu = self.measure_cpu_usage(duration=30)
        
        # Start DeepTrace agent
        self.start_agent()
        
        # Measure CPU usage with tracing
        tracing_cpu = self.measure_cpu_usage(duration=30)
        
        # Calculate overhead
        overhead = ((tracing_cpu - baseline_cpu) / baseline_cpu) * 100
        
        # Assert overhead is within acceptable limits
        assert overhead < 5.0, f"CPU overhead {overhead}% exceeds 5% limit"
    
    def test_memory_overhead(self):
        baseline_memory = self.measure_memory_usage()
        self.start_agent()
        tracing_memory = self.measure_memory_usage()
        
        overhead_mb = (tracing_memory - baseline_memory) / (1024 * 1024)
        assert overhead_mb < 100, f"Memory overhead {overhead_mb}MB exceeds 100MB limit"
    
    def test_latency_impact(self):
        # Measure baseline latency
        baseline_latency = self.measure_request_latency(requests=1000)
        
        self.start_agent()
        
        # Measure latency with tracing
        tracing_latency = self.measure_request_latency(requests=1000)
        
        # Calculate impact
        impact = ((tracing_latency - baseline_latency) / baseline_latency) * 100
        assert impact < 10.0, f"Latency impact {impact}% exceeds 10% limit"
```

## System Testing

### 1. Scalability Testing

#### High-Throughput Testing
```bash
#!/bin/bash
# tests/system/test_scalability.sh

# Test configuration
MAX_RPS=10000
DURATION=300
AGENTS=10

echo "Starting scalability test: ${MAX_RPS} RPS for ${DURATION}s with ${AGENTS} agents"

# Deploy multiple agents
for i in $(seq 1 $AGENTS); do
    docker run -d --name agent-$i \
        -v /sys/kernel/debug:/sys/kernel/debug:ro \
        --privileged \
        deeptrace-agent:latest
done

# Generate high-throughput traffic
wrk -t12 -c400 -d${DURATION}s -R${MAX_RPS} \
    --script=tests/scripts/complex-workload.lua \
    http://localhost:9080/

# Monitor system metrics
./monitor-performance.sh &
MONITOR_PID=$!

# Wait for test completion
sleep $DURATION

# Stop monitoring
kill $MONITOR_PID

# Collect results
./collect-test-results.sh
```

#### Multi-Host Testing
```yaml
# tests/system/docker-compose.multihost.yml
version: '3.8'
services:
  # Simulate multiple hosts with separate networks
  host1:
    image: deeptrace-test-host:latest
    networks:
      - host1-net
    environment:
      - HOST_ID=1
  
  host2:
    image: deeptrace-test-host:latest
    networks:
      - host2-net
    environment:
      - HOST_ID=2
  
  deeptrace-server:
    image: deeptrace-server:latest
    networks:
      - host1-net
      - host2-net
      - server-net
    ports:
      - "7901:7901"

networks:
  host1-net:
  host2-net:
  server-net:
```

### 2. Reliability Testing

#### Fault Injection Testing
```python
# tests/system/test_fault_tolerance.py
class TestFaultTolerance:
    def test_agent_failure_recovery(self):
        # Start normal operation
        self.start_tracing_system()
        self.generate_baseline_traffic()
        
        # Inject agent failure
        self.kill_agent()
        
        # Continue traffic generation
        self.generate_traffic_during_failure()
        
        # Restart agent
        self.restart_agent()
        
        # Verify recovery
        self.verify_trace_continuity()
    
    def test_network_partition(self):
        # Simulate network partition between agent and server
        self.create_network_partition()
        
        # Verify agent buffers data locally
        self.verify_local_buffering()
        
        # Restore network
        self.restore_network()
        
        # Verify data synchronization
        self.verify_data_sync()
    
    def test_elasticsearch_failure(self):
        # Stop Elasticsearch
        self.stop_elasticsearch()
        
        # Verify graceful degradation
        self.verify_graceful_degradation()
        
        # Restart Elasticsearch
        self.start_elasticsearch()
        
        # Verify recovery
        self.verify_system_recovery()
```

## Performance Testing

### 1. Benchmark Suite

#### System Call Overhead Benchmarks
```rust
// benches/syscall_overhead.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_syscall_overhead(c: &mut Criterion) {
    c.bench_function("read_syscall_baseline", |b| {
        b.iter(|| {
            // Baseline read syscall without eBPF
            unsafe {
                libc::read(black_box(0), black_box(std::ptr::null_mut()), black_box(0))
            }
        })
    });
    
    c.bench_function("read_syscall_with_ebpf", |b| {
        // Load eBPF program
        let _program = load_ebpf_program();
        
        b.iter(|| {
            // Read syscall with eBPF monitoring
            unsafe {
                libc::read(black_box(0), black_box(std::ptr::null_mut()), black_box(0))
            }
        })
    });
}

criterion_group!(benches, benchmark_syscall_overhead);
criterion_main!(benches);
```

#### Correlation Algorithm Benchmarks
```rust
// benches/correlation.rs
fn benchmark_correlation_algorithms(c: &mut Criterion) {
    let spans = generate_test_spans(10000);
    
    c.bench_function("deeptrace_correlation", |b| {
        let correlator = DeepTraceCorrelator::new();
        b.iter(|| {
            correlator.correlate_spans(black_box(spans.clone()))
        })
    });
    
    c.bench_function("fifo_correlation", |b| {
        let correlator = FifoCorrelator::new();
        b.iter(|| {
            correlator.correlate_spans(black_box(spans.clone()))
        })
    });
}
```

### 2. Load Testing

#### Stress Test Configuration
```yaml
# tests/load/stress-test.yml
scenarios:
  - name: "high_throughput"
    duration: "10m"
    target_rps: 5000
    services:
      - "productpage"
      - "reviews"
      - "ratings"
  
  - name: "burst_traffic"
    duration: "5m"
    pattern: "burst"
    peak_rps: 10000
    base_rps: 1000
  
  - name: "sustained_load"
    duration: "1h"
    target_rps: 2000
    
monitoring:
  metrics:
    - cpu_usage
    - memory_usage
    - network_io
    - disk_io
    - trace_completeness
    - correlation_accuracy
```

## Automated Testing Pipeline

### 1. Continuous Integration

#### GitHub Actions Workflow
```yaml
# .github/workflows/test.yml
name: DeepTrace Tests

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  unit-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          components: rustfmt, clippy
      
      - name: Run unit tests
        run: |
          cargo test --all-features
          cargo clippy -- -D warnings
          cargo fmt --all -- --check
  
  integration-tests:
    runs-on: ubuntu-latest
    needs: unit-tests
    steps:
      - uses: actions/checkout@v3
      
      - name: Setup test environment
        run: |
          sudo apt-get update
          sudo apt-get install -y docker-compose
          make test-setup
      
      - name: Run integration tests
        run: |
          docker-compose -f tests/docker-compose.test.yml up -d
          pytest tests/integration/ -v
          docker-compose -f tests/docker-compose.test.yml down
  
  performance-tests:
    runs-on: ubuntu-latest
    needs: integration-tests
    if: github.event_name == 'push' && github.ref == 'refs/heads/main'
    steps:
      - uses: actions/checkout@v3
      
      - name: Run performance benchmarks
        run: |
          cargo bench
          python tests/performance/run_benchmarks.py
      
      - name: Upload performance results
        uses: actions/upload-artifact@v3
        with:
          name: performance-results
          path: target/criterion/
```

### 2. Test Reporting

#### Coverage Reports
```bash
#!/bin/bash
# scripts/generate-coverage.sh

# Generate Rust coverage
cargo tarpaulin --out Html --output-dir coverage/rust/

# Generate Python coverage
pytest --cov=tests --cov-report=html:coverage/python/ tests/

# Generate combined report
./scripts/combine-coverage-reports.py
```

#### Performance Regression Detection
```python
# scripts/check_performance_regression.py
import json
import sys

def check_regression(baseline_file, current_file, threshold=0.05):
    with open(baseline_file) as f:
        baseline = json.load(f)
    
    with open(current_file) as f:
        current = json.load(f)
    
    regressions = []
    
    for metric in baseline:
        if metric in current:
            baseline_value = baseline[metric]
            current_value = current[metric]
            
            if current_value > baseline_value * (1 + threshold):
                regression = {
                    'metric': metric,
                    'baseline': baseline_value,
                    'current': current_value,
                    'regression': (current_value - baseline_value) / baseline_value
                }
                regressions.append(regression)
    
    if regressions:
        print("Performance regressions detected:")
        for r in regressions:
            print(f"  {r['metric']}: {r['regression']:.2%} increase")
        sys.exit(1)
    else:
        print("No performance regressions detected")

if __name__ == "__main__":
    check_regression("baseline_performance.json", "current_performance.json")
```

## Test Data Management

### 1. Test Data Generation

#### Synthetic Trace Generation
```python
# tests/data/generate_synthetic_traces.py
class SyntheticTraceGenerator:
    def __init__(self):
        self.services = ["frontend", "auth", "user-service", "payment", "inventory"]
        self.operations = ["GET", "POST", "PUT", "DELETE"]
    
    def generate_trace(self, complexity="medium"):
        if complexity == "simple":
            return self.generate_simple_trace()
        elif complexity == "medium":
            return self.generate_medium_trace()
        else:
            return self.generate_complex_trace()
    
    def generate_simple_trace(self):
        # Generate 2-3 span trace
        spans = []
        root_span = self.create_span("frontend", "GET /api/users", is_root=True)
        spans.append(root_span)
        
        child_span = self.create_span("user-service", "SELECT users", parent=root_span)
        spans.append(child_span)
        
        return {"trace_id": root_span["trace_id"], "spans": spans}
```

### 2. Test Environment Cleanup

#### Automated Cleanup
```bash
#!/bin/bash
# scripts/cleanup-test-env.sh

echo "Cleaning up test environment..."

# Stop all test containers
docker stop $(docker ps -q --filter "label=test=deeptrace") 2>/dev/null || true

# Remove test containers
docker rm $(docker ps -aq --filter "label=test=deeptrace") 2>/dev/null || true

# Clean test data from Elasticsearch
curl -X DELETE "localhost:9200/test-*" 2>/dev/null || true

# Remove test networks
docker network prune -f

# Clean temporary files
rm -rf /tmp/deeptrace-test-*

echo "Test environment cleanup completed"
```

---

This comprehensive testing guide ensures DeepTrace maintains high quality, performance, and reliability across all components and deployment scenarios. Regular execution of these tests helps catch issues early and maintains system integrity.
