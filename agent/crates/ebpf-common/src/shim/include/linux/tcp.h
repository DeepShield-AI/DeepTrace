#ifndef __SHIM_TCP_H__
#define __SHIM_TCP_H__

#include "net/inet_connection_sock.h"

// https://elixir.bootlin.com/linux/v6.6/source/include/linux/tcp.h#L177
struct tcp_sock {
    /* inet_connection_sock has to be the first member of tcp_sock */
    struct inet_connection_sock inet_conn;

    u32 copied_seq; /* Head of yet unread data		*/
    
    u32 write_seq; /* Tail(+1) of data held in tcp send buffer */
} __attribute__((preserve_access_index));

#endif // __SHIM_TCP_H__
