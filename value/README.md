# Dare

A flexible `Value` enum implementation in Rust, supporting various data types and providing type-safe operations.

## Features

- Supports multiple data types: null, boolean, float, integer, string, array.
- Easy conversion with `From` trait implementations
- Type-checking and accessor methods

## Usage

```rust
use dare::Value;

let number = Value::from(42);
let string = Value::from("Hello");
let array = Value::from(vec![Value::from(1), Value::from(2)]);

assert!(number.is_number());
assert_eq!(string.as_string(), Some("Hello".to_string()));

println!("{}", array); // Outputs: [1, 2]
```
