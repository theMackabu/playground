# A even smaller tokio

## Features

- Basic task spawning and execution
- Asynchronous runtime with a simple event loop
- Custom `Future` implementation for delays
- Thread-safe task scheduling using channels

## Usage

To use mini-tokio in your Rust project:

1. Create a new `MiniTokio` instance
2. Spawn tasks using the `spawn` function
3. Run the runtime using the `run` method

Example:

```rust
let mini_tokio = MiniTokio::new();

mini_tokio.spawn(async {
	 // Your async code here
});

mini_tokio.run();
```

## Note

This is a minimal implementation for educational purposes and is not intended for production use. For real-world applications, consider using the full Tokio runtime or other production-ready async runtimes.
