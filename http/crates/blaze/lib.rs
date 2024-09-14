use __internals_do_not_use_or_you_will_be_fired::{HttpFuture, RespFuture};

pub use codegen::{main, route, routes};
pub use http;
pub use tokio;

pub mod header;
mod macros;

use header::{ContentType, TryIntoHeaderValue};
use http::header::*;
use serde::Serialize;
use std::{borrow::Cow, collections::HashMap, net::SocketAddr, sync::Arc};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, Result as IoResult},
    net::{TcpListener, TcpStream},
};

pub mod prelude {
    pub use super::{Error, HttpResponse, Method, Request, Responder, Router, Server};
}

pub trait Responder: Send {
    fn respond(self: Box<Self>) -> RespFuture;
}

pub type Str = &'static str;

pub type HttpResponse = Result<Response, Error>;

pub mod __internals_do_not_use_or_you_will_be_fired {
    use super::{Error, Responder, Response};
    use std::{future::Future, pin::Pin};

    pub type RespFuture = Pin<Box<dyn Future<Output = Result<Response, Error>> + Send>>;
    pub type HttpFuture = Pin<Box<dyn Future<Output = Result<Box<dyn Responder>, Error>> + Send>>;
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
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    InternalServerError = 500,
}

impl StatusCode {
    pub fn to_code(&self) -> u16 { *self as u16 }

    pub fn reason_phrase(&self) -> &'static str {
        match self {
            StatusCode::Ok => "OK",
            StatusCode::Created => "Created",
            StatusCode::NoContent => "No Content",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::Unauthorized => "Unauthorized",
            StatusCode::Forbidden => "Forbidden",
            StatusCode::NotFound => "Not Found",
            StatusCode::InternalServerError => "Internal Server Error",
        }
    }
}

impl From<u16> for StatusCode {
    fn from(code: u16) -> Self {
        match code {
            200 => StatusCode::Ok,
            201 => StatusCode::Created,
            204 => StatusCode::NoContent,
            400 => StatusCode::BadRequest,
            401 => StatusCode::Unauthorized,
            403 => StatusCode::Forbidden,
            404 => StatusCode::NotFound,
            500 => StatusCode::InternalServerError,
            _ => panic!("Unsupported status code"),
        }
    }
}

#[derive(Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HeaderMap,
    pub body: Vec<u8>,
    pub params: HashMap<String, String>,
    pub query: HashMap<String, String>,
}

impl Request {
    pub fn route_param(&self, name: &str) -> Option<&String> { self.params.get(name) }
    pub fn query_param(&self, name: &str) -> Option<&String> { self.query.get(name) }
}

#[derive(Clone)]
pub struct Response {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Vec<u8>,
}

impl Response {
    fn new() -> Self {
        Response {
            status: StatusCode::Ok,
            headers: HeaderMap::new(),
            body: Vec::new(),
        }
    }

    pub fn ok() -> Self { Self::new() }

    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn content_type<V>(&mut self, value: V) -> &mut Self
    where
        V: TryIntoHeaderValue,
    {
        match value.try_into_value() {
            Ok(value) => {
                self.headers.insert(CONTENT_TYPE, value);
            }
            // Err(err) => self.error = Some(err.into()),
            Err(_) => self.status = StatusCode::from(500),
        };
        self
    }

    pub fn insert_header(mut self, header: (HeaderName, HeaderValue)) -> Self {
        self.headers.insert(header.0, header.1);
        self
    }

    pub fn body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = body.into();
        self
    }

    pub fn json<T: Serialize>(mut self, value: &T) -> Result<Self, serde_json::Error> {
        let body = serde_json::to_vec(value)?;
        self.content_type(ContentType::json());
        self.body = body;

        Ok(self)
    }

    pub async fn write_headers<W: AsyncWriteExt + Unpin>(&self, stream: &mut W) -> IoResult<()> {
        for (key, value) in self.headers.iter() {
            let header_name = key.as_str();
            let header_value = value
                .to_str()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, format!("Invalid header value: {}", e)))?;

            let header = format!("{}: {}\r\n", header_name, header_value);
            stream.write_all(header.as_bytes()).await?;
        }
        Ok(())
    }
}

