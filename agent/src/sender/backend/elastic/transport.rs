use super::{Error, utils};
use crate::{
	config::{ElasticAccess, elastic_config},
	sender::{
		SendError,
		strategy::{Sendable, TransportStrategy},
	},
};
use arc_swap::access::Access;
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
use std::time::Duration;
use url::Url;

pub struct Elastic {
	client: Elasticsearch,
	config: ElasticAccess,
	buf: Vec<JsonBody<serde_json::Value>>,
}

impl Elastic {
	pub async fn new() -> Result<Self, SendError> {
		let config = elastic_config();
		let c = config.load();
		let url = Url::parse(&c.node_url).map_err(|e| Error::Url(e))?;
		let conn_pool = SingleNodeConnectionPool::new(url);

		let transport = TransportBuilder::new(conn_pool)
			.disable_proxy()
			.auth(Credentials::Basic(c.username.clone(), c.password.clone()))
			.timeout(Duration::from_secs(c.request_timeout))
			.build()
			.map_err(Error::Build)?;

		info!("Connecting to Elasticsearch at {}", c.node_url);

		let client = Elasticsearch::new(transport);
		utils::set_refresh_interval(&client, &c.index_name, json!("-1")).await?;

		Ok(Self { client, buf: Vec::with_capacity(c.bulk_size), config })
	}
}

impl<S: Sendable + Serialize> TransportStrategy<S> for Elastic {
	type Error = Error;
	async fn send(&mut self, item: S) -> Result<(), Self::Error> {
		self.buf.push(
			json!({
				"index": {
					"_index": self.config.load().index_name,
				}
			})
			.into(),
		);
		self.buf.push(json!(item).into());
		if self.buf.len() >= 2 * self.config.load().bulk_size {
			<Self as TransportStrategy<S>>::flush(self).await?;
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
		debug!("Bulk response status: {}", status);

		if status.is_success() {
			Ok(())
		} else {
			let error_msg = response.text().await?;
			error!("Elasticsearch error: {error_msg}");
			Err(Error::Response(error_msg))
		}
	}
}
