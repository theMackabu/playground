// required dependencies
use blaze::{prelude::*, routes, Json, Redirect, Response, Str};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Hello {
    name: String,
    furry: bool,
}

#[blaze::route(get, "/hello/result")]
async fn hello_result(_req: Request) -> Result<Response, Error> {
    let body = "Hello, World! (From a Result)".as_bytes().to_vec();
    Ok(Response::ok().body(body))
}

#[blaze::route(get, "/hello/impl")]
async fn hello_impl(_req: Request) -> impl Responder {
    let body = "Hello, World! (From a impl)".as_bytes().to_vec();
    Response::ok().body(body)
}

#[blaze::route(get, "/hello/response")]
async fn hello_response(_req: Request) -> HttpResponse {
    let body = "Hello, World! (From a HttpResponse)".as_bytes().to_vec();
    Ok(Response::ok().body(body))
}

#[blaze::route(get, "/hello/{name}")]
async fn hello(_req: Request, name: String) -> String { format!("Hello, {name}!") }

#[blaze::route(get, "/redirect")]
async fn redirect(_req: Request) -> HttpResponse { Ok(Redirect::temporary("/json")?) }

#[blaze::route(get, "/json")]
async fn json(_req: Request) -> Json<Hello> {
    Json(Hello {
        name: "themackabu".into(),
        furry: true,
    })
}

#[blaze::route(post, "/json")]
async fn get_body(req: Request) -> Result<Json<Hello>, Error> {
    let data: Hello = req.json()?;
    Ok(Json(data))
}

#[blaze::route(default = true)]
async fn not_found(_req: Request) -> Str { "Hello, World!" }

#[blaze::main]
fn main() {
    // logging output
    tracing_subscriber::fmt().pretty().with_thread_names(true).with_max_level(tracing::Level::TRACE).init();

    let router = routes! {
        json,
        hello,
        redirect,
        get_body,
        not_found,
        hello_impl,
        hello_result,
        hello_response,
    };

    Server::new("127.0.0.1", 8080).serve(router)?
}
