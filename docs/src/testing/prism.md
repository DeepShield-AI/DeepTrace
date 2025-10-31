# Prism Testing Guide

Prism includes a comprehensive testing framework designed to ensure reliability, accuracy, and performance of the metric collection system. The testing infrastructure consists of multiple layers, from unit tests for individual components to sophisticated integration tests that validate end-to-end functionality.

## Testing Philosophy

Our testing approach is built on several key principles:

### Accuracy First
Every metric collected by Prism must be accurate and verifiable. Our tests generate known data and verify that the parsing and processing logic produces exactly the expected results.

### Comprehensive Coverage
Testing covers all metric collection modules, data processing pipelines, and output formats to ensure no component is left unvalidated.

## Test Categories

### Unit Tests
Individual component testing for core functionality:
- Metric parsing logic
- Data structure operations
- Configuration management

### Integration Tests
End-to-end testing with realistic system data:
- Complete metric collection workflows
- Multi-module interaction testing
- Output format validation

## Running Tests

### Quick Test Run
```bash
cd agent
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test -p observ-cpu
```

### Comprehensive Testing
```bash
# Run integration tests
cargo run --bin procfs_integration_tests -- --count 10 --verbose

# Run with custom configuration
cargo run --bin procfs_integration_tests -- --count 5 --output test-results
```

## Integration Tests

Prism's integration testing framework is a sophisticated system designed to validate the complete metric collection pipeline from data generation to final output. The integration tests ensure that all components work together correctly and that the system produces accurate results under various conditions.

### Overview

The integration testing framework, located in `tests/procfs-integration-tests/`, provides comprehensive end-to-end validation of Prism's metric collection capabilities. It generates realistic procfs data, processes it through Prism's collection modules, and validates that the results match expected values with perfect accuracy.

### Architecture

#### Test Framework Components

```
Integration Test Framework
├── Data Generation
│   ├── Random procfs file generation
│   └── Realistic system resource simulation
├── Metric Collection
│   ├── Prism module invocation
│   ├── Environment isolation
│   └── Process separation
└── Validation
    ├── Field-by-field verification
    ├── Unit conversion validation
    └── Performance measurement
```

#### Key Features

- **Random Data Generation**: Creates realistic but controlled test data
- **Process Isolation**: Each test runs in a separate process to avoid conflicts
- **Comprehensive Validation**: Verifies every collected metric field
- **Performance Monitoring**: Tracks collection performance and overhead

### Test Data Generation

#### Supported Metrics

The framework generates test data for all major system metrics:

##### CPU Metrics (`/proc/stat`)
- Random CPU core count (1-16 cores)
- Context switch statistics
- Process and thread counts
- Boot time and system uptime

##### Memory Metrics (`/proc/meminfo`)
- Total memory size
- Realistic memory usage patterns
- Cache and buffer allocations
- Swap space configuration
- Active/inactive memory distribution

##### Virtual Memory Statistics (`/proc/vmstat`)
- Page allocation and deallocation statistics
- Memory pressure indicators
- NUMA topology statistics
- I/O and swap activity metrics
- Slab cache utilization

##### Disk Metrics (`/proc/diskstats`)
- Multiple device types (SATA, NVMe, loop devices)
- Read/write operation statistics
- I/O timing and queue depth metrics
- Sector-level transfer statistics

##### Network Metrics (`/proc/net/dev`)
- Traffic statistics (bytes, packets)
- Error and drop counters
- Realistic usage patterns

### Test Execution

#### Command Line Interface

The integration test framework provides a modern command-line interface:

```bash
# Basic usage
cargo run --bin procfs_integration_tests

# Multiple test runs
cargo run --bin procfs_integration_tests -- --count 5

# Verbose output with detailed validation
cargo run --bin procfs_integration_tests -- --count 3 --verbose

# Custom output directory
cargo run --bin procfs_integration_tests -- --output custom-results

# Show help and version information
cargo run --bin procfs_integration_tests --help
cargo run --bin procfs_integration_tests --version
```

