use axum::{middleware, routing::get, Router};

use crate::middlewares::token;

pub fn register_route() -> Router {
    let route = common_route();
    route.nest("/user", user_route()).nest("/api", api_route())
}

fn common_route() -> Router {
    let router = Router::new().route("/", get(|| async { "index" }));
    router
}
/// 用户相关路由
fn user_route() -> Router {
    let route = Router::new()
        .route("/:id", get(|| async { "user/:id" }))
        .route("/list", get(|| async { "user/list" }))
        .route_layer(middleware::from_fn(token));
    route
}

/// api相关路由
fn api_route() -> Router {
    let api_routes = Router::new()
        .route("/teams", get(|| async { "api/teams" }))
        .route("/info", get(|| async { "/api/info" }));
    api_routes
}
