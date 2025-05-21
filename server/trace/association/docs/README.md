# Code Structure

## Source Code (`src/`)
Contains various span correlation algorithms:
1. `fifo/`  
   First-In-First-Out algorithm that correlates incoming and outgoing spans within each component
2. `vpath/`  
   *(TODO - Algorithm under development)*
3. `TraceWeaver/`  
   *(TODO - Algorithm under development)*

## Test Code (`test/`)
Contains test scripts for evaluating the accuracy of different correlation algorithms

# Testing

## Prerequisites
- Ensure you have executed the [database script](../../../database/test/database.py) to populate test spans in the database

## Running Tests
Execute the span correlation accuracy test from the `test/` directory:
```bash
cd test
python3 rps_acc.py