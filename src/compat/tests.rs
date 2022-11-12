use actix_web::{HttpResponse, test::{self, TestRequest}, App, web, http::header};
use serde::{Deserialize, Serialize};

use crate::compat::BincodeSerde;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct SerdeObject {
    number: i32,
    text: String,
}

impl Default for SerdeObject {
    fn default() -> Self {
        SerdeObject {
            number: 32,
            text: "thirty-seven".to_string(),
        }
    }
}

async fn test_route(object: BincodeSerde<SerdeObject>) -> HttpResponse {
    let obj = object.into_inner();
    assert_eq!(obj, SerdeObject::default());
    let config = bincode::config::standard();
    let body = bincode::serde::encode_to_vec(obj, config).unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::test]
async fn extractor() {
    let object = SerdeObject::default();

    let app = test::init_service(App::new().route("/", web::post().to(test_route))).await;
    let config = bincode::config::standard();


    let body = bincode::serde::encode_to_vec(object, config).unwrap();

    let req = TestRequest::post()
        .uri("/")
        .set_payload(body)
        .insert_header((header::CONTENT_TYPE, "application/octet-stream"))
        .to_request();

    let response = test::call_service(&app, req).await;

    assert!(response.status().is_success())
}
