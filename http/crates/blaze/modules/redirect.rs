use crate::{
    pkg::error::Error,
    types::http::{Response, StatusCode},
};

use http::header::{HeaderValue, LOCATION};

pub fn temporary(location: impl Into<String>) -> Result<Response, Error> { create_redirect(StatusCode::TemporaryRedirect, location) }

pub fn permanent(location: impl Into<String>) -> Result<Response, Error> { create_redirect(StatusCode::MovedPermanently, location) }

fn create_redirect(status: StatusCode, location: impl Into<String>) -> Result<Response, Error> {
    let mut response = Response::new();
    response.status = status;
    response.headers.insert(LOCATION, HeaderValue::from_str(&location.into())?);
    Ok(response)
}
