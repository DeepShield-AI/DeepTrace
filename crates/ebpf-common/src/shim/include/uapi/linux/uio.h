#ifndef __SHIM_UAPI_LINUX_UIO_H__
#define __SHIM_UAPI_LINUX_UIO_H__

#include "types.h"
// https://elixir.bootlin.com/linux/v6.6/source/include/uapi/linux/uio.h#L17
struct iovec
{
	void *iov_base;	/* BSD uses caddr_t (1003.1g requires void *) */
	__kernel_size_t iov_len; /* Must be size_t (1003.1g) */
} __attribute__((preserve_access_index));

#endif // __SHIM_UAPI_LINUX_UIO_H__