## Usage

**把ebpf程序注入内核**

```bash
RUST_LOG=info cargo run --release --config 'target."cfg(all())".runner="sudo -E"'
```

**反复调用某一syscall并测量所需时间**（重复调用10^5次，100次取平均值）

```bash
bash run.sh <syscall>
```

\<syscall\> = write | read | sendto | recvfrom | sendmsg | sendmmsg | recvmsg | recvmmsg |writev | readv | empty | ssl_write | ssl_read |ssl | empty

在测试recvfrom, sendto, recvmsg, sendmsg, recvmmsg, sendmmsg时，需将发送的系统调用与相对应的系统调用一起进行测试。

**检验可执行文件调用了哪些syscall、调用了多少次**

```bash
cd bin
strace -c ./<filename>
```

先编译出可执行文件再用strace