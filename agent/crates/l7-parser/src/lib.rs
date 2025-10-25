#![no_std]
#![allow(static_mut_refs)]

use infer::Infer;
pub use infer::protocol_infer;
use types::Classification;

mod cassandra;
mod dns;
mod http1;
mod kafka;
mod memcached;
mod mongodb;
mod mysql;
mod redis;
mod rocketmq;
mod thrift;

mod infer;
mod types;
mod utils;