#### Test Process

Each integration test follows this workflow:

1. **Environment Preparation**
   - Create isolated test directory
   - Generate random procfs data
   - Set environment variables for procfs root

2. **Metric Collection**
   - Initialize Prism metric collection modules
   - Invoke collection functions (e.g., `prism_cpu::stat()`)
   - Capture all collected metrics

3. **Validation**
   - Compare collected values with generated expected values
   - Verify unit conversions and data transformations
   - Check field completeness and accuracy

4. **Result Recording**
   - Generate detailed validation reports
   - Record performance metrics

### Test Output and Reporting

#### Directory Structure

Each test run creates a timestamped session directory:

```
output/20250920-183856/
├── test-001/
│   ├── procfs/              # Generated procfs files
│   │   ├── stat
│   │   ├── meminfo
│   │   ├── vmstat
│   │   ├── diskstats
│   │   └── net/dev
├── test-002/
└── test-003/
```

#### Console Output

The test framework provides comprehensive console output:

```
Starting Prism ProcFS Random Integration Tests
==================================================
Running 3 tests
Test session directory: output/20250922-142804/

Running test 1/3
  Test directory: output/20250922-142804/test-001/
  Running prism collectors and validating results
    Validating CPU metrics
      CPU field validation successful
    Validating Memory metrics
      Memory field validation successful
    Validating VmStat metrics
      VmStat field validation successful
    Validating Disk metrics
      Disk field validation successful
    Validating Network metrics
      Network field validation successful
  Test #1 validation completed successfully
✅ Test #1 passed

Test session completed!
Results: 3 passed, 0 failed
All test results saved in: output/20250922-142804/
```

## Unit Tests

Unit tests form the foundation of Prism's testing strategy, providing focused validation of individual components, functions, and modules. These tests ensure that each piece of functionality works correctly in isolation before being integrated into the larger system.

### Testing Strategy

#### Component Isolation

Unit tests focus on testing individual components in isolation:
- **Pure Functions**: Test mathematical calculations and data transformations
- **Data Structures**: Validate custom data types and their operations
- **Parsing Logic**: Verify correct interpretation of procfs file formats

#### Test Coverage Goals

- **Functionality Coverage**: Every public function and method
- **Branch Coverage**: All conditional logic paths

### Test Organization

#### Module Structure

Unit tests are organized alongside the code they test:

```
crates/
├── prism-cpu/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── stat.rs
│   │   └── ...
│   └── tests/
|
├── prism-memory/
│   ├── src/
│   └── tests/
└── ...
```

### Running Unit Tests

#### Basic Test Execution

```bash
# Run all unit tests
cargo test --release

# Run tests for specific crate
cargo test -p prism-cpu

# Run tests with output
cargo test -- --nocapture
```

#### Advanced Test Options

```bash
# Run tests in release mode (for performance testing)
cargo test --release

# Run tests with specific number of threads
cargo test --release -- --test-threads=1

# Run ignored tests
cargo test --release -- --ignored
```

#### Test Coverage

```bash
# Install coverage tool
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html

# Coverage for specific crate
cargo tarpaulin -p prism-cpu --out Html
```

## Extending Integration Tests

### Adding New Metrics

To add support for new metric types:

1. **Generator Extension**: Add data generation logic in `generators.rs`
2. **Validator Implementation**: Create validation logic in `validators.rs`
3. **Test Integration**: Update main test loop to include new metrics
4. **Documentation**: Update test documentation and examples

### Configuration Options

Integration tests support various configuration options:
- **Test Count**: Number of test iterations to run
- **Output Directory**: Custom location for test results
- **Verbosity Level**: Control amount of output detail

This comprehensive testing approach ensures Prism's reliability, accuracy, and performance across all deployment scenarios.
