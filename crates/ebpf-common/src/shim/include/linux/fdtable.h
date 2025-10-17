#ifndef __SHIM_FDTABLE_H__
#define __SHIM_FDTABLE_H__

#include "fs.h"

// https://elixir.bootlin.com/linux/v6.6/source/include/linux/fdtable.h#L27
struct fdtable {
    unsigned int max_fds;
    
    struct file **fd; /* current fd array */
} __attribute__((preserve_access_index));

// https://elixir.bootlin.com/linux/v6.6/source/include/linux/fdtable.h#L49
struct files_struct {
    struct fdtable *fdt;
} __attribute__((preserve_access_index));

#endif
