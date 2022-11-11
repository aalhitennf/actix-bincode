# actix-bincode

> Bincode payload extractor for Actix Web

## Example

```rust,ignore
use actix_bincode::Bincode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Object {
    pub num: i32,
    pub text: String,
}

async fn index(object: Bincode<Object>) -> Result<HttpResponse> {
    println!("num: {}", object.num);
    println!("text: {}", object.text);
    HttpResponse::Ok().finish()
}
```

## License

This project is licensed under 

- MIT license ([LICENSE](LICENSE) or [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT))

