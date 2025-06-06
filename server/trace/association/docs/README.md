# Code Structure

## Source Code (`src/`)
Contains various span correlation algorithms:
1. `fifo/`  
   First-In-First-Out algorithm that correlates incoming and outgoing spans within each component
2. `vpath/`  
3. `TraceWeaver/`  
4. `wap5`
5. `deeptrace`

## Test Code (`test/`)
Contains test scripts for evaluating the accuracy of different correlation algorithms

# Testing

## Prerequisites
- Ensure you have executed the [database script](../../../database/test/database.py) to populate test spans in the database

## Running Tests
Execute the span correlation accuracy test from the `test/` directory:
```bash
python3 -m trace.association.test.intra_test --algo deeptrace|fifo|vpath|wap5|traceweaver_v1
```