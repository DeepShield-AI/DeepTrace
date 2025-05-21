// use super::*;
// use crate::utils::tests::load_pcap;
// use anyhow::Ok;
// use trace_common::constants::MAX_PAYLOAD_SIZE;
// use trace_tests::{load_redis, request, stop_redis};

use rand::{distr::Alphanumeric, Rng};

const FILE_DIR: &str = "../../../tests/protocols/redis";
// #[test]
// fn test_redis_pcap() -> Result<(), u32> {
// 	let files = vec![
// 		("redis.pcap", "redis.result"),
// 		("redis-error.pcap", "redis-error.result"),
// 		("redis-debug.pcap", "redis-debug.result"),
// 	];
// 	for (actual, expected) in files {
// 		let actual = format!("{}/{}", FILE_DIR, actual);
// 		let expected = format!("{}/{}", FILE_DIR, expected);
// 		let actual = run(&actual).map_err(|_| 0_u32)?;
// 		let expected = std::fs::read_to_string(expected).map_err(|_| 0_u32)?;
// 		assert_eq!(actual, expected, "{} != {}", actual, expected);
// 	}
// 	Ok(())
// }

// fn run(actual: &str) -> Result<String, u32> {
// 	let packets = load_pcap(actual, MAX_PAYLOAD_SIZE as usize)?;
// 	if packets.is_empty() {
// 		return Err(0);
// 	}
// 	let mut output = String::new();
// 	for (_, payload) in packets {
// 		let Ok(header) = redis(&payload, payload.len() as u32) else {
// 			continue;
// 		};
// 		output.push_str(&format!("{:?}, {}\n", header.message_type(), header));
// 	}
// 	Ok(output)
// }

fn random_string() -> String {
	rand::rng().sample_iter(Alphanumeric).take(24).map(char::from).collect()
}
#[tokio::test]
async fn test_redis() -> anyhow::Result<()> {
	use redis::{Client, Commands};
	use testcontainers::{
		core::{IntoContainerPort, WaitFor},
		runners::AsyncRunner,
		GenericImage,
	};
	let redis = GenericImage::new("redis", "6.2.4")
		.with_exposed_port(6379.tcp())
		.with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
		.start()
		.await
		.unwrap();
	let host = redis.get_host().await.unwrap();
	let port = redis.get_host_port_ipv4(6379.tcp()).await.unwrap();
	println!("Redis is running on {}:{}", host, port);
	// Here you can add your test logic, e.g., connecting to Redis and performing operations
	// let client = Client::open(format!("redis://{}:{}", host, port)).unwrap();
	// let mut con = client.get_connection().unwrap();
	// let key = random_string();
	// let _: () = con.set(&key, &key).unwrap();
	// let value: String = con.get(&key).unwrap();
	// assert_eq!(value, key);
	// println!("{}", key);
	// redis.stop().await.unwrap();
	println!("Redis stopped");
	Ok(())
}
