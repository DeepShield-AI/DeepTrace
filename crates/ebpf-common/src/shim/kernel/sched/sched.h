#ifndef __SHIM_KERNEL_SCHED_SCHED_H__
#define __SHIM_KERNEL_SCHED_SCHED_H__

#include "types.h"

// https://elixir.bootlin.com/linux/v6.6/source/kernel/sched/sched.h#L962
struct rq {
    u64 nr_switches;
} __attribute__((preserve_access_index));

// https://elixir.bootlin.com/linux/v6.6/source/kernel/sched/sched.h#L546
struct cfs_rq {
    struct rq *rq; /* CPU runqueue to which this cfs_rq is attached */
} __attribute__((preserve_access_index));

#endif // __SHIM_KERNEL_SCHED_SCHED_H__