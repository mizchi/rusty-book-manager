use std::net::{Ipv4Addr, SocketAddr};

use axum::{http::StatusCode, routing::get, Router};
use tokio::net::TcpListener;

async fn hello_world() -> &'static str {
    "Hello, World!"
}
async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/health", get(health_check));

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server running on port {}", addr.port());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[tokio::test]
async fn health_check_works() {
    let status_code = health_check().await;
    assert_eq!(status_code, StatusCode::OK);
    // let app = Router::new().route("/health", get(health_check));

    // let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 0);
    // let server = axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();

    // let url = format!("http://{}", server.local_addr());
    // let response = reqwest::get(&url).await.unwrap();
    // assert!(response.status().is_success());
}
