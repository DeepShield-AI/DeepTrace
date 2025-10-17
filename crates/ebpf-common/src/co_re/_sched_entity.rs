use super::{
	CoRe, cfs_rq,
	generate::{self, shim_sched_entity_cfs_rq, shim_sched_entity_cfs_rq_exists},
};
use crate::macros::kernel_shim;

pub type sched_entity = CoRe<generate::sched_entity>;

impl sched_entity {
	kernel_shim!(pub, sched_entity, cfs_rq, cfs_rq);
}
