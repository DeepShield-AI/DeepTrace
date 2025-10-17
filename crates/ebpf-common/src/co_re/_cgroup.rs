use super::{
	CoRe,
	generate::{self, shim_cgroup_kn, shim_cgroup_kn_exists},
	kernfs_node,
};
use crate::macros::kernel_shim;

pub type cgroup = CoRe<generate::cgroup>;

impl cgroup {
	kernel_shim!(pub, cgroup, kn, kernfs_node);
}
