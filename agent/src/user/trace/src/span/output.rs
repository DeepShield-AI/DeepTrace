use log::error;
use std::sync::Arc;
use tokio::{
	fs::File,
	io::{AsyncWriteExt, BufWriter},
	sync::{
		Mutex,
		mpsc::UnboundedReceiver,
	},
};

use crate::FLUSH_INTERVAL;
pub async fn spans_output(
	writer: Arc<Mutex<BufWriter<File>>>,
	mut span_receiver: UnboundedReceiver<String>,
) {
	let mut count = 0;
	let mut writer = writer.lock().await;
	writer.write_all(b"{\n\t\"spans\": [").await.unwrap();
	while let Some(line) = span_receiver.recv().await {
		if let Err(e) = writer.write_all(line.as_bytes()).await {
			error!("Failed to write span: {}", e);
			break;
		}
		writer.write_all(b",\n").await.unwrap();
		count += 1;
		if count % FLUSH_INTERVAL == 0 {
			if let Err(e) = writer.flush().await {
				error!("Flush span failed: {}", e);
				break;
			}
		}
	}
	if let Err(e) = writer.flush().await {
		error!("Final flush failed: {}", e);
	}
}
