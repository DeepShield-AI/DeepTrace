#ifndef __SHIM_SOCKET_H__
#define __SHIM_SOCKET_H__

#include "types.h"
#include "uapi/linux/uio.h"

// https://elixir.bootlin.com/linux/v6.6/source/include/linux/socket.h#L82
struct user_msghdr
{
	struct iovec    *msg_iov;	/* scatter/gather array */
	__kernel_size_t	msg_iovlen;		/* # elements in msg_iov */
} __attribute__((preserve_access_index));

// https://elixir.bootlin.com/linux/v6.6/source/include/linux/socket.h#L93
struct mmsghdr {
	struct user_msghdr  msg_hdr;
	unsigned int        msg_len;
} __attribute__((preserve_access_index));

#endif // __SHIM_SOCKET_H__