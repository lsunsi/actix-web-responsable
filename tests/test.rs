use actix_web::{get, http::StatusCode, test, web, App};
use web::Path;

#[derive(actix_web_responsable::Responder)]
enum Response {
    #[status_code(200)]
    Yay(String),
    #[status_code(400)]
    Meh(u16),
    #[status_code(500)]
    Nay,
}

#[get("/{a}")]
async fn route(Path((a,)): Path<(String,)>) -> Response {
    match a.as_ref() {
        "si" => Response::Yay("sim".to_owned()),
        "no" => Response::Nay,
        _ => Response::Meh(5),
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
