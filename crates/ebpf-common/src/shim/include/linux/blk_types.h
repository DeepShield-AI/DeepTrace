#ifndef __SHIM_BLK_TYPES_H__
#define __SHIM_BLK_TYPES_H__

#include "types.h"
#include "block/blk_cgroup.h"
#include "blkdev.h"

// https://elixir.bootlin.com/linux/v6.6/source/include/linux/blk_types.h#L40
struct block_device {
    struct gendisk * bd_disk;
} __attribute__((preserve_access_index));

// https://elixir.bootlin.com/linux/v6.6/source/include/linux/blk_types.h#L264
struct bio {
    struct block_device * bi_bdev;
    /*
     * Represents the association of the css and request_queue for the bio.
     * If a bio goes direct to device, it will not have a blkg as it will
     * not have a request_queue associated with it.  The reference is put
     * on release of the bio.
     */
    struct blkcg_gq * bi_blkg;
} __attribute__((preserve_access_index));

#endif // __SHIM_BLK_TYPES_H__
