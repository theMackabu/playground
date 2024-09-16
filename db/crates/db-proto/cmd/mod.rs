mod dump;
mod get;
mod load;
mod ping;
mod publish;
mod set;
mod subscribe;
mod unknown;

pub use dump::Dump;
pub use get::Get;
pub use load::Load;
pub use ping::Ping;
pub use publish::Publish;
pub use set::Set;
pub use subscribe::{Subscribe, Unsubscribe};
pub use unknown::Unknown;

use crate::prelude::*;

#[derive(Debug)]
pub enum Command {
    Get(Get),
    Publish(Publish),
    Set(Set),
    Subscribe(Subscribe),
    Unsubscribe(Unsubscribe),
    Ping(Ping),
    Dump(Dump),
    Load(Load),
    Unknown(Unknown),
}

impl Command {
    pub fn from_frame(frame: Frame) -> crate::Result<Command> {
        let mut parse = Parse::new(frame)?;
        let command_name = parse.next_string()?.to_lowercase();

        let command = match &command_name[..] {
            "get" => Command::Get(Get::parse_frames(&mut parse)?),
            "publish" => Command::Publish(Publish::parse_frames(&mut parse)?),
            "set" => Command::Set(Set::parse_frames(&mut parse)?),
            "subscribe" => Command::Subscribe(Subscribe::parse_frames(&mut parse)?),
            "unsubscribe" => Command::Unsubscribe(Unsubscribe::parse_frames(&mut parse)?),
            "ping" => Command::Ping(Ping::parse_frames(&mut parse)?),
            "dump" => Command::Dump(Dump::parse_frames(&mut parse)?),
            "load" => Command::Load(Load::parse_frames(&mut parse)?),
            _ => return Ok(Command::Unknown(Unknown::new(command_name))),
        };

        parse.finish()?;
        Ok(command)
    }

    pub async fn apply(self, db: &Db, dst: &mut Connection, shutdown: &mut Shutdown) -> crate::Result<()> {
        use Command::*;

        match self {
            Get(cmd) => cmd.apply(db, dst).await,
            Publish(cmd) => cmd.apply(db, dst).await,
            Set(cmd) => cmd.apply(db, dst).await,
            Subscribe(cmd) => cmd.apply(db, dst, shutdown).await,
            Ping(cmd) => cmd.apply(dst).await,
            Dump(cmd) => cmd.apply(db, dst).await,
            Load(cmd) => cmd.apply(db, dst).await,
            Unknown(cmd) => cmd.apply(dst).await,
            Unsubscribe(_) => Err("`Unsubscribe` is unsupported in this context".into()),
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            Command::Get(_) => "get",
            Command::Publish(_) => "pub",
            Command::Set(_) => "set",
            Command::Subscribe(_) => "subscribe",
            Command::Unsubscribe(_) => "unsubscribe",
            Command::Ping(_) => "ping",
            Command::Dump(_) => "dump",
            Command::Load(_) => "load",
            Command::Unknown(cmd) => cmd.get_name(),
        }
    }
}
