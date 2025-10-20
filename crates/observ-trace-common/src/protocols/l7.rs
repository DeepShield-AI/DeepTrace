use num_enum::{FromPrimitive, IntoPrimitive};

#[cfg_attr(feature = "user", derive(Eq, Hash, serde::Serialize))]
#[derive(FromPrimitive, IntoPrimitive, PartialEq, Copy, Clone)]
#[repr(u8)]
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
	Cassandra = 83,

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

#[cfg(feature = "user")]
impl std::fmt::Display for L7Protocol {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Unknown => f.write_str("Unknown"),
			Self::HTTP1 => f.write_str("HTTP/1"),
			Self::Http2 => f.write_str("HTTP/2"),
			Self::Dubbo => f.write_str("Dubbo"),
			Self::Grpc => f.write_str("gRPC"),
			Self::SofaRPC => f.write_str("SofaRPC"),
			Self::FastCGI => f.write_str("FastCGI"),
			Self::Brpc => f.write_str("BRPC"),
			Self::Tars => f.write_str("TARS"),
			Self::SomeIp => f.write_str("SomeIP"),
			Self::Thrift => f.write_str("Thrift"),
			Self::MySQL => f.write_str("MySQL"),
			Self::PostgreSQL => f.write_str("PostgreSQL"),
			Self::Oracle => f.write_str("Oracle"),
			Self::Redis => f.write_str("Redis"),
			Self::MongoDB => f.write_str("MongoDB"),
			Self::Memcached => f.write_str("Memcached"),
			Self::Cassandra => f.write_str("Cassandra"),
			Self::Kafka => f.write_str("Kafka"),
			Self::MQTT => f.write_str("MQTT"),
			Self::AMQP => f.write_str("AMQP"),
			Self::OpenWire => f.write_str("OpenWire"),
			Self::NATS => f.write_str("NATS"),
			Self::Pulsar => f.write_str("Pulsar"),
			Self::ZMTP => f.write_str("ZMTP"),
			Self::RocketMQ => f.write_str("RocketMQ"),
			Self::DNS => f.write_str("DNS"),
			Self::TLS => f.write_str("TLS"),
			Self::Ping => f.write_str("Ping"),
			Self::Custom => f.write_str("Custom"),
			Self::Max => f.write_str("Max"),
		}
	}
}
