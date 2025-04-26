use std::string::FromUtf8Error;

use bytes::{Buf, Bytes};
use http::HeaderMap;
use http_body::Body;
use http_body_util::BodyExt;

/// Convenient wrapper for reading [`Body`] content from [`http::Response`].
///
/// It is useful in the most common response body reading cases.
#[derive(Debug, Clone)]
pub struct BodyReader<B> {
    body: B,
    #[allow(dead_code)]
    headers: HeaderMap,
}

/// Read body errors.
#[derive(Debug)]
pub enum BodyReaderError<E, D> {
    /// An error occurred while reading the body.
    Read(E),
    /// An error occurred while decoding the body content.
    Decode(D),
}

impl<E, D> std::fmt::Display for BodyReaderError<E, D>
where
    E: std::fmt::Display,
    D: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BodyReaderError::Read(err) => err.fmt(f),
            BodyReaderError::Decode(err) => err.fmt(f),
        }
    }
}

impl<E, D> std::error::Error for BodyReaderError<E, D>
where
    E: std::fmt::Debug + std::fmt::Display,
    D: std::fmt::Debug + std::fmt::Display,
{
}

impl<B> BodyReader<B> {
    /// Reads the full response body as [`Bytes`].
    ///
    /// # Example
    ///
    /// ```
    #[doc = include_str!("../examples/read_bytes.rs")]
    /// ```
    pub async fn bytes(self) -> Result<Bytes, B::Error>
    where
        B: Body,
        B::Data: Buf,
    {
        let body_bytes = self.body.collect().await?.to_bytes();
        Ok(body_bytes)
    }

    /// Reads the full response text.
    ///
    /// # Note
    ///
    /// The method will only attempt to decode the response as `UTF-8`, regardless of the
    /// `Content-Type` header.
    ///
    /// # Errors
    ///
    /// This method fails if the response body cannot be decoded as UTF-8.
    ///
    /// # Example
    ///
    /// ```
    #[doc = include_str!("../examples/read_utf8.rs")]
    /// ```
    pub async fn utf8(self) -> Result<String, BodyReaderError<B::Error, FromUtf8Error>>
    where
        B: Body,
        B::Data: Buf,
    {
        let bytes = self.bytes().await.map_err(BodyReaderError::Read)?;
        String::from_utf8(bytes.into()).map_err(BodyReaderError::Decode)
    }

    /// Deserializes the response body as JSON.
    ///
    /// # Errors
    ///
    /// This method fails whenever the response body is not valid JSON
    /// or it cannot be properly deserialized to the target type `T`.
    ///
    /// # Examples
    ///
    /// ```
    #[doc = include_str!("../examples/read_json.rs")]
    /// ```
    #[cfg(feature = "json")]
    #[cfg_attr(docsrs, doc(cfg(feature = "json")))]
    pub async fn json<T>(self) -> Result<T, BodyReaderError<B::Error, serde_json::Error>>
    where
        T: serde::de::DeserializeOwned,
        B: Body,
        B::Data: Buf,
    {
        let bytes = self.bytes().await.map_err(BodyReaderError::Read)?;
        serde_json::from_slice(&bytes).map_err(BodyReaderError::Decode)
    }

    /// Deserializes the response body as form data.
    ///
    /// # Errors
    ///
    /// This method fails whenever the response body is not valid form data
    /// or it cannot be properly deserialized to the target type `T`.
    ///
    /// # Examples
    ///
    /// ```
    #[doc = include_str!("../examples/read_form.rs")]
    /// ```
    #[cfg(feature = "form")]
    #[cfg_attr(docsrs, doc(cfg(feature = "form")))]
    pub async fn form<T>(self) -> Result<T, BodyReaderError<B::Error, serde_urlencoded::de::Error>>
    where
        T: serde::de::DeserializeOwned,
        B: Body,
        B::Data: Buf,
    {
        let bytes = self.bytes().await.map_err(BodyReaderError::Read)?;
        serde_urlencoded::from_bytes(&bytes).map_err(BodyReaderError::Decode)
    }

    /// Maps the body content using the provided function.
    ///
    /// # Example
    ///
    /// ```
    #[doc = include_str!("../examples/map_body.rs")]
    /// ```
    pub fn map<F, T>(self, f: F) -> BodyReader<T>
    where
        F: FnOnce(B) -> T,
        T: Body,
        T::Data: Buf,
    {
        BodyReader {
            body: f(self.body),
            headers: self.headers,
        }
    }
}

impl<B> From<http::Response<B>> for BodyReader<B> {
    fn from(response: http::Response<B>) -> Self {
        let (parts, body) = response.into_parts();
        Self {
            body,
            headers: parts.headers,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::BodyReaderError;

    type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

    // Check that the error can be converted into a boxed error.
    #[test]
    fn test_body_reader_error_into_boxed() {
        // Read error
        let read_error = std::io::Error::new(std::io::ErrorKind::Other, "read error");
        let error: BodyReaderError<std::io::Error, BoxError> = BodyReaderError::Read(read_error);
        let boxed_error: BoxError = Box::new(error);

        assert_eq!(boxed_error.to_string(), "read error");
        // Decode error
        let decode_error = std::io::Error::new(std::io::ErrorKind::Other, "decode error");
        let error: BodyReaderError<BoxError, std::io::Error> =
            BodyReaderError::Decode(decode_error);
        let boxed_error: BoxError = Box::new(error);

        assert_eq!(boxed_error.to_string(), "decode error");
    }
}
