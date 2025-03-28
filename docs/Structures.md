# Structures Documentation for DeepTrace

DeepTrace employs a variety of data structures to manage complex interactions and maintain state within the system efficiently. These structures are pivotal for smanaging network flows and tracking performance metrics. Below is a detailed overview of each structure used in DeepTrace.

## Enumeration Types

Enumeration types in DeepTrace define sets of named constants, which are used to represent various states or types across the system. They are essential for readability and maintenance of the code.

### `enum SyscallName`

- **Values**:
  - `Read`
  - `RecvMsg`
  - `RecvMMsg`
  - `ReadV`
  - `RecvFrom`
  - `Write`
  - `SendMsg`
  - `SendMMsg`
  - `SendTo`
  - `WriteV`
  - `Unknown`
- **Description**: Represents the names of system calls that DeepTrace hooks into for processing. These identifiers help in mapping the intercepted system call data to appropriate handling functions. This enum type is mainly used for debugging.

### `enum SyscallType`

- **Values**:
  - `Ingress`
  - `Egress`
- **Description**: Categorizes the types of system calls into ingress and egress operations.

### `enum Buffer`

- **Values**:
  - `Normal(NormalBuffer)`
  - `Vectored(VectoredBuffer)`
  - `Msg(MsgBuffer)`
- **Description**: Used to represent different buffer types for system calls, in order to collect different information separately.

## Key Structures

### `struct Quintuple`

- **Fields**:
  - `src_addr: u32`: Source IP address
  - `dst_addr: u32`: Destination IP address
  - `src_port: u16`: Source port
  - `dst_port: u16`: Destination port
  - `skc_family: u16`: Protocol family
- **Description**: Represents the 5-tuple key used for identifying network flows. This structure is critical for mapping network packets to their corresponding context in various maps.

### `struct Args`

- **Fields**:
  - `fd: u32`: File descriptor
  - `seq: u32`: TCP sequence number when enter
  - `timestamp: u64`: Enter timestamp
  - `buffer: Buffer`: Buffer information related to system calls
- **Description**: When triggered by a system call, store information for subsequent processing.


### `struct Data`

- **Fields**:
  - `tgid: u32`: TGID that triggers system call
  - `pid: u32`: PID that triggers system call
  - `enter_seq: u32`: Enter TCP sequence number
  - `exit_seq: u32`: Exit TCP sequence number
  - `timestamp_ns: u64`: Exit timestamp
  - `len: u32`: 
  - `syscall: SyscallName`:
  - `direction: SyscallType`:
  - `quintuple: Quintuple`:
  - `comm: [u8; TASK_CMD_LEN]`:
  - `buf: [u8; MAX_PAYLOAD_SIZE]`
- **Description**: Used to wrap the TCP socket structure retrieved during system call handling, facilitating easy access and manipulation of socket-specific data.