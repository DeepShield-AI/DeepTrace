#ifndef __SHIM_NET_INET_SOCK_H__
#define __SHIM_NET_INET_SOCK_H__

#include "net/sock.h"

// https://elixir.bootlin.com/linux/v6.6/source/include/net/inet_sock.h#L209
struct inet_sock {
	/* sk and pinet6 has to be the first two members of inet_sock */
	struct sock		sk;
	__be32			inet_saddr;
	__be16			inet_sport;
} __attribute__((preserve_access_index));

#endif // __SHIM_NET_INET_SOCK_H__
