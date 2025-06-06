# Trace Assembler Utility

This module provides functions to assemble distributed tracing spans into trace groups based on their `span_id` and `parent_id` relationships.

## Functions

### `assemble_trace(spans)`

Groups a list of span objects into traces according to their parent-child relationships.

**Parameters:**
- `spans`: A list of span objects. Each span should have at least the following attributes:
  - `span_id`
  - `parent_id`
  - `trace_id`
  - `component_name`
  - `endpoint`
  - `start_time`
  - `end_time`
  - `tgid`
  - `pid`
  - `protocol`
  - `direction`
  - `duration`
  - `src_ip`
  - `src_port`
  - `dst_ip`
  - `dst_port`

**Returns:**
- A list of trace dictionaries. Each dictionary contains:
  - `trace_id`: The trace ID.
  - `span_num`: Number of spans in the trace.
  - `spans`: List of span information dictionaries belonging to the trace.

### `search_span(span, paret_childs)`

Performs a breadth-first search to collect all related spans (children and parent) starting from a given span.

### `check_finish(spans, visited)`

Checks if all spans with a parent have been visited.

## Description

- The assembler builds a parent-child mapping for all spans.
- It then traverses the mapping to group spans into traces, ensuring each span is only included once.
- The result is a list of traces, each containing all related spans.

