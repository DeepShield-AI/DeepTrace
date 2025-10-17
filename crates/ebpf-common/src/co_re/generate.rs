pub type __u64 = ::core::ffi::c_ulonglong;
pub type u64_ = __u64;
pub type __u32 = ::core::ffi::c_uint;
pub type u32_ = __u32;
pub type __u16 = ::core::ffi::c_ushort;
pub type u16_ = __u16;
pub type __be16 = __u16;
pub type __be32 = __u32;
pub type __kernel_ulong_t = ::core::ffi::c_ulong;
pub type __kernel_size_t = __kernel_ulong_t;
pub type blk_opf_t = u32_;
pub type __portpair = __u32;
pub type __addrpair = __u64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kernfs_node {
	pub id: u64_,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgroup {
	pub kn: *mut kernfs_node,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cgroup_subsys_state {
	pub cgroup: *mut cgroup,
	pub id: ::core::ffi::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct blkcg {
	pub css: cgroup_subsys_state,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct blkcg_gq {
	pub blkcg: *mut blkcg,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gendisk {
	pub major: ::core::ffi::c_int,
	pub first_minor: ::core::ffi::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct block_device {
	pub bd_disk: *mut gendisk,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct bio {
	pub bi_bdev: *mut block_device,
	pub bi_blkg: *mut blkcg_gq,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct request {
	pub cmd_flags: blk_opf_t,
	pub __data_len: ::core::ffi::c_uint,
	pub bio: *mut bio,
	pub start_time_ns: u64_,
	pub io_start_time_ns: u64_,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct file {
	pub private_data: *mut ::core::ffi::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fdtable {
	pub max_fds: ::core::ffi::c_uint,
	pub fd: *mut *mut file,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct files_struct {
	pub fdt: *mut fdtable,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rq {
	pub nr_switches: u64_,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cfs_rq {
	pub rq: *mut rq,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sched_entity {
	pub cfs_rq: *mut cfs_rq,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct task_struct {
	pub se: sched_entity,
	pub files: *mut files_struct,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock_common {
	pub __bindgen_anon_1: sock_common__bindgen_ty_1,
	pub __bindgen_anon_2: sock_common__bindgen_ty_2,
	pub skc_family: ::core::ffi::c_ushort,
	pub skc_state: ::core::ffi::c_uchar,
	pub skc_ipv6only: ::core::ffi::c_uchar,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union sock_common__bindgen_ty_1 {
	pub skc_addrpair: __addrpair,
	pub __bindgen_anon_1: sock_common__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sock_common__bindgen_ty_1__bindgen_ty_1 {
	pub skc_daddr: __be32,
	pub skc_rcv_saddr: __be32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union sock_common__bindgen_ty_2 {
	pub skc_portpair: __portpair,
	pub __bindgen_anon_1: sock_common__bindgen_ty_2__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sock_common__bindgen_ty_2__bindgen_ty_1 {
	pub skc_dport: __be16,
	pub skc_num: __u16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock {
	pub __sk_common: sock_common,
	pub sk_type: u16_,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_sock {
	pub sk: sock,
	pub inet_saddr: __be32,
	pub inet_sport: __be16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_connection_sock {
	pub icsk_inet: inet_sock,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_sock {
	pub inet_conn: inet_connection_sock,
	pub copied_seq: u32_,
	pub write_seq: u32_,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct socket {
	pub type_: ::core::ffi::c_short,
	pub sk: *mut sock,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct iovec {
	pub iov_base: *mut ::core::ffi::c_void,
	pub iov_len: __kernel_size_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct user_msghdr {
	pub msg_iov: *mut iovec,
	pub msg_iovlen: __kernel_size_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mmsghdr {
	pub msg_hdr: user_msghdr,
	pub msg_len: ::core::ffi::c_uint,
}
unsafe extern "C" {
	pub fn shim_task_struct_se(task_struct: *mut task_struct) -> *mut sched_entity;
}
unsafe extern "C" {
	pub fn shim_task_struct_se_user(task_struct: *mut task_struct) -> *mut sched_entity;
}
unsafe extern "C" {
	pub fn shim_task_struct_se_exists(task_struct: *mut task_struct) -> bool;
}
unsafe extern "C" {
	pub fn shim_sched_entity_cfs_rq(sched_entity: *mut sched_entity) -> *mut cfs_rq;
}
unsafe extern "C" {
	pub fn shim_sched_entity_cfs_rq_user(sched_entity: *mut sched_entity) -> *mut cfs_rq;
}
unsafe extern "C" {
	pub fn shim_sched_entity_cfs_rq_exists(sched_entity: *mut sched_entity) -> bool;
}
unsafe extern "C" {
	pub fn shim_cfs_rq_rq(cfs_rq: *mut cfs_rq) -> *mut rq;
}
unsafe extern "C" {
	pub fn shim_cfs_rq_rq_user(cfs_rq: *mut cfs_rq) -> *mut rq;
}
unsafe extern "C" {
	pub fn shim_cfs_rq_rq_exists(cfs_rq: *mut cfs_rq) -> bool;
}
unsafe extern "C" {
	pub fn shim_rq_nr_switches(rq: *mut rq) -> ::core::ffi::c_ulonglong;
}
unsafe extern "C" {
	pub fn shim_rq_nr_switches_user(rq: *mut rq) -> ::core::ffi::c_ulonglong;
}
unsafe extern "C" {
	pub fn shim_rq_nr_switches_exists(rq: *mut rq) -> bool;
}
unsafe extern "C" {
	pub fn shim_request_cmd_flags(request: *mut request) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
	pub fn shim_request_cmd_flags_user(request: *mut request) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
	pub fn shim_request_cmd_flags_exists(request: *mut request) -> bool;
}
unsafe extern "C" {
	pub fn shim_request___data_len(request: *mut request) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
	pub fn shim_request___data_len_user(request: *mut request) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
	pub fn shim_request___data_len_exists(request: *mut request) -> bool;
}
unsafe extern "C" {
	pub fn shim_request_bio(request: *mut request) -> *mut bio;
}
unsafe extern "C" {
	pub fn shim_request_bio_user(request: *mut request) -> *mut bio;
}
unsafe extern "C" {
	pub fn shim_request_bio_exists(request: *mut request) -> bool;
}
unsafe extern "C" {
	pub fn shim_request_start_time_ns(request: *mut request) -> ::core::ffi::c_ulonglong;
}
unsafe extern "C" {
	pub fn shim_request_start_time_ns_user(request: *mut request) -> ::core::ffi::c_ulonglong;
}
unsafe extern "C" {
	pub fn shim_request_start_time_ns_exists(request: *mut request) -> bool;
}
unsafe extern "C" {
	pub fn shim_request_io_start_time_ns(request: *mut request) -> ::core::ffi::c_ulonglong;
}
unsafe extern "C" {
	pub fn shim_request_io_start_time_ns_user(request: *mut request) -> ::core::ffi::c_ulonglong;
}
unsafe extern "C" {
	pub fn shim_request_io_start_time_ns_exists(request: *mut request) -> bool;
}
unsafe extern "C" {
	pub fn shim_bio_bi_bdev(bio: *mut bio) -> *mut block_device;
}
unsafe extern "C" {
	pub fn shim_bio_bi_bdev_user(bio: *mut bio) -> *mut block_device;
}
unsafe extern "C" {
	pub fn shim_bio_bi_bdev_exists(bio: *mut bio) -> bool;
}
unsafe extern "C" {
	pub fn shim_bio_bi_blkg(bio: *mut bio) -> *mut blkcg_gq;
}
unsafe extern "C" {
	pub fn shim_bio_bi_blkg_user(bio: *mut bio) -> *mut blkcg_gq;
}
unsafe extern "C" {
	pub fn shim_bio_bi_blkg_exists(bio: *mut bio) -> bool;
}
unsafe extern "C" {
	pub fn shim_block_device_bd_disk(block_device: *mut block_device) -> *mut gendisk;
}
unsafe extern "C" {
	pub fn shim_block_device_bd_disk_user(block_device: *mut block_device) -> *mut gendisk;
}
unsafe extern "C" {
	pub fn shim_block_device_bd_disk_exists(block_device: *mut block_device) -> bool;
}
unsafe extern "C" {
	pub fn shim_gendisk_major(gendisk: *mut gendisk) -> ::core::ffi::c_int;
}
unsafe extern "C" {
	pub fn shim_gendisk_major_user(gendisk: *mut gendisk) -> ::core::ffi::c_int;
}
unsafe extern "C" {
	pub fn shim_gendisk_major_exists(gendisk: *mut gendisk) -> bool;
}
unsafe extern "C" {
	pub fn shim_gendisk_first_minor(gendisk: *mut gendisk) -> ::core::ffi::c_int;
}
unsafe extern "C" {
	pub fn shim_gendisk_first_minor_user(gendisk: *mut gendisk) -> ::core::ffi::c_int;
}
unsafe extern "C" {
	pub fn shim_gendisk_first_minor_exists(gendisk: *mut gendisk) -> bool;
}
unsafe extern "C" {
	pub fn shim_blkcg_gq_blkcg(blkcg_gq: *mut blkcg_gq) -> *mut blkcg;
}
unsafe extern "C" {
	pub fn shim_blkcg_gq_blkcg_user(blkcg_gq: *mut blkcg_gq) -> *mut blkcg;
}
unsafe extern "C" {
	pub fn shim_blkcg_gq_blkcg_exists(blkcg_gq: *mut blkcg_gq) -> bool;
}
unsafe extern "C" {
	pub fn shim_blkcg_css(blkcg: *mut blkcg) -> *mut cgroup_subsys_state;
}
unsafe extern "C" {
	pub fn shim_blkcg_css_user(blkcg: *mut blkcg) -> *mut cgroup_subsys_state;
}
unsafe extern "C" {
	pub fn shim_blkcg_css_exists(blkcg: *mut blkcg) -> bool;
}
unsafe extern "C" {
	pub fn shim_cgroup_subsys_state_cgroup(
		cgroup_subsys_state: *mut cgroup_subsys_state,
	) -> *mut cgroup;
}
unsafe extern "C" {
	pub fn shim_cgroup_subsys_state_cgroup_user(
		cgroup_subsys_state: *mut cgroup_subsys_state,
	) -> *mut cgroup;
}
unsafe extern "C" {
	pub fn shim_cgroup_subsys_state_cgroup_exists(
		cgroup_subsys_state: *mut cgroup_subsys_state,
	) -> bool;
}
unsafe extern "C" {
	pub fn shim_cgroup_subsys_state_id(
		cgroup_subsys_state: *mut cgroup_subsys_state,
	) -> ::core::ffi::c_int;
}
unsafe extern "C" {
	pub fn shim_cgroup_subsys_state_id_user(
		cgroup_subsys_state: *mut cgroup_subsys_state,
	) -> ::core::ffi::c_int;
}
unsafe extern "C" {
	pub fn shim_cgroup_subsys_state_id_exists(
		cgroup_subsys_state: *mut cgroup_subsys_state,
	) -> bool;
}
unsafe extern "C" {
	pub fn shim_cgroup_kn(cgroup: *mut cgroup) -> *mut kernfs_node;
}
unsafe extern "C" {
	pub fn shim_cgroup_kn_user(cgroup: *mut cgroup) -> *mut kernfs_node;
}
unsafe extern "C" {
	pub fn shim_cgroup_kn_exists(cgroup: *mut cgroup) -> bool;
}
unsafe extern "C" {
	pub fn shim_kernfs_node_id(kernfs_node: *mut kernfs_node) -> ::core::ffi::c_ulonglong;
}
unsafe extern "C" {
	pub fn shim_kernfs_node_id_user(kernfs_node: *mut kernfs_node) -> ::core::ffi::c_ulonglong;
}
unsafe extern "C" {
	pub fn shim_kernfs_node_id_exists(kernfs_node: *mut kernfs_node) -> bool;
}
unsafe extern "C" {
	pub fn shim_task_struct_files(task_struct: *mut task_struct) -> *mut files_struct;
}
unsafe extern "C" {
	pub fn shim_task_struct_files_user(task_struct: *mut task_struct) -> *mut files_struct;
}
unsafe extern "C" {
	pub fn shim_task_struct_files_exists(task_struct: *mut task_struct) -> bool;
}
unsafe extern "C" {
	pub fn shim_files_struct_fdt(files_struct: *mut files_struct) -> *mut fdtable;
}
unsafe extern "C" {
	pub fn shim_files_struct_fdt_user(files_struct: *mut files_struct) -> *mut fdtable;
}
unsafe extern "C" {
	pub fn shim_files_struct_fdt_exists(files_struct: *mut files_struct) -> bool;
}
unsafe extern "C" {
	pub fn shim_fdtable_max_fds(fdtable: *mut fdtable) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
	pub fn shim_fdtable_max_fds_user(fdtable: *mut fdtable) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
	pub fn shim_fdtable_max_fds_exists(fdtable: *mut fdtable) -> bool;
}
unsafe extern "C" {
	pub fn shim_fdtable_fd(fdtable: *mut fdtable) -> *mut *mut file;
}
unsafe extern "C" {
	pub fn shim_fdtable_fd_user(fdtable: *mut fdtable) -> *mut *mut file;
}
unsafe extern "C" {
	pub fn shim_fdtable_fd_exists(fdtable: *mut fdtable) -> bool;
}
unsafe extern "C" {
	pub fn shim_file_private_data(file: *mut file) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
	pub fn shim_file_private_data_user(file: *mut file) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
	pub fn shim_file_private_data_exists(file: *mut file) -> bool;
}
unsafe extern "C" {
	pub fn shim_socket_type(socket: *mut socket) -> ::core::ffi::c_short;
}
unsafe extern "C" {
	pub fn shim_socket_type_user(socket: *mut socket) -> ::core::ffi::c_short;
}
unsafe extern "C" {
	pub fn shim_socket_type_exists(socket: *mut socket) -> bool;
}
unsafe extern "C" {
	pub fn shim_socket_sk(socket: *mut socket) -> *mut sock;
}
unsafe extern "C" {
	pub fn shim_socket_sk_user(socket: *mut socket) -> *mut sock;
}
unsafe extern "C" {
	pub fn shim_socket_sk_exists(socket: *mut socket) -> bool;
}
unsafe extern "C" {
	pub fn shim_user_msghdr_msg_iov(user_msghdr: *mut user_msghdr) -> *mut iovec;
}
unsafe extern "C" {
	pub fn shim_user_msghdr_msg_iov_user(user_msghdr: *mut user_msghdr) -> *mut iovec;
}
unsafe extern "C" {
	pub fn shim_user_msghdr_msg_iov_exists(user_msghdr: *mut user_msghdr) -> bool;
}
unsafe extern "C" {
	pub fn shim_user_msghdr_msg_iovlen(user_msghdr: *mut user_msghdr) -> ::core::ffi::c_ulong;
}
unsafe extern "C" {
	pub fn shim_user_msghdr_msg_iovlen_user(user_msghdr: *mut user_msghdr) -> ::core::ffi::c_ulong;
}
unsafe extern "C" {
	pub fn shim_user_msghdr_msg_iovlen_exists(user_msghdr: *mut user_msghdr) -> bool;
}
unsafe extern "C" {
	pub fn shim_iovec_iov_base(iovec: *mut iovec) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
	pub fn shim_iovec_iov_base_user(iovec: *mut iovec) -> *mut ::core::ffi::c_void;
}
unsafe extern "C" {
	pub fn shim_iovec_iov_base_exists(iovec: *mut iovec) -> bool;
}
unsafe extern "C" {
	pub fn shim_iovec_iov_len(iovec: *mut iovec) -> ::core::ffi::c_ulong;
}
unsafe extern "C" {
	pub fn shim_iovec_iov_len_user(iovec: *mut iovec) -> ::core::ffi::c_ulong;
}
unsafe extern "C" {
	pub fn shim_iovec_iov_len_exists(iovec: *mut iovec) -> bool;
}
unsafe extern "C" {
	pub fn shim_mmsghdr_msg_hdr(mmsghdr: *mut mmsghdr) -> *mut user_msghdr;
}
unsafe extern "C" {
	pub fn shim_mmsghdr_msg_hdr_user(mmsghdr: *mut mmsghdr) -> *mut user_msghdr;
}
unsafe extern "C" {
	pub fn shim_mmsghdr_msg_hdr_exists(mmsghdr: *mut mmsghdr) -> bool;
}
unsafe extern "C" {
	pub fn shim_mmsghdr_msg_len(mmsghdr: *mut mmsghdr) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
	pub fn shim_mmsghdr_msg_len_user(mmsghdr: *mut mmsghdr) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
	pub fn shim_mmsghdr_msg_len_exists(mmsghdr: *mut mmsghdr) -> bool;
}
unsafe extern "C" {
	pub fn shim_tcp_sock_inet_conn(tcp_sock: *mut tcp_sock) -> *mut inet_connection_sock;
}
unsafe extern "C" {
	pub fn shim_tcp_sock_inet_conn_user(tcp_sock: *mut tcp_sock) -> *mut inet_connection_sock;
}
unsafe extern "C" {
	pub fn shim_tcp_sock_inet_conn_exists(tcp_sock: *mut tcp_sock) -> bool;
}
unsafe extern "C" {
	pub fn shim_tcp_sock_copied_seq(tcp_sock: *mut tcp_sock) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
	pub fn shim_tcp_sock_copied_seq_user(tcp_sock: *mut tcp_sock) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
	pub fn shim_tcp_sock_copied_seq_exists(tcp_sock: *mut tcp_sock) -> bool;
}
unsafe extern "C" {
	pub fn shim_tcp_sock_write_seq(tcp_sock: *mut tcp_sock) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
	pub fn shim_tcp_sock_write_seq_user(tcp_sock: *mut tcp_sock) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
	pub fn shim_tcp_sock_write_seq_exists(tcp_sock: *mut tcp_sock) -> bool;
}
unsafe extern "C" {
	pub fn shim_inet_connection_sock_icsk_inet(
		inet_connection_sock: *mut inet_connection_sock,
	) -> *mut inet_sock;
}
unsafe extern "C" {
	pub fn shim_inet_connection_sock_icsk_inet_user(
		inet_connection_sock: *mut inet_connection_sock,
	) -> *mut inet_sock;
}
unsafe extern "C" {
	pub fn shim_inet_connection_sock_icsk_inet_exists(
		inet_connection_sock: *mut inet_connection_sock,
	) -> bool;
}
unsafe extern "C" {
	pub fn shim_inet_sock_sk(inet_sock: *mut inet_sock) -> *mut sock;
}
unsafe extern "C" {
	pub fn shim_inet_sock_sk_user(inet_sock: *mut inet_sock) -> *mut sock;
}
unsafe extern "C" {
	pub fn shim_inet_sock_sk_exists(inet_sock: *mut inet_sock) -> bool;
}
unsafe extern "C" {
	pub fn shim_inet_sock_inet_saddr(inet_sock: *mut inet_sock) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
	pub fn shim_inet_sock_inet_saddr_user(inet_sock: *mut inet_sock) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
	pub fn shim_inet_sock_inet_saddr_exists(inet_sock: *mut inet_sock) -> bool;
}
unsafe extern "C" {
	pub fn shim_inet_sock_inet_sport(inet_sock: *mut inet_sock) -> ::core::ffi::c_ushort;
}
unsafe extern "C" {
	pub fn shim_inet_sock_inet_sport_user(inet_sock: *mut inet_sock) -> ::core::ffi::c_ushort;
}
unsafe extern "C" {
	pub fn shim_inet_sock_inet_sport_exists(inet_sock: *mut inet_sock) -> bool;
}
unsafe extern "C" {
	pub fn shim_sock_type_SOCK_STREAM() -> ::core::ffi::c_uint;
}
unsafe extern "C" {
	pub fn shim_sock_type_SOCK_STREAM_exists() -> bool;
}
unsafe extern "C" {
	pub fn shim_sock_type_SOCK_DGRAM() -> ::core::ffi::c_uint;
}
unsafe extern "C" {
	pub fn shim_sock_type_SOCK_DGRAM_exists() -> bool;
}
unsafe extern "C" {
	pub fn shim_sock___sk_common(sock: *mut sock) -> *mut sock_common;
}
unsafe extern "C" {
	pub fn shim_sock___sk_common_user(sock: *mut sock) -> *mut sock_common;
}
unsafe extern "C" {
	pub fn shim_sock___sk_common_exists(sock: *mut sock) -> bool;
}
unsafe extern "C" {
	pub fn shim_sock_sk_type(sock: *mut sock) -> ::core::ffi::c_ushort;
}
unsafe extern "C" {
	pub fn shim_sock_sk_type_exists(sock: *mut sock) -> bool;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_addrpair(sock_common: *mut sock_common)
	-> ::core::ffi::c_ulonglong;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_addrpair_user(
		sock_common: *mut sock_common,
	) -> ::core::ffi::c_ulonglong;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_addrpair_exists(sock_common: *mut sock_common) -> bool;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_daddr(sock_common: *mut sock_common) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_daddr_user(sock_common: *mut sock_common) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_daddr_exists(sock_common: *mut sock_common) -> bool;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_rcv_saddr(sock_common: *mut sock_common) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_rcv_saddr_user(
		sock_common: *mut sock_common,
	) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_rcv_saddr_exists(sock_common: *mut sock_common) -> bool;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_portpair(sock_common: *mut sock_common) -> ::core::ffi::c_uint;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_portpair_user(sock_common: *mut sock_common)
	-> ::core::ffi::c_uint;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_portpair_exists(sock_common: *mut sock_common) -> bool;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_dport(sock_common: *mut sock_common) -> ::core::ffi::c_ushort;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_dport_user(sock_common: *mut sock_common) -> ::core::ffi::c_ushort;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_dport_exists(sock_common: *mut sock_common) -> bool;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_num(sock_common: *mut sock_common) -> ::core::ffi::c_ushort;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_num_user(sock_common: *mut sock_common) -> ::core::ffi::c_ushort;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_num_exists(sock_common: *mut sock_common) -> bool;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_family(sock_common: *mut sock_common) -> ::core::ffi::c_ushort;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_family_user(sock_common: *mut sock_common)
	-> ::core::ffi::c_ushort;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_family_exists(sock_common: *mut sock_common) -> bool;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_state(sock_common: *mut sock_common) -> ::core::ffi::c_uchar;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_state_user(sock_common: *mut sock_common) -> ::core::ffi::c_uchar;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_state_exists(sock_common: *mut sock_common) -> bool;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_ipv6only(sock_common: *mut sock_common) -> ::core::ffi::c_uchar;
}
unsafe extern "C" {
	pub fn shim_sock_common_skc_ipv6only_exists(sock_common: *mut sock_common) -> bool;
}
