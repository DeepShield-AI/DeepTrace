## Metric 模块指标采集文档
## 4. 磁盘 (Disk) 指标采集

### 采集指标

#### 磁盘性能指标 (DiskMetric)
- **设备名称** (`device`): 磁盘设备标识符（如 sda、nvme0n1）
- **读取完成次数** (`read_completed`): 成功完成的读取操作次数
- **读取合并次数** (`read_merged`): 由于寻道时间重叠而合并的读取请求次数
- **读取扇区数** (`sectors_read`): 读取的扇区总数（每个扇区512字节）
- **读取耗时** (`time_spent_read`): 用于读取操作的总时间（毫秒）
- **写入完成次数** (`write_completed`): 成功完成的写入操作次数
- **写入合并次数** (`write_merged`): 由于寻道时间重叠而合并的写入请求次数
- **写入扇区数** (`sectors_written`): 写入的扇区总数（每个扇区512字节）
- **写入耗时** (`time_spent_writing`): 用于写入操作的总时间（毫秒）
- **正在进行的I/O操作数** (`io_in_progress`): 当前正在进行的I/O操作数量
- **I/O操作总耗时** (`time_spent_io`): 用于I/O操作的总时间（毫秒）
- **加权I/O操作耗时** (`weighted_time_spent_io`): 加权的I/O操作时间（用于计算平均队列长度）
- **平均I/O队列长度** (`aqu_sz`): 磁盘I/O请求队列的平均长度
- **I/O操作平均等待时间** (`await_time`): I/O操作的平均等待时间（毫秒）
- **I/O操作平均服务时间** (`svctm_time`): I/O操作的平均服务时间（毫秒）

#### 磁盘使用情况 (DiskUsage)
- **文件系统** (`filesystem`): 文件系统设备路径（如 /dev/sda1）
- **总大小** (`size`): 文件系统总容量（字节）
- **已使用大小** (`used`): 已使用的磁盘空间（字节）
- **可用大小** (`available`): 可用的磁盘空间（字节）
- **使用百分比** (`use_percent`): 磁盘空间使用率（百分比）
- **挂载点** (`mounted_on`): 文件系统的挂载路径

#### Ext4缓存统计 (Ext4CacheStats)
- **缓存命中次数** (`hits`): Ext4文件系统缓存命中的次数
- **缓存未命中次数** (`misses`): Ext4文件系统缓存未命中的次数

### 采集方式


#### 磁盘性能指标采集
- 获取所有块设备`device`
- 通过读取 `/sys/block/{device}/stat` 文件获取磁盘统计信息
- 默认采集间隔为 1 秒

#### 磁盘使用情况采集
- 通过读取 `/proc/mounts` 文件获取挂载的文件系统信息
- 过滤出块设备挂载点（如 /dev/sd*、/dev/nvme*）
- 使用 `statvfs` 系统调用获取文件系统的详细统计信息


#### Ext4缓存统计采集
- 通过读取 `/proc/fs/ext4/{device}/es_shrinker_info` 文件获取Ext4缓存统计信息
- 解析 "cache hits/misses" 行来提取缓存命中和未命中次数

### 计算方法

#### 磁盘性能计算
- **平均等待时间 (await_time)**: `time_spent_io / (read_completed + write_completed)`
- **平均服务时间 (svctm_time)**: `(time_spent_read + time_spent_writing) / (read_completed + write_completed)`
- **平均队列长度 (aqu_sz)**: `weighted_time_spent_io / time_spent_io`

#### 磁盘使用率计算
- **使用百分比**: `used / size * 100%`

#### 扇区大小转换
- 每个扇区大小为 512 字节

### 数据结构

```rust
#[derive(Debug, Clone)]
pub struct DiskMetric {
    pub device: String,                 // 设备名称
    pub read_completed: u64,            // 读取完成次数
    pub read_merged: u64,               // 读取合并次数
    pub sectors_read: u64,              // 读取扇区数
    pub time_spent_read: u64,           // 读取耗时
    pub write_completed: u64,           // 写入完成次数
    pub write_merged: u64,              // 写入合并次数
    pub sectors_written: u64,           // 写入扇区数
    pub time_spent_writing: u64,        // 写入耗时
    pub io_in_progress: u64,            // 正在进行的I/O操作数
    pub time_spent_io: u64,             // I/O操作总耗时
    pub weighted_time_spent_io: u64,    // 加权I/O操作耗时
    pub aqu_sz: f64,                    // 平均I/O队列长度
    pub await_time: f64,                // 平均I/O等待时间
    pub svctm_time: f64,                // 平均I/O服务时间
}

#[derive(Debug, Clone)]
pub struct DiskUsage {
    pub filesystem: String,             // 文件系统名称
    pub size: u64,                      // 总大小
    pub used: u64,                      // 已使用大小
    pub available: u64,                 // 可用大小
    pub use_percent: f64,               // 使用百分比
    pub mounted_on: String,             // 挂载点
}

#[derive(Debug, Clone)]
pub struct Ext4CacheStats {
    pub hits: u64,                      // 缓存命中次数
    pub misses: u64,                    // 缓存未命中次数
}
```

### 数据存储

磁盘数据存储在output目录下以 `disk` 为前缀的文件夹中，包含三个CSV文件：

**disk_metrics.csv**: 存储磁盘性能指标
   - 包含详细的列标题说明和所有性能指标数据，每行记录一个设备在特定时间点的性能数据

 **disk_usages.csv**: 存储磁盘使用情况
   - 包含文件系统使用情况的详细信息，每行记录一个文件系统在特定时间点的使用情况

 **ext4_cache.csv**: 存储Ext4缓存统计信息
   - 包含Ext4文件系统缓存命中和未命中的统计信息，每行记录特定时间点的缓存统计信息

所有文件都包含时间戳信息，便于进行时间序列分析。 