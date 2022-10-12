mod middlewares;
mod utils;

use std::net::SocketAddr;

use axum::{middleware, routing::get, Router};
#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    //设置tracing
    utils::tracing();

    tracing::debug!("listen on {}", addr);

    let user_routes = Router::new()
        .route("/:id", get(|| async { "user/:id" }))
        .route("/list", get(|| async { "user/list" }));

    let api_routes = Router::new()
        .route("/teams", get(|| async { "api/teams" }))
        .route("/info", get(|| async { "/api/info" }));

    let app = Router::new()
        .route("/", get(index))
        .nest("/users", user_routes)
        .route_layer(middleware::from_fn(middlewares::auth_user))
        .route("/test", get(test_api))
        .nest("/api", api_routes);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> &'static str {
    "hello"
}

async fn test_api() -> &'static str {
    "test"
}
