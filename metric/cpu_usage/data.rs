pub struct CpuUsageDetail {
    pub cpu_id: usize,
    pub user_time: u64,       // 用户态时间
    pub user_percentage: f64, // 用户态百分比
    pub nice_time: u64,       // 低优先级用户态时间
    pub nice_percentage: f64, // 低优先级用户态百分比
    pub system_time: u64,     // 内核态时间
    pub system_percentage: f64, // 内核态百分比
    pub idle_time: u64,       // 空闲时间
    pub idle_percentage: f64, // 空闲百分比
    pub total_time: u64,      // 总时间
    pub timestamp: u64,       // 时间戳
    pub usage: f64,           // 总使用率
    pub load_avg_1min: f64,   // 1分钟平均负载
}