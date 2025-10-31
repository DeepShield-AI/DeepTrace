# Testing Guide

This comprehensive testing guide covers all aspects of testing DeepTrace and Prism, from unit tests to end-to-end integration testing. Proper testing ensures reliability, performance, and correctness of the distributed tracing system.

## Testing Components

DeepTrace includes several testable components:

- **DeepTrace Agent**: Core eBPF-based data collection
- **Prism Agent**: Lightweight observability agent
- **Server Components**: Data processing and storage
- **Protocol Inference**: Automatic protocol detection
- **Span Construction**: Distributed trace correlation

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
  docker.io \
  python3 \
  python3-pip \
  curl

# Install Python packages for testing
pip3 install requests elasticsearch pymongo redis python-binary-memcached
```

#### Test Infrastructure
```bash
# Clone repository
git clone https://github.com/DeepShield-AI/DeepTrace.git
cd DeepTrace

# No additional setup required - use provided deployment scripts
```

### 2. Test Data Generation

#### Sample Applications
```bash
# Deploy test microservices
cd tests/workload/bookinfo
sudo bash deploy.sh

# Deploy social network application
cd tests/workload/socialnetwork
bash deploy.sh

# Generate test traffic
cd tests/workload/bookinfo
sudo bash client.sh
```

## Unit Testing

### 1. eBPF Program Testing

#### Running eBPF Tests
```bash
# Run basic functionality tests
cd tests/eBPF/functionality
python3 server.py &
python3 client.py

# Run overhead tests
cd tests/eBPF/overhead
bash run.sh write
```

### 2. Agent Testing

Agent testing is performed through integration tests using actual workload applications.

### 3. Server Testing

Server component testing includes correlation algorithm validation using real trace data from workload applications.

## Integration Testing

Integration tests use real workload applications (BookInfo, Social Network) to verify end-to-end tracing functionality.

### Test Workflow

1. Deploy workload applications using provided scripts
2. Start DeepTrace agent with monitoring
3. Generate traffic using client scripts
4. Verify trace data in Elasticsearch
5. Cleanup test environment

### Performance Testing

Performance testing measures system call overhead using the scripts in `tests/eBPF/overhead/`.

## System Testing

System testing validates DeepTrace deployment and operation in production-like environments using the provided workload applications.

## Test Environment Cleanup

Cleanup test environments using the provided cleanup scripts:

```bash
# Cleanup BookInfo workload
cd tests/workload/bookinfo
sudo bash clear.sh

# Cleanup Social Network workload  
cd tests/workload/socialnetwork
bash clear.sh
```
