# A even smaller tokio

## Features

- Basic task spawning and execution
- Asynchronous runtime with a simple event loop
- Custom `Future` implementation for delays
- Thread-safe task scheduling using channels

## Usage

To use mini-tokio in your Rust project:

1. Create a new `Runtime` instance
2. Spawn tasks using the `block_on` function
3. Or use the mini_tokio::main attribute

Examples:

```rust
use mini_tokio::Runtime;

let rt = Runtime::new();

mini_tokio.block_on(async {
   // Your async code here
});
```

```rust
#[mini_tokio::main]
async fn main() {
   // Your async code here
}
```

## Note

This is a minimal implementation for educational purposes and is not intended for production use. For real-world applications, consider using the full Tokio runtime or other production-ready async runtimes.
