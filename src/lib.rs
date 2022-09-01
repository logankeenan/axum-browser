use wasm_bindgen::prelude::*;
use axum::{response::{Html, Response}, routing::get, Router, Error};
use axum::body::{Body, Bytes};

use http::Request;
use tower_service::Service;
use std::str;

#[wasm_bindgen]
pub async fn make_request(uri: String) -> String {
    let mut router :Router = Router::new()
        .route("/", get(index))
        .route("/mars", get(mars));

    let request: Request<Body> = Request::builder()
        .uri(uri)
        .body("".into())
        .unwrap();

    let mut response: Response = router.call(request).await.unwrap();
    let data: Option<Result<Bytes, Error>> = http_body::Body::data(response.body_mut()).await;

    let result: Bytes = data.unwrap().unwrap();

    str::from_utf8(&*result).unwrap().to_string()
}

async fn index() -> Html<&'static str> {
    Html("<h1>Hello World from Axum!!!</h1><br/><a href=\"mars\">Hello Mars</a>")
}

async fn mars() -> Html<&'static str> {
    Html("<h1>Hello Mars from Axum!!!</h1></h1><br/><a href=\"/\">Hello World</a>")
}
