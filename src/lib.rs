//! # Sift-rs
//! 
//! A Rust implementation of MongoDB query filtering, inspired by the JavaScript sift.js library.
//! This crate provides powerful query capabilities for filtering data structures using MongoDB-style syntax.
//! 
//! ## Features
//! 
//! - Supports most MongoDB query operators: `$eq`, `$ne`, `$gt`, `$gte`, `$lt`, `$lte`, `$in`, `$nin`, `$exists`, `$regex`, `$and`, `$or`, `$not`, `$all`, `$size`, `$mod`, `$type`, `$elemMatch`
//! - Works with any data structure that implements `serde::Serialize`
//! - Type-safe query construction
//! - Performance optimized for Rust
//! 
//! ## Basic Usage
//! 
//! ```rust
//! use sift_rs::{sift, Query};
//! use serde_json::json;
//! 
//! let data = vec![
//!     json!({"name": "Alice", "age": 30}),
//!     json!({"name": "Bob", "age": 25}),
//!     json!({"name": "Charlie", "age": 35}),
//! ];
//! 
//! let query = json!({"age": {"$gte": 30}});
//! let results: Vec<_> = data.into_iter()
//!     .filter(|item| sift(&query, item).unwrap())
//!     .collect();
//! 
//! assert_eq!(results.len(), 2);
//! ```

pub mod core;
pub mod operations;
pub mod query;
pub mod utils;

// Import modular operations
pub mod operation_modules {
    pub mod size_operation;
    pub mod elem_match_operation;
    pub mod logic_operations;
    pub mod exists_operation;
    pub mod regex_operation;
    pub mod mod_operation;
    pub mod where_operation;
    pub mod type_operation;
}

// Re-export all operators
pub use operation_modules::size_operation::SizeOperator;
pub use operation_modules::elem_match_operation::ElemMatchOperator;
pub use operation_modules::logic_operations::{AndOperator, OrOperator, NorOperator, NotOperator};
pub use operation_modules::exists_operation::ExistsOperator;
pub use operation_modules::regex_operation::RegexOperator;
pub use operation_modules::mod_operation::ModOperator;
pub use operation_modules::where_operation::WhereOperator;
pub use operation_modules::type_operation::TypeOperator;

pub use core::*;
pub use query::*;

use serde_json::Value;
use std::error::Error;
use std::fmt;

/// Main error type for sift operations
#[derive(Debug, Clone)]
pub enum SiftError {
    InvalidQuery(String),
    InvalidValue(String),
    UnsupportedOperation(String),
    SerializationError(String),
    EvaluationError(String),
}

impl fmt::Display for SiftError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SiftError::InvalidQuery(msg) => write!(f, "Invalid query: {}", msg),
            SiftError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
            SiftError::UnsupportedOperation(msg) => write!(f, "Unsupported operation: {}", msg),
            SiftError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            SiftError::EvaluationError(msg) => write!(f, "Evaluation error: {}", msg),
        }
    }
}

impl Error for SiftError {}

/// Result type for sift operations
pub type SiftResult<T> = Result<T, SiftError>;

/// Main sift function - tests if a value matches a query
/// 
/// # Arguments
/// 
/// * `query` - The MongoDB-style query to apply
/// * `value` - The value to test against the query
/// 
/// # Returns
/// 
/// Returns `Ok(true)` if the value matches the query, `Ok(false)` if it doesn't,
/// or an `Err` if there's a problem with the query or value.
/// 
/// # Examples
/// 
/// ```rust
/// use sift_rs::sift;
/// use serde_json::json;
/// 
/// let value = json!({"name": "Alice", "age": 30});
/// let query = json!({"age": {"$gt": 25}});
/// 
/// assert!(sift(&query, &value).unwrap());
/// ```
pub fn sift(query: &Value, value: &Value) -> SiftResult<bool> {
    let query_obj = Query::from_value(query)?;
    query_obj.test(value)
}

/// Creates a closure that can be used to filter iterators
/// 
/// # Arguments
/// 
/// * `query` - The MongoDB-style query to apply
/// 
/// # Returns
/// 
/// Returns a closure that takes a value and returns whether it matches the query
/// 
/// # Examples
/// 
/// ```rust
/// use sift_rs::create_filter;
/// use serde_json::json;
/// 
/// let data = vec![
///     json!({"name": "Alice", "age": 30}),
///     json!({"name": "Bob", "age": 25}),
/// ];
/// 
/// let query = json!({"age": {"$gte": 30}});
/// let filter = create_filter(&query).unwrap();
/// 
/// let results: Vec<_> = data.into_iter().filter(filter).collect();
/// assert_eq!(results.len(), 1);
/// ```
pub fn create_filter(query: &Value) -> SiftResult<impl Fn(&Value) -> bool> {
    let query_obj = Query::from_value(query)?;
    
    Ok(move |value: &Value| -> bool {
        query_obj.test(value).unwrap_or(false)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_basic_equality() {
        let value = json!({"name": "Alice", "age": 30});
        let query = json!({"name": "Alice"});
        
        assert!(sift(&query, &value).unwrap());
    }

    #[test]
    fn test_comparison_operators() {
        let value = json!({"age": 30});
        
        assert!(sift(&json!({"age": {"$gt": 25}}), &value).unwrap());
        assert!(!sift(&json!({"age": {"$gt": 35}}), &value).unwrap());
        assert!(sift(&json!({"age": {"$gte": 30}}), &value).unwrap());
        assert!(sift(&json!({"age": {"$lt": 35}}), &value).unwrap());
        assert!(sift(&json!({"age": {"$lte": 30}}), &value).unwrap());
    }

    #[test]
    fn test_in_operator() {
        let value = json!({"category": "fruits"});
        let query = json!({"category": {"$in": ["fruits", "vegetables"]}});
        
        assert!(sift(&query, &value).unwrap());
    }

    #[test]
    fn test_array_values() {
        let data = vec![
            json!({"name": "Alice", "age": 30}),
            json!({"name": "Bob", "age": 25}),
            json!({"name": "Charlie", "age": 35}),
        ];

        let query = json!({"age": {"$gte": 30}});
        let results: Vec<_> = data.into_iter()
            .filter(|item| sift(&query, item).unwrap())
            .collect();

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_create_filter() {
        let data = vec![
            json!({"status": "active"}),
            json!({"status": "inactive"}),
            json!({"status": "active"}),
        ];

        let query = json!({"status": "active"});
        let filter = create_filter(&query).unwrap();
        
        let results: Vec<_> = data.into_iter().filter(filter).collect();
        assert_eq!(results.len(), 2);
    }
}
