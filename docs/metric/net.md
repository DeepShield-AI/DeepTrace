### 3. 网络 (Network) 指标采集

#### 采集指标
- **网络接口名称** (`interface`): 网络接口标识符
- **接收字节数** (`rx_bytes`): 接收的总字节数
- **发送字节数** (`tx_bytes`): 发送的总字节数
- **接收包数** (`rx_packets`): 接收的数据包数量
- **发送包数** (`tx_packets`): 发送的数据包数量
- **丢弃的接收包数** (`rx_dropped`): 因缓冲区满而丢弃的接收包数
- **丢弃的发送包数** (`tx_dropped`): 因缓冲区满而丢弃的发送包数
- **主动打开的 TCP 连接数** (`active_opens`): 主动建立的 TCP 连接数
- **接收的 TCP 段数** (`in_segs`): 接收的 TCP 段数
- **发送的 TCP 段数** (`out_segs`): 发送的 TCP 段数
- **重传的 TCP 段数** (`retrans_segs`): 重传的 TCP 段数
- **输入错误数** (`in_errs`): 网络输入错误数
- **发送的 TCP 重置数** (`out_rsts`): 发送的 TCP 重置数
- **当前已建立连接数** (`curr_estab`): 当前已建立的 TCP 连接数
- **被动打开的 TCP 连接数** (`passive_opens`): 被动建立的 TCP 连接数
- **接收的 UDP 数据报数** (`in_datagrams`): 接收的 UDP 数据报数
- **发送的 UDP 数据报数** (`out_datagrams`): 发送的 UDP 数据报数
- **时间戳** (`timestamp`): 指标采集时间（UNIX 时间戳）

#### 采集方式
- 通过读取 `/proc/net/dev` 文件获取网络接口基本统计信息(从interface到tx_droppred)
- 通过读取 `/proc/net/snmp` 文件获取剩余 TCP 和 UDP 协议详细统计信息
- 采集间隔为 1 秒（默认值）

#### 计算方法
- 所有网络指标都是直接从系统文件中读取的原始统计值


#### 数据结构
```rust
pub struct NetMetric {
    pub interface: String,      // 网络接口名称
    pub rx_bytes: u64,          // 接收字节数
    pub tx_bytes: u64,          // 发送字节数
    pub rx_packets: u64,        // 接收包数
    pub tx_packets: u64,        // 发送包数
    pub rx_dropped: u64,        // 丢弃的接收包数
    pub tx_dropped: u64,        // 丢弃的发送包数
    pub active_opens: u64,      // 主动打开的 TCP 连接数
    pub in_segs: u64,           // 接收的 TCP 段数
    pub out_segs: u64,          // 发送的 TCP 段数
    pub retrans_segs: u64,      // 重传的 TCP 段数
    pub in_errs: u64,           // 输入错误数
    pub out_rsts: u64,          // 发送的 TCP 重置数
    pub curr_estab: u64,        // 当前已建立连接数
    pub passive_opens: u64,     // 被动打开的 TCP 连接数
    pub in_datagrams: u64,      // 接收的 UDP 数据报数
    pub out_datagrams: u64,     // 发送的 UDP 数据报数
    pub timestamp: u64,         // 时间戳
}
```

#### 数据存储
- 数据存储在 `output/net_usage.csv` 文件中