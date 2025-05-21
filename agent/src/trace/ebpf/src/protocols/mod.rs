#![allow(static_mut_refs)]
pub(crate) use infer::infer_protocol;
use infer::Infer;

mod dns;
mod http1;
mod kafka;
mod memcached;
mod mongodb;
mod mysql;
mod redis;
mod thrift;

mod infer;
mod utils;
