
#[repr(u8)]
pub enum HeaderType {
	Invalid = 0,
	Eth = 0x1,
	Arp = 0x2,
	Ipv4 = 0x20,
	Ipv4Icmp = 0x21,
	Ipv6 = 0x40,
	Ipv4Tcp = 0x80,
	Ipv4Udp = 0x81,
	Ipv6Tcp = 0xb0,
	Ipv6Udp = 0xb1,
}

#[allow(non_upper_case_globals)]
impl HeaderType {
	pub const L2: HeaderType = HeaderType::Eth;
	pub const L3: HeaderType = HeaderType::Ipv4;
	pub const L3Ipv6: HeaderType = HeaderType::Ipv6;
	pub const L4: HeaderType = HeaderType::Ipv4Tcp;
	pub const L4Ipv6: HeaderType = HeaderType::Ipv6Tcp;

	pub const fn min_packet_size(self) -> usize {
		match self {
			Self::Eth => 14,               // 不包括DOT1Q
			Self::Arp => 14 + 28,          // 不包括DOT1Q
			Self::Ipv4 => 14 + 20,         // 不包括DOT1Q + IPv4 option0,
			Self::Ipv4Icmp => 14 + 20 + 8, // 不包括DOT1Q + IPv4 option 0x21,
			Self::Ipv6 => 14 + 20, // 不包括DOT1Q + IPv6 option，IPv6大于IPv4的20个字节计算在m.l2L3OptSize里面0,
			Self::Ipv4Tcp => 14 + 20 + 20, // 不包括DOT1Q + IPv4 option0x80,
			Self::Ipv4Udp => 14 + 20 + 8, // 不包括DOT1Q + IPv4 option0x81,
			Self::Ipv6Tcp => 14 + 40 + 20, // 不包括DOT1Q + IPv6 option，IPv6大于40字节的option计算在m.l2L3OptSize里面0xb0,
			Self::Ipv6Udp => 14 + 40 + 8, // 不包括DOT1Q + IPv6 option，IPv6大于40字节的option计算在m.l2L3OptSize里面0xb1,
			Self::Invalid => unreachable!(),
		}
	}

	pub const fn min_header_size(self) -> usize {
		match self {
			Self::Eth => 14,
			Self::Arp => 28,
			Self::Ipv4 => 20,
			Self::Ipv4Icmp => 8,
			Self::Ipv6 => 20,
			Self::Ipv4Tcp => 20,
			Self::Ipv4Udp => 8,
			Self::Ipv6Tcp => 20,
			Self::Ipv6Udp => 8,
			Self::Invalid => unreachable!(),
		}
	}
}
