#[cfg(test)]
mod tests;

use std::{ops::Deref, pin::Pin};

use actix_web::{dev::Payload, web::BytesMut, FromRequest, HttpMessage, HttpRequest};
use bincode::config::Configuration;
use futures::{Future, StreamExt};

use crate::{error::BincodePayloadError, config::BincodeConfig};


/// Extract and deserialize bincode from payload with serde compatibility
///
///     use actix_web::HttpResponse;
///     use actix_bincode::BincodeSerde;
///     use serde::{Deserialize, Serialize};  
///
///     #[derive(Deserialize, Serialize)]  
///     struct Object {  
///         text: String,  
///     }  
///  
///     // Route
///     pub async fn index(object: BincodeSerde<Object>) -> HttpResponse {  
///         println!("{}", object.text);
///         let config = bincode::config::standard();
///         let body = bincode::serde::encode_to_vec(object.into_inner(), config).unwrap();
///         HttpResponse::Ok().body(body)
///     }  
#[derive(Clone, Debug)]
pub struct BincodeSerde<T>(T);

// Extractor for serde derived struct
impl<T> FromRequest for BincodeSerde<T>
where
    T: serde::de::DeserializeOwned,
{
    type Error = BincodePayloadError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        // Validate content type
        if req.content_type() != "application/octet-stream" {
            let content_type = req.content_type().to_string();
            return Box::pin(async { Err(BincodePayloadError::ContentType(content_type)) });
        }

        // Read limit if present
        let limit = req.app_data::<BincodeConfig>().map_or(262_144, |c| c.limit);

        // Read bincode config
        let bincode_config = req.app_data::<Configuration>().map_or(bincode::config::standard(), |c| c.clone());

        let mut payload = payload.take();

        Box::pin(async move {
            let mut buffer: BytesMut = BytesMut::new();

            while let Some(bytes) = payload.next().await {
                let bytes = bytes?;

                // Prevent too large payloads
                if buffer.len() + bytes.len() > limit {
                    return Err(BincodePayloadError::Overflow(limit));
                }

                buffer.extend(bytes);
            }

            match bincode::serde::decode_from_slice::<T, _>(&buffer.to_vec(), bincode_config) {
                Ok((obj, _)) => Ok(BincodeSerde(obj)),
                Err(e) => Err(BincodePayloadError::Decode(e)),
            }
        })
    }
}


impl<T: serde::ser::Serialize> BincodeSerde<T> {
    /// Take the inner type
    pub fn into_inner(self) -> T {
        self.0
    }
    /// Serializes body into bytes
    pub fn into_bytes(self, config: Option<Configuration>) -> Result<BytesMut, BincodePayloadError> {
        let mut bytes = BytesMut::new();
        let ser = bincode::serde::encode_to_vec(&self.into_inner(), config.unwrap_or(bincode::config::standard()))?;
        bytes.extend(ser);
        Ok(bytes)
    }
}

// For easier usability, skip the zero
impl<T> Deref for BincodeSerde<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
