mod error;
mod response;
mod route;
mod tracing;

pub use self::tracing::tracing;
pub use error::Error;
pub use response::Response;
pub use route::register_route;
type Result<T> = std::result::Result<T, self::Error>;
pub type AppResult<T> = Result<self::Response<T>>;
