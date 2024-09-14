pub use codegen::{main, route, routes};
pub use tokio;

use serde::Serialize;
use std::collections::HashMap;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub trait Responder: Send {
    fn respond(self: Box<Self>) -> Pin<Box<dyn Future<Output = Result<Response, Error>> + Send>>;
}

pub type HttpResponse = Result<Response, Error>;

pub type HttpFuture = Pin<Box<dyn Future<Output = Result<Box<dyn Responder>, Error>> + Send>>;

#[derive(Clone, Debug, PartialEq)]
pub enum Method {
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
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Clone)]
pub struct Response {
    pub status: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(status: StatusCode, body: Vec<u8>) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/plain".to_string());
        Response { status, headers, body }
    }
}

impl Responder for Response {
    fn respond(self: Box<Self>) -> Pin<Box<dyn Future<Output = Result<Response, Error>> + Send>> { Box::pin(async move { Ok(*self) }) }
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
    let req = parse_request(&buffer[..n])?;

    let mut response = Response::new(StatusCode::NotFound, b"Not Found".to_vec());

    for (method, path, handler) in router.routes.iter() {
        if req.method == *method && req.path == *path {
            match handler(req.clone()).await {
                Ok(responder) => {
                    response = responder.respond().await?;
                    break;
                }
                Err(e) => {
                    response = Response::new(StatusCode::InternalServerError, format!("Internal Server Error: {}", e).into_bytes());
                    break;
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

    for (key, value) in response.headers.iter() {
        let header = format!("{}: {}\r\n", key, value);
        stream.write_all(header.as_bytes()).await?;
    }

    stream.write_all(b"\r\n").await?;
    stream.write_all(&response.body).await?;

    Ok(())
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
    let path = parts.next().ok_or_else(|| Error("Invalid path".into()))?.to_string();

    let mut headers = HashMap::new();
    for line in lines {
        if line.is_empty() {
            break;
        }
        if let Some((key, value)) = line.split_once(':') {
            headers.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    Ok(Request {
        method,
        path,
        headers,
        body: Vec::new(), // For simplicity, we're not parsing the body
    })
}

pub struct Json<T>(pub T);

impl<T: Serialize + Send + 'static> Responder for Json<T> {
    fn respond(self: Box<Self>) -> Pin<Box<dyn Future<Output = Result<Response, Error>> + Send>> {
        Box::pin(async move {
            let body = serde_json::to_string(&self.0)?;
            let mut response = Response::new(StatusCode::Ok, body.into_bytes());
            response.headers.insert("Content-Type".to_string(), "application/json".to_string());
            Ok(response)
        })
    }
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
