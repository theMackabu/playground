pub use blaze_macros::{main, route, routes};
pub use modules::*;
pub use pkg::error::Error;

pub use types::alias::*;
pub use types::http::*;

pub use http;
pub use tokio;

pub mod modules;
pub mod pkg;
pub mod types;

pub mod internals {
    use super::pkg::error::Error;
    use super::types::http::{Responder, Response};
    use std::{future::Future, pin::Pin};

    pub type RespFuture = Pin<Box<dyn Future<Output = Result<Response, Error>> + Send>>;
    pub type HttpFuture = Pin<Box<dyn Future<Output = Result<Box<dyn Responder>, Error>> + Send>>;
}

pub mod prelude {
    pub use super::pkg::error::Error;
    pub use super::types::alias::*;
    pub use super::types::http::*;
}

pub(crate) mod logging {
    pub use tracing::{debug, error, info, instrument, trace, warn};
}
