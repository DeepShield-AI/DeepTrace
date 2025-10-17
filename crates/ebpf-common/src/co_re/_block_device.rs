use super::{
	CoRe, gendisk,
	generate::{self, shim_block_device_bd_disk, shim_block_device_bd_disk_exists},
};
use crate::macros::kernel_shim;

pub type block_device = CoRe<generate::block_device>;

impl block_device {
	kernel_shim!(pub, block_device, bd_disk, gendisk);
}
