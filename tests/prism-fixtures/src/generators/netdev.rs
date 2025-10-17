use alloc::{string::String, vec::Vec};
use fake::{Dummy, Fake, Faker};
use std::fmt;

pub struct FakeInterfaces(pub Vec<FakeInterface>);

#[derive(Debug, Dummy)]
pub struct FakeInterface {
	pub name: String,
	#[dummy(faker = "1..=1_000_000_000")]
	// TODO: change this type to usize
	pub rx_bytes: u64,
	#[dummy(faker = "1..=1_000_000_000")]
	pub rx_packets: u64,
	pub rx_errors: u64,
	pub rx_dropped: u64,
	pub rx_fifo: u64,
	pub rx_frame: u64,
	pub rx_compressed: u64,
	pub rx_multicast: u64,
	#[dummy(faker = "1..=1_000_000_000")]
	pub tx_bytes: usize,
	#[dummy(faker = "1..=1_000_000_000")]
	pub tx_packets: u64,
	pub tx_errors: u64,
	pub tx_dropped: u64,
	pub tx_fifo: u64,
	pub tx_colls: u64,
	pub tx_carrier: u64,
	pub tx_compressed: u64,
}

impl FakeInterfaces {
	pub fn generate() -> Self {
		Self((Faker, 1..=16).fake())
	}
}

impl fmt::Display for FakeInterfaces {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		writeln!(
			f,
			"Inter-|   Receive                                                |  Transmit"
		)?;
		writeln!(
			f,
			" face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed"
		)?;
		<Vec<FakeInterface> as AsRef<[FakeInterface]>>::as_ref(&self.0)
            .iter()
            .try_for_each(|iface| writeln!(
                f,
                "{:>6}: {:>7} {:>7} {:>4} {:>4} {:>4} {:>5} {:>10} {:>9} {:>8} {:>7} {:>4} {:>4} {:>4} {:>5} {:>7} {:>10}",
                iface.name,
                iface.rx_bytes,
                iface.rx_packets,
                iface.rx_errors,
                iface.rx_dropped,
                iface.rx_fifo,
                iface.rx_frame,
                iface.rx_compressed,
                iface.rx_multicast,
                iface.tx_bytes,
                iface.tx_packets,
                iface.tx_errors,
                iface.tx_dropped,
                iface.tx_fifo,
                iface.tx_colls,
                iface.tx_carrier,
                iface.tx_compressed
            ))?;
		Ok(())
	}
}
