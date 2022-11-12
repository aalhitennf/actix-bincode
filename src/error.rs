#![allow(clippy::module_name_repetitions)]

use actix_web::{error::PayloadError, HttpResponse, ResponseError};
use bincode::error::{DecodeError, EncodeError};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum BincodePayloadError {
    /// Payload size is bigger than limit
    #[display(fmt = "Payload size is bigger than {_0}")]
    Overflow(usize),

    /// Content type error
    #[display(fmt = "Content type error: {_0}")]
    ContentType(String),

    /// Decode error
    #[display(fmt = "Bincode decode error: {_0}")]
    Decode(DecodeError),

    /// Encode error
    #[display(fmt = "Bincode encode error: {_0}")]
    Encode(EncodeError),

    /// Payload error
    #[display(fmt = "Error reading payload: {_0}")]
    Payload(PayloadError),
}

impl ResponseError for BincodePayloadError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            BincodePayloadError::Overflow(_) => HttpResponse::PayloadTooLarge().into(),
            _ => HttpResponse::BadRequest().into(),
        }
    }
}

impl From<actix_web::error::PayloadError> for BincodePayloadError {
    fn from(value: actix_web::error::PayloadError) -> Self {
        BincodePayloadError::Payload(value)
    }
}

impl From<DecodeError> for BincodePayloadError {
    fn from(value: DecodeError) -> Self {
        BincodePayloadError::Decode(value)
    }
}

impl From<EncodeError> for BincodePayloadError {
    fn from(value: EncodeError) -> Self {
        BincodePayloadError::Encode(value)
    }
}
