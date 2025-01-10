pub use fast_proto::clients::{BlockingClient, BufferedClient, Client, Message, Subscriber};
pub use fast_proto::{Error, Result, DEFAULT_PORT};

pub mod prelude {
    pub use fast_proto::clients::*;
    pub use fast_proto::prelude::*;
    pub use fast_proto::DEFAULT_PORT;
    pub use fast_proto::{Error, Result};
}

pub mod internals {
    pub use fast_proto::clients;
    pub use fast_proto::cmd;
    pub use fast_proto::pkg;
}
