use crate::prelude::*;

use bytes::Bytes;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tracing::{info, instrument};

#[derive(Debug)]
pub struct Dump {
    path: PathBuf,
}

impl Dump {
    pub fn new(path: PathBuf) -> Dump { Dump { path } }

    pub fn parse_frames(parse: &mut Parse) -> crate::Result<Dump> {
        let path = parse.next_string()?;
        Ok(Dump { path: PathBuf::from(path) })
    }

    #[instrument(skip(self, db, dst))]
    pub async fn apply(self, db: &Db, dst: &mut Connection) -> crate::Result<()> {
        let serializable_state = db.dump();
        let serialized = bincode::serialize(&serializable_state)?;

        let mut file = File::create(&self.path)?;
        file.write_all(&serialized)?;
        info!("Database state dumped to {:?}", self.path);

        let response = Frame::Simple("OK".to_string());
        dst.write_frame(&response).await?;

        Ok(())
    }

    pub fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from("dump".as_bytes()));
        frame.push_bulk(Bytes::from(self.path.to_string_lossy().into_owned()));
        frame
    }
}
