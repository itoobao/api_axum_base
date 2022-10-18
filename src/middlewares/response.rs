use axum::{
    body::BoxBody,
    http::{header, HeaderValue, Response},
};
use std::convert::Infallible;

pub async fn after(mut res: Response<BoxBody>) -> Result<Response<BoxBody>, Infallible> {
    res.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );
    res.headers_mut().insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("Content-Type"),
    );
    res.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    Ok(res)
}
