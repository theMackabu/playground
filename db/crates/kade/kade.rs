pub use kade_proto::clients::{BlockingClient, BufferedClient, Client, Message, Subscriber};
pub use kade_proto::{Error, Result, DEFAULT_PORT};

pub mod prelude {
    pub use kade_proto::clients::*;
    pub use kade_proto::prelude::*;
    pub use kade_proto::DEFAULT_PORT;
    pub use kade_proto::{Error, Result};
}

pub mod internals {
    pub use kade_proto::clients;
    pub use kade_proto::cmd;
    pub use kade_proto::pkg;
}
