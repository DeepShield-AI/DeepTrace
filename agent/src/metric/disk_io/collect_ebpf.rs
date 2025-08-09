// collect_ebpf.rs
use aya::{Ebpf, maps::HashMap};
use crate::metric::disk_io::EbpfMetric;
use std::collections::HashMap as StdHashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// 使用惰性静态变量存储开始时间
use std::sync::Once;

static mut START_TIME: u64 = 0;
static INIT: Once = Once::new();

fn get_elapsed_seconds() -> u64 {
    INIT.call_once(|| {
        unsafe {
            START_TIME = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        }
    });
    
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    unsafe {
        current_time - START_TIME
    }
}

pub fn collect_ebpf_metrics(ebpf: &Ebpf) -> Result<StdHashMap<u32, EbpfMetric>, anyhow::Error> {
    let mut metrics = StdHashMap::new();

    // 获取自收集开始以来的秒数
    let elapsed_seconds = get_elapsed_seconds();
    let now = if elapsed_seconds > 0 { elapsed_seconds } else { 1 }; // 避免除以零

    // 从各个 eBPF maps 中收集数据
    let read_iops_map = HashMap::<_, u32, u64>::try_from(ebpf.map("READ_IOPS").ok_or(anyhow::anyhow!("Failed to get READ_IOPS map"))?)?;
    let write_iops_map = HashMap::<_, u32, u64>::try_from(ebpf.map("WRITE_IOPS").ok_or(anyhow::anyhow!("Failed to get WRITE_IOPS map"))?)?;
    let read_merge_map = HashMap::<_, u32, u64>::try_from(ebpf.map("READ_MERGE").ok_or(anyhow::anyhow!("Failed to get READ_MERGE map"))?)?;
    let write_merge_map = HashMap::<_, u32, u64>::try_from(ebpf.map("WRITE_MERGE").ok_or(anyhow::anyhow!("Failed to get WRITE_MERGE map"))?)?;
    let read_bps_map = HashMap::<_, u32, u64>::try_from(ebpf.map("READ_BPS").ok_or(anyhow::anyhow!("Failed to get READ_BPS map"))?)?;
    let write_bps_map = HashMap::<_, u32, u64>::try_from(ebpf.map("WRITE_BPS").ok_or(anyhow::anyhow!("Failed to get WRITE_BPS map"))?)?;
    let read_issue_nsec_map = HashMap::<_, u32, u64>::try_from(ebpf.map("READ_ISSUE_NSEC").ok_or(anyhow::anyhow!("Failed to get READ_ISSUE_NSEC map"))?)?;
    let write_issue_nsec_map = HashMap::<_, u32, u64>::try_from(ebpf.map("WRITE_ISSUE_NSEC").ok_or(anyhow::anyhow!("Failed to get WRITE_ISSUE_NSEC map"))?)?;
    let read_nsec_map = HashMap::<_, u32, u64>::try_from(ebpf.map("READ_NSEC").ok_or(anyhow::anyhow!("Failed to get READ_NSEC map"))?)?;
    let write_nsec_map = HashMap::<_, u32, u64>::try_from(ebpf.map("WRITE_NSEC").ok_or(anyhow::anyhow!("Failed to get WRITE_NSEC map"))?)?;
    let read_io_count_map = HashMap::<_, u32, u64>::try_from(ebpf.map("READ_IO_COUNT").ok_or(anyhow::anyhow!("Failed to get READ_IO_COUNT map"))?)?;
    let write_io_count_map = HashMap::<_, u32, u64>::try_from(ebpf.map("WRITE_IO_COUNT").ok_or(anyhow::anyhow!("Failed to get WRITE_IO_COUNT map"))?)?;


    // 遍历 READ_IOPS map 获取所有设备的键
    for entry in read_iops_map.iter() {
        let (key, _) = entry?;
        let device_id = key;

        // 获取基础数据
        let read_iops = read_iops_map.get(&device_id, 0).unwrap_or(0);
        let write_iops = write_iops_map.get(&device_id, 0).unwrap_or(0);
        let read_bps = read_bps_map.get(&device_id, 0).unwrap_or(0);
        let write_bps = write_bps_map.get(&device_id, 0).unwrap_or(0);
        let read_issue_nsec_total = read_issue_nsec_map.get(&device_id, 0).unwrap_or(0);
        let write_issue_nsec_total = write_issue_nsec_map.get(&device_id, 0).unwrap_or(0);
        let read_nsec_total = read_nsec_map.get(&device_id, 0).unwrap_or(0);
        let write_nsec_total = write_nsec_map.get(&device_id, 0).unwrap_or(0);
        let read_io_count = read_io_count_map.get(&device_id, 0).unwrap_or(0);
        let write_io_count = write_io_count_map.get(&device_id, 0).unwrap_or(0);

        // 计算 IOPS 和 BPS（每秒）
        let read_iops_per_sec = (read_iops + now - 1) / now;
        let write_iops_per_sec = (write_iops + now - 1) / now;
        let read_bps_per_sec = (read_bps + now - 1) / now;
        let write_bps_per_sec = (write_bps + now - 1) / now;


        // 计算平均下发时间（排队时间）= 总等待时间 / IO操作次数
        let read_issue_nsec_avg = if read_io_count > 0 {
            read_issue_nsec_total / read_io_count
        } else {
            0
        };

        let write_issue_nsec_avg = if write_io_count > 0 {
            write_issue_nsec_total / write_io_count
        } else {
            0
        };

        // 计算平均处理时间（总时间）= 总处理时间 / IO操作次数
        let read_nsec_avg = if read_io_count > 0 {
            read_nsec_total / read_io_count
        } else {
            0
        };

        let write_nsec_avg = if write_io_count > 0 {
            write_nsec_total / write_io_count
        } else {
            0
        };

        // 收集该设备的所有指标
        let metric = EbpfMetric {
            device: device_id.to_string(), // 添加设备ID作为设备名称
            read_iops: read_iops_per_sec,
            write_iops: write_iops_per_sec,
            read_merge: read_merge_map.get(&device_id, 0).unwrap_or(0),
            write_merge: write_merge_map.get(&device_id, 0).unwrap_or(0),
            read_bps: read_bps_per_sec,
            write_bps: write_bps_per_sec,
            read_issue_nsec: read_issue_nsec_avg,     // 平均下发时间（排队时间）
            write_issue_nsec: write_issue_nsec_avg,   // 平均下发时间（排队时间）
            read_nsec: read_nsec_avg,                 // 平均处理时间（总时间）
            write_nsec: write_nsec_avg,               // 平均处理时间（总时间）
        };

        metrics.insert(device_id, metric);
    }

    // 同样检查 WRITE_IOPS map 中的设备（以防只写不读的设备）
    for entry in write_iops_map.iter() {
        let (key, _) = entry?;
        let device_id = key;

        // 如果还没有这个设备的数据，则收集
        if !metrics.contains_key(&device_id) {
            // 获取基础数据
            let read_iops = read_iops_map.get(&device_id, 0).unwrap_or(0);
            let write_iops = write_iops_map.get(&device_id, 0).unwrap_or(0);
            let read_bps = read_bps_map.get(&device_id, 0).unwrap_or(0);
            let write_bps = write_bps_map.get(&device_id, 0).unwrap_or(0);
            let read_issue_nsec_total = read_issue_nsec_map.get(&device_id, 0).unwrap_or(0);
            let write_issue_nsec_total = write_issue_nsec_map.get(&device_id, 0).unwrap_or(0);
            let read_nsec_total = read_nsec_map.get(&device_id, 0).unwrap_or(0);
            let write_nsec_total = write_nsec_map.get(&device_id, 0).unwrap_or(0);
            let read_io_count = read_io_count_map.get(&device_id, 0).unwrap_or(0);
            let write_io_count = write_io_count_map.get(&device_id, 0).unwrap_or(0);

            // 计算 IOPS 和 BPS（每秒）
            let read_iops_per_sec = read_iops / now;
            let write_iops_per_sec = write_iops / now;
            let read_bps_per_sec = read_bps / now;
            let write_bps_per_sec = write_bps / now;

            // 计算平均下发时间（排队时间）= 总等待时间 / IO操作次数
            let read_issue_nsec_avg = if read_io_count > 0 {
                read_issue_nsec_total / read_io_count
            } else {
                0
            };

            let write_issue_nsec_avg = if write_io_count > 0 {
                write_issue_nsec_total / write_io_count
            } else {
                0
            };

            // 计算平均处理时间（总时间）= 总处理时间 / IO操作次数
            let read_nsec_avg = if read_io_count > 0 {
                read_nsec_total / read_io_count
            } else {
                0
            };

            let write_nsec_avg = if write_io_count > 0 {
                write_nsec_total / write_io_count
            } else {
                0
            };

            let metric = EbpfMetric {
                device: device_id.to_string(), // 添加设备ID作为设备名称
                read_iops: read_iops_per_sec,
                write_iops: write_iops_per_sec,
                read_merge: read_merge_map.get(&device_id, 0).unwrap_or(0),
                write_merge: write_merge_map.get(&device_id, 0).unwrap_or(0),
                read_bps: read_bps_per_sec,
                write_bps: write_bps_per_sec,
                read_issue_nsec: read_issue_nsec_avg,   // 平均下发时间（排队时间）
                write_issue_nsec: write_issue_nsec_avg, // 平均下发时间（排队时间）
                read_nsec: read_nsec_avg,               // 平均处理时间（总时间）
                write_nsec: write_nsec_avg,             // 平均处理时间（总时间）
            };

            metrics.insert(device_id, metric);
        }
    }

    Ok(metrics)
}