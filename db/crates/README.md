# Kade Internals

This crate is part of [Kade](https://themackabu.dev/project/kade) - a cross-platform high-performance queue pipeline and key-value store.

> 🚀 **Looking for Kade documentation?** Check out the [main project page](https://crates.io/crates/kade) for full documentation, examples, and setup instructions.

## Overview

Core components of the project:

- [`kade-proto`](https://crates.io/crates/kade-proto): Protocol implementation and client libraries
- [`kade-server`](https://crates.io/crates/kade-server): Server implementation and connection handling
- [`kade`](https://crates.io/crates/kade): Main executable and CLI interface

## Technical Details

Built with:

- tokio + async I/O
- Efficient byte handling and buffer management
- Serialized state persistence
- Structured logging and tracing
