use std::any::Any;

use axum::{response::Response, body::Body};
use hyper::StatusCode;

// Reference: https://docs.rs/tower-http/latest/tower_http/catch_panic/

pub fn handler(err: Box<dyn Any + Send + 'static>) -> Response<Body> {

    let details = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "Unknown panic message".to_string()
    };

    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from(details))
        .unwrap()
}