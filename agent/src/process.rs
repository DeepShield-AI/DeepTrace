use anyhow::anyhow;
use log::error;
use mercury_common::structs::Data;
use std::ffi::CStr;
use tokio::{
	io::{AsyncWrite, AsyncWriteExt},
	sync::mpsc,
};

const FLUSH_INTERVAL: usize = 1;

pub async fn handle_output(
	mut writer: impl AsyncWrite + Unpin + Send,
	mut global_rx: mpsc::UnboundedReceiver<String>,
) {
	let mut count = 0;
	while let Some(line) = global_rx.recv().await {
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

// pub async fn handle_data(data: Data, tx: &mpsc::Sender<String>) -> anyhow::Result<Option<()>> {
// 	let sigil = Sigil::new();
// 	sigil.set();
// 	println!("{}", data);
// 	let result = sigil.infer_and_parse(&data.buffer());
// 	match result {
// 		Ok(message) => {
// 			let message = format!(
// 				"{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}\n",
// 				data.tgid,
// 				data.syscall,
// 				CStr::from_bytes_until_nul(&data.comm)?.to_string_lossy().into_owned(),
// 				data.quintuple,
// 				data.timestamp_ns,
// 				data.enter_seq,
// 				data.exit_seq,
// 				data.len,
// 				// data.bytes_sent,
// 				message.protocol(),
// 				message.message_type(),
// 				message.sequences_id(),
// 				message.summary()
// 			);
// 			tx.send(message).await?;
// 			Ok(Some(()))
// 		},
// 		Err(e) => Err(anyhow!(e)),
// 	}
// }
