use super::{
	CoRe,
	generate::{self, shim_kernfs_node_id, shim_kernfs_node_id_exists},
};
use crate::macros::kernel_shim;

pub type kernfs_node = CoRe<generate::kernfs_node>;

impl kernfs_node {
	kernel_shim!(pub, kernfs_node, id, u64);
}
