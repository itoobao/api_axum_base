use thiserror::Error as ThisError;

use super::Response;

#[derive(ThisError, Debug)]
pub enum Error {
    //各个业务模块的错误类型
    //#[error(transparent)]
    //CacheError(#[from] itoobao_cache::Error),
    #[error("occus error")]
    CommonError,
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let data: Response<&str> = match self {
            Self::CommonError => Response::err(0, "发生错误".to_string()),
        };
        axum::Json(data).into_response()
    }
}
