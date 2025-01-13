use crate::prelude::*;
use bytes::Bytes;
use std::env::consts;
use tracing::{debug, instrument};

#[derive(Debug, Default)]
pub struct Version {}

impl Version {
    pub fn new() -> Version { Version {} }

    pub fn parse_frames(parse: &mut Parse) -> crate::Result<Version> {
        match parse.next_bytes() {
            Ok(_) => Ok(Version::new()),
            Err(ParseError::EndOfStream) => Ok(Version::default()),
            Err(e) => Err(e.into()),
        }
    }

    #[instrument(skip(self, dst))]
    pub async fn apply(self, dst: &mut Connection) -> crate::Result<()> {
        let msg = format!("v{} {}-{}", env!("CARGO_PKG_VERSION"), consts::OS, consts::ARCH);
        let response = Frame::Simple(msg);

        debug!(?response);
        dst.write_frame(&response).await?;

        Ok(())
    }

    pub fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from("version".as_bytes()));
        frame
    }
}
