use std::string::FromUtf8Error;

use bytes::{Buf, Bytes};
use http::HeaderMap;
use http_body::Body;
use http_body_util::BodyExt;
use thiserror::Error;

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
#[derive(Debug, Error)]
#[error(transparent)]
pub enum BodyReaderError<E, D> {
    /// An error occurred while reading the body.
    Read(E),
    /// An error occurred while decoding the body content.
    Decode(D),
}

impl<B> BodyReader<B> {
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
