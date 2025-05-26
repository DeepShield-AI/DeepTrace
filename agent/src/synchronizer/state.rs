use crate::{
	app::state,
	config::{agent_config, elastic_config},
};
use arc_swap::access::Access;
use chrono::Local;
use chrono_tz::Asia::Shanghai;
use elasticsearch::{
	BulkParts, Elasticsearch,
	auth::Credentials,
	http::{
		request::JsonBody,
		transport::{SingleNodeConnectionPool, TransportBuilder},
	},
};
use log::{debug, error, info};
use serde::Serialize;
use serde_json::json;
use std::{sync::atomic::Ordering, time::Duration};
use tokio::time::interval;
use url::Url;

#[derive(Serialize)]
struct State {
	timestamp: String,
	state: String,
	name: String,
}

impl State {
	fn new() -> Self {
		let timestamp = Local::now().with_timezone(&Shanghai).time();
		let state = match state().load(Ordering::Relaxed) {
			true => "terminate".to_string(),
			false => "running".to_string(),
		};
		let name = agent_config().load().name.clone();
		Self { timestamp: format!("{:?}", timestamp), state, name }
	}
}

pub(super) async fn health_checker() {
	let config = elastic_config();
	let c = config.load();
	let url = Url::parse(&c.node_url).expect("Invalid URL");
	let conn_pool = SingleNodeConnectionPool::new(url);

	let transport = TransportBuilder::new(conn_pool)
		.disable_proxy()
		.auth(Credentials::Basic(c.username.clone(), c.password.clone()))
		.timeout(Duration::from_secs(c.request_timeout))
		.build()
		.expect("Failed to build transport");

	info!("Sync agent state to Elasticsearch at {}", c.node_url);

	let client = Elasticsearch::new(transport);

	let mut interval = interval(Duration::from_secs(10));

	loop {
		interval.tick().await;

		let mut bulk_body: Vec<JsonBody<serde_json::Value>> = Vec::with_capacity(2);
		bulk_body.push(
			json!({
				"index": {
					"_index": agent_config().load().state_index,
				}
			})
			.into(),
		);
		bulk_body.push(json!(State::new()).into());

		let response = client
			.bulk(BulkParts::None)
			.body(bulk_body)
			.send()
			.await
			.expect("Failed to send request");

		let status = response.status_code();
		debug!("State sync response status: {}", status);

		if !status.is_success() {
			let error_msg = response.text().await.expect("Failed to read response");
			error!("Elasticsearch error: {error_msg}");
		}
	}
}
