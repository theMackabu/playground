use super::Frame;
use bytes::Bytes;
use std::{fmt, str, vec};

#[derive(Debug)]
pub struct Parse {
    parts: vec::IntoIter<Frame>,
}

#[derive(Debug)]
pub enum ParseError {
    EndOfStream,
    Other(crate::Error),
}

impl Parse {
    pub fn new(frame: Frame) -> Result<Parse, ParseError> {
        let array = match frame {
            Frame::Array(array) => array,
            frame => return Err(format!("protocol error; expected array, got {:?}", frame).into()),
        };

        Ok(Parse { parts: array.into_iter() })
    }

    fn next(&mut self) -> Result<Frame, ParseError> { self.parts.next().ok_or(ParseError::EndOfStream) }

    pub fn next_string(&mut self) -> Result<String, ParseError> {
        match self.next()? {
            Frame::Simple(s) => Ok(s),
            Frame::Bulk(data) => str::from_utf8(&data[..]).map(|s| s.to_string()).map_err(|_| "protocol error; invalid string".into()),
            frame => Err(format!("protocol error; expected simple frame or bulk frame, got {:?}", frame).into()),
        }
    }

    pub fn next_bytes(&mut self) -> Result<Bytes, ParseError> {
        match self.next()? {
            Frame::Simple(s) => Ok(Bytes::from(s.into_bytes())),
            Frame::Bulk(data) => Ok(data),
            frame => Err(format!("protocol error; expected simple frame or bulk frame, got {:?}", frame).into()),
        }
    }

    pub fn next_int(&mut self) -> Result<u64, ParseError> {
        use atoi::atoi;
        const MSG: &str = "protocol error; invalid number";

        match self.next()? {
            Frame::Integer(v) => Ok(v),
            Frame::Simple(data) => atoi::<u64>(data.as_bytes()).ok_or_else(|| MSG.into()),
            Frame::Bulk(data) => atoi::<u64>(&data).ok_or_else(|| MSG.into()),
            frame => Err(format!("protocol error; expected int frame but got {:?}", frame).into()),
        }
    }

    pub fn finish(&mut self) -> Result<(), ParseError> {
        if self.parts.next().is_none() {
            Ok(())
        } else {
            Err("protocol error; expected end of frame, but there was more".into())
        }
    }
}

impl From<String> for ParseError {
    fn from(src: String) -> ParseError { ParseError::Other(src.into()) }
}

impl From<&str> for ParseError {
    fn from(src: &str) -> ParseError { src.to_string().into() }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EndOfStream => "protocol error; unexpected end of stream".fmt(f),
            ParseError::Other(err) => err.fmt(f),
        }
    }
}

impl std::error::Error for ParseError {}
