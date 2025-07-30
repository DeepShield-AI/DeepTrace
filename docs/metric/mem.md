### 2. 内存 (Memory) 指标采集

#### 采集指标
- **已使用内存** (`mem_used`): 系统已使用物理内存
- **应用程序使用内存** (`mem_used_app`): 应用程序实际使用的内存
- **虚拟内存大小** (`mem_vsz`): 所有进程虚拟内存总和
- **空闲内存** (`mem_free`): 空闲物理内存
- **NUMA 缺失次数** (`numa_miss`): NUMA 架构中的内存访问错误次数
- **脏页数** (`dirty`): 等待写入磁盘的内存页数
- **回写页数** (`writeback`): 正在写入磁盘的内存页数
- **缓冲区内存** (`buffers`): 用于文件系统缓冲的内存
- **时间戳** (`timestamp`): 指标采集时间（UNIX 时间戳）

#### 采集方式
- 通过读取 `/proc/meminfo` 文件获取内存基本信息
- 通过读取 `/proc/vmstat` 文件获取 NUMA 缺失信息
- 遍历 `/proc/[pid]/status` 文件获取所有进程的虚拟内存大小
- 采集间隔为 1 秒（默认值）

#### 计算方法
- **已使用内存**: `mem_total - mem_free`
- **应用程序使用内存**: `mem_total - mem_free - buffers - cached`
- **虚拟内存大小**: 遍历所有进程的 `/proc/[pid]/status` 文件，累加 `VmSize` 字段
- **NUMA 缺失次数**: 从 `/proc/vmstat` 文件中读取 `numa_miss` 行

#### 数据结构
```rust
pub struct MemMetric {
    pub mem_used: u64,      // 已使用内存
    pub mem_used_app: u64,  // 应用程序使用内存
    pub mem_vsz: u64,       // 虚拟内存总大小
    pub mem_free: u64,      // 空闲内存
    pub numa_miss: u64,     // NUMA 缺失次数
    pub dirty: u64,         // 脏页数
    pub writeback: u64,     // 回写页数
    pub buffers: u64,       // 缓冲区内存
    pub timestamp: u64,     // 时间戳
}
```

#### 数据存储
- 数据存储在 `output/mem_usage.csv` 文件中
