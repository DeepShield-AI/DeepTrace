# DeepTrace Overhead
| 序号  | command  | time(no ebpf) | time(with ebpf) | overhead per command |
| ---- | -------- | ------------- | --------------- | -------------------- |
| 1    | write    | 1666.05 ns    | 4700.51 ns      | 3034.46 ns           |
| 2    | read     | 1003.61 ns    | 3490.66 ns      | 2487.05 ns           |
| 3    | sendto   | 4420.42 ns    | 6908.72 ns      | 2488.30 ns           |
| 4    | recvfrom | 4562.74 ns    | 7144.61 ns      | 2581.87 ns           |
| 5    | sendmsg  | 3870.99 ns    | 6441.03 ns      | 2570.04 ns           |
| 6    | sendmmsg | 4122.47 ns    | 6855.46 ns      | 2732.99 ns           |
| 7    | recvmsg  | 4014.56 ns    | 7146.20 ns      | 3131.64 ns           |
| 8    | recvmmsg | 4210.29 ns    | 7079.80 ns      | 2869.51 ns           |
| 9    | writev   | 1836.04 ns    | 4587.10 ns      | 2751.06 ns           |
| 10   | readv    | 1074.79 ns    | 3568.32 ns      | 2493.53 ns           |

# DeepFlow Overhead
| 序号  | command  | time(no ebpf) | time(with ebpf) | overhead per command |
| ---- | -------- | ------------- | --------------- | -------------------- |
| 1    | write    | 1569.67 ns    | 1612.48 ns      | 42.81 ns             |
| 2    | read     | 895.74 ns     | 1055.82 ns      | 160.08 ns            |
| 3    | sendto   | 5273.00 ns    | 10821.09 ns     | 5548.09 ns           |
| 4    | recvfrom | 5533.70 ns    | 10921.81 ns     | 5388.11 ns           |
| 5    | sendmsg  | 4923.54 ns    | 5022.96 ns      | 99.42 ns             |
| 6    | sendmmsg | 5339.07 ns    | 5204.77 ns      |                      |
| 7    | recvmsg  | 5706.59 ns    | 7216.53 ns      | 1509.94 ns           |
| 8    | recvmmsg | 7393.37 ns    | 6562.17 ns      |                      |
| 9    | writev   | 1659.25 ns    | 1702.65 ns      | 43.4 ns              |
| 10   | readv    | 1086.03 ns    | 1147.55 ns      | 61.52 ns             |