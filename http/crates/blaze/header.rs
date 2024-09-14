use crate::macros;
use http::{header::InvalidHeaderValue, HeaderValue};
use mime::Mime;

macros::common_header! {
   (ContentType, CONTENT_TYPE) => [Mime]
}

impl ContentType {
    /// Constructs a `Content-Type: application/json` header.
    #[inline]
    pub fn json() -> ContentType { ContentType(mime::APPLICATION_JSON) }

    /// Constructs a `Content-Type: text/plain; charset=utf-8` header.
    #[inline]
    pub fn plaintext() -> ContentType { ContentType(mime::TEXT_PLAIN_UTF_8) }

    /// Constructs a `Content-Type: text/html; charset=utf-8` header.
    #[inline]
    pub fn html() -> ContentType { ContentType(mime::TEXT_HTML_UTF_8) }

    /// Constructs a `Content-Type: text/xml` header.
    #[inline]
    pub fn xml() -> ContentType { ContentType(mime::TEXT_XML) }

    /// Constructs a `Content-Type: application/www-form-url-encoded` header.
    #[inline]
    pub fn form_url_encoded() -> ContentType { ContentType(mime::APPLICATION_WWW_FORM_URLENCODED) }

    /// Constructs a `Content-Type: image/jpeg` header.
    #[inline]
    pub fn jpeg() -> ContentType { ContentType(mime::IMAGE_JPEG) }

    /// Constructs a `Content-Type: image/png` header.
    #[inline]
    pub fn png() -> ContentType { ContentType(mime::IMAGE_PNG) }

    /// Constructs a `Content-Type: application/octet-stream` header.
    #[inline]
    pub fn octet_stream() -> ContentType { ContentType(mime::APPLICATION_OCTET_STREAM) }
}

pub trait TryIntoHeaderValue: Sized {
    fn try_into_value(self) -> Result<HeaderValue, InvalidHeaderValue>;
}

impl From<InvalidHeaderValue> for crate::Error {
    fn from(err: InvalidHeaderValue) -> Self { crate::Error(format!("Invalid header value: {}", err)) }
}

impl TryIntoHeaderValue for HeaderValue {
    #[inline]
    fn try_into_value(self) -> Result<HeaderValue, InvalidHeaderValue> { Ok(self) }
}

impl TryIntoHeaderValue for &HeaderValue {
    #[inline]
    fn try_into_value(self) -> Result<HeaderValue, InvalidHeaderValue> { Ok(self.clone()) }
}

impl TryIntoHeaderValue for ContentType {
    fn try_into_value(self) -> Result<HeaderValue, InvalidHeaderValue> { HeaderValue::from_str(self.0.as_ref()) }
}

impl TryIntoHeaderValue for &str {
    #[inline]
    fn try_into_value(self) -> Result<HeaderValue, InvalidHeaderValue> { self.parse() }
}

impl TryIntoHeaderValue for &[u8] {
    #[inline]
    fn try_into_value(self) -> Result<HeaderValue, InvalidHeaderValue> { HeaderValue::from_bytes(self) }
}

impl TryIntoHeaderValue for Vec<u8> {
    #[inline]
    fn try_into_value(self) -> Result<HeaderValue, InvalidHeaderValue> { HeaderValue::try_from(self) }
}

impl TryIntoHeaderValue for String {
    #[inline]
    fn try_into_value(self) -> Result<HeaderValue, InvalidHeaderValue> { HeaderValue::try_from(self) }
}

impl TryIntoHeaderValue for usize {
    #[inline]
    fn try_into_value(self) -> Result<HeaderValue, InvalidHeaderValue> { HeaderValue::try_from(self.to_string()) }
}

impl TryIntoHeaderValue for i64 {
    #[inline]
    fn try_into_value(self) -> Result<HeaderValue, InvalidHeaderValue> { HeaderValue::try_from(self.to_string()) }
}

impl TryIntoHeaderValue for u64 {
    #[inline]
    fn try_into_value(self) -> Result<HeaderValue, InvalidHeaderValue> { HeaderValue::try_from(self.to_string()) }
}

impl TryIntoHeaderValue for i32 {
    #[inline]
    fn try_into_value(self) -> Result<HeaderValue, InvalidHeaderValue> { HeaderValue::try_from(self.to_string()) }
}

impl TryIntoHeaderValue for u32 {
    #[inline]
    fn try_into_value(self) -> Result<HeaderValue, InvalidHeaderValue> { HeaderValue::try_from(self.to_string()) }
}

impl TryIntoHeaderValue for Mime {
    #[inline]
    fn try_into_value(self) -> Result<HeaderValue, InvalidHeaderValue> { HeaderValue::from_str(self.as_ref()) }
}
