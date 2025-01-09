pub mod clients;
pub mod cmd;
pub mod pkg;

pub use clients::{BlockingClient, BufferedClient, Client};
pub use cmd::Command;

pub const DEFAULT_PORT: u16 = 6379;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Result<T> = std::result::Result<T, Error>;

pub mod prelude {
    pub use super::pkg::{
        db::{Db, DbDropGuard},
        parse::{Parse, ParseError},
        shutdown::Shutdown,
        Connection, Frame,
    };
    pub use super::Command;
}
