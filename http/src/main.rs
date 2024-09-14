use serde::{Deserialize, Serialize};
use server::{route, Error, HttpResponse, Request, Responder, Response, Router, Server};

#[derive(Serialize, Deserialize)]
struct Hello {
    name: String,
    age: i64,
}

#[route(GET, "/hello/result")]
async fn hello_result(_req: Request) -> Result<Response, Error> {
    let body = "Hello, World! (From a Result)".as_bytes().to_vec();
    Ok(Response::new(200.into(), body))
}

#[route(GET, "/hello/impl")]
async fn hello_impl(_req: Request) -> impl Responder {
    let body = "Hello, World! (From a impl)".as_bytes().to_vec();
    Response::new(200.into(), body)
}

#[route(GET, "/hello/response")]
async fn hello_response(_req: Request) -> HttpResponse {
    let body = "Hello, World! (From a HttpResponse)".as_bytes().to_vec();
    Ok(Response::new(200.into(), body))
}

#[route(GET, "/hello")]
async fn hello(_req: Request) -> Response {
    let body = "Hello, World!".as_bytes().to_vec();
    Response::new(200.into(), body)
}

#[server::main]
fn main() {
    let mut router = Router::new();

    __ROUTE_HELLO_RESULT(&mut router);
    __ROUTE_HELLO(&mut router);
    __ROUTE_HELLO_IMPL(&mut router);
    __ROUTE_HELLO_RESPONSE(&mut router);

    Server::new("127.0.0.1", 8080).serve(router)?
}
