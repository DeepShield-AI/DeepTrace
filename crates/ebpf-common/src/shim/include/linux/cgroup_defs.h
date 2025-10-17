#ifndef __SHIM_CGROUP_H__
#define __SHIM_CGROUP_H__

#include "kernfs.h"

// https://elixir.bootlin.com/linux/v6.6/source/include/linux/cgroup-defs.h#L392
struct cgroup {
    struct kernfs_node *kn; /* cgroup kernfs entry */
} __attribute__((preserve_access_index));

// https://elixir.bootlin.com/linux/v6.6/source/include/linux/cgroup-defs.h#L155
struct cgroup_subsys_state {
    /* PI: the cgroup that this css is attached to */
    struct cgroup *cgroup;
    /*
     * PI: Subsys-unique ID.  0 is unused and root is always 1.  The
     * matching css can be looked up using css_from_id().
     */
    int id;
} __attribute__((preserve_access_index));

#endif
