# actix-bincode

![crates.io](https://img.shields.io/crates/v/actix-bincode?label=latest) [![dependency status](https://deps.rs/crate/actix-bincode/0.3.0/status.svg)](https://deps.rs/crate/actix-bincode/0.3.0)

[Bincode](https://crates.io/crates/bincode) payload extractor for Actix Web

### NOTICE: This crate uses Bincode version 2.0.0-rc.3  


### Example

```rust
use actix_bincode::Bincode;
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

### Serde example

```rust
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

### Configuring bincode

Extractor tries to read configuration from actix app data, and defaults to standard if none present:  

```rust
let config = bincode::config::standard().with_big_endian();

let app = App::new().app_data(config);

```

### License

This project is licensed under 

- MIT license ([LICENSE](LICENSE) or [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT))

