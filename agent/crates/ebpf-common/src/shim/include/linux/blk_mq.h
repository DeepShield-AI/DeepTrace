#ifndef __SHIM_BLK_MQ_H__
#define __SHIM_BLK_MQ_H__

#include "types.h"
#include "blk_types.h"

// https://elixir.bootlin.com/linux/v6.6/source/include/linux/blk-mq.h#L80
struct request {
    blk_opf_t cmd_flags; /* op and common flags */

    unsigned int __data_len; /* total data len */

    struct bio * bio;

    /* Time that this request was allocated for this IO. */
    u64 start_time_ns;
    /* Time that I/O was submitted to the device. */
    u64 io_start_time_ns;
} __attribute__((preserve_access_index));

#endif // __SHIM_BLK_MQ_H__
