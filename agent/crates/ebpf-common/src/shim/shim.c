// For kernel v6.6

// do not depend on types from the kernel headers
#include "shim.h"

#include "blk_mq.h"

#include "sched.h"

#include "tcp.h"
#include "net.h"
#include "socket.h"

/*
IMPORTANT: it seems defining and using typedefs (for structs) in shim makes it fail at linking, so don't do it.
Using anonymous structs seems to make the linking fail
*/

SHIM_REF(task_struct, se);
SHIM(sched_entity, cfs_rq);
SHIM(cfs_rq, rq);
SHIM(rq, nr_switches);

SHIM(request, cmd_flags);
SHIM(request, __data_len);
SHIM(request, bio);
SHIM(request, start_time_ns);
SHIM(request, io_start_time_ns);

SHIM(bio, bi_bdev);
SHIM(bio, bi_blkg);

SHIM(block_device, bd_disk);
SHIM(gendisk, major);
SHIM(gendisk, first_minor);

SHIM(blkcg_gq, blkcg);
SHIM_REF(blkcg, css);
SHIM(cgroup_subsys_state, cgroup);
SHIM(cgroup_subsys_state, id);

SHIM(cgroup, kn);
SHIM(kernfs_node, id);

SHIM(task_struct, files);
SHIM(files_struct, fdt);

SHIM(fdtable, max_fds);
SHIM(fdtable, fd);
SHIM(file, private_data);

SHIM(socket, type);
SHIM(socket, sk);

SHIM(user_msghdr, msg_iov);
SHIM(user_msghdr, msg_iovlen);

SHIM(iovec, iov_base);
SHIM(iovec, iov_len);

SHIM_REF(mmsghdr, msg_hdr);
SHIM(mmsghdr, msg_len);

// __attribute__((always_inline)) struct tcp_sock *shim_tcp_sock_from_file(struct file *file)
// {
// 	struct socket *socket = file->private_data;
// 	short type = __builtin_preserve_access_index(socket->type);
//   if (type != SOCK_STREAM && type != SOCK_DGRAM) {
//     return NULL;
//   }
// 	return (struct tcp_sock *)(socket->sk);
// }

SHIM_REF(tcp_sock, inet_conn);
SHIM(tcp_sock, copied_seq);
SHIM(tcp_sock, write_seq);

SHIM_REF(inet_connection_sock, icsk_inet);

SHIM_REF(inet_sock, sk);
SHIM(inet_sock, inet_saddr);
SHIM(inet_sock, inet_sport);

SHIM_ENUM_VALUE(sock_type, SOCK_STREAM);
SHIM_ENUM_VALUE(sock_type, SOCK_DGRAM);

SHIM_REF(sock, __sk_common);
SHIM_BITFIELD(sock, sk_type);

SHIM(sock_common, skc_addrpair);
SHIM(sock_common, skc_daddr);
SHIM(sock_common, skc_rcv_saddr);
SHIM(sock_common, skc_portpair);
SHIM(sock_common, skc_dport);
SHIM(sock_common, skc_num);
SHIM(sock_common, skc_family);
SHIM(sock_common, skc_state);
SHIM_BITFIELD(sock_common, skc_ipv6only);
