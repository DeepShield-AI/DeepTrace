use flate2::{
	Compression,
	write::{GzEncoder, ZlibEncoder},
};
use std::io::Write;
pub enum Encoder {
	Raw,
	Zlib,
	Gzip,
}

impl Encoder {
	pub fn encode(&self, encode_buffer: &[u8]) -> std::io::Result<Option<Vec<u8>>> {
		let result = match self {
			Self::Raw => None,
			Self::Zlib => {
				let mut encoder =
					ZlibEncoder::new(Vec::with_capacity(encode_buffer.len()), Compression::best());
				encoder.write_all(encode_buffer)?;
				Some(encoder.finish()?)
			},
			Self::Gzip => {
				let mut encoder =
					GzEncoder::new(Vec::with_capacity(encode_buffer.len()), Compression::default());
				encoder.write_all(encode_buffer)?;
				Some(encoder.finish()?)
			}, // Self::Zstd => {
			   //     let mut encoder = ZstdEncoder::new(Vec::with_capacity(encode_buffer.len()), 0)?;
			   //     encoder.write_all(&encode_buffer)?;
			   //     Some(encoder.finish()?)
			   // }
		};
		Ok(result)
	}
}
