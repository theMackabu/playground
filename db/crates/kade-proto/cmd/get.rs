use crate::prelude::*;

use bytes::Bytes;
use tracing::{debug, instrument};

#[derive(Debug)]
pub struct Get {
    key: String,
}

impl Get {
    pub fn new(key: impl ToString) -> Get { Get { key: key.to_string() } }

    pub fn key(&self) -> &str { &self.key }

    pub fn parse_frames(parse: &mut Parse) -> crate::Result<Get> {
        let key = parse.next_string()?;

        Ok(Get { key })
    }

    #[instrument(skip(self, db, dst))]
    pub async fn apply(self, db: &Db, dst: &mut Connection) -> crate::Result<()> {
        let response = if let Some(value) = db.get(&self.key) { Frame::Bulk(value) } else { Frame::Null };
        debug!(?response);

        dst.write_frame(&response).await?;

        Ok(())
    }

    pub fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from("get".as_bytes()));
        frame.push_bulk(Bytes::from(self.key.into_bytes()));
        frame
    }
}
