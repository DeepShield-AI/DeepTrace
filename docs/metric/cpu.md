## Metric 模块指标采集文档

### 1. CPU 指标采集

#### 采集指标
- **CPU 使用率** (`cpu_usage`): 每个核心的 CPU 使用百分比
- **用户态使用率** (`user_usage`): 用户态 CPU 使用百分比
- **系统态使用率** (`system_usage`): 系统态 CPU 使用百分比
- **空闲率** (`idle_usage`): 空闲 CPU 百分比
- **I/O 等待率** (`iowait_usage`): I/O 等待时间百分比
- **中断使用率** (`irq_usage`): 硬中断处理时间百分比
- **软中断使用率** (`softirq_usage`): 软中断处理时间百分比
- **虚拟化使用率** (`steal_usage`): 虚拟化环境中其他系统占用时间百分比
- **Guest 使用率** (`guest_usage`): 运行虚拟处理器时间百分比
- **Guest Nice 使用率** (`guest_nice_usage`): 低优先级虚拟处理器时间百分比
- **系统负载** (`cpu_load`): 系统平均负载
- **上下文切换次数** (`context_switches`): 系统上下文切换次数
- **缺页中断次数** (`page_faults`): 内存缺页中断次数
- **时间戳** (`timestamp`): 指标采集时间（UNIX 时间戳）

#### 采集方式
- 通过读取 `/proc/loadavg` 文件获取系统负载信息:cpu_load
- 通过读取 `/proc/vmstat` 文件获取缺页中断信息:page_faluts
- 通过读取 `/proc/stat` 文件获取 CPU 统计数据，用于下一步计算
- 采集间隔为 1 秒（默认值）

#### 计算方法
- **CPU 使用率**: `(user + nice + system) / total * 100%`
- **各项使用率**: `对应项 / total * 100%`，其中 `total = user + nice + system + idle + iowait + irq + softirq + steal`
- **系统负载**: 直接从 `/proc/loadavg` 文件读取
- **上下文切换次数**: 从 `/proc/stat` 文件中读取 `ctxt` 行
- **缺页中断次数**: 从 `/proc/vmstat` 文件中读取 `pgfault` 行

#### 数据结构
```rust
pub struct CpuMetric {
    pub cpu_id: usize,           // CPU 核心 ID
    pub cpu_load: f64,           // 系统负载
    pub cpu_usage: f64,          // CPU 使用率
    pub user: u64,               // 用户态时间
    pub user_usage: f64,         // 用户态使用率
    pub nice: u64,               // 低优先级用户态时间
    pub nice_usage: f64,         // 低优先级用户态使用率
    pub system: u64,             // 系统态时间
    pub system_usage: f64,       // 系统态使用率
    pub idle: u64,               // 空闲时间
    pub idle_usage: f64,         // 空闲率
    pub iowait_usage: f64,       // I/O 等待率
    pub irq_usage: f64,          // 硬中断使用率
    pub softirq_usage: f64,      // 软中断使用率
    pub steal_usage: f64,        // 虚拟化偷取时间率
    pub guest_usage: f64,        // 虚拟处理器时间率
    pub guest_nice_usage: f64,   // 低优先级虚拟处理器时间率
    pub bt_usage: f64,           // 未使用
    pub context_switches: u64,   // 上下文切换次数
    pub page_faults: u64,        // 缺页中断次数
    pub timestamp: u64,          // 时间戳
}
```

#### 数据存储
- 数据存储在 `output/cpu_usage.csv` 文件中
