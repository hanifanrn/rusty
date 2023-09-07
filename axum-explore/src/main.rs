#![allow(unused)]

use std::net::SocketAddr;
use axum::Router;
use axum::routing::get;
use axum::response::{Html, IntoResponse};
use axum::extract::Query;
use serde::Deserialize;


#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route("/hello", get(handler_hello));

    //region:     --- start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {addr} \n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
    //end region: --- start server
}


//region:    --- handler hello
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello, {params:?}", "HANDLER");
    
    let name = params.name.as_deref().unwrap_or("World!"); 
    return Html("Hello <strong>World!</strong>");
}
//endregion: --- handler hello








