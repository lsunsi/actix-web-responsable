# *actix-web* Responsable

```rust
use actix_web_responsable::Responder;

#[derive(Responder)]
enum MyResponse {
	Ok(String),
	Unauthorized,
	InternalServerError,
}

#[actix_web::get("/")]
async fn get() -> MyResponse {
	MyResponse::Ok("yay".to_owned())
}

```

### *thanks*
