use crate::maps::{
    CONTEXT_SWITCHES, CPU_MIGRATIONS, KSOFTIRQD_DELAY,SOFTIRQ_TIMESTAMPS,
};
use aya_ebpf::{
    cty::{c_long, c_ulong},
    helpers::bpf_ktime_get_ns,
    macros::tracepoint,
    maps::{PerCpuArray, HashMap},
    programs::TracePointContext,
};
use trace_common::structs::{Direction, Syscall};

// 定义软中断类型数量
const MAX_SOFTIRQS: usize = 10;

/// name: sched_switch
/// ID: 330
///
///     field:char prev_comm[16];	offset:8;	size:16;	signed:0;
///     field:pid_t prev_pid;	offset:24;	size:4;	signed:1;
///     field:int prev_prio;	offset:28;	size:4;	signed:1;
///     field:long prev_state;	offset:32;	size:8;	signed:1;
///     field:char next_comm[16];	offset:40;	size:16;	signed:0;
///     field:pid_t next_pid;	offset:56;	size:4;	signed:1;
///     field:int next_prio;	offset:60;	size:4;	signed:1;
// 简化版的 sched_switch 函数
#[tracepoint(category = "sched", name = "sched_switch")]
fn sched_switch(ctx: TracePointContext) -> u32 {
    // 读取 prev_pid (offset: 24, size: 4)
    let Ok(_prev_pid) = (unsafe { ctx.read_at::<c_long>(24) }) else { return 0 };

    // 读取 next_pid (offset: 56, size: 4)
    let Ok(_next_pid) = (unsafe { ctx.read_at::<c_long>(56) }) else { return 0 };

    // 获取当前CPU ID
    let cpu_id = unsafe { bpf_get_smp_processor_id() };

    // 简单地递增计数器，避免复杂的 match 逻辑
    unsafe {
        let val = CONTEXT_SWITCHES.get(&cpu_id).unwrap_or(&0);
        let new_val = val + 1;
        CONTEXT_SWITCHES.insert(&cpu_id, &new_val, 0);
    }

    0
}
// name: sched_migrate_task
// ID: 329
// format:
// 	field:unsigned short common_type;	offset:0;	size:2;	signed:0;
// 	field:unsigned char common_flags;	offset:2;	size:1;	signed:0;
// 	field:unsigned char common_preempt_count;	offset:3;	size:1;	signed:0;
// 	field:int common_pid;	offset:4;	size:4;	signed:1;

// 	field:char comm[16];	offset:8;	size:16;	signed:0;
// 	field:pid_t pid;	offset:24;	size:4;	signed:1;
// 	field:int prio;	offset:28;	size:4;	signed:1;
// 	field:int orig_cpu;	offset:32;	size:4;	signed:1;
// 	field:int dest_cpu;	offset:36;	size:4;	signed:1;

// print fmt: "comm=%s pid=%d prio=%d orig_cpu=%d dest_cpu=%d", REC->comm, REC->pid, REC->prio, REC->orig_cpu, REC->dest_cpu
// 简化版的 sched_migrate_task 函数
#[tracepoint(category = "sched", name = "sched_migrate_task")]
fn sched_migrate_task(ctx: TracePointContext) -> u32 {
   // 读取 orig_cpu (offset: 32, size: 4)
    let Ok(orig_cpu) = (unsafe { ctx.read_at::<c_long>(32) }) else { return 0 };

    // 读取 dest_cpu (offset: 36, size: 4)
    let Ok(dest_cpu) = (unsafe { ctx.read_at::<c_long>(36) }) else { return 0 };

    if orig_cpu != dest_cpu {
        let cpu_id = orig_cpu as u32;

        // 简单递增计数器
        unsafe {
            let val = CPU_MIGRATIONS.get(&cpu_id).unwrap_or(&0);
            let new_val = val + 1;
            CPU_MIGRATIONS.insert(&cpu_id, &new_val, 0);
        }
    }

    0
}

// 简化版的 softirq_entry 函数
#[tracepoint(category = "irq", name = "softirq_entry")]
fn softirq_entry(ctx: TracePointContext) -> u32 {
  //  读取软中断向量号 (offset: 12, size: 4)
    let Ok(softirq_nr) = (unsafe { ctx.read_at::<u32>(12) }) else { return 0 };

    // 确保软中断号在有效范围内
    if softirq_nr as usize >= MAX_SOFTIRQS {
        return 0;
    }

    // 记录时间戳
    let timestamp = unsafe { bpf_ktime_get_ns() };
    
    // 存储时间戳到对应位置
    if let Some(timestamp_ref) = unsafe { SOFTIRQ_TIMESTAMPS.get_ptr_mut(softirq_nr as u32) } {
        unsafe { *timestamp_ref = timestamp; }
    }

    0
}

// 简化版的 softirq_exit 函数
#[tracepoint(category = "irq", name = "softirq_exit")]
fn softirq_exit(ctx: TracePointContext) -> u32 {
    let timestamp = unsafe { bpf_ktime_get_ns() };

    // 读取软中断向量号 (offset: 12, size: 4)
    let Ok(softirq_nr) = (unsafe { ctx.read_at::<u32>(12) }) else { return 0 };

    // 确保软中断号在有效范围内
    if softirq_nr as usize >= MAX_SOFTIRQS {
        return 0;
    }

    // 获取对应的开始时间戳
    let start_time = if let Some(timestamp_ref) = unsafe { SOFTIRQ_TIMESTAMPS.get_ptr(softirq_nr as u32) } {
        unsafe { *timestamp_ref }
    } else {
        return 0;
    };

    // 计算延迟并累加
    if start_time > 0 && timestamp > start_time {
        let delay = timestamp - start_time;

        // 获取当前CPU ID
        let cpu_id = unsafe { bpf_get_smp_processor_id() };

        // 累加延迟
        unsafe {
            let val = KSOFTIRQD_DELAY.get(&cpu_id).unwrap_or(&0);
            let new_val = val + delay;
            KSOFTIRQD_DELAY.insert(&cpu_id, &new_val, 0);
        }
    }

    0
}

// 添加获取当前CPU ID的辅助函数

unsafe fn bpf_get_smp_processor_id() -> u32 {
    let ret: u32;
    aya_ebpf::helpers::bpf_get_smp_processor_id()
}