use super::CsvEncoder;
use csv_core::{QuoteStyle, Terminator, WriterBuilder};
/// Builds a [`CsvEncoder`] with various configuration knobs.
///
/// This builder can be used to tweak the field delimiter, record terminator
/// and more. Once a [`CsvEncoder`] is built, its configuration cannot be
/// changed.
#[derive(Debug)]
pub struct CsvEncoderBuilder {
	builder: WriterBuilder,
	capacity: usize,
	flexible: bool,
	has_headers: bool,
}

impl Default for CsvEncoderBuilder {
	fn default() -> CsvEncoderBuilder {
		CsvEncoderBuilder {
			builder: WriterBuilder::default(),
			capacity: 8 * (1 << 10),
			flexible: false,
			has_headers: true,
		}
	}
}

impl CsvEncoderBuilder {
	/// Create a new builder for configuring CSV writing.
	///
	/// To convert a builder into a writer, call one of the methods starting
	/// with `from_`.
	pub fn new() -> Self {
		let mut builder = CsvEncoderBuilder::default();
		builder.quote_style(QuoteStyle::Never);
		builder
	}

	/// The field delimiter to use when writing CSV.
	///
	/// The default is `b','`.
	pub fn delimiter(&mut self, delimiter: u8) -> &mut Self {
		self.builder.delimiter(delimiter);
		self
	}

	/// Whether to write a header row before writing any other row.
	///
	/// When this is enabled and the `serialize` method is used to write data
	/// with something that contains field names (i.e., a struct), then a
	/// header row is written containing the field names before any other row
	/// is written.
	///
	/// This option has no effect when using other methods to write rows. That
	/// is, if you don't use `serialize`, then you must write your header row
	/// explicitly if you want a header row.
	///
	/// This is enabled by default.
	pub const fn has_headers(&mut self, yes: bool) -> &mut Self {
		self.has_headers = yes;
		self
	}

	/// Whether the number of fields in records is allowed to change or not.
	///
	/// When disabled (which is the default), writing CSV data will return an
	/// error if a record is written with a number of fields different from the
	/// number of fields written in a previous record.
	///
	/// When enabled, this error checking is turned off.
	pub const fn flexible(&mut self, yes: bool) -> &mut Self {
		self.flexible = yes;
		self
	}

	/// The record terminator to use when writing CSV.
	///
	/// A record terminator can be any single byte. The default is `\n`.
	///
	/// Note that RFC 4180 specifies that record terminators should be `\r\n`.
	/// To use `\r\n`, use the special `Terminator::CRLF` value.
	pub fn terminator(&mut self, terminator: Terminator) -> &mut Self {
		self.builder.terminator(terminator);
		self
	}

	/// The quoting style to use when writing CSV.
	///
	/// By default, this is set to `QuoteStyle::Necessary`, which will only
	/// use quotes when they are necessary to preserve the integrity of data.
	///
	/// Note that unless the quote style is set to `Never`, an empty field is
	/// quoted if it is the only field in a record.
	pub fn quote_style(&mut self, style: QuoteStyle) -> &mut Self {
		self.builder.quote_style(style);
		self
	}

	/// The quote character to use when writing CSV.
	///
	/// The default is `b'"'`.
	pub fn quote(&mut self, quote: u8) -> &mut Self {
		self.builder.quote(quote);
		self
	}

	/// Enable double quote escapes.
	///
	/// This is enabled by default, but it may be disabled. When disabled,
	/// quotes in field data are escaped instead of doubled.
	pub fn double_quote(&mut self, yes: bool) -> &mut Self {
		self.builder.double_quote(yes);
		self
	}

	/// The escape character to use when writing CSV.
	///
	/// In some variants of CSV, quotes are escaped using a special escape
	/// character like `\` (instead of escaping quotes by doubling them).
	///
	/// By default, writing these idiosyncratic escapes is disabled, and is
	/// only used when `double_quote` is disabled.
	pub fn escape(&mut self, escape: u8) -> &mut Self {
		self.builder.escape(escape);
		self
	}

	/// The comment character that will be used when later reading the file.
	///
	/// If `quote_style` is set to `QuoteStyle::Necessary`, a field will
	/// be quoted if the comment character is detected anywhere in the field.
	///
	/// The default value is None.
	pub fn comment(&mut self, comment: Option<u8>) -> &mut Self {
		self.builder.comment(comment);
		self
	}

	/// Set the capacity (in bytes) of the internal buffer used in the CSV
	/// writer. This defaults to a reasonable setting.
	pub const fn buffer_capacity(&mut self, capacity: usize) -> &mut Self {
		self.capacity = capacity;
		self
	}

	pub fn build(self) -> CsvEncoder {
		CsvEncoder {
			buffer: vec![0; self.capacity],
			capacity: self.capacity,
			flexible: self.flexible,
			has_headers: self.has_headers,
			writer: self.builder.build(),
		}
	}
}
