# Sift-rs API Server

A REST API server built with Axum that provides MongoDB-style query validation using the sift-rs library.

## Quick Start

### 1. Start the Server

```bash
cargo run
```

The server will start on `http://localhost:3000`

### 2. API Endpoints

#### Health Check
```
GET /health
```

#### Validate Data
```
POST /validate
Content-Type: application/json

[
  {
    "input": { /* your data object */ },
    "query": { /* MongoDB-style query */ }
  }
]
```

Response:
```json
[
  { "valid": true },
  { "valid": false }
]
```

#### Get Sample Data
```
GET /sample
```

## Usage Examples

### Basic Validation

```bash
curl -X POST http://localhost:3000/validate \
  -H "Content-Type: application/json" \
  -d '[
    {
      "input": {"name": "Alice", "age": 30},
      "query": {"age": {"$gte": 25}}
    },
    {
      "input": {"name": "Bob", "age": 20},
      "query": {"age": {"$gte": 25}}
    }
  ]'
```

Response:
```json
[
  {"valid": true},
  {"valid": false}
]
```

### Complex Queries

```bash
curl -X POST http://localhost:3000/validate \
  -H "Content-Type: application/json" \
  -d '[
    {
      "input": {
        "user": {
          "age": 28,
          "status": "active",
          "tags": ["developer", "rust"]
        }
      },
      "query": {
        "$and": [
          {"user.age": {"$gte": 25}},
          {"user.status": "active"},
          {"user.tags": {"$in": ["developer", "manager"]}}
        ]
      }
    }
  ]'
```

## Supported MongoDB Operators

### Comparison Operators
- `$eq` - Equal to
- `$ne` - Not equal to
- `$gt` - Greater than
- `$gte` - Greater than or equal to
- `$lt` - Less than
- `$lte` - Less than or equal to

### Array Operators
- `$in` - Value in array
- `$nin` - Value not in array
- `$all` - Array contains all elements
- `$size` - Array size equals

### Existence & Type
- `$exists` - Field exists
- `$type` - Field type check

### Regular Expressions
- `$regex` - Regular expression match
- `$options` - Regex options (e.g., "i" for case-insensitive)

### Mathematical
- `$mod` - Modulo operation

### Logical Operators
- `$and` - All conditions must be true
- `$or` - At least one condition must be true
- `$not` - Condition must not be true
- `$nor` - None of the conditions must be true

### Array Element Matching
- `$elemMatch` - Array contains element matching criteria

### JavaScript Expressions
- `$where` - JavaScript-like expressions using `this` context

## Running Examples

### Comprehensive Examples
Run the comprehensive test suite that covers all operators:

```bash
# Start the server first
cargo run

# In another terminal, run the examples
cargo run --example comprehensive_examples
```

This will test:
- ✅ All basic comparison operators
- ✅ Array operators
- ✅ Existence and type checking
- ✅ Regular expressions
- ✅ Mathematical operations
- ✅ Date comparisons
- ✅ Logical operators
- ✅ Element matching for arrays of objects
- ✅ Medium complexity nested objects
- ✅ High complexity business data structures
- ✅ Complex $where expressions

### Basic Examples
```bash
cargo run --example basic_usage
```

## Example Query Types

### 1. Simple Comparison
```json
{
  "input": {"age": 30},
  "query": {"age": {"$gte": 25}}
}
```

### 2. Date Ranges
```json
{
  "input": {"created_at": "2024-03-15T10:30:00Z"},
  "query": {"created_at": {"$gte": "2024-01-01T00:00:00Z"}}
}
```

### 3. Array Operations
```json
{
  "input": {"tags": ["developer", "rust", "backend"]},
  "query": {"tags": {"$all": ["developer", "rust"]}}
}
```

### 4. Nested Object Queries
```json
{
  "input": {
    "user": {
      "profile": {"age": 28},
      "settings": {"notifications": true}
    }
  },
  "query": {
    "$and": [
      {"user.profile.age": {"$gte": 25}},
      {"user.settings.notifications": true}
    ]
  }
}
```

### 5. Complex Element Matching
```json
{
  "input": {
    "orders": [
      {"amount": 100, "status": "completed"},
      {"amount": 250, "status": "pending"}
    ]
  },
  "query": {
    "orders": {
      "$elemMatch": {
        "$and": [
          {"amount": {"$gte": 200}},
          {"status": "pending"}
        ]
      }
    }
  }
}
```

### 6. JavaScript-like Expressions
```json
{
  "input": {"a": 10, "b": 5, "sum": 15},
  "query": {"$where": "this.a + this.b === this.sum"}
}
```

### 7. Regular Expressions
```json
{
  "input": {"email": "user@example.com"},
  "query": {
    "email": {
      "$regex": "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
    }
  }
}
```

## Error Handling

The API returns appropriate HTTP status codes:
- `200 OK` - Successful validation
- `400 Bad Request` - Invalid JSON or query syntax
- `500 Internal Server Error` - Server error

Error response format:
```json
{
  "error": "ValidationFailed",
  "message": "Failed to validate item 0: Invalid query syntax"
}
```

## Performance Notes

- The API processes validation requests in sequence
- Each input/query pair is validated independently
- Complex queries with deep nesting may take longer to process
- The server supports CORS for browser-based applications

## Development

### Building
```bash
cargo build
```

### Testing
```bash
cargo test
```

### Running with Logs
```bash
RUST_LOG=info cargo run
```

This will show detailed logs of all validation requests and results.
