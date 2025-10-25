use super::{
	CoRe,
	generate::{self, shim_rq_nr_switches, shim_rq_nr_switches_exists},
};
use crate::macros::kernel_shim;

pub type rq = CoRe<generate::rq>;

impl rq {
	kernel_shim!(pub, rq, nr_switches, u64);
}
