#ifndef __SHIM_NET_SOCK_H__
#define __SHIM_NET_SOCK_H__

#include "types.h"

// https://elixir.bootlin.com/linux/v6.6/source/include/net/sock.h#L163
struct sock_common {
    union {
        __addrpair skc_addrpair;
        struct {
            __be32 skc_daddr;
            __be32 skc_rcv_saddr;
        };
    };
    /* skc_dport && skc_num must be grouped as well */
    union {
        __portpair skc_portpair;
        struct {
            __be16 skc_dport;
            __u16 skc_num;
        };
    };

    unsigned short skc_family;
    unsigned char skc_state;
    unsigned char skc_ipv6only;
} __attribute__((preserve_access_index));

// https://elixir.bootlin.com/linux/v6.6/source/include/net/sock.h#L357
struct sock {
    /*
	 * Now struct inet_timewait_sock also uses sock_common, so please just
	 * don't add nothing before this first member (__sk_common) --acme
	 */
    struct sock_common __sk_common;
    
    u16 sk_type;
} __attribute__((preserve_access_index));

#endif // __SHIM_NET_SOCK_H__
