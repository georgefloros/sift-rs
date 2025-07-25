# Sift-rs

A Rust implementation of MongoDB query filtering, inspired by the JavaScript [sift.js](https://github.com/crcn/sift.js) library. This crate provides powerful query capabilities for filtering data structures using MongoDB-style syntax.

[![Crates.io](https://img.shields.io/crates/v/sift-rs.svg)](https://crates.io/crates/sift-rs)
[![Documentation](https://docs.rs/sift-rs/badge.svg)](https://docs.rs/sift-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- 🔍 **Comprehensive MongoDB operators**: `$eq`, `$ne`, `$gt`, `$gte`, `$lt`, `$lte`, `$in`, `$nin`, `$exists`, `$regex`, `$and`, `$or`, `$not`, `$all`, `$size`, `$mod`, `$type`, `$elemMatch`, `$nor`
- 🚀 **High performance**: Optimized for Rust with zero-copy operations where possible
- 🔒 **Type-safe**: Leverages Rust's type system for compile-time safety
- 🌐 **JSON compatibility**: Works seamlessly with `serde_json::Value`
- 📦 **Lightweight**: Minimal dependencies and small footprint
- 🔧 **Extensible**: Support for custom query operators
- 📚 **Well documented**: Comprehensive documentation with examples

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sift-rs = "0.1"
serde_json = "1.0"
```

## Quick Start

```rust
use sift_rs::sift;
use serde_json::json;

fn main() {
    let data = vec![
        json!({"name": "Alice", "age": 30, "city": "New York"}),
        json!({"name": "Bob", "age": 25, "city": "San Francisco"}),
        json!({"name": "Charlie", "age": 35, "city": "New York"}),
    ];

    // Find users older than 28
    let query = json!({"age": {"$gt": 28}});
    let results: Vec<_> = data.into_iter()
        .filter(|item| sift(&query, item).unwrap())
        .collect();

    println!("Users older than 28: {}", results.len()); // 2
}
```

## Usage Examples

### Basic Equality

```rust
use sift_rs::sift;
use serde_json::json;

let user = json!({"name": "Alice", "status": "active"});
let query = json!({"status": "active"});

assert!(sift(&query, &user).unwrap());
```

### Comparison Operators

```rust
// Greater than, less than, etc.
let query = json!({"age": {"$gte": 18, "$lt": 65}});
let user = json!({"age": 25});

assert!(sift(&query, &user).unwrap());
```

### Array Operations

```rust
// $in operator
let query = json!({"category": {"$in": ["electronics", "books"]}});
let product = json!({"category": "electronics"});

assert!(sift(&query, &product).unwrap());

// $all operator - array must contain all values
let query = json!({"tags": {"$all": ["rust", "programming"]}});
let article = json!({"tags": ["rust", "programming", "tutorial"]});

assert!(sift(&query, &article).unwrap());
```

### Nested Field Queries

```rust
// Dot notation for nested fields
let query = json!({"user.profile.age": {"$gte": 21}});
let data = json!({
    "user": {
        "profile": {
            "age": 25
        }
    }
});

assert!(sift(&query, &data).unwrap());
```

### Logical Operators

```rust
// $and operator
let query = json!({
    "$and": [
        {"age": {"$gte": 18}},
        {"status": "active"}
    ]
});

// $or operator
let query = json!({
    "$or": [
        {"category": "premium"},
        {"price": {"$lt": 100}}
    ]
});

// $not operator
let query = json!({"age": {"$not": {"$lt": 18}}});
```

### Regular Expressions

```rust
let query = json!({"name": {"$regex": "^A", "$options": "i"}});
let user = json!({"name": "Alice"});

assert!(sift(&query, &user).unwrap());
```

### Complex Queries

```rust
let complex_query = json!({
    "status": "active",
    "age": {"$gte": 25, "$lt": 40},
    "$or": [
        {"department": "engineering"},
        {"salary": {"$gte": 80000}}
    ],
    "skills": {"$in": ["rust", "python", "javascript"]}
});
```

### Reusable Filters

```rust
use sift_rs::create_filter;

let filter = create_filter(&json!({"age": {"$gte": 18}})).unwrap();

let users = vec![
    json!({"name": "Alice", "age": 25}),
    json!({"name": "Bob", "age": 17}),
];

let adults: Vec<_> = users.into_iter().filter(filter).collect();
```

## Supported Operators

### Comparison Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `$eq` | Equal to | `{"age": {"$eq": 25}}` |
| `$ne` | Not equal to | `{"status": {"$ne": "inactive"}}` |
| `$gt` | Greater than | `{"price": {"$gt": 100}}` |
| `$gte` | Greater than or equal | `{"age": {"$gte": 18}}` |
| `$lt` | Less than | `{"score": {"$lt": 50}}` |
| `$lte` | Less than or equal | `{"quantity": {"$lte": 10}}` |

### Array Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `$in` | Value in array | `{"category": {"$in": ["a", "b"]}}` |
| `$nin` | Value not in array | `{"status": {"$nin": ["banned"]}}` |
| `$all` | Array contains all values | `{"tags": {"$all": ["new", "sale"]}}` |
| `$size` | Array has specific length | `{"items": {"$size": 3}}` |
| `$elemMatch` | Array element matches query | `{"scores": {"$elemMatch": {"$gt": 80}}}` |

### Logical Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `$and` | Logical AND | `{"$and": [{"a": 1}, {"b": 2}]}` |
| `$or` | Logical OR | `{"$or": [{"a": 1}, {"b": 2}]}` |
| `$not` | Logical NOT | `{"age": {"$not": {"$lt": 18}}}` |
| `$nor` | Logical NOR | `{"$nor": [{"a": 1}, {"b": 2}]}` |

### Field Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `$exists` | Field exists | `{"email": {"$exists": true}}` |
| `$type` | Field has specific type | `{"age": {"$type": "number"}}` |
| `$regex` | Regular expression match | `{"name": {"$regex": "^A"}}` |
| `$mod` | Modulo operation | `{"count": {"$mod": [2, 0]}}` |

## Performance Considerations

- Queries are compiled once and can be reused multiple times
- Use `create_filter()` for repeated filtering operations
- Complex nested queries may have performance implications
- Consider indexing strategies for large datasets

## Error Handling

All operations return `Result<T, SiftError>` for proper error handling:

```rust
use sift_rs::{sift, SiftError};

match sift(&query, &data) {
    Ok(matches) => println!("Query result: {}", matches),
    Err(SiftError::InvalidQuery(msg)) => eprintln!("Invalid query: {}", msg),
    Err(SiftError::UnsupportedOperation(op)) => eprintln!("Unsupported operator: {}", op),
    Err(e) => eprintln!("Error: {}", e),
}
```

## Examples

Run the comprehensive example:

```bash
cargo run --example basic_usage
```

This will demonstrate all major features with sample data.

## Comparison with JavaScript sift.js

| Feature | sift.js | sift-rs | Notes |
|---------|---------|---------|-------|
| Basic operators | ✅ | ✅ | Full compatibility |
| Logical operators | ✅ | ✅ | Complete support |
| Array operations | ✅ | ✅ | Including $elemMatch |
| Regex support | ✅ | ✅ | With options |
| Custom operators | ✅ | ⚠️ | Planned for future |
| Performance | Good | Excellent | Rust's zero-cost abstractions |
| Type safety | Runtime | Compile-time | Rust advantage |

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development

```bash
# Clone the repository
git clone https://github.com/username/sift-rs.git
cd sift-rs

# Run tests
cargo test

# Run examples
cargo run --example basic_usage

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy -- -D warnings
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [sift.js](https://github.com/crcn/sift.js) by Craig Condon
- Built with [serde_json](https://github.com/serde-rs/json) for JSON handling
- Uses [regex](https://github.com/rust-lang/regex) for pattern matching

## Roadmap

- [ ] Custom operator support
- [ ] Performance optimizations for large datasets
- [ ] Support for more MongoDB operators
- [ ] Integration with popular Rust databases
- [ ] Async query execution
- [ ] Query optimization and analysis tools

---

Made with ❤️ in Rust
