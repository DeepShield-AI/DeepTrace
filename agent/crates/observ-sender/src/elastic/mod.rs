use bytes::BytesMut;
use elasticsearch::{
	BulkParts, Elasticsearch,
	auth::Credentials,
	http::{
		Url,
		transport::{SingleNodeConnectionPool, TransportBuilder},
	},
};
pub use error::ElasticError;
use log::info;
use observ_config::{ElasticSenderConfig, elastic_sender_config};
use observ_core::{Sendable, Sender};
use serde::Serialize;
use serde_json::json;
use std::time::Duration;

mod error;

pub struct ElasticSender {
	client: Elasticsearch,
	config: ElasticSenderConfig,
	buf: Vec<BytesMut>,
}

impl ElasticSender {
	pub fn new(config_name: String) -> Result<Self, ElasticError> {
		let config = elastic_sender_config(&config_name);
		let url = Url::parse(&config.node_url).map_err(|_| ElasticError::ParseUrl)?;
		let conn_pool = SingleNodeConnectionPool::new(url);

		let transport = TransportBuilder::new(conn_pool)
			.disable_proxy()
			.auth(Credentials::Basic(config.username.clone(), config.password.clone()))
			.timeout(Duration::from_secs(config.request_timeout))
			.build()?;

		let client = Elasticsearch::new(transport);

		Ok(Self { client, buf: Vec::with_capacity(config.bulk_size), config })
	}
}

impl<S: Sendable + Serialize> Sender<S> for ElasticSender {
	type Error = ElasticError;
	async fn send(&mut self, item: BytesMut) -> Result<(), Self::Error> {
		let index = json!({
			"index": {
				"_index": self.config.index_name,
			}
		})
		.to_string();
		self.buf.push(BytesMut::from(index.as_bytes()));
		self.buf.push(item);
		if self.buf.len() > self.config.bulk_size * 2 {
			<Self as Sender<S>>::flush(self).await?;
		}
		Ok(())
	}

	async fn flush(&mut self) -> Result<(), Self::Error> {
		let bulk_body = self.buf.drain(..).collect();
		let response = self.client.bulk(BulkParts::None).body(bulk_body).send().await?;
		let status = response.status_code();
		info!("Elastic response: {}", status);
		if !status.is_success() {
			let err = response.text().await?;

			return Err(ElasticError::Response(err));
		}
		Ok(())
	}
}
