use anyhow::anyhow;
use mercury_common::Data;
use sigil::Sigil;
use std::ffi::CStr;
use tokio::sync::mpsc;

pub async fn handle_data(data: Data, tx: &mpsc::Sender<String>) -> anyhow::Result<Option<()>> {
	let sigil = Sigil::new();
	sigil.set();
	println!("{}", data);
	let result = sigil.infer_and_parse(&data.buffer());
	match result {
		Ok(message) => {
			let message = format!(
				"{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}\n",
				data.tgid,
				data.syscall,
				CStr::from_bytes_until_nul(&data.comm)?.to_string_lossy().into_owned(),
				data.quintuple,
				data.timestamp_ns,
				data.enter_seq,
				data.exit_seq,
				data.len,
				data.bytes_sent,
				message.protocol(),
				message.message_type(),
				message.sequences_id(),
				message.summary()
			);
			tx.send(message).await?;
			Ok(Some(()))
		},
		Err(e) => Err(anyhow!(e)),
	}
}
