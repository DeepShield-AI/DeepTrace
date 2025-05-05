use log::error;
use tokio::{
	io::{AsyncWrite, AsyncWriteExt},
	sync::mpsc::UnboundedReceiver
};

use crate::FLUSH_INTERVAL;

pub async fn ebpf_output(
	mut writer: impl AsyncWrite + Unpin + Send,
	mut ebpf_receiver: UnboundedReceiver<String>,
) {
	let mut count = 0;
	while let Some(line) = ebpf_receiver.recv().await {
		if let Err(e) = writer.write_all(line.as_bytes()).await {
			error!("Failed to write: {}", e);
			break;
		}
		count += 1;
		if count % FLUSH_INTERVAL == 0 {
			if let Err(e) = writer.flush().await {
				error!("Flush failed: {}", e);
				break;
			}
		}
	}
	if let Err(e) = writer.flush().await {
		error!("Final flush failed: {}", e);
	}
}