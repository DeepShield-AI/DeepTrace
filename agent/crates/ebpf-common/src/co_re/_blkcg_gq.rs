use super::{
	CoRe, blkcg,
	generate::{self, shim_blkcg_gq_blkcg, shim_blkcg_gq_blkcg_exists},
};
use crate::macros::kernel_shim;

pub type blkcg_gq = CoRe<generate::blkcg_gq>;

impl blkcg_gq {
	kernel_shim!(pub, blkcg_gq, blkcg, blkcg);
}
