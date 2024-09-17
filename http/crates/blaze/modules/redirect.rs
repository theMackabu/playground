use crate::{
    pkg::error::Error,
    types::http::{Response, StatusCode},
};

use http::header::{HeaderValue, LOCATION};

pub fn temporary(location: impl Into<String>) -> Result<Response, Error> { create_redirect(Response::new(), StatusCode::TemporaryRedirect, location) }

pub fn permanent(location: impl Into<String>) -> Result<Response, Error> { create_redirect(Response::new(), StatusCode::MovedPermanently, location) }

pub(crate) fn create_redirect(mut res: Response, status: StatusCode, location: impl Into<String>) -> Result<Response, Error> {
    res.status = status;
    res.headers.insert(LOCATION, HeaderValue::from_str(&location.into())?);
    Ok(res)
}
