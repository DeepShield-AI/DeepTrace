
## Overview

This document provides a comprehensive guide to the processes and mechanisms employed by DeepTrace to collect information across network transactions. 
DeepTrace utilizes advanced monitoring and tracing techniques to maintain and manage the state of network communications efficiently, leveraging eBPF technology to intercept and manipulate network packets at the kernel level. 

Please follow through the workflow sections below to grasp how each part of the process integrates.

## Workflow

The workflow described herein details step-by-step how DeepTrace parses TCP options, handles system calls, and propagates context through various network components to achieve precise and reliable data tracking. The included diagram (see below) illustrates the overall workflow, aiding in the visual understanding of the context propagation mechanisms.

### Step 1: System call triggered

- **Function**: `sys_enter_{syscall name}` in file `ebpf/src/network/{syscall name}.rs`
- **Operations**:
  1. Filter pid.
  2. Extract system call parameters.
  3. Pass it to the processing program for processing.

### Step 2: Handling System Calls

- **Functions**: `try_enter` and `try_exit`
- **Operations**:
  1. When entering the system call, extract parameters and store them in the `INGRESS/EGRESS` map
  2. Collect information and pass it to user mode when exiting system calls.
  3. The currently collected information includes tgid, pid, timestamp, quintuple, enter sequence number, exit sequence number, syscall name, syscall type, buffer size and buffer information.