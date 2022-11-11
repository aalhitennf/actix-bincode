#![forbid(unsafe_code)]
#![deny(rust_2018_idioms, nonstandard_style)]
#![warn(future_incompatible)]

pub mod config;
pub mod error;

use std::{ops::Deref, pin::Pin};

use actix_web::{dev::Payload, web::BytesMut, FromRequest, HttpMessage, HttpRequest};
use futures::{Future, StreamExt};

/// Extract and deserialize bincode from payload
///
///     use actix_web::HttpResponse;
///     use actix_bincode::Bincode;
///     use serde::{Deserialize, Serialize};  
///
///     #[derive(Deserialize, Serialize)]  
///     struct Object {  
///         text: String,  
///     }  
///  
///     // Route
///     pub async fn index(object: Bincode<Object>) -> HttpResponse {  
///         println!("{}", object.text);
///         HttpResponse::Ok().body("OK")
///     }  
#[derive(Clone, Debug)]
pub struct Bincode<T>(T);

impl<T> FromRequest for Bincode<T>
where
    T: serde::de::DeserializeOwned + serde::ser::Serialize,
{
    type Error = error::BincodePayloadError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        // Validate content type
        if req.content_type() != "application/octet-stream" {
            let content_type = req.content_type().to_string();
            return Box::pin(async { Err(error::BincodePayloadError::ContentType(content_type)) });
        }

        // Read limit if present
        let limit = req.app_data::<config::BincodeConfig>().map_or(262_144, |c| c.limit);

        let mut payload = payload.take();

        Box::pin(async move {
            let mut buffer: BytesMut = BytesMut::new();

            while let Some(bytes) = payload.next().await {
                let bytes = bytes?;

                // Prevent too large payloads
                if buffer.len() + bytes.len() > limit {
                    return Err(error::BincodePayloadError::Overflow(limit));
                }

                buffer.extend(bytes);
            }

            match bincode::deserialize::<T>(&buffer) {
                Ok(value) => Ok(Bincode(value)),
                Err(e) => Err(error::BincodePayloadError::Deserialize(e)),
            }
        })
    }
}

impl<T: serde::ser::Serialize> Bincode<T> {
    /// Take the inner type
    pub fn into_inner(self) -> T {
        self.0
    }
    /// Serializes body into bytes
    pub fn into_bytes(self) -> Result<BytesMut, error::BincodePayloadError> {
        let mut bytes = BytesMut::new();
        let ser = bincode::serialize(&self.into_inner())?;
        bytes.extend(ser);
        Ok(bytes)
    }
}

// For easier usability, skip the zero
impl<T> Deref for Bincode<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use actix_web::http::{header, StatusCode};
    use actix_web::test;
    use actix_web::test::TestRequest;
    use actix_web::web;
    use actix_web::{App, HttpResponse};
    use serde::{Deserialize, Serialize};

    use super::Bincode;

    #[derive(Deserialize, Serialize)]
    struct TestObject {
        number: i32,
        text: String,
    }

    async fn test_route(object: Bincode<TestObject>) -> HttpResponse {
        assert_eq!(object.number, 32);
        assert_eq!(object.text, "thirty-seven");

        HttpResponse::Ok().body("OK")
    }

    #[actix_web::test]
    async fn extractor() {
        let object = TestObject {
            number: 32,
            text: "thirty-seven".to_string(),
        };

        let app = test::init_service(App::new().route("/", web::post().to(test_route))).await;
        let body = bincode::serialize(&object).unwrap();

        let req = TestRequest::post()
            .uri("/")
            .set_payload(body)
            .insert_header((header::CONTENT_TYPE, "application/octet-stream"))
            .to_request();

        let response = test::call_service(&app, req).await;

        assert!(response.status().is_success())
    }

    #[actix_web::test]
    async fn content_type() {
        let object = TestObject {
            number: 32,
            text: "thirty-seven".to_string(),
        };

        let app = test::init_service(App::new().route("/", web::post().to(test_route))).await;

        let body = bincode::serialize(&object).unwrap();

        let req = TestRequest::post()
            .uri("/")
            .set_payload(body)
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .to_request();

        let response = test::call_service(&app, req).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn content_size() {
        let app = test::init_service(App::new().route("/", web::post().to(test_route))).await;

        let vec: Vec<TestObject> = (0..100_000)
            .map(|i| TestObject {
                number: i,
                text: i.to_string(),
            })
            .collect();

        let body = bincode::serialize(&vec).unwrap();

        let req = TestRequest::post()
            .uri("/")
            .set_payload(body)
            .insert_header((header::CONTENT_TYPE, "application/octet-stream"))
            .to_request();

        let response = test::call_service(&app, req).await;

        assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    }
}
