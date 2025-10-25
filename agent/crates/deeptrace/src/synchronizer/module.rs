use super::{SynchronizerError, config_listener};
use crate::{
	Module,
	app::runtime::{block_on, spawn, spawn_blocking},
	config::server_config,
	synchronizer::state,
};
use arc_swap::access::Access;
use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info};
use serde_json::json;
use std::time::Duration;
use tokio::{
	net::TcpStream,
	task::JoinHandle,
	time::{self, interval, sleep},
};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::Message};
pub(crate) struct Synchronizer {
	handles: Option<Vec<JoinHandle<()>>>,
}

impl Synchronizer {
	pub fn new() -> Self {
		Self { handles: None }
	}
}

impl Module for Synchronizer {
	type Error = SynchronizerError;
	fn name(&self) -> &str {
		"Synchronizer"
	}
	fn start(&mut self) -> Result<(), Self::Error> {
		info!("Starting {} module...", self.name());

		let mut handles = vec![];
		let config_listener = spawn_blocking(|| {
			block_on(async {
				let _ = config_listener().launch().await;
			})
		});
		// let state_checker = spawn_blocking(|| {
		// 	block_on(async {
		// 		state::health_checker().await;
		// 	})
		// });
		handles.push(config_listener);
		// handles.push(state_checker);
		self.handles = Some(handles);

		info!("{} module started", self.name());
		Ok(())
	}
	async fn stop(&mut self) -> Result<(), Self::Error> {
		if let Some(handles) = self.handles.take() {
			for handle in handles {
				if !handle.is_finished() {
					info!("Waiting for {} module to stop...", self.name());
					handle.abort();
					// handle.await.unwrap();
				}
			}
		}
		info!("{} module stopped.", self.name());
		Ok(())
	}
}

// impl Synchronizer {
// 	async fn run() {
// 		let mut retry_count = 0;
// 		loop {
// 			match connect_to_server().await {
// 				Ok(ws_stream) => {
// 					info!("Connected to server");
// 					retry_count = 0;

// 					let (mut ws_sender, mut ws_receiver) = ws_stream.split();

// 					let heartbeat_handle = spawn({
// 						async move {
// 							const HEARTBEAT_INTERVAL: u64 = 30;
// 							let mut interval = interval(Duration::from_secs(HEARTBEAT_INTERVAL));
// 							loop {
// 								interval.tick().await;

// 								let heartbeat = Message::Text(
// 									json!({
// 										"type": "heartbeat",
// 										"status": "alive",
// 										"timestamp": chrono::Utc::now().to_rfc3339()
// 									})
// 									.to_string(),
// 								);

// 								if let Err(e) = ws_sender.send(heartbeat).await {
// 									error!("Heartbeat send error: {}", e);
// 									break;
// 								}
// 							}
// 						}
// 					});

// 					loop {
// 						tokio::select! {
// 							msg = ws_receiver.next() => {
// 								match msg {
// 									Some(Ok(msg)) => {
// 										if let Err(e) = handle_server_message(msg).await {
// 											error!("Error handling message: {}", e);
// 											break;
// 										}
// 									}
// 									Some(Err(e)) => {
// 										error!("WebSocket receive error: {}", e);
// 										break;
// 									}
// 									None => {
// 										info!("Connection closed by server");
// 										break;
// 									}
// 								}
// 							}

// 							_ = &mut heartbeat_handle => {
// 								error!("Heartbeat task exited");
// 								break;
// 							}
// 						}
// 					}

// 					heartbeat_handle.abort();
// 					info!("Connection closed, reconnecting...");
// 				},
// 				Err(e) => {
// 					retry_count += 1;
// 					const RECONNECT_DELAY: u64 = 5;
// 					let delay = RECONNECT_DELAY * retry_count.min(10);
// 					error!(
// 						"Connection failed: {} (retry {}). Retrying in {} seconds...",
// 						e, retry_count, delay
// 					);
// 					sleep(Duration::from_secs(delay)).await;
// 				},
// 			}
// 		}
// 	}
// }

// async fn connect_to_server()
// -> Result<WebSocketStream<MaybeTlsStream<TcpStream>>, Box<dyn std::error::Error>> {
// 	let config = server_config().load();
// 	let url = format!("ws://{}:{}/{}", config.ip, config.port, config.path);
// 	let (ws_stream, _) = connect_async(url).await?;
// 	Ok(ws_stream)
// }

// async fn handle_server_message(
// 	msg: Message,
// ) -> Result<(), Box<dyn std::error::Error>> {
// 	match msg {
// 		Message::Text(text) => {
// 			debug!("Received text message: {}", text);

// 			if let Ok(payload) = serde_json::from_str::<serde_json::Value>(&text) {
// 				match payload.get("type").and_then(|t| t.as_str()) {
// 					Some("config") => {
// 						info!("Received config update: {}", payload["content"]);
// 					},
// 					_ => debug!("Unknown message type"),
// 				}
// 			}
// 		},
// 		Message::Binary(bin) => debug!("Received binary message ({} bytes)", bin.len()),
// 		Message::Ping(data) => {
// 			debug!("Received ping, sending pong");
// 			ws_sender.send(Message::Pong(data)).await?;
// 		},
// 		Message::Pong(_) => debug!("Received pong"),
// 		Message::Close(_) => {
// 			info!("Received close frame");
// 			return Err("Connection closed by server".into());
// 		},
// 		Message::Frame(frame) => todo!(),
// 	}

// 	Ok(())
// }
