use super::MemMetric;
use log::{info, warn};
use core::num;
use std::{
	fs::{self,File},
	io::{BufRead, BufReader},
	sync::{
		Arc,
		atomic::{AtomicBool, Ordering},
	},
	time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::time;
use aya::Ebpf;
use num_cpus;
use aya::maps::MapData;
use aya::maps::HashMap as AyaHashMap;
pub struct MemCollector {
	/// Sampling rate
	interval: Duration,
}
fn collect_numa_miss()->u64{
    let file = match File::open("/proc/vmstat") {
        Ok(f) => f,
        Err(e) => {
            warn!("Failed to open /proc/vmstat: {}", e);
            return 0;
        },
    };
    let reader = BufReader::new(file);
    
    for line in reader.lines().filter_map(|r| r.ok()) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[0] == "numa_miss" {
            if let Ok(numa_miss) = parts[1].parse::<u64>() {
                return numa_miss;
            }
        }
    }
    
    0

}
fn calculate_mem_vsz() -> u64 {
    let mut total_vsz: u64 = 0;
    // 遍历 /proc 目录下的所有子目录（即所有进程）
    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            // 检查是否为数字（即有效的 PID）
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if let Ok(pid) = name.parse::<u32>() {
                    // 读取 /proc/<pid>/status 文件
                    let status_path = path.join("status");
                    if status_path.exists() && status_path.is_file() {
                        if let Ok(file) = fs::File::open(&status_path) {
                            let reader = BufReader::new(file);
                            // 查找 VmSize 字段
                            for line in reader.lines().filter_map(|r| r.ok()) {
                                if line.starts_with("VmSize:") {
                                    let parts: Vec<&str> = line.split_whitespace().collect();
                                    if parts.len() >= 2 {
                                        if let Ok(vsz) = parts[1].parse::<u64>() {
                                            total_vsz += vsz;
                                            break; // 找到 VmSize 后退出循环
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    total_vsz
}

impl MemCollector{
    pub fn new()->Self{
        Self{
            interval:Duration::from_secs(1),
        }
    }
fn process_ebpf_data(&self, ebpf: Option<&Ebpf>) -> (u64, u64) {
    let mut wakeup_kswapd = 0;
    let mut page_alloc_extfrag = 0;
    
    if let Some(ebpf) = ebpf {
        // 获取 CPU 核心数量
        let cpu_count = num_cpus::get();
        
        // 读取 WAKEUP_KSWAPD map
        if let Ok(map) = AyaHashMap::<&MapData, u32, u64>::try_from(ebpf.map("WAKEUP_KSWAPD").unwrap()) {
            for cpu_id in 0..cpu_count {
                let cpu_id_u32 = cpu_id as u32;
                if let Ok(value) = map.get(&cpu_id_u32, 0) {
                    wakeup_kswapd += value;
                 
                }
            }
        
        }

        // 读取 PAGE_ALLOC_EXTFRAG map
        if let Ok(map) = AyaHashMap::<&MapData, u32, u64>::try_from(ebpf.map("PAGE_ALLOC_EXTFRAG").unwrap()) {
            for cpu_id in 0..cpu_count {
                let cpu_id_u32 = cpu_id as u32;
                if let Ok(value) = map.get(&cpu_id_u32, 0) {
                    page_alloc_extfrag += value;
                   
                }
            }
 
        }
    }
    
    (wakeup_kswapd, page_alloc_extfrag)
}
    pub fn collect(&self,ebpf:Option<&Ebpf>)->Vec<MemMetric>{
        let mut results=Vec::new();
        let mut meminfo = File::open("/proc/meminfo").expect("Failed to open /proc/meminfo");
        let reader = BufReader::new(meminfo);

        let mut mem_total = 0;
        let mut mem_free = 0;
        let mut buffers = 0;
        let mut cached = 0;
        let mut dirty = 0;
        let mut writeback = 0;

        for line in reader.lines().filter_map(|r| r.ok()) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                match parts[0] {
                    "MemTotal:" => mem_total = parts[1].parse::<u64>().unwrap_or(0),
                    "MemFree:" => mem_free = parts[1].parse::<u64>().unwrap_or(0),
                    "Buffers:" => buffers = parts[1].parse::<u64>().unwrap_or(0),
                    "Cached:" => cached = parts[1].parse::<u64>().unwrap_or(0),
                    "Dirty:" => dirty = parts[1].parse::<u64>().unwrap_or(0),
                    "Writeback:" => writeback = parts[1].parse::<u64>().unwrap_or(0),
                    _ => {}
                }
            }
        }

        // Calculate derived metrics
        let mem_used = mem_total - mem_free;
        let mem_used_app =  mem_total - mem_free - buffers - cached;
        let mem_vsz = calculate_mem_vsz();
        let numa_miss = collect_numa_miss();
        let (wakeup_kswapd,page_alloc_extfrag)=self.process_ebpf_data(ebpf);
            results.push(MemMetric {
            mem_used,
            mem_used_app,
            mem_vsz,
            mem_free,
            numa_miss,
            dirty,
            writeback,
            buffers,
            wakeup_kswapd,
            page_alloc_extfrag,
            timestamp: SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs(),
        });

        results
        }
    pub async fn sleep_duration(&self){
        time::sleep(self.interval).await;
    }
}