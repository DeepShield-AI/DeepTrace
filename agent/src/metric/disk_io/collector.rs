use crate::metric::disk_io::{DiskMetric, DiskUsage, Ext4CacheStats,EbpfMetric};
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::time::Duration;
use log::{error, warn};
use std::io::{BufRead, BufReader};
use tokio::time::sleep;
use nix::sys::statvfs::statvfs;
use aya::Ebpf;
use crate::metric::disk_io::collect_ebpf::collect_ebpf_metrics;
use libc::{major, minor};
use std::os::unix::fs::MetadataExt;
pub struct DiskCollector {
    devices: Vec<String>,
}

impl DiskCollector {
    pub fn new() -> Self {
        Self {
            devices: Self::get_block_devices("/sys/block").unwrap_or_else(|_| vec!["sda".to_string()]),
        }
    }

    fn get_block_devices(block_path:&str) -> Result<Vec<String>, ()> {
        let mut devices = Vec::new();
        if let Ok(entries) = std::fs::read_dir(block_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    if let Some(name) = file_name.to_str() {
                        if name.starts_with("sd") || name.starts_with("nvme") {
                            devices.push(name.to_string());
                        }
                    }
                }
            }
    }

        if devices.is_empty() {
            Err(())
        } else {
            Ok(devices)
        }
}

 pub fn collect_metrics(&self) -> Vec<DiskMetric> {
        let mut metrics = Vec::new();

        for device in &self.devices {
            let metric_result = self.collect_device_metrics(device);
            match metric_result {
                Ok(mut metric) => {
                    // 处理 eBPF 数据
                    DiskCollector::calculate_io_times(std::slice::from_mut(&mut metric));
                    metrics.push(metric);
                }
                Err(e) => {
                    error!("Failed to collect metrics for device {}: {}", device, e);
                }
            }
        }

        metrics
    }
    pub fn collect_ebpf_metrics(&self, ebpf: Option<&Ebpf>) -> Vec<EbpfMetric> {
        if let Some(ebpf_ref) = ebpf {
            match collect_ebpf_metrics(ebpf_ref) {
                Ok(metrics) => {
                    metrics.into_iter().map(|(device_id, mut metric)| {
                        // 我们需要将设备ID转换为设备名称
                        let device_name = self.get_device_name(device_id);
                        metric.device = device_name;
                        metric
                    }).collect()
                },
                Err(e) => {
                    error!("Failed to collect eBPF metrics: {}", e);
                    Vec::new()
                }
            }
        } else {
            Vec::new()
        }
    }
    fn get_device_name(&self, device_id: u32) -> String {
    // 设备ID的高12位是主设备号，低20位是次设备号
    let ebpf_major = ((device_id >> 20) & 0xfff) as u32;  // 高12位为主设备号
    let ebpf_minor = (device_id & 0xfffff) as u32;  // 低20位为次设备号 
    
    // 遍历已知的设备列表查找匹配的设备
    for device in &self.devices {
        let dev_path = format!("/dev/{}", device);
        if let Ok(metadata) = fs::metadata(&dev_path) {
            
            use std::os::unix::fs::MetadataExt;
            let rdev = metadata.rdev();
            let rdev_major = unsafe { major(rdev) };
            let rdev_minor = unsafe { minor(rdev) };

            if rdev_major == ebpf_major && rdev_minor == ebpf_minor {
                return device.clone();
            }
        }
    }
    
    // 如果找不到匹配的设备，返回设备ID的字符串表示
    device_id.to_string()
}
    fn collect_device_metrics(&self, device: &str) -> Result<DiskMetric, String> {
        // 从/sys/block/{device}/stat读取指标
        let stat_path = format!("/sys/block/{}/stat", device);
        
        if !Path::new(&stat_path).exists() {
            return Err(format!("Device stat file not found: {}", stat_path));
        }
        
        let stat_content = fs::read_to_string(stat_path).map_err(|e| e.to_string())?;
        let parts: Vec<&str> = stat_content.trim().split_whitespace().collect();
        
        if parts.len() < 11 {
            return Err(format!("Invalid stat format for device {}", device));
        }
        
        Ok(DiskMetric {
            device: device.to_string(),
            read_completed: parts[0].parse::<u64>().unwrap_or(0),
            read_merged: parts[1].parse::<u64>().unwrap_or(0),
            sectors_read: parts[2].parse::<u64>().unwrap_or(0),
            time_spent_read: parts[3].parse::<u64>().unwrap_or(0),
            write_completed: parts[4].parse::<u64>().unwrap_or(0),
            write_merged: parts[5].parse::<u64>().unwrap_or(0),
            sectors_written: parts[6].parse::<u64>().unwrap_or(0),
            time_spent_writing: parts[7].parse::<u64>().unwrap_or(0),
            io_in_progress: parts[8].parse::<u64>().unwrap_or(0),
            time_spent_io: parts[9].parse::<u64>().unwrap_or(0),
            weighted_time_spent_io: parts[10].parse::<u64>().unwrap_or(0),
            aqu_sz: 0.0,
            await_time: 0.0,  
            svctm_time: 0.0,  
        })
    }

