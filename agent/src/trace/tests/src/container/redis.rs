use anyhow::anyhow;
// use bb8_redis::{
// 	RedisConnectionManager,
// 	bb8::{self, Pool},
// };
use rand::{Rng, distr::Alphanumeric};
use redis::{AsyncCommands, Client, Cmd, Commands};
use std::time::Duration;
use testcontainers::{
	core::{IntoContainerPort, WaitFor},
	runners::AsyncRunner,
	GenericImage,
};
use tokio::{process::Command, task, time};
pub async fn load_redis() {
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
        let client = Client::open(format!("redis://{}:{}", host, port)).unwrap();
        let mut con = client.get_connection().unwrap();
        let key = random_string();
        let _: () = con.set(&key, &key).unwrap();
        let value: String = con.get(&key).unwrap();
        assert_eq!(value, key);
        println!("{}", key);
        redis.stop().await.unwrap();
        println!("Redis stopped");
}

pub async fn stop_redis() -> anyhow::Result<()> {
	let _ = Command::new("docker")
		.args(&["stop", "deeptrace_redis_test_workload"])
		.output()
		.await?;

	let _ = Command::new("docker")
		.args(&["rm", "deeptrace_redis_test_workload"])
		.output()
		.await?;
	println!("Redis container stopped and removed successfully");
	time::sleep(Duration::from_secs(1)).await;
	Ok(())
}

fn random_string() -> String {
	rand::rng().sample_iter(Alphanumeric).take(24).map(char::from).collect()
}

// async fn execute_command(
// 	pool: &Pool<RedisConnectionManager>,
// ) -> anyhow::Result<(String, String, String)> {
// 	let mut conn = pool.get().await.map_err(|e| {
// 		redis::RedisError::from((redis::ErrorKind::IoError, "Pool error", e.to_string()))
// 	})?;
// 	let key = random_string();
// 	let _: () = conn.set(&key, &key).await?;

// 	let response: String = Cmd::getrange(&key, 0, 24).query_async(&mut *conn).await?;

// 	Ok((format!("GETRANGE {} 0 24", key), key, response))
// }

pub async fn request(url: ()) -> anyhow::Result<()> {
	let mut client = redis::Client::open("redis://0.0.0.0:6379")?;
	let s = client.get_connection_info();
	println!("Connection info: {:?}", s);
	let key = random_string();
	let _: () = client.set(&key, &key)?;
	// let mut conn = client.get_connection()?;

	// let _: () = conn.set(&key, &key)?;
	// let _: () = conn.set(&key, &key)?;
	// let response = conn.getrange(&key, 0, 24)?;
	// let response = Cmd::getrange(&key, 0, 24).exec(&mut conn)?;
	// println!("Command: GETRANGE {} 0 24\nRequest: {}\nResponse: {}", key, key, response);
	// let manager = RedisConnectionManager::new("redis://0.0.0.0:6379")?;
	// let pool = bb8::Pool::builder().max_size(10).build(manager).await?;
	// let mut handles = vec![];

	// for _ in 0..1 {
	// 	let pool = pool.clone();
	// 	handles.push(task::spawn(async move {
	// 		match execute_command(&pool).await {
	// 			Ok((cmd, req, res)) => {
	// 				println!("Command: {}\nRequest: {}\nResponse: {}", cmd, req, res);
	// 			},
	// 			Err(e) => eprintln!("Error: {}", e),
	// 		}
	// 	}));
	// }

	// for handle in handles {
	// 	handle.await?;
	// }

	Ok(())
}
