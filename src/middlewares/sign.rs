use std::fmt::Debug;

use async_trait::async_trait;
use axum::{
    extract::{FromRequest, RequestParts},
    http::StatusCode,
};
pub struct Sign;

#[async_trait]
impl<B> FromRequest<B> for Sign
where
    B: Send + Debug,
{
    type Rejection = StatusCode;
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let body = req.take_body();

        Ok(Self)
    }
}