pub fn collect_usages(&self) -> Vec<DiskUsage> {
        let mut usages = Vec::new();

        // 读取 /proc/mounts 文件
        let mounts_file = match std::fs::File::open("/proc/mounts") {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to open /proc/mounts: {}", e);
                return usages;
            }
        };

        let reader = BufReader::new(mounts_file);
        for line in reader.lines().filter_map(|r| r.ok()) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                continue;
            }

            let device = parts[0];
            let mount_point = parts[1];

            // 过滤出块设备挂载点（如 /dev/sd*）
            if device.starts_with("/dev/sd") || device.starts_with("/dev/nvme") {
                // 使用 nix::statvfs 获取准确的文件系统统计信息
                match statvfs(mount_point) {
                    Ok(stat) => {
                        let block_size = stat.block_size() as u64;
                        let total_blocks = stat.blocks();
                        let free_blocks = stat.blocks_free();
                        let available_blocks = stat.blocks_available();
                        
                        let size = total_blocks * block_size;
                        let available = available_blocks * block_size;
                        let used = size - (free_blocks * block_size);
                        let use_percent = if size > 0 {
                            (used as f64 / size as f64) * 100.0
                        } else {
                            0.0
                        };

                        usages.push(DiskUsage {
                            filesystem: device.to_string(),
                            size,
                            used,
                            available,
                            use_percent,
                            mounted_on: mount_point.to_string(),
                        });
                    }
                    Err(e) => {
                        error!("Failed to get filesystem stats for {}: {}", mount_point, e);
                        // 如果 statvfs 失败，仍然添加基本信息
                        usages.push(DiskUsage {
                            filesystem: device.to_string(),
                            size: 0,
                            used: 0,
                            available: 0,
                            use_percent: 0.0,
                            mounted_on: mount_point.to_string(),
                        });
                    }
                }
            }
        }

        usages
    }

    fn get_block_device_size(&self, device: &str) -> u64 {
        // 读取块设备的大小
        let size_path = format!("/sys/block/{}/size", device.trim_start_matches("/dev/"));
        if let Ok(size_str) = fs::read_to_string(size_path) {
            if let Ok(size) = size_str.trim().parse::<u64>() {
                return size * 512; // 块设备的大小以 512 字节块为单位
            }
        }

        0
    }

    pub fn collect_ext4_cache_stats(&self) -> Result<Ext4CacheStats, String> {
        let mut total_hits = 0;
        let mut total_misses = 0;

        // 获取所有块设备
        let devices = Self::get_block_devices("/proc/fs/ext4").map_err(|_| "Failed to get block devices")?;
        for device in devices { 
          
            let es_shrinker_info_path = format!("/proc/fs/ext4/{}/es_shrinker_info", device);
            if let Ok(contents) = fs::read_to_string(&es_shrinker_info_path) {
        
                for line in contents.lines() {
                    // 解析类似 "70879/7717 cache hits/misses" 的行
                    if line.contains("cache hits/misses") {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if !parts.is_empty() {
                            // 提取 "70879/7717" 部分
                            if let Some(ratio_part) = parts.first() {
                                let ratio_parts: Vec<&str> = ratio_part.split('/').collect();
                                if ratio_parts.len() == 2 {
                                    if let Ok(hits) = ratio_parts[0].parse::<u64>() {
                                        total_hits += hits;
                                    }
                                    if let Ok(misses) = ratio_parts[1].parse::<u64>() {
                                        total_misses += misses;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
         

        Ok(Ext4CacheStats {
            hits: total_hits,
            misses: total_misses,
        })
}

    pub fn calculate_io_times( metrics: &mut [DiskMetric]) {
        for metric in metrics.iter_mut() {
            let total_io = metric.read_completed + metric.write_completed;
            if total_io > 0 {
                metric.await_time = (metric.time_spent_io as f64) / (total_io as f64);
            } else {
                metric.await_time = 0.0;
            }
            
            let total_time = metric.time_spent_read + metric.time_spent_writing;
            if total_io > 0 {
                metric.svctm_time = (total_time as f64) / (total_io as f64);
            } else {
                metric.svctm_time = 0.0;
            }
            if metric.time_spent_io > 0 {
            metric.aqu_sz = (metric.weighted_time_spent_io as f64) / (metric.time_spent_io as f64);
            } else {
            metric.aqu_sz = 0.0;
            }
        }
    }



    pub async fn sleep_duration(&self) {
        sleep(Duration::from_secs(1)).await;
    }
}