use super::{
	CoRe,
	generate::{
		self, shim_gendisk_first_minor, shim_gendisk_first_minor_exists, shim_gendisk_major,
		shim_gendisk_major_exists,
	},
};
use crate::macros::kernel_shim;

pub type gendisk = CoRe<generate::gendisk>;

impl gendisk {
	kernel_shim!(pub, gendisk, major, i32);
	kernel_shim!(pub, gendisk, first_minor, i32);
}
