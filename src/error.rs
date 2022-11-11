#![allow(clippy::module_name_repetitions)]

use actix_web::{error::PayloadError, HttpResponse, ResponseError};
use bincode::Error;
use derive_more::Display;

#[derive(Debug, Display)]
pub enum BincodePayloadError {
    /// Payload size is bigger than limit
    #[display(fmt = "Payload size is bigger than {_0}")]
    Overflow(usize),

    /// Content type error
    #[display(fmt = "Content type error: {_0}")]
    ContentType(String),

    /// Deserialize error
    #[display(fmt = "Bincode deserialize error: {_0}")]
    Deserialize(Error),

    /// Serialize error
    #[display(fmt = "Bincode serialize error: {_0}")]
    Serialize(Error),

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

impl From<bincode::Error> for BincodePayloadError {
    fn from(value: bincode::Error) -> Self {
        BincodePayloadError::Serialize(value)
    }
}
