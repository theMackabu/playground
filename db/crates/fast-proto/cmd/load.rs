use crate::pkg::db::SerializableState;
use crate::prelude::*;

use bytes::Bytes;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use tracing::{info, instrument};

#[derive(Debug)]
pub struct Load {
    path: PathBuf,
}

impl Load {
    pub fn new(path: PathBuf) -> Load { Load { path } }

    pub fn parse_frames(parse: &mut Parse) -> crate::Result<Load> {
        let path = parse.next_string()?;
        Ok(Load { path: PathBuf::from(path) })
    }

    #[instrument(skip(self, db, dst))]
    pub async fn apply(self, db: &Db, dst: &mut Connection) -> crate::Result<()> {
        let mut file = File::open(&self.path)?;
        let mut serialized = Vec::new();
        file.read_to_end(&mut serialized)?;

        let serializable_state: SerializableState = bincode::deserialize(&serialized)?;
        db.load(serializable_state);
        info!("Database state loaded from {:?}", self.path);

        let response = Frame::Simple("OK".to_string());
        dst.write_frame(&response).await?;

        Ok(())
    }

    pub fn into_frame(self) -> Frame {
        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from("load".as_bytes()));
        frame.push_bulk(Bytes::from(self.path.to_string_lossy().into_owned()));
        frame
    }
}
