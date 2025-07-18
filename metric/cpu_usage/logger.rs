// ... existing code ...
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::{Arc, Mutex};
use crate::metric::cpu_usage::CpuUsageDetail;
// Deleted:pub struct CpuUsageLogger {
// Deleted:     writer: Arc<Mutex<BufWriter<File>>>,
// Deleted:}
// ... existing code ...
pub struct CpuUsageLogger {
    detail_writer: Arc<Mutex<BufWriter<File>>>,  // 新增详细指标写入器
}

impl CpuUsageLogger {
    pub fn new(file_path: &str) -> std::io::Result<Self> {
        // 创建详细指标文件
        let detail_file_path = format!("{}_detail.csv", file_path.trim_end_matches(".csv"));
        let detail_file = BufWriter::new(File::create(detail_file_path)?);
        let detail_writer = Arc::new(Mutex::new(detail_file));
        {
            let mut dw = detail_writer.lock().unwrap();
            writeln!(dw, "# timestamp: seconds since UNIX epoch")?;
            writeln!(dw, "# cpu_id: 0,1,2... (0-based)")?;
            writeln!(dw, "# user_time: 用户态CPU时间")?;
            writeln!(dw, "# user_percentage: 用户态CPU使用率")?;
            writeln!(dw, "# nice_time: 低优先级用户态CPU时间")?;
            writeln!(dw, "# nice_percentage: 低优先级用户态CPU使用率")?;
            writeln!(dw, "# system_time: 内核态CPU时间")?;
            writeln!(dw, "# system_percentage: 内核态CPU使用率")?;
            writeln!(dw, "# idle_time: 空闲CPU时间")?;
            writeln!(dw, "# idle_percentage: 空闲CPU使用率")?;
            writeln!(dw, "# total_time: 总CPU时间")?;
            writeln!(dw, "# usage: 总CPU使用率")?;
            writeln!(dw, "# load_avg: 1分钟平均负载")?;
            writeln!(dw, "timestamp,cpu_id,user_time,user_percentage,nice_time,nice_percentage,system_time,system_percentage,idle_time,idle_percentage,total_time,usage,load_avg_1min")?;
        }
        Ok(Self {detail_writer })
    }

    pub fn write(&self, detail: &CpuUsageDetail) {
        let mut writer = self.detail_writer.lock().unwrap();
        let _ = writeln!(
            writer, 
            "{},{},{},{},{},{},{},{},{},{},{},{}", 
            detail.timestamp,
            detail.cpu_id,
            detail.user_time,
            detail.user_percentage,
            detail.nice_time,
            detail.nice_percentage,
            detail.system_time,
            detail.system_percentage,
            detail.idle_time,
            detail.idle_percentage,
            detail.total_time,
            detail.usage,
            detail.load_avg_1min
        );
    }
    pub fn flush(&self) {
   
        let _ = self.detail_writer.lock().unwrap().flush();
    }

    pub fn clone(&self) -> Self {
        Self {
      
            detail_writer: self.detail_writer.clone(),
        }
    }
}