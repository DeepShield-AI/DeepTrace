#ifndef __SHIM_SCHED_H__
#define __SHIM_SCHED_H__

#include "types.h"
#include "fdtable.h"
#include "kernel/sched/sched.h"

// https://elixir.bootlin.com/linux/v6.6/source/include/linux/sched.h#L548
struct sched_entity {
    struct cfs_rq *cfs_rq;
} __attribute__((preserve_access_index));

// https://elixir.bootlin.com/linux/v6.6/source/include/linux/sched.h#L743
struct task_struct {
    struct sched_entity se;
    struct files_struct *files;
} __attribute__((preserve_access_index));

#endif
