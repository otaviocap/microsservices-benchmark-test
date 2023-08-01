use axum::{Router, body::Body, extract::{State, Request}, response::{Response, IntoResponse}};
use hyper::{client::HttpConnector, StatusCode, Uri};
use tower_http::catch_panic::CatchPanicLayer;

use crate::{router, panic_handler};

#[derive(Clone, Debug)]
struct Configuration {
    client: hyper::client::Client<HttpConnector, Body>,
    router: router::Router
}

pub async fn run_server() {

    let router: router::Router = router::get_routes(); 

    let client = hyper::Client::builder().build(HttpConnector::new());

    let configs = Configuration {
        client,
        router
    };

    let app = Router::new().fallback(handler).layer(CatchPanicLayer::custom(panic_handler::handler)).with_state(configs);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    println!("Rust Gateway listening on http://{}", listener.local_addr().unwrap());
    
    axum::serve(listener, app).await.unwrap();
}


async fn handler(State(config): State<Configuration>, mut req: Request) -> Result<Response, StatusCode> {
    println!("Request: {req:?}");

    let path = req.uri().path();

    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);

    let query_parameters = path_query.split('/').collect::<Vec<&str>>();

    let selected_api = (*query_parameters
        .iter()
        .find(|e| !e.is_empty())
        .expect("Expecting an API")).to_string();

    let new_uri = config.router
        .iter()
        .find(|&e| e.name == selected_api)
        .expect(format!("The selected API: {selected_api}, doesn't exist or isn't configured correctly!")
            .as_str()
        );

    *req.uri_mut() = Uri::try_from(
        String::new()
        + &new_uri.redirect 
        + &query_parameters
            .get(2..)
            .unwrap_or_default()
            .join("/")
    ).expect("Invalid URI");

    Ok(config.client
        .request(req)
        .await
        .unwrap_or_else(| e | 
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(hyper::Body::from(e.message().to_string()))
                .unwrap()
        )
        .into_response())
}