use num_enum::{FromPrimitive, IntoPrimitive};
use serde::Serialize;

#[repr(u8)]
#[derive(
	FromPrimitive, IntoPrimitive, Copy, Clone, Default, PartialEq, Eq, Hash, Serialize, Debug,
)]
pub enum L7Protocol {
	#[default]
	Unknown = 0,

	// HTTP
	HTTP1 = 20,
	Http2 = 21,

	// RPC
	Dubbo = 40,
	Grpc = 41,
	SofaRPC = 43,

	FastCGI = 44,
	Brpc = 45,
	Tars = 46,
	SomeIp = 47,

	Thrift = 48,
	// SQL
	MySQL = 60,
	PostgreSQL = 61,
	Oracle = 62,

	// NoSQL
	Redis = 80,
	MongoDB = 81,
	Memcached = 82,

	// MQ
	Kafka = 100,
	MQTT = 101,
	AMQP = 102,
	OpenWire = 103,
	NATS = 104,
	Pulsar = 105,
	ZMTP = 106,
	RocketMQ = 107,

	// INFRA
	DNS = 120,
	TLS = 121,
	Ping = 122,

	Custom = 127,

	Max = 255,
}

impl From<&L7Protocol> for &'static str {
	fn from(protocol: &L7Protocol) -> Self {
		match protocol {
			L7Protocol::Unknown => "Unknown",
			L7Protocol::HTTP1 => "HTTP/1",
			L7Protocol::Http2 => "HTTP/2",
			L7Protocol::Dubbo => "Dubbo",
			L7Protocol::Grpc => "gRPC",
			L7Protocol::SofaRPC => "SofaRPC",
			L7Protocol::FastCGI => "FastCGI",
			L7Protocol::Brpc => "BRPC",
			L7Protocol::Tars => "TARS",
			L7Protocol::SomeIp => "SomeIP",
			L7Protocol::Thrift => "Thrift",
			L7Protocol::MySQL => "MySQL",
			L7Protocol::PostgreSQL => "PostgreSQL",
			L7Protocol::Oracle => "Oracle",
			L7Protocol::Redis => "Redis",
			L7Protocol::MongoDB => "MongoDB",
			L7Protocol::Memcached => "Memcached",
			L7Protocol::Kafka => "Kafka",
			L7Protocol::MQTT => "MQTT",
			L7Protocol::AMQP => "AMQP",
			L7Protocol::OpenWire => "OpenWire",
			L7Protocol::NATS => "NATS",
			L7Protocol::Pulsar => "Pulsar",
			L7Protocol::ZMTP => "ZMTP",
			L7Protocol::RocketMQ => "RocketMQ",
			L7Protocol::DNS => "DNS",
			L7Protocol::TLS => "TLS",
			L7Protocol::Ping => "Ping",
			L7Protocol::Custom => "Custom",
			L7Protocol::Max => "Max",
		}
	}
}

#[cfg(feature = "user")]
impl std::fmt::Display for L7Protocol {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.into())
	}
}
