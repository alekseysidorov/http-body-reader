#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
#![warn(missing_docs)]

//! # Overview
//!
#![doc = include_utils::include_md!("README.md:description")]
//!

pub use body_reader::{BodyReader, BodyReaderError};

mod body_reader;

/// Extension trait for the [`http::Response`].
pub trait ResponseExt<T>: Sized {
    /// Consumes the response and returns a body reader wrapper.
    fn body_reader(self) -> BodyReader<T>;
}

impl<T> ResponseExt<T> for http::Response<T> {
    fn body_reader(self) -> BodyReader<T> {
        BodyReader::from(self)
    }
}
