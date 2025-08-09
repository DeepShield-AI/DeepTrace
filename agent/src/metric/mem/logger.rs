// mem/logger.rs
use super::MemMetric;
use std::{
    fs::File,
    io::{BufWriter, Write},
    sync::{Arc, Mutex},
};

pub struct MemLogger {
    detail_writer: Arc<Mutex<BufWriter<File>>>,
}

impl MemLogger {
    pub fn new(file_path: &str) -> Result<Self, std::io::Error> {
        let detail_file_path = format!("{}_detail.csv", file_path.trim_end_matches(".csv"));
        let detail_file = BufWriter::new(File::create(detail_file_path)?);
        let detail_writer = Arc::new(Mutex::new(detail_file));

        // 初始化 writer 时，写入列标题和说明
        {
            let mut dw = detail_writer.lock().unwrap();
            writeln!(dw, "# timestamp: seconds since UNIX epoch")?;
            writeln!(dw, "# mem_used: Total used memory (MemTotal - MemFree)")?;
            writeln!(dw, "# mem_used_app: Application used memory (mem_used - buffers - cached)")?;
            writeln!(dw, "# mem_vsz: Virtual memory size of all processes")?;
            writeln!(dw, "# mem_free: Free memory")?;
            writeln!(dw, "# numa_miss: NUMA miss count")?;
            writeln!(dw, "# dirty: Dirty memory pages")?;
            writeln!(dw, "# writeback: Memory pages under writeback")?;
            writeln!(dw, "# buffers: Buffer cache memory")?;
            writeln!(dw, "# wakeup_kswapd: Number of times kswapd was woken up")?;
            writeln!(dw, "# page_alloc_extfrag: Page allocation external fragmentation")?;
            writeln!(dw, "timestamp,mem_used,mem_used_app,mem_vsz,mem_free,numa_miss,dirty,writeback,buffers,wakeup_kswapd,page_alloc_extfrag")?;
        }

        Ok(Self { detail_writer })
    }

    pub fn write(&self, detail: &MemMetric) {
        let mut writer = self.detail_writer.lock().unwrap();
        let _ = writeln!(
            writer,
            "{},{},{},{},{},{},{},{},{},{},{}",
            detail.timestamp,
            detail.mem_used,
            detail.mem_used_app,
            detail.mem_vsz,
            detail.mem_free,
            detail.numa_miss,
            detail.dirty,
            detail.writeback,
            detail.buffers,
            detail.wakeup_kswapd,
            detail.page_alloc_extfrag
        );
    }

    pub fn flush(&self) {
        let _ = self.detail_writer.lock().unwrap().flush();
    }

    pub fn clone(&self) -> Self {
        Self {
            detail_writer: self.detail_writer.clone()
        }
    }
}