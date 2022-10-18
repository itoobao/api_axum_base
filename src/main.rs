mod common;
mod middlewares;
mod utils;
use std::net::SocketAddr;

use axum::middleware;
use tower::util::AndThenLayer;
#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    //设置tracing
    common::tracing();

    tracing::debug!("listen on {}", addr);

    let route = common::register_route()
        //请求-前置处理
        .route_layer(middleware::from_fn(middlewares::before))
        //响应-处理
        .layer(AndThenLayer::new(middlewares::after));
    axum::Server::bind(&addr)
        .serve(route.into_make_service())
        .await
        .unwrap();
}
