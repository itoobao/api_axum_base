use crate::common::Error;
use axum::{
    http::{header, Method, Request, StatusCode},
    middleware::Next,
    response::Response,
};
//验证用户token中间件
pub async fn token<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, Error> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());
    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(Error::CommonError);
    };
    Ok(next.run(req).await)
}