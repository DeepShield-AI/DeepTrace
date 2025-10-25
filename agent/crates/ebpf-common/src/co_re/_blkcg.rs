use super::{
	CoRe, cgroup_subsys_state,
	generate::{self, shim_blkcg_css, shim_blkcg_css_exists},
};
use crate::macros::kernel_shim;

pub type blkcg = CoRe<generate::blkcg>;

impl blkcg {
	kernel_shim!(pub, blkcg, css, cgroup_subsys_state);
}
