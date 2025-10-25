#ifndef __SHIM_BLKDEV_H__
#define __SHIM_BLKDEV_H__

// https://elixir.bootlin.com/linux/v6.6/source/include/linux/blkdev.h#L128
struct gendisk {
    /*
    * major/first_minor/minors should not be set by any new driver, the
    * block core will take care of allocating them automatically.
    */
    int major;
    
    int first_minor;
} __attribute__((preserve_access_index));

#endif // __SHIM_BLKDEV_H__
