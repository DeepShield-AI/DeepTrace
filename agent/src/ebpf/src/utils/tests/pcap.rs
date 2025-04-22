use pcap::{self, PacketHeader};
use pnet::packet::{
	ethernet::{EtherTypes, EthernetPacket},
	icmp::IcmpPacket,
	ip::IpNextHeaderProtocols,
	ipv4::Ipv4Packet,
	ipv6::Ipv6Packet,
	tcp::TcpPacket,
	udp::UdpPacket,
	Packet,
};
use std::path::Path;

pub(crate) fn load_pcap<P>(path: P, length: usize) -> Result<Vec<(PacketHeader, Vec<u8>)>, u32>
where
	P: AsRef<Path>,
{
	let mut capture = pcap::Capture::from_file(path).map_err(|_| 0_u32)?;
	let mut ret = vec![];
	while let Ok(packet) = capture.next_packet() {
		if let Some(payload) = parse(&packet.data[..packet.data.len().min(length)]) {
			if !payload.is_empty() {
				ret.push((*packet.header, payload));
			}
		}
	}
	Ok(ret)
}

fn parse(packet_data: &[u8]) -> Option<Vec<u8>> {
	if let Some(eth) = EthernetPacket::new(packet_data) {
		match eth.get_ethertype() {
			EtherTypes::Ipv4 => handle_ipv4(eth.payload()),
			EtherTypes::Ipv6 => handle_ipv6(eth.payload()),
			_ => None,
		}
	} else {
		handle_raw_ip(packet_data)
	}
}

fn handle_ipv4(payload: &[u8]) -> Option<Vec<u8>> {
	let ipv4 = Ipv4Packet::new(payload)?;
	match ipv4.get_next_level_protocol() {
		IpNextHeaderProtocols::Tcp => TcpPacket::new(ipv4.payload()).map(|p| p.payload().to_vec()),
		IpNextHeaderProtocols::Udp => UdpPacket::new(ipv4.payload()).map(|p| p.payload().to_vec()),
		IpNextHeaderProtocols::Icmp =>
			IcmpPacket::new(ipv4.payload()).map(|p| p.payload().to_vec()),
		_ => None,
	}
}

fn handle_ipv6(payload: &[u8]) -> Option<Vec<u8>> {
	let ipv6 = Ipv6Packet::new(payload)?;
	match ipv6.get_next_header() {
		IpNextHeaderProtocols::Tcp => TcpPacket::new(ipv6.payload()).map(|p| p.payload().to_vec()),
		IpNextHeaderProtocols::Udp => UdpPacket::new(ipv6.payload()).map(|p| p.payload().to_vec()),
		IpNextHeaderProtocols::Icmpv6 =>
			IcmpPacket::new(ipv6.payload()).map(|p| p.payload().to_vec()),
		_ => None,
	}
}

fn handle_raw_ip(packet_data: &[u8]) -> Option<Vec<u8>> {
	if let Some(ipv4) = Ipv4Packet::new(packet_data) {
		handle_ipv4(ipv4.payload())
	} else if let Some(ipv6) = Ipv6Packet::new(packet_data) {
		handle_ipv6(ipv6.payload())
	} else {
		None
	}
}
