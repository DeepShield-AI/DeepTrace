# DeepTrace Overhead
| 序号  | command  | time(no ebpf) | time(with ebpf) | overhead per command |
| ---- | -------- | ------------- | --------------- | -------------------- |
| 1    | write    | 1569.67 ns    | 4124.60 ns      | 2554.93 ns           |
| 2    | read     | 996.51 ns     | 3503.46 ns      | 2506.95 ns           |
| 3    | sendto   | 5273.00 ns    | 8087.96 ns      | 2814.96 ns           |
| 4    | recvfrom | 5533.70 ns    | 8221.69 ns      | 2687.99 ns           |
| 5    | sendmsg  | 4923.54 ns    | 7729.67 ns      | 2806.13 ns           |
| 6    | sendmmsg | 5339.07 ns    | 7931.74 ns      | 2592.67 ns           |
| 7    | recvmsg  | 5706.59 ns    | 12863.59 ns     | 7157.00 ns           |
| 8    | recvmmsg | 7393.37 ns    | 8384.79 ns      | 991.42 ns            |
| 9    | writev   | 1659.25 ns    | 4288.58 ns      | 2629.33 ns           |
| 10   | readv    | 1067.64 ns    | 3590.53 ns      | 2522.89 ns           |

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