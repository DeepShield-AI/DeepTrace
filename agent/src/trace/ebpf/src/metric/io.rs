use crate::maps::{
    READ_IO_COUNT, WRITE_IO_COUNT, READ_NSEC, WRITE_NSEC, READ_IOPS, WRITE_IOPS, WRITE_MERGE,
    WRITE_ISSUE_NSEC, READ_ISSUE_NSEC, WRITE_BPS, READ_BPS, READ_MERGE,INSERT_READ_TIMESTAMP,INSERT_WRITE_TIMESTAMP
};
use aya_ebpf::{
    cty::{c_long, c_ulong},
    helpers::bpf_ktime_get_ns,
    macros::tracepoint,
    maps::{PerCpuArray, HashMap},
    programs::TracePointContext,
};
// name: block_rq_insert
// ID: 1261
#[tracepoint(category = "block", name = "block_rq_insert")]
fn block_rq_insert(ctx: TracePointContext) -> u32 {
    // 读取 device (offset: 8, size: 4)
    let Ok(device) = (unsafe { ctx.read_at::<u32>(8) }) else { return 0 };

    // 读取 rwbs (offset: 32, size: 8)
    let Ok(rwbs) = (unsafe { ctx.read_at::<[u8; 8]>(32) }) else { return 0 };

    let key = device;
    let timestamp = unsafe { bpf_ktime_get_ns() };

    let op = if rwbs.starts_with(b"R") { 0 } else if rwbs.starts_with(b"W") { 1 } else { -1 };

    // 根据操作类型存储插入时间
    if op == 0 { // 读操作
        unsafe {
            INSERT_READ_TIMESTAMP.insert(&key, &timestamp, 0);
        }
    } else if op == 1 { // 写操作
        unsafe {
            INSERT_WRITE_TIMESTAMP.insert(&key, &timestamp, 0);
        }
    }

    0
}
// name: block_rq_issue
// ID: 1259
#[tracepoint(category = "block", name = "block_rq_issue")]
fn block_rq_issue(ctx: TracePointContext) -> u32 {
    // 读取 device (offset: 8, size: 4)
    let Ok(device) = (unsafe { ctx.read_at::<u32>(8) }) else { return 0 };

    // 读取 rwbs (offset: 32, size: 8)
    let Ok(rwbs) = (unsafe { ctx.read_at::<[u8; 8]>(32) }) else { return 0 };

    // 读取 bytes (offset: 28, size: 4)
    let Ok(bytes) = (unsafe { ctx.read_at::<u32>(28) }) else { return 0 };

    let key = device;
    let timestamp = unsafe { bpf_ktime_get_ns() };

    // 判断操作类型（读或写）
    let op = if rwbs.starts_with(b"R") { 0 } else if rwbs.starts_with(b"W") { 1 } else { -1 };

    // 更新 issue 时间
    if op == 0 { // 读操作
        unsafe {
            // 计算从insert到issue的时间（排队时间）
            if let Some(insert_time) = INSERT_READ_TIMESTAMP.get(&key) {
                let duration = timestamp - *insert_time;
                let val = READ_ISSUE_NSEC.get(&key).unwrap_or(&0);
                let new_val = val + duration;
                READ_ISSUE_NSEC.insert(&key, &new_val, 0);
            }



            // 更新 BPS（读操作发起时统计）
            let bps_val = READ_BPS.get(&key).unwrap_or(&0);
            let new_bps_val = bps_val + bytes as u64;
            READ_BPS.insert(&key, &new_bps_val, 0);
        }
    } else if op == 1 { // 写操作
        unsafe {
            // 计算从insert到issue的时间（排队时间）
            if let Some(insert_time) = INSERT_WRITE_TIMESTAMP.get(&key) {
                let duration = timestamp - *insert_time;
                let val = WRITE_ISSUE_NSEC.get(&key).unwrap_or(&0);
                let new_val = val + duration;
                WRITE_ISSUE_NSEC.insert(&key, &new_val, 0);
            }



            // 更新 BPS（写操作发起时统计）
            let bps_val = WRITE_BPS.get(&key).unwrap_or(&0);
            let new_bps_val = bps_val + bytes as u64;
            WRITE_BPS.insert(&key, &new_bps_val, 0);
        }
    }
    0
}
// name: block_rq_complete
// ID: 1256
#[tracepoint(category = "block", name = "block_rq_complete")]
fn block_rq_complete(ctx: TracePointContext) -> u32 {
    // 读取 device (offset: 8, size: 4)
    let Ok(device) = (unsafe { ctx.read_at::<u32>(8) }) else { return 0 };

    // 读取 rwbs (offset: 32, size: 8)
    let Ok(rwbs) = (unsafe { ctx.read_at::<[u8; 8]>(32) }) else { return 0 };

    let key = device;
    let timestamp = unsafe { bpf_ktime_get_ns() };

    // 判断操作类型（读或写）
    let op = if rwbs.starts_with(b"R") { 0 } else if rwbs.starts_with(b"W") { 1 } else { -1 };

    // 更新完成时间
    if op == 0 { // 读操作
        unsafe {
            // 计算从insert到complete的总时间
            if let Some(insert_time) = INSERT_READ_TIMESTAMP.get(&key) {
                let duration = timestamp - *insert_time;
                let val = READ_NSEC.get(&key).unwrap_or(&0);
                let new_val = val + duration;
                READ_NSEC.insert(&key, &new_val, 0);
            }

            // 更新 IO 操作次数
            let count_val = READ_IO_COUNT.get(&key).unwrap_or(&0);
            let new_count_val = count_val + 1;
            READ_IO_COUNT.insert(&key, &new_count_val, 0);

            // 更新 IOPS
            let iops_val = READ_IOPS.get(&key).unwrap_or(&0);
            let new_iops_val = iops_val + 1;
            READ_IOPS.insert(&key, &new_iops_val, 0);
        }
    } else if op == 1 { // 写操作
        unsafe {
            // 计算从insert到complete的总时间
            if let Some(insert_time) = INSERT_WRITE_TIMESTAMP.get(&key) {
                let duration = timestamp - *insert_time;
                let val = WRITE_NSEC.get(&key).unwrap_or(&0);
                let new_val = val + duration;
                WRITE_NSEC.insert(&key, &new_val, 0);
            }

            // 更新 IO 操作次数
            let count_val = WRITE_IO_COUNT.get(&key).unwrap_or(&0);
            let new_count_val = count_val + 1;
            WRITE_IO_COUNT.insert(&key, &new_count_val, 0);

            // 更新 IOPS
            let iops_val = WRITE_IOPS.get(&key).unwrap_or(&0);
            let new_iops_val = iops_val + 1;
            WRITE_IOPS.insert(&key, &new_iops_val, 0);
        }
    }

    0
}

// name: block_rq_merge
// ID: 1258
#[tracepoint(category = "block", name = "block_rq_merge")]
fn block_rq_merge(ctx: TracePointContext) -> u32 {
    // 读取 device (offset: 8, size: 4)
    let Ok(device) = (unsafe { ctx.read_at::<u32>(8) }) else { return 0 };

    // 读取 rwbs (offset: 32, size: 8)
    let Ok(rwbs) = (unsafe { ctx.read_at::<[u8; 8]>(32) }) else { return 0 };

    let key = device;

    // 判断操作类型（读或写）
    let op = if rwbs.starts_with(b"R") { 0 } else if rwbs.starts_with(b"W") { 1 } else { -1 };

    if op == 0 { // 读操作
        unsafe {
            let val = READ_MERGE.get(&key).unwrap_or(&0);
            let new_val = val + 1;
            READ_MERGE.insert(&key, &new_val, 0);
        }
    } else if op == 1 { // 写操作
        unsafe {
            let val = WRITE_MERGE.get(&key).unwrap_or(&0);
            let new_val = val + 1;
            WRITE_MERGE.insert(&key, &new_val, 0);
        }
    }

    0
}