use super::{
	CoRe, cgroup,
	generate::{
		self, shim_cgroup_subsys_state_cgroup, shim_cgroup_subsys_state_cgroup_exists,
		shim_cgroup_subsys_state_id, shim_cgroup_subsys_state_id_exists,
	},
};
use crate::macros::kernel_shim;

pub type cgroup_subsys_state = CoRe<generate::cgroup_subsys_state>;

impl cgroup_subsys_state {
	kernel_shim!(pub, cgroup_subsys_state, cgroup, cgroup);
	kernel_shim!(pub, cgroup_subsys_state, id, i32);
}
