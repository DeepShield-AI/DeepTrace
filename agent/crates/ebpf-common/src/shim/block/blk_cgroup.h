#ifndef __SHIM_BLOCK_BLK_CGROUP_H__
#define __SHIM_BLOCK_BLK_CGROUP_H__

#include "cgroup_defs.h"

 // https://elixir.bootlin.com/linux/v6.6/source/block/blk-cgroup.h#L93
struct blkcg {
    struct cgroup_subsys_state css;
} __attribute__((preserve_access_index));

// https://elixir.bootlin.com/linux/v6.6/source/block/blk-cgroup.h#L55
struct blkcg_gq {
    struct blkcg * blkcg;
} __attribute__((preserve_access_index));

#endif // __SHIM_BLOCK_BLK_CGROUP_H__
