use axum::{
    http::{Method, Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn before<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    //支持ajax fetch的option请求
    if req.method() == Method::OPTIONS {
        return Err(StatusCode::OK);
    }

    Ok(next.run(req).await)
}
