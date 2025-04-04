# Maps Documentation for DeepTrace

DeepTrace uses several eBPF maps (implemented in `agent/src/ebpf/src/maps.rs`) to manage and propagate context across different system components efficiently. These maps are crucial for maintaining state, managing data, and ensuring that context is propagated correctly through network events and system calls. Below is a detailed description of each map used in the DeepTrace implementation.

## Overview of Maps

DeepTrace employs a variety of maps, each serving specific roles in 

- Storing kernel information and communication
- Context propagation
- Metrics collection.
- Not exceeding the limit of 512 bytes of memory space.

### `PIDS`

Filters system call hooks based on process identifiers generated in user space, ensuring that only relevant processes are monitored. This is engineer implementation.

- **Type**: HashMap
- **Key**: `u32`
- **Value**: `u32`
- **Max Entries**: 256

### `TASK_STRUCT`

The size of task_struct structure is `13696`, so it is assigned as PerCpuArray.

- **Type**: PerCpuArray<task_struct>
- **Max Entries**: 1

### `FILES_STRUCT`

The size of files_struct structure is `704`, so it is assigned as PerCpuArray.

- **Type**: PerCpuArray<files_struct>
- **Max Entries**: 1

### `FILE`

The size of file structure is `232`, and it is allocated as percguarray to save stack space

- **Type**: PerCpuArray<file>
- **Max Entries**: 1

### `TCP_SOCK`

The size of tcp_sock structure is `2304`, so it is assigned as PerCpuArray.

- **Type**: PerCpuArray<tcp_sock>
- **Max Entries**: 1

### `INGRESS`

Stores arguments from ingress system calls, facilitating efficient data retrieval during system call exits. It maps a `tgid|pid` to `Args` which includes references to relevant TCP sockets structure.

- **Type**: HashMap
- **Key**: `u64` (`tgid|pid`)
- **Value**: `Args`
- **Max Entries**: 10240

### `EGRESS`

Similar to INGRESS, but saves the arguments of egress.

- **Type**: HashMap
- **Key**: `u64` (`tgid|pid`)
- **Value**: `Args`
- **Max Entries**: 10240

### `Message`

Utilized for sending data to user space. It facilitates efficient data transfer between kernel space and user space, crucial for real-time monitoring.

- **Type**: RingBuf
- **Max Entries**: sizeof::<Data>() * 1 << 12 ()