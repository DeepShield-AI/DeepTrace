use crate::NetworkMetricError;
use observ_macros::ProcParser;
use observ_metric_common::procfs_root;
use observ_metric_utils::read_to_string;
use uom::si::{f64::Information, information::byte};

#[derive(ProcParser)]
#[fmt = "table"]
pub struct Interface {
	#[arg(index = 0)]
	pub name: String,
	#[arg(index = 1, unit = byte)]
	pub rx_bytes: Information,
	#[arg(index = 2)]
	pub rx_packets: u64,
	#[arg(index = 3)]
	pub rx_errors: u64,
	#[arg(index = 4)]
	pub rx_dropped: u64,
	#[arg(index = 5)]
	pub rx_fifo: u64,
	#[arg(index = 6)]
	pub rx_frame: u64,
	#[arg(index = 7)]
	pub rx_compressed: u64,
	#[arg(index = 8)]
	pub rx_multicast: u64,
	#[arg(index = 9, unit = byte)]
	pub tx_bytes: Information,
	#[arg(index = 10)]
	pub tx_packets: u64,
	#[arg(index = 11)]
	pub tx_errors: u64,
	#[arg(index = 12)]
	pub tx_dropped: u64,
	#[arg(index = 13)]
	pub tx_fifo: u64,
	#[arg(index = 14)]
	pub tx_colls: u64,
	#[arg(index = 15)]
	pub tx_carrier: u64,
	#[arg(index = 16)]
	pub tx_compressed: u64,
}

impl Interface {
	fn parse_all_filtered(input: &str) -> Result<Vec<Self>, NetworkMetricError> {
		let lines: Vec<&str> = input.lines().collect();
		let input = if lines.len() > 2 {
			lines.into_iter().skip(2).collect::<Vec<_>>().join("\n")
		} else {
			return Err(NetworkMetricError::from(std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				"Invalid input: too few lines",
			)));
		};

		let parsed = Self::parse_all(&input)
			.map_err(|e| NetworkMetricError::ParseError(Box::new(e)))?;

		let interfaces = parsed
			.into_iter()
			.map(|mut iface| {
				iface.name = iface.name.trim_end_matches(':').to_string();
				iface
			})
			.collect();

		Ok(interfaces)
	}
}

pub async fn interfaces() -> Result<Vec<Interface>, NetworkMetricError> {
	let content = read_to_string(procfs_root().join("net/dev")).await?;
	Interface::parse_all_filtered(&content)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_parse_proc_net_dev() {
		let sample_data = r#"Inter-|   Receive                                                |  Transmit
 face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed
    lo: 79717484101 434004870    0    0    0     0          0         0 79717484101 434004870    0    0    0     0       0          0
  eno1: 38610437858 127920780    0    5    0     0          0   2107495 21991231595 102806664    0    0    0     0       0          0
"#;

		let interfaces = Interface::parse_all_filtered(sample_data).unwrap();
		assert_eq!(interfaces.len(), 2);

		let lo = &interfaces[0];
		assert_eq!(lo.name, "lo");
		assert_eq!(lo.rx_bytes.get::<byte>(), 79717484101.0);
		assert_eq!(lo.tx_bytes.get::<byte>(), 79717484101.0);

		let eno1 = &interfaces[1];
		assert_eq!(eno1.name, "eno1");
		assert_eq!(eno1.rx_bytes.get::<byte>(), 38610437858.0);
		assert_eq!(eno1.tx_bytes.get::<byte>(), 21991231595.0);
	}

	#[test]
	fn test_get_interface_stats() {
		let sample_data = r#"Inter-|   Receive                                                |  Transmit
 face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed
    lo: 79717484101 434004870    0    0    0     0          0         0 79717484101 434004870    0    0    0     0       0          0
  eno1: 38610437858 127920780    0    5    0     0          0   2107495 21991231595 102806664    0    0    0     0       0          0
"#;

		// 模拟解析（在实际使用中会读取 /proc/net/dev）
		let interfaces = Interface::parse_all_filtered(sample_data).unwrap();
		let lo_stats = interfaces.iter().find(|iface| iface.name == "lo").unwrap();

		assert_eq!(lo_stats.rx_bytes.get::<byte>(), 79717484101.0);
		assert_eq!(lo_stats.tx_bytes.get::<byte>(), 79717484101.0);
		assert_eq!(lo_stats.rx_packets, 434004870);
		assert_eq!(lo_stats.tx_packets, 434004870);
	}
}
