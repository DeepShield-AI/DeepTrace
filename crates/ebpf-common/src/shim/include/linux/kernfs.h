#ifndef __SHIM_KERNFS_H__
#define __SHIM_KERNFS_H__

#include "types.h"

// https://elixir.bootlin.com/linux/v6.6/source/include/linux/kernfs.h#L190
struct kernfs_node {
    /*
        * 64bit unique ID.  On 64bit ino setups, id is the ino.  On 32bit,
        * the low 32bits are ino and upper generation.
        */
    u64 id;
} __attribute__((preserve_access_index));

#endif
