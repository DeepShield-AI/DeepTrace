#![cfg_attr(not(feature = "user"), no_std)]

#[derive(Debug, Clone, Copy)]
pub struct MetaData {
	pub identifier: u64,
	pub epoch: u32,
	pub jiffies: u64,
	pub taint: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct Version {
	pub name: u64,
	pub prev: u64,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Point<T> {
	pub meta: MetaData,
	pub version: Version,
	pub data: T,
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
	pub meta: MetaData,
	pub from: u64,
	pub to: u64,
	pub allowed: bool,
}

/// ref: <https://elixir.bootlin.com/linux/v6.6/source/include/linux/sched.h#L743>
#[derive(Debug, Clone, Copy)]
pub struct Task {
	pub pid: u32,
	// pub vpid: u32,
	/// usec
	pub utime: u64,
	pub stime: u64,
	/// namespace
	pub utsns: u32,
	pub ipcns: u32,
	pub mntns: u32,
	pub pidns: u32,
	pub netns: u32,
	pub cgroupns: u32,
}

pub struct Cred {}

pub struct Inode {
	pub ino: u64,
	pub mode: u16,
}

pub type TaskPoint = Point<Task>;
pub type InodePoint = Point<Inode>;

impl<T> Point<T> {
	pub fn encode(&self) -> &[u8] {
		unsafe {
			core::slice::from_raw_parts(
				self as *const Self as *const u8,
				core::mem::size_of::<Point<T>>(),
			)
		}
	}
}

impl Edge {
	pub fn encode(&self) -> &[u8] {
		unsafe {
			core::slice::from_raw_parts(
				self as *const Self as *const u8,
				core::mem::size_of::<Edge>(),
			)
		}
	}
}
