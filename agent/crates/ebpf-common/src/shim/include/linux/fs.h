#ifndef __SHIM_FILE_H__
#define __SHIM_FILE_H__

// https://elixir.bootlin.com/linux/v6.6/source/include/linux/fs.h#L992
struct file {
    /* needed for tty driver, and maybe others */
    void *private_data;
} __attribute__((preserve_access_index)); /* lest something weird decides that 2 is OK */

#endif
