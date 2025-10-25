use super::{CodecEncodeError, Encoder};
use bytes::BytesMut;
use csv_core::{WriteResult, Writer};
use observ_core::Sendable;
mod builder;
pub use builder::CsvEncoderBuilder;
use bytes::{BufMut, buf::UninitSlice};
// use serde::Serialize;

/// An already configured CSV writer.
///
/// A CSV writer takes as input Rust values and writes those values in a valid
/// CSV format as output.
///
/// While CSV writing is considerably easier than parsing CSV, a proper writer
/// will do a number of things for you:
///
/// 1. Quote fields when necessary.
/// 2. Check that all records have the same number of fields.
/// 3. Write records with a single empty field correctly.
/// 4. Automatically serialize normal Rust types to CSV records. When that
///    type is a struct, a header row is automatically written corresponding
///    to the fields of that struct.
/// 5. Use buffering intelligently and otherwise avoid allocation. (This means
///    that callers should not do their own buffering.)
///
/// All of the above can be configured using a
/// [`WriterBuilder`](struct.WriterBuilder.html).
/// However, a `Writer` has a couple of convenience constructors (`from_path`
/// and `from_writer`) that use the default configuration.
///
/// Note that the default configuration of a `Writer` uses `\n` for record
/// terminators instead of `\r\n` as specified by RFC 4180. Use the
/// `terminator` method on `WriterBuilder` to set the terminator to `\r\n` if
/// it's desired.
pub struct CsvEncoder {
	buffer: Vec<u8>,
	capacity: usize,
	flexible: bool,
	has_headers: bool,
	writer: Writer,
}

// impl<S> Encoder<S> for CsvEncoder
// where S: Sendable + Serialize {
// 	type Error = CodecEncodeError;
//
// 	fn encode(&mut self, record: S, out: &mut BytesMut) -> Result<(), Self::Error> {
// 		let hint = if self.capacity > 0 { self.capacity } else { 8 * 1024 };
// 		out.reserve(hint);
//
// 		self.buffer.clear();
// 		record.encode(&mut self.buffer)?;
//
// 		let mut field = &self.buffer[..];
//
// 		loop {
// 			let (res, read, written) = {
// 				let chunk = out.chunk_mut();
// 				let buf = as_init_slice(chunk);
// 				self.writer.field(field, buf)
// 			};
// 			field = &field[read..];
// 			unsafe { out.advance_mut(written) };
// 			match res {
// 				WriteResult::InputEmpty => break,
// 				WriteResult::OutputFull => out.reserve(8 * 1024),
// 			}
// 		}
//
// 		loop {
// 			let res = {
// 				let chunk = out.chunk_mut();
// 				let buf = as_init_slice(chunk);
// 				let (res, n) = self.writer.terminator(buf);
// 				unsafe { out.advance_mut(n) };
// 				res
// 			};
// 			if matches!(res, WriteResult::InputEmpty) {
// 				break;
// 			}
// 			out.reserve(8 * 1024);
// 		}
//
// 		Ok(())
// 	}
// }

impl<I, S> Encoder<I> for CsvEncoder
where
	I: IntoIterator<Item = S>,
	S: Sendable,
{
	type Error = CodecEncodeError;

	fn encode(&mut self, records: I, out: &mut BytesMut) -> Result<(), Self::Error> {
		let hint = if self.capacity > 0 { self.capacity } else { 8 * 1024 };
		out.reserve(hint);

		#[inline]
		fn write_delimiter(writer: &mut Writer, out: &mut BytesMut) {
			loop {
				let res = {
					let chunk = out.chunk_mut();
					let buf = as_init_slice(chunk);
					let (res, n) = writer.delimiter(buf);
					unsafe { out.advance_mut(n) };
					res
				};
				if matches!(res, WriteResult::InputEmpty) {
					break;
				}
				out.reserve(8 * 1024);
			}
		}

		#[inline]
		fn write_field(writer: &mut Writer, mut field: &[u8], out: &mut BytesMut) {
			loop {
				let (res, read, written) = {
					let chunk = out.chunk_mut();
					let buf = as_init_slice(chunk);
					writer.field(field, buf)
				};
				field = &field[read..];
				unsafe { out.advance_mut(written) };
				match res {
					WriteResult::InputEmpty => break,
					WriteResult::OutputFull => out.reserve(8 * 1024),
				}
			}
		}

		#[inline]
		fn write_finish(writer: &mut Writer, out: &mut BytesMut) {
			loop {
				let res = {
					let chunk = out.chunk_mut();
					let buf = as_init_slice(chunk);
					let (res, n) = writer.finish(buf);
					unsafe { out.advance_mut(n) };
					res
				};
				if matches!(res, WriteResult::InputEmpty) {
					break;
				}
				out.reserve(8 * 1024);
			}
		}

		#[inline]
		fn write_terminator(writer: &mut Writer, out: &mut BytesMut) {
			loop {
				let res = {
					let chunk = out.chunk_mut();
					let buf = as_init_slice(chunk);
					let (res, n) = writer.terminator(buf);
					unsafe { out.advance_mut(n) };
					res
				};
				if matches!(res, WriteResult::InputEmpty) {
					break;
				}
				out.reserve(8 * 1024);
			}
		}

		for field in records.into_iter() {
			let mut buf = BytesMut::new();
			let _ = field.encode(&mut buf);
			write_field(&mut self.writer, &buf, out);
			write_terminator(&mut self.writer, out);
		}
		Ok(())
	}
}

