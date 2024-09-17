use std::fmt;

#[derive(Debug)]
pub struct Error(pub String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.0) }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self { Error(err.to_string()) }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self { Error(err.to_string()) }
}

impl From<Error> for std::io::Error {
    fn from(err: Error) -> Self { std::io::Error::new(std::io::ErrorKind::Other, err.0) }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self { Error(err.to_string()) }
}
