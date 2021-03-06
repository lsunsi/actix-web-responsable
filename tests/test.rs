use actix_web::{get, http::StatusCode, test, web, App};
use actix_web_responsable::Responder;
use web::Path;

#[derive(Responder)]
enum Response {
    Ok(String),
    BadRequest(u16),
    InternalServerError,
}

#[get("/{a}")]
async fn route(path: Path<String>) -> Response {
    match path.as_str() {
        "si" => Response::Ok("sim".to_owned()),
        "no" => Response::InternalServerError,
        _ => Response::BadRequest(5),
    }
}

#[actix_rt::test]
async fn test_yay() {
    let mut app = test::init_service(App::new().service(route)).await;
    let req = test::TestRequest::with_uri("/si").to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);

    let body: String = test::read_body_json(resp).await;
    assert_eq!(body, "sim".to_owned());
}

#[actix_rt::test]
async fn test_meh() {
    let mut app = test::init_service(App::new().service(route)).await;
    let req = test::TestRequest::with_uri("/mehbeh").to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    let body: u16 = test::read_body_json(resp).await;
    assert_eq!(body, 5);
}

#[actix_rt::test]
async fn test_nay() {
    let mut app = test::init_service(App::new().service(route)).await;
    let req = test::TestRequest::with_uri("/no").to_request();

    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);

    let body = test::read_body(resp).await;
    assert!(body.is_empty());
}
