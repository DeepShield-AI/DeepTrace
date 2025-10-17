use super::{
	CoRe, blkcg_gq, block_device,
	generate::{
		self, shim_bio_bi_bdev, shim_bio_bi_bdev_exists, shim_bio_bi_blkg, shim_bio_bi_blkg_exists,
	},
};
use crate::macros::kernel_shim;

pub type bio = CoRe<generate::bio>;

impl bio {
	kernel_shim!(pub, bio, bi_bdev, block_device);
	kernel_shim!(pub, bio, bi_blkg, blkcg_gq);
}
