use bytes::BytesMut;
use elasticsearch::{
	BulkParts, Elasticsearch,
	auth::Credentials,
	cert::CertificateValidation,
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
			.cert_validation(CertificateValidation::None)
			.timeout(Duration::from_secs(config.request_timeout))
			.build()?;

		let client = Elasticsearch::new(transport);

		Ok(Self { client, buf: Vec::with_capacity(config.bulk_size), config })
	}
}

impl<S: Sendable + Serialize> Sender<S> for ElasticSender {
	type Error = ElasticError;
	async fn send(&mut self, item: BytesMut) -> Result<(), Self::Error> {
		let mut index = json!({
			"index": {
				"_index": self.config.index_name,
			}
		})
		.to_string();
		index.push('\n');
		self.buf.push(BytesMut::from(index.as_bytes()));
		self.buf.push(item);
		if self.buf.len() > self.config.bulk_size * 2 {
			<Self as Sender<S>>::flush(self).await?;
		}
		Ok(())
	}

	async fn flush(&mut self) -> Result<(), Self::Error> {
		if self.buf.is_empty() {
			return Ok(());
		}
		let bulk_body = self.buf.drain(..).collect();
		let response = self.client.bulk(BulkParts::None).body(bulk_body).send().await?;
		let status = response.status_code();
		info!("Elastic response: {}", status);
		if !status.is_success() {
			let err = response.text().await?;

			return Err(ElasticError::Response(err));
		}
		// Check for errors in the response body even if status is 200
		let body: serde_json::Value =
			response.json().await.map_err(|e| ElasticError::Response(e.to_string()))?;
		if let Some(errors) = body.get("errors") {
			if errors.as_bool().unwrap_or(false) {
				log::error!("Elastic bulk response contains errors: {:?}", body);
			}
		}

		Ok(())
	}
}
