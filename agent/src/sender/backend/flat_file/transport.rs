use super::{Error, utils};
use crate::{
	app::runtime::spawn,
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

const PREFIX: &[u8] = b"{\n\t\"spans\": [";
const SUFFIX: &[u8] = b"\n\t]\n}";
const SEPARATOR: &[u8] = b",\n";

pub struct FlatFile {
	output: BufWriter<File>,
	path: PathBuf,
	written_size: usize,
	first: bool,
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
			create_dir_all(dir).await.map_err(|e| Error::IO(e))?;
		}

		let file = OpenOptions::new()
			.create(true)
			.write(true)
			.truncate(true)
			.open(&path)
			.await
			.map_err(Error::IO)?;
		let writer = BufWriter::with_capacity(c.file_buffer_size, file);
		Ok(FlatFile {
			output: writer,
			path: PathBuf::from(&path),
			written_size: 0,
			first: true,
			buf: BytesMut::with_capacity(c.mem_buffer_size),
			config,
		})
	}
	async fn rotate_file(&mut self) -> Result<(), Error> {
		self.output.flush().await?;
		let path = utils::format_filename(&self.path);
		let file = OpenOptions::new().create(true).write(true).open(&path).await?;

		let old = mem::replace(
			&mut self.output,
			BufWriter::with_capacity(self.config.load().file_buffer_size, file),
		);

		spawn(async move {
			let _ = old.into_inner().shutdown().await;
		});

		self.path = path;
		self.written_size = 0;
		self.first = true;
		Ok(())
	}
}

impl<S: Sendable + Serialize> TransportStrategy<S> for FlatFile {
	type Error = Error;
	async fn send(&mut self, item: S) -> Result<(), Self::Error> {
		let config = self.config.load();
		let json = serde_json::to_vec_pretty(&item)?;
		if self.buf.len() + json.len() > config.mem_buffer_size {
			self.buf.extend_from_slice(SUFFIX);
			<Self as TransportStrategy<S>>::flush(self).await?;
			self.buf.clear();
		}

		if !self.first {
			self.buf.extend_from_slice(SEPARATOR);
		} else {
			self.buf.extend_from_slice(PREFIX);
			self.first = false;
		}

		self.buf.extend_from_slice(&json);
		Ok(())
	}

	async fn flush(&mut self) -> Result<(), Self::Error> {
		if !self.buf.is_empty() {
			self.output.write_all(&self.buf).await?;
			self.output.flush().await?;
			self.written_size += self.buf.len();
			self.buf.clear();
		}

		if self.written_size > self.config.load().file_size_limit {
			self.rotate_file().await?;
		}
		Ok(())
	}
}
