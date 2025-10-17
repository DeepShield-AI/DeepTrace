use super::{
	CoRe,
	generate::{self, shim_cfs_rq_rq, shim_cfs_rq_rq_exists},
	rq,
};
use crate::macros::kernel_shim;

pub type cfs_rq = CoRe<generate::cfs_rq>;

impl cfs_rq {
	kernel_shim!(pub, cfs_rq, rq, rq);
}