impl From<&mut Response> for Response {
    fn from(response: &mut Response) -> Self { std::mem::replace(response, Response::default()) }
}

impl Default for Response {
    fn default() -> Self { Self::new() }
}

impl Responder for Response {
    fn respond(self: Box<Self>) -> RespFuture { Box::pin(async move { Ok(*self) }) }
}

#[derive(Clone)]
pub struct Router {
    pub routes: Vec<(Method, String, Arc<dyn Fn(Request) -> HttpFuture + Send + Sync>)>,
}

impl Router {
    pub fn new() -> Self { Router { routes: Vec::new() } }

    pub fn add<F>(&mut self, method: Method, path: String, handler: F) -> &mut Self
    where
        F: Fn(Request) -> HttpFuture + Send + Sync + 'static,
    {
        self.routes.push((method, path, Arc::new(handler)));
        self
    }

    pub fn add_default<F>(&mut self, handler: F) -> &mut Self
    where
        F: Fn(Request) -> HttpFuture + Send + Sync + 'static,
    {
        self.routes.push((Method::ALL, String::default(), Arc::new(handler)));
        self
    }
}

pub struct Server {
    addr: SocketAddr,
}

impl Server {
    pub fn new(host: &str, port: u16) -> Self {
        let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();
        Server { addr }
    }

    pub async fn serve(self, router: Router) -> Result<(), Error> {
        let listener = TcpListener::bind(self.addr).await?;
        println!("Server running on http://{}", self.addr);

        loop {
            let (stream, _) = listener.accept().await?;
            let router = router.clone();
            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream, router).await {
                    eprintln!("Error handling connection: {}", e);
                }
            });
        }
    }
}

async fn handle_connection(mut stream: TcpStream, router: Router) -> Result<(), Error> {
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    let mut req = parse_request(&buffer[..n])?;

    let mut response = Response {
        status: StatusCode::NotFound,
        headers: HeaderMap::new(),
        body: b"Not Found".to_vec(),
    };

    for (method, path, handler) in router.routes.iter() {
        if req.method == *method && paths_match(&req.path, path) {
            req.params = extract_params(&req.path, path);

            match handler(req.clone()).await {
                Ok(responder) => {
                    response = responder.respond().await?;
                    break;
                }
                Err(e) => {
                    response = Response {
                        status: StatusCode::InternalServerError,
                        headers: HeaderMap::new(),
                        body: format!("Internal Server Error: {}", e).into_bytes(),
                    };

                    break;
                }
            }
        }
    }

    if response.status == StatusCode::NotFound {
        for (method, _, handler) in router.routes.iter() {
            if *method == Method::ALL {
                match handler(req.clone()).await {
                    Ok(responder) => {
                        response = responder.respond().await?;
                        break;
                    }
                    Err(e) => {
                        response = Response {
                            status: StatusCode::InternalServerError,
                            headers: HeaderMap::new(),
                            body: format!("Internal Server Error: {}", e).into_bytes(),
                        };

                        break;
                    }
                }
            }
        }
    }

    let response_string = format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n",
        response.status.to_code(),
        response.status.reason_phrase(),
        response.body.len()
    );

    stream.write_all(response_string.as_bytes()).await?;
    response.write_headers(&mut stream).await?;

    stream.write_all(b"\r\n").await?;
    stream.write_all(&response.body).await?;

    Ok(())
}

fn paths_match(request_path: &str, route_path: &str) -> bool {
    let req_segments: Vec<&str> = request_path.split('/').collect();
    let route_segments: Vec<&str> = route_path.split('/').collect();

    if req_segments.len() != route_segments.len() {
        return false;
    }

    for (req_seg, route_seg) in req_segments.iter().zip(route_segments.iter()) {
        if !route_seg.starts_with('{') && req_seg != route_seg {
            return false;
        }
    }

    true
}

