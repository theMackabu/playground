use serde::{Deserialize, Serialize};
use server::{route, routes, Error, HttpResponse, Request, Responder, Response, Router, Server};

#[derive(Serialize, Deserialize)]
struct Hello {
    name: String,
    age: i64,
}

#[route(get, "/hello/result")]
async fn hello_result(_req: Request) -> Result<Response, Error> {
    let body = "Hello, World! (From a Result)".as_bytes().to_vec();
    Ok(Response::new(200.into(), body))
}

#[route(get, "/hello/impl")]
async fn hello_impl(_req: Request) -> impl Responder {
    let body = "Hello, World! (From a impl)".as_bytes().to_vec();
    Response::new(200.into(), body)
}

#[route(get, "/hello/response")]
async fn hello_response(_req: Request) -> HttpResponse {
    let body = "Hello, World! (From a HttpResponse)".as_bytes().to_vec();
    Ok(Response::new(200.into(), body))
}

#[route(get, "/hello")]
async fn hello(_req: Request) -> Response {
    let body = "Hello, World!".as_bytes().to_vec();
    Response::new(200.into(), body)
}

#[route(default = true)]
async fn default(_req: Request) -> Response {
    let body = "Hello, World!".as_bytes().to_vec();
    Response::new(200.into(), body)
}

#[server::main]
fn main() {
    let router = routes! {
        hello,
        hello_impl,
        hello_result,
        hello_response
    };

    Server::new("127.0.0.1", 8080).serve(router)?
}
