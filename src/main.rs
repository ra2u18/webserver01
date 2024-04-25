#![allow(unused)]

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::extract::Query;
use axum::response::Html;
use axum::{routing::get, Router};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(handler_hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->>Listening on {addr}\n");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes_hello).await.unwrap();

    // region -- Handler Hello
    #[derive(Debug, Deserialize)]
    struct HelloParams {
        name: Option<String>,
    }

    // e.g. `/hello?name=Jen`
    async fn handler_hello(Query(params): Query<HelloParams>) -> impl axum::response::IntoResponse {
        println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

        let name = params.name.as_deref().unwrap_or("World!");

        Html(format!("Hello <strong>{name}</strong>"))
    }
}
