# <img src="" width="30" height="30" align="center" /> Kade

Cross platform high-performance queue pipeline KV store

Previously, tools like:

- Redis required complex configuration
- DynamoDB lacked simple local development
- RabbitMQ needed heavy infrastructure setup

Kade attempts to solve these problems:

- It requires no configuration - just run and go
- It provides simple persistence out of the box
- Use it via CLI, libraries, or existing Redis tools!

## Features

- Key expiration support
- Subscription messaging system
- Persistent storage capabilities
- Both sync and async client support
- Connection pooling and buffered operations
- Graceful shutdown with state preservation

### Basic Usage

Here's a simple example demonstrating basic key-value operations:

```rust
use kade::prelude::*;

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

Start the Kade server using the provided binary:

```bash
            # optional storage dir
kade-server --state cached-data.kade
```

Server options:

- `--port`: Server port (default: 6379)
- `--host`: Bind address (default: 127.0.0.1)
- `--state`: Path to save/load database state
- `-v`: Increase logging verbosity
- `-q`: Decrease logging verbosity

### Using the CLI

Kade comes with an interface (`kade-cli`) for interacting with the server:

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

Kade provides multiple client implementations:

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

Kade is built with a modular architecture:

- `kade`: Main crate containing the client and server binaries
- `kade-proto`: Protocol implementation and client libraries
- `kade-server`: Server implementation

The server uses Tokio for async I/O and supports graceful shutdown with state preservation.
