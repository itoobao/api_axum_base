use axum::{
    http::{header, Request, StatusCode},
    middleware::Next,
    response::Response,
};

//验证用户token中间件
pub async fn auth_user<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());
    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    Ok(next.run(req).await)
}
