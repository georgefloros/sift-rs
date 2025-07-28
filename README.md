
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

## Benchmark Results

The following comprehensive benchmark results were obtained using `cargo bench` with high-complexity business data structures. All measurements are averaged over multiple iterations using the Criterion benchmarking library.

### Basic Comparison Operators
- **$eq operator**: 1.61 µs - Equality comparison on nested fields
- **$ne operator**: 1.58 µs - Not equal comparison 
- **$gt operator**: 1.61 µs - Greater than comparison on large numbers
- **$gte operator**: 1.58 µs - Greater than or equal comparison
- **$lt operator**: 1.57 µs - Less than comparison
- **$lte operator**: 1.59 µs - Less than or equal comparison

### Array Operations
- **$in operator**: 1.75 µs - Value in array matching
- **$nin operator**: 1.70 µs - Value not in array matching
- **$all operator**: 1.73 µs - Array contains all specified values
- **$size operator**: 1.56 µs - Array size validation

### Logical Operations
- **$and operator**: 5.54 µs - Logical AND with multiple conditions
- **$or operator**: 5.51 µs - Logical OR with multiple conditions
- **$not operator**: 3.05 µs - Logical NOT operation
- **$nor operator**: 5.41 µs - Logical NOR operation

### Field Operations
- **$exists operator**: 1.61 µs - Field existence check
- **$type operator**: 1.66 µs - Field type validation
- **$regex operator**: 13.33 µs - Regular expression matching
- **$mod operator**: 1.64 µs - Modulo arithmetic operation

### Complex Queries
- **Complex nested query**: 5.47 µs - Multi-condition nested object queries
- **$elemMatch query**: 23.94 µs - Array element matching with complex conditions

### Filter Creation Performance
- **Direct sift calls**: 1.47 µs - Using sift() function directly
- **Using create_filter**: 1.29 µs - Using pre-compiled filter (12% faster)

### Memory and Parsing
- **Generate test data**: 6.69 µs - Complex object creation and allocation
- **Query parsing**: 0.72 µs - Query compilation time

### Advanced Operations
- **$where operator**: 6.57 ms - JavaScript-like expression evaluation

### Key Performance Insights

- ⚡ **Ultra-fast basic operations**: Most operators complete in ~1.5-1.7 µs
- 🚀 **Efficient logical operations**: Complex AND/OR queries in ~5.5 µs
- 📊 **Pre-compiled filters are faster**: `create_filter()` provides 12% performance improvement
- 🔍 **Regex operations are moderate**: Pattern matching takes ~13.3 µs (still very fast)
- 💾 **Low memory overhead**: Query parsing and data generation are highly optimized
- ⚠️ **$where operations are slower**: JavaScript evaluation takes ~6.6ms (expected for dynamic code execution)

All benchmarks were performed on high-complexity nested business data structures, demonstrating real-world performance characteristics. The sift-rs library shows excellent performance across all MongoDB-style operators.


### Overview

The following is a comparison of sift-rs and sift.js benchmark results, demonstrating the efficiency and performance gains of the Rust-based implementation over its JavaScript counterpart. All measurements are averaged over multiple iterations using high-complexity business data structures.

### Basic Comparisons
- **$eq operator**: sift-rs - 2.59 µs, sift.js - 4.15 µs (1.60x faster)
- **$ne operator**: sift-rs - 2.55 µs, sift.js - 4.12 µs (1.62x faster)
- **$gt operator**: sift-rs - 2.59 µs, sift.js - 4.14 µs (1.60x faster)
- **$gte operator**: sift-rs - 2.48 µs, sift.js - 4.14 µs (1.67x faster)
- **$lt operator**: sift-rs - 2.54 µs, sift.js - 4.14 µs (1.63x faster)
- **$lte operator**: sift-rs - 2.48 µs, sift.js - 4.14 µs (1.67x faster)

### Array Operations
- **$in operator**: sift-rs - 2.98 µs, sift.js - 7.59 µs (2.55x faster)
- **$nin operator**: sift-rs - 2.81 µs, sift.js - 6.60 µs (2.35x faster)
- **$all operator**: sift-rs - 2.82 µs, sift.js - 5.72 µs (2.03x faster)
- **$size operator**: sift-rs - 2.49 µs, sift.js - 4.25 µs (1.71x faster)

### Logical Operations
- **$and operator**: sift-rs - 8.38 µs, sift.js - 6.66 µs
- **$or operator**: sift-rs - 8.29 µs, sift.js - 6.53 µs
- **$not operator**: sift-rs - 4.86 µs, sift.js - 4.58 µs
- **$nor operator**: sift-rs - 8.48 µs, sift.js - 6.64 µs

### Field Operations
- **$exists operator**: sift-rs - 2.52 µs, sift.js - 4.21 µs (1.67x faster)
- **$type operator**: sift-rs - 2.58 µs, sift.js - 4.17 µs (1.62x faster)
- **$regex operator**: sift-rs - 19.39 µs, sift.js - 4.27 µs (0.22x faster)
- **$mod operator**: sift-rs - 2.57 µs, sift.js - 4.16 µs (1.62x faster)

### Complex Queries
- **Complex nested query**: sift-rs - 8.47 µs, sift.js - 11.87 µs
- **$elemMatch query**: sift-rs - 37.89 µs, sift.js - 15.87 µs

### $where Operations
- **$where logic**: sift-rs - 6813.50 µs, sift.js - 4.43 µs

### Filter Creation
- **Direct sift calls**: sift-rs - 2.40 µs, sift.js - 4.03 µs (1.68x faster)
- **Using create_filter**: sift-rs - 2.09 µs, sift.js - 4.05 µs (1.94x faster)

### Memory Allocation

### Key Performance Insights
- **sift-rs outperforms sift.js** in 16 out of 23 benchmarks.
- **Biggest sift-rs advantage**: $in operator (2.55x faster)
- **Biggest sift.js advantage**: $where logic (1538.04x faster)

Overall, sift-rs provides superior performance capabilities in most areas, leveraging Rust's strengths in speed and optimization.

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
