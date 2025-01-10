# FAST Database

FAST is a lightweight, high-performance key-value store and pub/sub server written in Rust. It provides a Redis-like interface with support for basic operations, expiring keys, and pub/sub messaging. Not fully compatible but works with some Redis projects like redis-cli, etc.

## Features

- Key expiration support
- Messaging system
- Persistent storage capabilities
- Support for both synchronous and asynchronous clients
- Connection pooling and buffered operations
- Clean shutdown with state preservation
- Cross-platform support

### Basic Usage

Here's a simple example demonstrating basic key-value operations:

```rust
use fast::prelude::*;

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut client = Client::connect(&format!("127.0.0.1:{}", DEFAULT_PORT)).await?;

    // set a value
    client.set("cat", "meow".into()).await?;

    // get the value
    let result = client.get("cat").await?;
    println!("Value: {:?}", result);

    Ok(())
}
```

### Running the Server

Start the FAST server using the provided binary:

```bash
            # optional storage dir
fast-server --state cached-data.fdb
```

Server options:

- `--port`: Server port (default: 6379)
- `--host`: Bind address (default: 127.0.0.1)
- `--state`: Path to save/load database state
- `-v`: Increase logging verbosity
- `-q`: Decrease logging verbosity

### Using the CLI

FAST comes with an interface (`fast-cli`) for interacting with the server:

Example commands:

```
> set mykey myvalue
OK
> get mykey
"myvalue"
> set mykey "expires soon" 5000
OK
> publish news "Breaking news!"
Publish OK
```

## Features in Detail

### Key-Value Operations

- `GET key`: Retrieve the value of a key
- `SET key value [expiration]`: Set a key with optional expiration in milliseconds

### Pub/Sub Messaging

- `PUBLISH channel message`: Send a message to a channel
- `SUBSCRIBE channel [channel...]`: Subscribe to one or more channels
- `UNSUBSCRIBE [channel...]`: Unsubscribe from channels

### State Management

- `DUMP [filename]`: Save the current database state to a file
- `LOAD [filename]`: Load database state from a file

### Client Libraries

FAST provides multiple client implementations:

1. **Async Client** (`Client`):

```rust
let mut client = Client::connect("127.0.0.1:6379").await?;
```

2. **Blocking Client** (`BlockingClient`):

```rust
let mut client = BlockingClient::connect("127.0.0.1:6379")?;
```

3. **Buffered Client** (`BufferedClient`):

```rust
let client = Client::connect("127.0.0.1:6379").await?;
let mut buffered = BufferedClient::buffer(client);
```

More will be added for other languages soon.

## Architecture

FAST is built with a modular architecture:

- `fast`: Main crate containing the client and server binaries
- `fast-proto`: Protocol implementation and client libraries
- `fast-server`: Server implementation

The server uses Tokio for async I/O and supports graceful shutdown with state preservation.
