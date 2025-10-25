#ifndef __SHIM_NET_H__
#define __SHIM_NET_H__

#include "net/sock.h"

// https://elixir.bootlin.com/linux/v6.6/source/include/linux/net.h#L64
enum sock_type {
    SOCK_STREAM = 1,
    SOCK_DGRAM = 2,
    // SOCK_RAW = 3,
    // SOCK_RDM = 4,
    // SOCK_SEQPACKET = 5,
    // SOCK_DCCP = 6,
    // SOCK_PACKET = 10,
};

// https://elixir.bootlin.com/linux/v6.6/source/include/linux/net.h#L117
struct socket {
    short type;
    struct sock *sk;
} __attribute__((preserve_access_index));

#endif// __SHIM_NET_H__
