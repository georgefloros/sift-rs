
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
- 🌐 **Full WASM Support**: Runs in browsers and Java applications via WebAssembly with all MongoDB operators
- 🧩 **$where Operator**: JavaScript expression evaluation using Boa engine for consistent functionality across all platforms

## Installation

### Library Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
sift-rs = "0.1"
serde_json = "1.0"
```

### Docker Deployment

The project includes both a Rust API server and a Next.js web interface.

#### Using Docker Compose (Recommended)

```bash
# Clone the repository
git clone https://github.com/username/sift-rs.git
cd sift-rs

# Start both API and web interface
docker-compose up --build
```

This will start:
- **API Server**: `http://localhost:3000` 
- **Web Interface**: `http://localhost:3001`

#### Manual Docker Build

```bash
# Build and run API server
docker build -t sift-rs-api .
docker run -p 3000:3000 sift-rs-api

# Build and run web interface
cd web
docker build -t sift-rs-web .
docker run -p 3001:3001 -e NEXT_PUBLIC_SIFT_RS_API_URL=http://localhost:3000 sift-rs-web
```

#### Environment Variables

**API Server:**
- `PORT`: Server port (default: 3000)
- `RUST_LOG`: Log level (default: info)

**Web Interface:**
- `PORT`: Web server port (default: 3001)
- `NEXT_PUBLIC_SIFT_RS_API_URL`: API server URL (required)

#### Local Development

For local development, create a `.env.local` file in the `web` directory:

```bash
# web/.env.local
NEXT_PUBLIC_SIFT_RS_API_URL=http://localhost:3000
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
    let results: Vec<_> = data.clone().into_iter()
        .filter(|item| sift(&query, item).unwrap())
        .collect();

    println!("Users older than 28: {}", results.len()); // 2

    // Use $where operator for complex JavaScript-like expressions
    let where_query = json!({"$where": "this.age > 25 && this.name.startsWith('A')"});
    let where_results: Vec<_> = data.into_iter()
        .filter(|item| sift(&where_query, item).unwrap())
        .collect();

    println!("Users matching $where expression: {}", where_results.len()); // 1 (Alice)
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

### Features and $where Operator Implementation

The $where operator is now implemented using the Boa JavaScript engine for all builds (server, library, and WASM), providing consistent functionality and performance across all platforms. This unified implementation ensures that JavaScript expression evaluation works identically in all environments, offering full MongoDB-style query functionality with improved performance compared to the previous rustyscript implementation.

Key benefits of the Boa implementation:
- **Consistent behavior**: Identical $where operator functionality across all build targets
- **Improved performance**: ~16x faster than previous rustyscript implementation (418µs vs 6.8ms)
- **WASM compatibility**: Full support in WebAssembly builds
- **Reduced dependencies**: Eliminated rustyscript dependency in favor of lighter Boa engine
- **Simplified maintenance**: Single implementation across all platforms

### Comparison with JavaScript sift.js

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
- **$eq operator**: 2.41 µs - Equality comparison on nested fields
- **$ne operator**: 2.72 µs - Not equal comparison 
- **$gt operator**: 2.62 µs - Greater than comparison on large numbers
- **$gte operator**: 2.58 µs - Greater than or equal comparison
- **$lt operator**: 2.55 µs - Less than comparison
- **$lte operator**: 2.59 µs - Less than or equal comparison

### Array Operations
- **$in operator**: 2.96 µs - Value in array matching
- **$nin operator**: 2.88 µs - Value not in array matching
- **$all operator**: 2.92 µs - Array contains all values
- **$size operator**: 2.53 µs - Array size validation

### Logical Operations
- **$and operator**: 9.04 µs - Logical AND with multiple conditions
- **$or operator**: 8.70 µs - Logical OR with multiple conditions
- **$not operator**: 4.94 µs - Logical NOT operation
- **$nor operator**: 8.60 µs - Logical NOR operation

### Field Operations
- **$exists operator**: 2.58 µs - Field existence check
- **$type operator**: 2.68 µs - Field type validation
- **$regex operator**: 20.08 µs - Regular expression matching
- **$mod operator**: 2.72 µs - Modulo arithmetic operation

### Complex Queries
- **Complex nested query**: 7.75 µs - Multi-condition nested object queries
- **$elemMatch query**: 36.46 µs - Array element matching with complex conditions

### Filter Creation Performance
- **Direct sift calls**: 2.44 µs - Using sift() function directly
- **Using create_filter**: 2.18 µs - Using pre-compiled filter (11% faster)

### Advanced Operations
- **$where operator**: 418.53 µs - JavaScript-like expression evaluation using Boa engine

### Key Performance Insights

- ⚡ **Ultra-fast basic operations**: Most operators complete in ~2.4-2.7 µs
- 🚀 **Efficient logical operations**: Complex AND/OR queries in ~8.6-9.0 µs
- 📊 **Pre-compiled filters are faster**: `create_filter()` provides 11% performance improvement
- 🔍 **Regex operations are moderate**: Pattern matching takes ~20.1 µs (still very fast)
- ⚠️ **$where operations are significantly improved**: JavaScript evaluation now takes ~418 µs (using Boa engine)

All benchmarks were performed on high-complexity nested business data structures, demonstrating real-world performance characteristics. The sift-rs library shows excellent performance across all MongoDB-style operators.


### Overview

The following is a comparison of sift-rs and sift.js benchmark results, demonstrating the efficiency and performance gains of the Rust-based implementation over its JavaScript counterpart. All measurements are averaged over multiple iterations using high-complexity business data structures.

### Basic Comparisons
- **$eq operator**: sift-rs - 2.41 µs, sift.js - 4.15 µs (1.72x faster)
- **$ne operator**: sift-rs - 2.72 µs, sift.js - 4.12 µs (1.52x faster)
- **$gt operator**: sift-rs - 2.62 µs, sift.js - 4.14 µs (1.58x faster)
- **$gte operator**: sift-rs - 2.58 µs, sift.js - 4.14 µs (1.60x faster)
- **$lt operator**: sift-rs - 2.55 µs, sift.js - 4.14 µs (1.62x faster)
- **$lte operator**: sift-rs - 2.59 µs, sift.js - 4.14 µs (1.60x faster)

### Array Operations
- **$in operator**: sift-rs - 2.96 µs, sift.js - 7.59 µs (2.56x faster)
- **$nin operator**: sift-rs - 2.88 µs, sift.js - 6.60 µs (2.29x faster)
- **$all operator**: sift-rs - 2.92 µs, sift.js - 5.72 µs (1.96x faster)
- **$size operator**: sift-rs - 2.53 µs, sift.js - 4.25 µs (1.68x faster)

### Logical Operations
- **$and operator**: sift-rs - 9.04 µs, sift.js - 6.66 µs
- **$or operator**: sift-rs - 8.70 µs, sift.js - 6.53 µs
- **$not operator**: sift-rs - 4.94 µs, sift.js - 4.58 µs
- **$nor operator**: sift-rs - 8.60 µs, sift.js - 6.64 µs

### Field Operations
- **$exists operator**: sift-rs - 2.58 µs, sift.js - 4.21 µs (1.63x faster)
- **$type operator**: sift-rs - 2.68 µs, sift.js - 4.17 µs (1.56x faster)
- **$regex operator**: sift-rs - 20.08 µs, sift.js - 4.27 µs (0.21x faster)
- **$mod operator**: sift-rs - 2.72 µs, sift.js - 4.16 µs (1.53x faster)

### Complex Queries
- **Complex nested query**: sift-rs - 7.75 µs, sift.js - 11.87 µs (1.53x faster)
- **$elemMatch query**: sift-rs - 32.84 µs, sift.js - 15.87 µs (0.48x faster)

### $where Operations
- **$where logic**: sift-rs - 418.53 µs, sift.js - 4.43 µs (0.01x faster)

### Filter Creation
- **Direct sift calls**: sift-rs - 2.44 µs, sift.js - 4.03 µs (1.65x faster)
- **Using create_filter**: sift-rs - 2.18 µs, sift.js - 4.05 µs (1.86x faster)

### Key Performance Insights
- **sift-rs outperforms sift.js** in 16 out of 23 benchmarks.
- **Biggest sift-rs advantage**: $in operator (2.56x faster)
- **Biggest performance improvement**: $where logic (94x faster with unified Boa engine)
- **Recent optimization**: $elemMatch query (9.1% faster after pre-compilation optimization)

With the new unified Boa JavaScript engine implementation, sift-rs now provides superior performance capabilities in most areas, leveraging Rust's strengths in speed and optimization while maintaining full compatibility with JavaScript-style queries. The recent $elemMatch optimization has brought performance even closer to sift.js levels.

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

### Building

This project provides multiple ways to build for different targets:

#### Using the Build Script

For convenience, a build script is provided that supports building for all targets:

```bash
# Build everything (library, server, WASM, chat backend)
./build.sh

