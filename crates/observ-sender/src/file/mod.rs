use super::SendError;
use crate::Sendable;
use bytes::BytesMut;
use chrono::{DateTime, Duration, FixedOffset, TimeZone, Utc};
use log::info;
use observ_core::Sender;
use serde::Serialize;
use std::{
	fs::{OpenOptions, create_dir_all},
	mem,
	path::PathBuf,
	time::SystemTime,
};
use tokio::{
	fs::{File, rename},
	io::{AsyncWriteExt, BufWriter},
};
mod utils;

const CST_OFFSET: i32 = 8 * 3600;

pub struct FileSender {
	writer: BufWriter<File>,
	base_path: PathBuf,
	current_path: PathBuf,
	written_size: usize,
	buffer: BytesMut,
	next_rotate_time: SystemTime,
	current_date: String,
}

impl FileSender {
	pub fn new(path: impl AsRef<str>) -> Result<Self, SendError> {
		let base_path = PathBuf::from(path.as_ref());
		let now_cst = Self::current_cst_time();
		let current_date = now_cst.format("%Y%m%d").to_string();

		// crate data folder
		let date_folder = base_path
			.parent()
			.map(|p| p.join(&current_date))
			.unwrap_or_else(|| PathBuf::from(&current_date));

		create_dir_all(&date_folder).map_err(SendError::IO)?;

		let filename = base_path.file_name().unwrap_or_else(|| "agent.log".as_ref());
		let file_path = date_folder.join(filename);

		let file = OpenOptions::new()
			.create(true)
			.write(true)
			.truncate(true)
			.open(&file_path)
			.map_err(SendError::IO)?;

		let file = File::from_std(file);
		let writer = BufWriter::with_capacity(4 << 20, file);

		// calc next rotate time
		let next_rotate_time = Self::next_day_start_cst();

		Ok(Self {
			writer,
			base_path,
			current_path: file_path,
			written_size: 0,
			buffer: BytesMut::with_capacity(1 << 19),
			next_rotate_time,
			current_date,
		})
	}

	fn current_cst_time() -> DateTime<FixedOffset> {
		let utc_now = Utc::now();
		let cst_offset = FixedOffset::east_opt(CST_OFFSET).expect("Invalid offset");
		utc_now.with_timezone(&cst_offset)
	}

	fn next_day_start_cst() -> SystemTime {
		let now_cst = Self::current_cst_time();
		let next_rotate_cst = (now_cst.date_naive() + Duration::days(1))
			.and_hms_opt(0, 0, 0)
			.expect("Invalid time");

		// convert to DateTime with FixedOffset
		let cst_offset = FixedOffset::east_opt(CST_OFFSET).expect("Invalid offset");
		let next_rotate = cst_offset
			.from_local_datetime(&next_rotate_cst)
			.earliest()
			.expect("Invalid time");

		// convert to SystemTime
		SystemTime::from(next_rotate)
	}

	fn should_rotate_by_time(&self) -> bool {
		SystemTime::now() >= self.next_rotate_time
	}

	const fn should_rotate_by_size(&self) -> bool {
		self.written_size >= 512 * 1024 * 1024
	}

	async fn switch_to_new_date(&mut self) -> Result<(), SendError> {
		if !self.buffer.is_empty() {
			self.writer.write_all(&self.buffer).await.map_err(SendError::IO)?;
			self.writer.flush().await.map_err(SendError::IO)?;
			self.written_size += self.buffer.len();
			self.buffer.clear();
		} else {
			self.writer.flush().await.map_err(SendError::IO)?;
		}

		let new_date = Self::current_cst_time().format("%Y%m%d").to_string();

		let date_folder = self
			.base_path
			.parent()
			.map(|p| p.join(&new_date))
			.unwrap_or_else(|| PathBuf::from(&new_date));

		create_dir_all(&date_folder).map_err(SendError::IO)?;

		let filename = self.base_path.file_name().unwrap_or_else(|| "agent.log".as_ref());
		let new_path = date_folder.join(filename);

		let file = OpenOptions::new()
			.create(true)
			.write(true)
			.truncate(true)
			.open(&new_path)
			.map_err(SendError::IO)?;

		let old =
			mem::replace(&mut self.writer, BufWriter::with_capacity(4 << 20, File::from_std(file)));
		let _ = old.into_inner().shutdown().await;

		self.current_path = new_path;
		self.written_size = 0;
		self.current_date = new_date;
		self.next_rotate_time = Self::next_day_start_cst();

		Ok(())
	}

	async fn rotate_file(&mut self) -> Result<(), SendError> {
		if !self.buffer.is_empty() {
			self.writer.write_all(&self.buffer).await.map_err(SendError::IO)?;
			self.writer.flush().await.map_err(SendError::IO)?;
			self.written_size += self.buffer.len();
			self.buffer.clear();
		} else {
			self.writer.flush().await.map_err(SendError::IO)?;
		}

		let rotated = utils::format_filename(&self.current_path, "%Y%m%d_%H%M%S");
		rename(&self.current_path, &rotated).await.map_err(SendError::IO)?;

		let file = OpenOptions::new()
			.create(true)
			.write(true)
			.truncate(true)
			.open(&self.current_path)
			.map_err(SendError::IO)?;

		let old =
			mem::replace(&mut self.writer, BufWriter::with_capacity(4 << 20, File::from_std(file)));
		let _ = old.into_inner().shutdown().await;
		self.written_size = 0;

		Ok(())
	}
}

impl<S: Sendable + Serialize> Sender<S> for FileSender {
	type Error = SendError;

	async fn send(&mut self, item: BytesMut) -> Result<(), Self::Error> {
		if self.should_rotate_by_time() {
			self.switch_to_new_date().await?;
		}
		let config = observ_config::file_sender_config("metric");
		self.buffer.extend_from_slice(&item);
		if self.buffer.len() > (1 << 14) {
			<Self as Sender<S>>::flush(self).await?;
		}
		Ok(())
	}

	async fn flush(&mut self) -> Result<(), Self::Error> {
		if !self.buffer.is_empty() {
			self.writer.write_all(&self.buffer).await?;
			self.writer.flush().await?;
			self.written_size += self.buffer.len();
			self.buffer.clear();
		}

		if self.should_rotate_by_size() {
			self.rotate_file().await?;
		}
		Ok(())
	}
}
