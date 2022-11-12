# actix-bincode

## NOTICE: This crate uses Bincode version 2.0.0

[Bincode](https://crates.io/crates/bincode) payload extractor for Actix Web

## Example

```rust,ignore
use actix_bincode::BincodeSerde;
use bincode::{Decode, Encode};

#[derive(Decode, Encode)]
pub struct Object {
    pub num: i32,
    pub text: String,
}

async fn index(object: Bincode<Object>) -> HttpResponse {
    println!("num: {}", object.num);
    println!("text: {}", object.text);
    let config = bincode::config::standard();
    let body = bincode::encode_to_vec(object.into_inner(), config).unwrap();
    HttpResponse::Ok().body(body)
}
```

## Serde example

```rust,ignore
use actix_bincode::BincodeSerde;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Object {
    pub num: i32,
    pub text: String,
}

async fn index(object: BincodeSerde<Object>) -> HttpResponse {
    println!("num: {}", object.num);
    println!("text: {}", object.text);
    let config = bincode::config::standard();
    let body = bincode::serde::encode_to_vec(object.into_inner(), config).unwrap();
    HttpResponse::Ok().body(body)
}
```

## License

This project is licensed under 

- MIT license ([LICENSE](LICENSE))

