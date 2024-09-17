# Blaze

Blaze is a lightweight, asynchronous web framework for Rust, designed for simplicity and performance.

## Features

- Asynchronous request handling
- Routing with support for path parameters
- JSON serialization and deserialization
- Easy-to-use macros for defining routes

## Quick Start

1. Add Blaze to your `Cargo.toml`

2. Create a simple web server:

```rust
use blaze::{prelude::*, routes, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Hello {
	 name: &'static str,
	 message: String,
}

#[blaze::route(get, "/hello/{name}")]
async fn hello(_req: Request, name: String) -> Json<Hello> {
	 Json(Hello {
		  name: "Blaze",
		  message: format!("Hello, {}!", name),
	 })
}

#[blaze::main]
fn main() {
	 let router = routes! {
		  hello,
	 };

	 Server::bind("127.0.0.1", 8080).serve(router)?
}
```

3. Run your server and visit `http://localhost:8080/hello/world` in your browser.
