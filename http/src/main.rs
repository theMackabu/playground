use serde::{Deserialize, Serialize};
use server::{route, routes, Error, HttpResponse, Json, Request, Responder, Response, Router, Server, Str};

#[derive(Serialize, Deserialize)]
struct Hello {
    name: &'static str,
    furry: bool,
}

#[route(get, "/hello/result")]
async fn hello_result(_req: Request) -> Result<Response, Error> {
    let body = "Hello, World! (From a Result)".as_bytes().to_vec();
    Ok(Response::ok().body(body))
}

#[route(get, "/hello/impl")]
async fn hello_impl(_req: Request) -> impl Responder {
    let body = "Hello, World! (From a impl)".as_bytes().to_vec();
    Response::ok().body(body)
}

#[route(get, "/hello/response")]
async fn hello_response(_req: Request) -> HttpResponse {
    let body = "Hello, World! (From a HttpResponse)".as_bytes().to_vec();
    Ok(Response::ok().body(body))
}

#[route(get, "/hello/{name}")]
async fn hello(_req: Request, name: String) -> String { format!("Hello, {name}!") }

#[route(get, "/json")]
async fn json(_req: Request) -> Json<Hello> { Json(Hello { name: "themackabu", furry: true }) }

#[route(default = true)]
async fn not_found(_req: Request) -> Str { "Hello, World!" }

#[server::main]
fn main() {
    let router = routes! {
        json,
        hello,
        hello_impl,
        hello_result,
        hello_response,
        not_found,
    };

    Server::new("127.0.0.1", 8080).serve(router)?
}