#[inline]
fn as_init_slice(chunk: &mut UninitSlice) -> &mut [u8] {
	let len = chunk.len();
	let ptr = chunk.as_mut_ptr();
	unsafe { std::slice::from_raw_parts_mut(ptr, len) }
}

#[test]
fn serialize_fields() {
	let event = vec![
		"bar".to_string(),
		123.to_string(),
		"abc,bcd".to_string(),
		3.1415925.to_string(),
		"sp ace".to_string(),
		"2023-02-27T15:04:49.363+08:00".to_string(),
		"the \"quote\" should be escaped".to_string(),
		true.to_string(),
		"data".to_string(),
	];
	let mut encoder = CsvEncoderBuilder::new().build();
	let mut bytes = BytesMut::new();

	encoder.encode(event.clone(), &mut bytes).unwrap();
	// encoder.encode(event, &mut bytes).unwrap();
	println!("{}", String::from_utf8_lossy(&bytes));
	assert_eq!(
			bytes.freeze(),
			b"bar,123,\"abc,bcd\",3.1415925,sp ace,2023-02-27T15:04:49.363+08:00,\"the \"\"quote\"\" should be escaped\",true,data".as_slice()
		);
}
#[test]
fn correct_quoting() {
	let event = vec![
		"hello world".to_string(),
		1.to_string(),
		"foo\"bar".to_string(),
		"baz,bas".to_string(),
	];

	let mut default_bytes = BytesMut::new();
	let mut never_bytes = BytesMut::new();
	let mut always_bytes = BytesMut::new();
	let mut non_numeric_bytes = BytesMut::new();

	CsvEncoderBuilder::new()
		.build()
		.encode(event.clone(), &mut default_bytes)
		.unwrap();

	CsvEncoderBuilder::new()
		.quote_style(csv_core::QuoteStyle::Never)
		.build()
		.encode(event.clone(), &mut never_bytes)
		.unwrap();

	CsvEncoderBuilder::new()
		.quote_style(csv_core::QuoteStyle::Always)
		.build()
		.encode(event.clone(), &mut always_bytes)
		.unwrap();

	CsvEncoderBuilder::new()
		.quote_style(csv_core::QuoteStyle::NonNumeric)
		.build()
		.encode(event.clone(), &mut non_numeric_bytes)
		.unwrap();

	assert_eq!(default_bytes.freeze(), b"hello world,1,\"foo\"\"bar\",\"baz,bas\"".as_slice());
	assert_eq!(never_bytes.freeze(), b"hello world,1,foo\"bar,baz,bas".as_slice());
	assert_eq!(
		always_bytes.freeze(),
		b"\"hello world\",\"1\",\"foo\"\"bar\",\"baz,bas\"".as_slice()
	);
	assert_eq!(
		non_numeric_bytes.freeze(),
		b"\"hello world\",1,\"foo\"\"bar\",\"baz,bas\"".as_slice()
	);
}

#[test]
fn custom_delimiter() {
	let event = vec!["value1".to_string(), "value2".to_string()];
	let mut encoder = CsvEncoderBuilder::new().delimiter(b'\t').build();

	let mut bytes = BytesMut::new();

	encoder.encode(event, &mut bytes).unwrap();

	assert_eq!(bytes.freeze(), b"value1\tvalue2".as_slice());
}

#[test]
fn custom_escape_char() {
	let mut encoder = CsvEncoderBuilder::new().escape(b'\\').double_quote(false).build();
	let event = vec!["foo\"bar".to_string()];

	let mut bytes = BytesMut::new();

	encoder.encode(event, &mut bytes).unwrap();

	assert_eq!(bytes.freeze(), b"\"foo\\\"bar\"".as_slice());
}

#[test]
fn custom_quote_char() {
	let event = vec!["foo \" $ bar".to_string()];
	let mut encoder = CsvEncoderBuilder::new().quote(b'$').build();
	let mut bytes = BytesMut::new();

	encoder.encode(event, &mut bytes).unwrap();
	// TODO:
	assert_eq!(bytes.freeze(), b"$foo \" $$ bar$$$\n".as_slice());
}

#[test]
fn more_input_then_capacity() {
	let event = vec!["foo bar".to_string()];
	let mut encoder = CsvEncoderBuilder::new().buffer_capacity(3).build();

	let mut bytes = BytesMut::new();

	encoder.encode(event, &mut bytes).unwrap();

	assert_eq!(bytes.freeze(), b"foo bar\n".as_slice());
}
