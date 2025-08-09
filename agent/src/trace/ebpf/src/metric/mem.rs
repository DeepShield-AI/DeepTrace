
use aya_ebpf::{
    cty::{c_long},
    helpers::bpf_ktime_get_ns,
    macros::tracepoint,
    maps::HashMap,
    programs::TracePointContext,
};
use crate::maps::{PAGE_ALLOC_EXTFRAG,WAKEUP_KSWAPD};
#[tracepoint(category = "kmem", name = "mm_page_alloc_extfrag")]
fn page_alloc_extfrag(ctx: TracePointContext) -> u32 {
    let cpu_id = unsafe { bpf_get_smp_processor_id() };
                // 获取当前时间戳
     let timestamp = unsafe { bpf_ktime_get_ns() };
                // 更新 page_alloc_extfrag 计数器
    unsafe {
        let val = PAGE_ALLOC_EXTFRAG.get(&cpu_id).unwrap_or(&0);
        let new_val = val + 1;
        PAGE_ALLOC_EXTFRAG.insert(&cpu_id, &new_val, 0);
     }
    0
    }

#[tracepoint(category = "vmscan", name = "mm_vmscan_wakeup_kswapd")]
fn vmscan_kswapd_wake(ctx: TracePointContext) -> u32 {
    let cpu_id = unsafe { bpf_get_smp_processor_id() };

    // 获取当前时间戳
    let timestamp = unsafe { bpf_ktime_get_ns() };

    // 更新 wakeup_kswapd 计数器
    unsafe {
        let val = WAKEUP_KSWAPD.get(&cpu_id).unwrap_or(&0);
        let new_val = val + 1;
        WAKEUP_KSWAPD.insert(&cpu_id, &new_val, 0);
    }
    0
}
unsafe fn bpf_get_smp_processor_id() -> u32 {
    let ret: u32;
    aya_ebpf::helpers::bpf_get_smp_processor_id()
}

