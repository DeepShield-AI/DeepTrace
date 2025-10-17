use super::{Collector, MetricError, constants::*};
use log::warn;
use observ_event::{gauge, metric::Metric};
use observ_network::netdev::interfaces;
use std::io;
use uom::si::information::byte;

pub struct NetworkCollector;

impl NetworkCollector {
	pub(crate) const fn new() -> Result<Self, MetricError> {
		Ok(Self {})
	}
}

#[async_trait::async_trait]
impl Collector for NetworkCollector {
	fn name(&self) -> &'static str {
		"host network collector"
	}

	async fn collect(&self, buffer: &mut Vec<Metric>) -> Result<(), MetricError> {
		match interfaces().await {
			Ok(interfaces) => {
				for interface in interfaces {
					let interface_name = interface.name.clone();

					// Receive metrics
					buffer.push(gauge!(RX_BYTES, interface.get_rx_bytes().get::<byte>(), "interface" => interface_name.clone()));
					buffer.push(
						gauge!(RX_PACKETS, interface.get_rx_packets(), "interface" => interface_name.clone()),
					);
					buffer.push(
						gauge!(RX_ERRORS, interface.get_rx_errors(), "interface" => interface_name.clone()),
					);
					buffer.push(
						gauge!(RX_DROPPED, interface.get_rx_dropped(), "interface" => interface_name.clone()),
					);
					buffer.push(
						gauge!(RX_FIFO, interface.get_rx_fifo(), "interface" => interface_name.clone()),
					);
					buffer.push(
						gauge!(RX_FRAME, interface.get_rx_frame(), "interface" => interface_name.clone()),
					);
					buffer.push(
						gauge!(RX_COMPRESSED, interface.get_rx_compressed(), "interface" => interface_name.clone()),
					);
					buffer.push(
						gauge!(RX_MULTICAST, interface.get_rx_multicast(), "interface" => interface_name.clone()),
					);

					// Transmit metrics
					buffer.push(gauge!(TX_BYTES, interface.get_tx_bytes().get::<byte>(), "interface" => interface_name.clone()));
					buffer.push(
						gauge!(TX_PACKETS, interface.get_tx_packets(), "interface" => interface_name.clone()),
					);
					buffer.push(
						gauge!(TX_ERRORS, interface.get_tx_errors(), "interface" => interface_name.clone()),
					);
					buffer.push(
						gauge!(TX_DROPPED, interface.get_tx_dropped(), "interface" => interface_name.clone()),
					);
					buffer.push(
						gauge!(TX_FIFO, interface.get_tx_fifo(), "interface" => interface_name.clone()),
					);
					buffer.push(
						gauge!(TX_COLLS, interface.get_tx_colls(), "interface" => interface_name.clone()),
					);
					buffer.push(
						gauge!(TX_CARRIER, interface.get_tx_carrier(), "interface" => interface_name.clone()),
					);
					buffer.push(
						gauge!(TX_COMPRESSED, interface.get_tx_compressed(), "interface" => interface_name.clone()),
					);
				}
			},
			Err(error) => {
				warn!("Failed to collect network metrics: {error}");
				return Err(io::Error::last_os_error().into());
			},
		}
		Ok(())
	}
}
