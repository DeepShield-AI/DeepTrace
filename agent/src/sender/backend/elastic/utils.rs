use super::Error;
use elasticsearch::{Elasticsearch, indices::IndicesPutSettingsParts};
use log::warn;
use serde_json::{Value, json};

pub async fn set_refresh_interval(
	client: &Elasticsearch,
	index: &str,
	interval: Value,
) -> Result<(), Error> {
	let response = client
		.indices()
		.put_settings(IndicesPutSettingsParts::Index(&[index]))
		.body(json!({
			"index" : {
				"refresh_interval" : interval
			}
		}))
		.send()
		.await?;

	if !response.status_code().is_success() {
		warn!("Failed to update refresh interval");
	}

	Ok(())
}
