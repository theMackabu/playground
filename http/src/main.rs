use serde::{Deserialize, Serialize};
use server::{route, Error, Request, Responder, Response, Router, Server};

#[derive(Serialize, Deserialize)]
struct Hello {
    name: String,
    age: i64,
}

#[route(GET, "/hello/result")]
async fn hello_result(req: Request) -> Result<Box<dyn Responder>, Error> {
    let body = "Hello, World! (From a Result)".as_bytes().to_vec();
    Ok(Box::new(Response::new(200.into(), body)))
}

#[route(GET, "/hello")]
async fn hello(req: Request) -> Box<dyn Responder> {
    let body = "Hello, World!".as_bytes().to_vec();
    Box::new(Response::new(200.into(), body))
}

#[server::main]
fn main() {
    let mut router = Router::new();
    __ROUTE_HELLO_RESULT(&mut router);
    __ROUTE_HELLO(&mut router);
    Server::new("127.0.0.1", 8080).serve(router)?
}
