use super::{Error, utils};
use crate::{
	config::{FlatFileAccess, flat_file_config},
	sender::{SendError, Sendable, TransportStrategy},
};
use arc_swap::access::Access;
use bytes::BytesMut;
use serde::Serialize;
use std::{
	mem,
	path::{Path, PathBuf},
};
use tokio::{
	fs::{File, OpenOptions, create_dir_all},
	io::{AsyncWriteExt, BufWriter},
};

const SEPARATOR: &[u8] = b"\n";

pub struct FlatFile {
	output: BufWriter<File>,
	path: PathBuf,
	written_size: usize,
	buf: BytesMut,
	config: FlatFileAccess,
}

impl FlatFile {
	pub async fn new(path: impl AsRef<str>) -> Result<Self, SendError> {
		let config = flat_file_config();
		let c = config.load();
		let path = PathBuf::from(path.as_ref());

		let dir = path.parent().unwrap_or_else(|| Path::new(""));
		if !dir.exists() {
			create_dir_all(dir).await.map_err(Error::IO)?;
		}

		let file = OpenOptions::new()
			.create(true)
			.write(true)
			.truncate(true)
			.open(&path)
			.await
			.map_err(Error::IO)?;
		let writer = BufWriter::with_capacity(c.file_buffer_size << 20, file);
		Ok(FlatFile {
			output: writer,
			path,
			written_size: 0,
			buf: BytesMut::with_capacity(c.mem_buffer_size),
			config,
		})
	}
	async fn rotate_file(&mut self) -> Result<(), Error> {
		self.output.write_all(&self.buf).await?;
		self.output.flush().await?;

		let path = utils::format_filename(&self.path);
		let file = OpenOptions::new().create(true).truncate(true).write(true).open(&path).await?;

		let old = mem::replace(
			&mut self.output,
			BufWriter::with_capacity(self.config.load().file_buffer_size << 20, file),
		);

		let _ = old.into_inner().shutdown().await;

		self.path = path;
		self.written_size = 0;
		Ok(())
	}
}

impl<S: Sendable + Serialize> TransportStrategy<S> for FlatFile {
	type Error = Error;
	async fn send(&mut self, item: S) -> Result<(), Self::Error> {
		let config = self.config.load();
		let json = serde_json::to_vec(&item)?;
		if self.buf.len() + json.len() > config.mem_buffer_size << 20 {
			<Self as TransportStrategy<S>>::flush(self).await?;
		}

		self.buf.extend_from_slice(&json);
		self.buf.extend_from_slice(SEPARATOR);

		Ok(())
	}

	async fn flush(&mut self) -> Result<(), Self::Error> {
		if !self.buf.is_empty() {
			self.output.write_all(&self.buf).await?;
			self.output.flush().await?;
			self.written_size += self.buf.len();
			self.buf.clear();
		}

		if self.written_size > self.config.load().file_size_limit << 20 {
			self.rotate_file().await?;
		}
		Ok(())
	}
}
