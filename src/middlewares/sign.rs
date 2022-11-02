use std::fmt::Debug;

use async_trait::async_trait;
use axum::{
    extract::{FromRequest, RequestParts},
    http::{Method, StatusCode},
};
pub struct Sign;

#[async_trait]
impl<B> FromRequest<B> for Sign
where
    B: Send + Debug,
{
    type Rejection = StatusCode;
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        // if req.method() == Method::GET {
        //     let param = req.uri().query().unwrap_or_default();
        //     println!("{}", param);
        // } else {
        //     let param = match req.take_body() {
        //         Some(body) => body.into(),
        //         None => "",
        //     };
        //     println!("{}", param);
        // }

        Ok(Self)
    }
}
