#ifndef __SHIM_NET_INET_CONNECTION_SOCK_H__
#define __SHIM_NET_INET_CONNECTION_SOCK_H__

#include "net/inet_sock.h"

// https://elixir.bootlin.com/linux/v6.6/source/include/net/inet_connection_sock.h#L83
struct inet_connection_sock {
	/* inet_sock has to be the first member! */
	struct inet_sock	  icsk_inet;
} __attribute__((preserve_access_index));

#endif // __SHIM_NET_INET_CONNECTION_SOCK_H__
