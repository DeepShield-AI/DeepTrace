/// HTTP/1.1 200 OK\r\n (HTTP response is 17 characters)
/// GET x HTTP/1.1\r\n (HTTP response is 16 characters)
/// MAY be without "OK", ref: <https://www.rfc-editor.org/rfc/rfc7231>
pub(super) const HTTP1_MIN_SIZE: u32 = 15;
