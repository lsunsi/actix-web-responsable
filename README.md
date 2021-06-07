# *actix-web* Responsable

```rust
use actix_web_responsable::Responsable;

enum MyResponse {
	#[status_code = 200]
	Success(String),
	#[status_code = 401]
	UnexpectedUser,
}

#[actix_web::get]
async fn get() -> MyResponse {
	MyResponse::Success("yay")
}

```

### *thanks*
