use crate::{internals::*, pkg::error::Error};
use http::header::HeaderMap;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};

pub type HttpResponse = Result<Response, Error>;

pub trait Responder: Send {
    fn respond(self: Box<Self>) -> RespFuture;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Method {
    ALL,
    GET,
    POST,
    PUT,
    DELETE,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StatusCode {
    Ok = 200,
    Created = 201,
    NoContent = 204,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    TemporaryRedirect = 307,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    InternalServerError = 500,
}

#[derive(Clone, Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HeaderMap,
    pub body: Vec<u8>,
    pub params: HashMap<String, String>,
    pub query: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct Response {
    pub path: String,
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Vec<u8>,
}

#[derive(Clone)]
pub struct Router {
    pub routes: Vec<(Method, String, Arc<dyn Fn(Request) -> HttpFuture + Send + Sync>)>,
}

pub struct Server {
    pub(crate) addr: SocketAddr,
}
