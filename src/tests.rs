use actix_web::http::{header, StatusCode};
use actix_web::test;
use actix_web::test::TestRequest;
use actix_web::web;
use actix_web::{App, HttpResponse};
use bincode::{Decode, Encode};

use super::Bincode;

#[derive(Decode, Encode)]
struct BincodeObject {
    number: i32,
    text: String,
}

impl Default for BincodeObject {
    fn default() -> Self {
        BincodeObject { number: 32, text: String::from("thirty-seven") }
    }
}


async fn test_route(object: Bincode<BincodeObject>) -> HttpResponse {
    assert_eq!(object.number, 32);
    assert_eq!(object.text, "thirty-seven");
    let body = bincode::encode_to_vec(object.into_inner(), bincode::config::standard()).unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::test]
async fn extractor() {
    let app = test::init_service(App::new().route("/", web::post().to(test_route))).await;
    let config = bincode::config::standard();
    let body = bincode::encode_to_vec(BincodeObject::default(), config).unwrap();

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
    let app = test::init_service(App::new().route("/", web::post().to(test_route))).await;
    let config = bincode::config::standard();
    let body = bincode::encode_to_vec(BincodeObject::default(), config).unwrap();

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

    let objects: Vec<BincodeObject> = (0..100_000)
        .map(|i| BincodeObject {
            number: i,
            text: i.to_string(),
        })
        .collect();

    let config = bincode::config::standard();
    let body = bincode::encode_to_vec(objects, config).unwrap();
    
    let req = TestRequest::post()
        .uri("/")
        .set_payload(body)
        .insert_header((header::CONTENT_TYPE, "application/octet-stream"))
        .to_request();

    let response = test::call_service(&app, req).await;

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}