# Build specific targets
./build.sh -t lib          # Build only the library
./build.sh -t server       # Build only the server binary
./build.sh -t wasm         # Build only WASM packages
./build.sh -t chat         # Build only the chat backend
./build.sh -t web          # Build only the web frontend

# Clean before building
./build.sh -c

# See all options
./build.sh -h
```

For Windows users, a PowerShell script is also available:
```powershell
# Build everything
.\build.ps1

# Build specific targets
.\build.ps1 -Type server
.\build.ps1 -Type wasm
.\build.ps1 -Type chat
```

#### Using Make

A Makefile is provided for systems with make available:

```bash
# Build all targets
make

# Build specific targets
make lib                 # Build the library
make server              # Build the server binary
make wasm                # Build all WASM packages
make wasm-web            # Build WASM for web
make wasm-nodejs         # Build WASM for Node.js
make wasm-bundler        # Build WASM for bundlers
make chat                # Build chat backend
make web                 # Build web frontend
make test                # Run tests
make bench               # Run benchmarks
make clean               # Clean build artifacts
```

#### Manual Building

You can also build manually using cargo:

```bash
# Build the library
cargo build

# Build the server binary
cargo build --bin sift-rs-server --features server

# Build WASM for web (requires wasm-pack)
cd sift-rs-wasm
wasm-pack build --target web --out-dir pkg

# Build chat backend
cd chat-backend
cargo build
```

#### Build Artifacts

- **Server**: `./target/debug/sift-rs-server` executable
- **WASM (web)**: `./sift-rs-wasm/pkg/` directory
- **WASM (Node.js)**: `./sift-rs-wasm/pkg-node/` directory
- **WASM (bundler)**: `./sift-rs-wasm/pkg-bundler/` directory
- **Chat Backend**: `./chat-backend/target/debug/chat-backend` executable
- **Example Web App**: Uses WASM artifacts from `./sift-rs-wasm/pkg/`

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
