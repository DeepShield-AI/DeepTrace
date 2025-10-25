# observ ProcFS Integration Tests

A comprehensive testing framework that validates the accuracy of observ's metric collection modules by generating synthetic procfs data and verifying parsing correctness with fuzzy matching for floating-point precision.

## Features

- **Synthetic Data Generation**: Creates realistic procfs data with configurable value ranges
- **Multi-Module Coverage**: Tests CPU, memory, virtual memory, disk, and network metrics
- **Fuzzy Validation**: Uses relative error tolerance for floating-point comparisons
- **Process Isolation**: Independent test processes prevent static variable conflicts  
- **Timestamped Sessions**: Organized output with `YYYYMMDD-HHMMSS` directory structure
- **Configurable Testing**: Command-line control over test count and output location
- **Unit Conversion Testing**: Validates proper handling of time ticks, memory units, and disk sectors
- **Error Handling**: Robust parsing of edge cases and overflow conditions

## Project Structure

```
tests/
├── procfs-integration-tests/     # Main test binary
│   ├── src/
│   │   ├── main.rs              # Test orchestration and CLI
│   │   ├── run.rs               # Individual test execution
│   │   └── validators.rs        # Metric validation logic
│   └── Cargo.toml
├── observ-fixtures/              # Test data generation
│   ├── src/
│   │   ├── lib.rs
│   │   └── generators/
│   │       ├── mod.rs           # Generator coordinator
│   │       ├── stat.rs          # CPU data generation
│   │       ├── meminfo.rs       # Memory data generation
│   │       ├── vmstat.rs        # VmStat data generation
│   │       ├── diskstat.rs      # Disk data generation
│   │       └── netdev.rs        # Network data generation
│   └── Cargo.toml
└── output/YYYYMMDD-HHMMSS/      # Test session results
    └── test-001/
        └── procfs/              # Generated synthetic procfs
            ├── stat             # CPU statistics
            ├── meminfo          # Memory information  
            ├── vmstat           # Virtual memory stats
            ├── diskstats        # Disk I/O statistics
            └── net/dev          # Network interfaces
```

## Data Generation

The framework generates synthetic procfs data using the `fake` crate with realistic value ranges:

### CPU Statistics (`/proc/stat`)
- **Clock Ticks**: Large random values converted to seconds via system clock tick rate
- **CPU Cores**: Variable core count with individual timing statistics  
- **System Counters**: Context switches, boot time, process counts
- **Time Conversion**: Handles clock tick to second conversion with precision validation

### Memory Information (`/proc/meminfo`)  
- **Constrained Ranges**: Memory values use reasonable bounds (1-1000M KB) to prevent overflow
- **Field Mapping**: Explicit key mapping for fields like `NFS_Unstable` to handle naming mismatches
- **Unit Consistency**: All values in kilobytes with proper unit conversion testing

### Virtual Memory (`/proc/vmstat`)
- **Page Statistics**: Random page counts for different memory types
- **I/O Counters**: Page-in/page-out and swap statistics
- **NUMA Metrics**: Node-local memory access patterns

### Disk Statistics (`/proc/diskstats`)
- **Overflow Protection**: Safe sector-to-byte conversion using checked arithmetic
- **Device Variety**: Multiple device types with realistic I/O patterns
- **Timing Data**: I/O completion times and queue depths

### Network Interfaces (`/proc/net/dev`)
- **Interface Types**: Standard interfaces (lo, eth0, wlan0)
- **Traffic Counters**: RX/TX bytes, packets, errors, drops
- **Directory Creation**: Automatic creation of `/proc/net/` directory structure

## Validation Strategy

The framework uses fuzzy matching with relative error tolerance to handle floating-point precision issues:

### Fuzzy Validation Approach
- **Relative Error**: Uses `approx::relative_eq!` with configurable epsilon values
- **Unit Conversion**: Validates conversions between clock ticks, bytes, and other units
- **Precision Handling**: Accommodates rounding errors in floating-point arithmetic

### CPU Validation
- **Time Conversion**: Validates clock tick to second conversion using system `CLK_TCK`
- **Per-Core Statistics**: Checks user, nice, system, idle, iowait, irq, softirq, steal, guest times
- **System Counters**: Exact matching for context switches, boot time, process counts

### Memory Validation  
- **Field Mapping**: Handles procfs key name variations (e.g., `NFS_Unstable` vs `NfsUnstable`)
- **Unit Consistency**: Validates kilobyte values with floating-point tolerance
- **Comprehensive Coverage**: Tests all major memory fields including swap, cache, and kernel memory

### VmStat Validation
- **Page Counters**: Validates all virtual memory page statistics
- **I/O Statistics**: Checks page-in/out, swap-in/out counters
- **Memory Pressure**: Validates reclaim and compaction statistics

### Disk Validation
- **Overflow Safety**: Tests large sector values with safe arithmetic
- **Device Matching**: Validates device names, major/minor numbers
- **I/O Metrics**: Checks read/write operations, sectors, timing data

### Network Validation
- **Interface Matching**: Validates interface names and counts
- **Traffic Statistics**: Checks RX/TX bytes, packets, errors, drops
- **Unit Conversion**: Validates byte-based measurements

## Example Output

### Test Execution
```bash
$ cargo run --bin procfs_integration_tests
Starting observ ProcFS Random Integration Tests
==================================================
Running 1 test
Test session directory: output/20250921-152957/

Running test 1/1
  Test directory: output/20250921-152957/test-001/
  Running observ collectors and validating results
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
  ✅ Test #1 passed

Test session completed!
Results: 1 passed, 0 failed
```

### Generated Files
- **`procfs/stat`**: CPU time statistics with clock tick values
- **`procfs/meminfo`**: Memory information in kilobytes  
- **`procfs/vmstat`**: Virtual memory page statistics
- **`procfs/diskstats`**: Disk I/O operation counters
- **`procfs/net/dev`**: Network interface traffic statistics

## Usage

### Basic Commands

```bash
# Run single test
cargo run --bin procfs_integration_tests

# Run multiple tests  
cargo run --bin procfs_integration_tests --count 5

# Custom output directory with verbose logging
cargo run --bin procfs_integration_tests -c 3 -o results -v
```

### Command Line Options

- `-c, --count <NUMBER>`: Number of tests to run (default: 1)
- `-o, --output <DIR>`: Output directory (default: output)  
- `-v, --verbose`: Enable verbose output
- `--version`: Show version information

### Output Structure

Tests create timestamped directories: `output/YYYYMMDD-HHMMSS/test-001/procfs/`

Each test generates synthetic procfs files and validates parsing without creating `.parsed` output files.

## Implementation Details

### Test Flow
1. **Generate**: Create synthetic procfs files with realistic random data
2. **Isolate**: Set `PROCFS_ROOT` environment variable for test process  
3. **Parse**: Invoke actual observ metric collection functions
4. **Validate**: Compare parsed values against generated data using fuzzy matching
5. **Report**: Output validation results with pass/fail status

### Key Technical Solutions
- **Overflow Prevention**: Safe arithmetic for large values (sectors, clock ticks)
- **Field Mapping**: Explicit key mapping for procfs naming inconsistencies  
- **Precision Handling**: Relative error tolerance for floating-point comparisons
- **Process Isolation**: Independent test processes prevent static variable conflicts

### Dependencies
- `approx`: Fuzzy floating-point comparisons
- `fake`: Random test data generation  
- `observ-fixtures`: Synthetic procfs data generators
- `tokio`: Async runtime for metric collection
