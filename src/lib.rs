#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, nonstandard_style)]
#![warn(future_incompatible)]

pub mod config;
pub mod error;

#[cfg(feature = "serde")]
mod serde_compat;
#[cfg(feature = "serde")]
pub use serde_compat::BincodeSerde;

use config::{BincodeConfig, DEFAULT_LIMIT_BYTES};

use std::{ops::Deref, pin::Pin};

use actix_web::{dev::Payload, web::BytesMut, FromRequest, HttpMessage, HttpRequest};
use bincode::config::Configuration;
use futures::{Future, StreamExt};

/// Extract and decode bincode from payload
///
///     use actix_web::HttpResponse;
///     use actix_bincode::Bincode;
///     use bincode::{Decode, Encode};
///
///     #[derive(Decode, Encode)]
///     struct Object {
///         text: String,
///     }
///
///     // Route
///     pub async fn index(object: Bincode<Object>) -> HttpResponse {
///         println!("{}", object.text);
///         let body = object.into_bytes(None).unwrap(); // Use standard config
///         HttpResponse::Ok().body(body)
///     }
pub struct Bincode<T>(T);

// Extractor for bincode::Decode derived struct
impl<T> FromRequest for Bincode<T>
where
    T: bincode::Decode,
{
    type Error = error::BincodePayloadError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        // Validate content type
        if req.content_type() != "application/octet-stream" {
            let content_type = req.content_type().to_string();
            return Box::pin(async { Err(error::BincodePayloadError::ContentType(content_type)) });
        }

        // Read config if present
        let config = req
            .app_data::<BincodeConfig>()
            .map_or(BincodeConfig::default(), |c| *c);

        // Read bincode config
        let bincode_config = req
            .app_data::<Configuration>()
            .map_or(bincode::config::standard(), |c| *c);

        let mut payload = payload.take();

        Box::pin(async move {
            let mut buffer: BytesMut = BytesMut::with_capacity(config.buf_size);

            while let Some(bytes) = payload.next().await {
                let bytes = bytes?;

                // Prevent too large payloads
                if buffer.len() + bytes.len() > config.limit {
                    return Err(error::BincodePayloadError::Overflow(config.limit));
                }

                buffer.extend(bytes);
            }

            match bincode::decode_from_slice::<T, _>(&buffer, bincode_config) {
                Ok((obj, _)) => Ok(Bincode(obj)),
                Err(e) => Err(error::BincodePayloadError::Decode(e)),
            }
        })
    }
}

impl<T: bincode::Encode> Bincode<T> {
    /// Take the inner type
    pub fn into_inner(self) -> T {
        self.0
    }

    /// Serializes body into bytes
    #[allow(clippy::missing_errors_doc)]
    pub fn into_bytes(
        self,
        config: Option<Configuration>,
    ) -> Result<BytesMut, error::BincodePayloadError> {
        let mut bytes = BytesMut::with_capacity(DEFAULT_LIMIT_BYTES);
        let ser = bincode::encode_to_vec(
            &self.into_inner(),
            config.unwrap_or(bincode::config::standard()),
        )?;
        bytes.extend(ser);
        Ok(bytes)
    }
}

// For usability, skip the zero
impl<T> Deref for Bincode<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