fn extract_params(request_path: &str, route_path: &str) -> HashMap<String, String> {
    let mut params = HashMap::new();
    let req_segments: Vec<&str> = request_path.split('/').collect();
    let route_segments: Vec<&str> = route_path.split('/').collect();

    for (req_seg, route_seg) in req_segments.iter().zip(route_segments.iter()) {
        if route_seg.starts_with('{') && route_seg.ends_with('}') {
            let param_name = &route_seg[1..route_seg.len() - 1];
            params.insert(param_name.to_string(), req_seg.to_string());
        }
    }

    params
}

fn parse_query_string(query: &str) -> HashMap<String, String> {
    query
        .split('&')
        .filter(|s| !s.is_empty())
        .filter_map(|s| {
            let mut parts = s.splitn(2, '=');
            Some((parts.next()?.to_string(), parts.next().unwrap_or("").to_string()))
        })
        .collect()
}

fn parse_headers<'a, I>(lines: I) -> Result<HeaderMap, Error>
where
    I: Iterator<Item = &'a str>,
{
    let mut headers = HeaderMap::new();
    for line in lines {
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(':') {
            let header_name = HeaderName::from_bytes(key.trim().as_bytes()).map_err(|e| Error(format!("Invalid header name: {}", e)))?;
            let header_value = HeaderValue::from_str(value.trim()).map_err(|e| Error(format!("Invalid header value: {}", e)))?;
            headers.insert(header_name, header_value);
        }
    }
    Ok(headers)
}

fn parse_request(buffer: &[u8]) -> Result<Request, Error> {
    let request_str = String::from_utf8_lossy(buffer);
    let mut lines = request_str.lines();
    let first_line = lines.next().ok_or_else(|| Error("Invalid request".into()))?;
    let mut parts = first_line.split_whitespace();

    let method = match parts.next() {
        Some("GET") => Method::GET,
        Some("POST") => Method::POST,
        Some("PUT") => Method::PUT,
        Some("DELETE") => Method::DELETE,
        _ => return Err(Error("Invalid method".into())),
    };

    let full_path = parts.next().ok_or_else(|| Error("Invalid path".into()))?;
    let (path, query) = full_path.split_once('?').unwrap_or((full_path, ""));
    let query_params = parse_query_string(query);

    Ok(Request {
        method,
        headers: parse_headers(lines)?,
        query: query_params,
        body: Vec::new(),       // For simplicity, we're not parsing the body
        params: HashMap::new(), // This will be populated in handle_connection
        path: path.to_string(),
    })
}

pub struct Json<T>(pub T);

pub struct Text<'a>(pub Cow<'a, str>);

pub struct Bytes<'a>(pub Cow<'a, [u8]>);

impl<T: Serialize + Send + 'static> Responder for Json<T> {
    fn respond(self: Box<Self>) -> RespFuture {
        Box::pin(async move {
            let body = serde_json::to_string(&self.0)?;
            let mut response = Response::ok().body(body.into_bytes());
            response.content_type(ContentType::json());
            Ok(response)
        })
    }
}

impl Responder for String {
    fn respond(self: Box<Self>) -> RespFuture { Box::pin(async move { Ok(Response::ok().body(self.into_bytes()).content_type(ContentType::plaintext()).into()) }) }
}

impl Responder for Vec<u8> {
    fn respond(self: Box<Self>) -> RespFuture { Box::pin(async move { Ok(Response::ok().body(*self).content_type(ContentType::plaintext()).into()) }) }
}

impl Responder for &'static str {
    fn respond(self: Box<Self>) -> RespFuture { Box::pin(async move { Ok(Response::ok().body(self.as_bytes().to_vec()).content_type(ContentType::plaintext()).into()) }) }
}

impl Responder for &'static [u8] {
    fn respond(self: Box<Self>) -> RespFuture { Box::pin(async move { Ok(Response::ok().body(self.to_vec()).content_type(ContentType::plaintext()).into()) }) }
}

#[derive(Debug)]
pub struct Error(pub String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "{}", self.0) }
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
