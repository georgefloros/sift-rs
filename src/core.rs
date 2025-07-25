use crate::{SiftError, SiftResult};
use serde_json::Value;
use std::collections::HashMap;

/// Represents a query operation that can test values
pub trait Operation {
    fn test(&self, value: &Value, key: Option<&str>, parent: Option<&Value>) -> SiftResult<bool>;
    fn reset(&mut self) {}
}

/// Base trait for all query operations
pub trait QueryOperator: Send + Sync {
    fn create_operation(&self, params: &Value, parent_query: &Value) -> SiftResult<Box<dyn Operation>>;
    fn name(&self) -> &'static str;
}

/// Registry for query operators
pub struct OperatorRegistry {
    operators: HashMap<String, Box<dyn QueryOperator>>,
}

impl Default for OperatorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl OperatorRegistry {
    pub fn new() -> Self {
        let mut registry = OperatorRegistry {
            operators: HashMap::new(),
        };
        
        // Register default operators
        registry.register_default_operators();
        registry
    }

    pub fn register(&mut self, name: String, operator: Box<dyn QueryOperator>) {
        self.operators.insert(name, operator);
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn QueryOperator>> {
        self.operators.get(name)
    }

    fn register_default_operators(&mut self) {
        use crate::operations::*;
        use crate::operation_modules::size_operation::SizeOperator;
        use crate::operation_modules::elem_match_operation::ElemMatchOperator;
        use crate::operation_modules::logic_operations::{AndOperator, OrOperator, NorOperator, NotOperator};
        use crate::operation_modules::exists_operation::ExistsOperator;
        use crate::operation_modules::regex_operation::RegexOperator;
        use crate::operation_modules::mod_operation::ModOperator;
        use crate::operation_modules::where_operation::WhereOperator;
        use crate::operation_modules::type_operation::TypeOperator;
        
        self.register("$eq".to_string(), Box::new(EqOperator));
        self.register("$ne".to_string(), Box::new(NeOperator));
        self.register("$gt".to_string(), Box::new(GtOperator));
        self.register("$gte".to_string(), Box::new(GteOperator));
        self.register("$lt".to_string(), Box::new(LtOperator));
        self.register("$lte".to_string(), Box::new(LteOperator));
        self.register("$in".to_string(), Box::new(InOperator));
        self.register("$nin".to_string(), Box::new(NinOperator));
        self.register("$all".to_string(), Box::new(AllOperator));
        self.register("$exists".to_string(), Box::new(ExistsOperator));
        self.register("$regex".to_string(), Box::new(RegexOperator));
        self.register("$and".to_string(), Box::new(AndOperator));
        self.register("$or".to_string(), Box::new(OrOperator));
        self.register("$not".to_string(), Box::new(NotOperator));
        self.register("$size".to_string(), Box::new(SizeOperator));
        self.register("$mod".to_string(), Box::new(ModOperator));
        self.register("$type".to_string(), Box::new(TypeOperator));
        self.register("$elemMatch".to_string(), Box::new(ElemMatchOperator));
        self.register("$nor".to_string(), Box::new(NorOperator));
        self.register("$where".to_string(), Box::new(WhereOperator));
    }
}

/// Configuration options for query evaluation
#[derive(Clone, Debug)]
pub struct QueryOptions {
    pub case_sensitive: bool,
    pub strict_arrays: bool,
}

impl Default for QueryOptions {
    fn default() -> Self {
        QueryOptions {
            case_sensitive: true,
            strict_arrays: false,
        }
    }
}

/// The main query evaluation context
pub struct QueryContext {
    pub registry: OperatorRegistry,
    pub options: QueryOptions,
}

impl Default for QueryContext {
    fn default() -> Self {
        QueryContext {
            registry: OperatorRegistry::new(),
            options: QueryOptions::default(),
        }
    }
}

impl QueryContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_options(options: QueryOptions) -> Self {
        QueryContext {
            registry: OperatorRegistry::new(),
            options,
        }
    }
}

/// Represents a compiled query that can be executed against values
pub struct CompiledQuery {
    operations: Vec<Box<dyn Operation>>,
    context: QueryContext,
}

impl CompiledQuery {
    pub fn new(operations: Vec<Box<dyn Operation>>, context: QueryContext) -> Self {
        CompiledQuery { operations, context }
    }

    pub fn test(&self, value: &Value) -> SiftResult<bool> {
        if self.operations.is_empty() {
            return Ok(true);
        }

        // For multiple operations at the root level, they're implicitly AND-ed
        for operation in &self.operations {
            if !operation.test(value, None, None)? {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
}

/// Utility functions for value comparison and manipulation
pub mod utils {
    use super::*;
    use regex::Regex;

    /// Compare two values for equality, handling different JSON types appropriately
    pub fn values_equal(a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Null, Value::Null) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => {
                // Handle both integer and float comparisons
                match (a.as_i64(), b.as_i64()) {
                    (Some(a), Some(b)) => a == b,
                    _ => match (a.as_f64(), b.as_f64()) {
                        (Some(a), Some(b)) => (a - b).abs() < f64::EPSILON,
                        _ => false,
                    }
                }
            }
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Array(a), Value::Array(b)) => {
                a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| values_equal(x, y))
            }
            (Value::Object(a), Value::Object(b)) => {
                a.len() == b.len() && 
                a.iter().all(|(key, value)| {
                    b.get(key).map_or(false, |other_value| values_equal(value, other_value))
                })
            }
            _ => false,
        }
    }

    /// Compare two values numerically, returns Some(ordering) if both are numbers
    pub fn compare_numbers(a: &Value, b: &Value) -> Option<std::cmp::Ordering> {
        match (a, b) {
            (Value::Number(a), Value::Number(b)) => {
                match (a.as_f64(), b.as_f64()) {
                    (Some(a), Some(b)) => Some(a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Compare two values, supporting both numbers and ISO8601 date strings
    pub fn compare_values(a: &Value, b: &Value) -> Option<std::cmp::Ordering> {
        use chrono::{DateTime, Utc};
        
        // First try numeric comparison
        if let Some(ordering) = compare_numbers(a, b) {
            return Some(ordering);
        }
        
        // Try date comparison if both are strings
        match (a, b) {
            (Value::String(a_str), Value::String(b_str)) => {
                // Try to parse both as ISO8601 dates
                match (a_str.parse::<DateTime<Utc>>(), b_str.parse::<DateTime<Utc>>()) {
                    (Ok(a_date), Ok(b_date)) => Some(a_date.cmp(&b_date)),
                    _ => {
                        // Fall back to string comparison if not dates
                        Some(a_str.cmp(b_str))
                    }
                }
            }
            _ => None,
        }
    }

    /// Get a nested value from an object using dot notation
    pub fn get_nested_value<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = value;

        for part in parts {
            match current {
                Value::Object(obj) => {
                    current = obj.get(part)?;
                }
                Value::Array(arr) => {
                    // Handle array indexing
                    if let Ok(index) = part.parse::<usize>() {
                        current = arr.get(index)?;
                    } else {
                        // Look for the field in each array element
                        return None;
                    }
                }
                _ => return None,
            }
        }

        Some(current)
    }

    /// Walk through array elements and nested paths
    pub fn walk_array_values<F>(value: &Value, path: &str, mut callback: F) -> bool 
    where
        F: FnMut(&Value) -> bool,
    {
        if path.is_empty() {
            return callback(value);
        }

        let parts: Vec<&str> = path.split('.').collect();
        walk_value_recursive(value, &parts, 0, &mut callback)
    }

    fn walk_value_recursive<F>(value: &Value, parts: &[&str], depth: usize, callback: &mut F) -> bool
    where
        F: FnMut(&Value) -> bool,
    {
        if depth >= parts.len() {
            return callback(value);
        }

        let current_part = parts[depth];

        match value {
            Value::Array(arr) => {
                // For arrays, try both direct indexing and field access on elements
                if let Ok(index) = current_part.parse::<usize>() {
                    if let Some(element) = arr.get(index) {
                        if walk_value_recursive(element, parts, depth + 1, callback) {
                            return true;
                        }
                    }
                } else {
                    // Look for the field in each array element
                    for element in arr {
                        if let Value::Object(obj) = element {
                            if let Some(field_value) = obj.get(current_part) {
                                if walk_value_recursive(field_value, parts, depth + 1, callback) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
            Value::Object(obj) => {
                if let Some(field_value) = obj.get(current_part) {
                    if walk_value_recursive(field_value, parts, depth + 1, callback) {
                        return true;
                    }
                }
            }
            _ => return false,
        }

        false
    }

    /// Test if a value matches a regex pattern
    pub fn test_regex(value: &Value, pattern: &str, options: Option<&str>) -> SiftResult<bool> {
        if let Value::String(s) = value {
            let mut regex_pattern = pattern.to_string();
            
            // Handle regex options
            if let Some(opts) = options {
                if opts.contains('i') {
                    regex_pattern = format!("(?i){}", regex_pattern);
                }
                if opts.contains('m') {
                    regex_pattern = format!("(?m){}", regex_pattern);
                }
                if opts.contains('s') {
                    regex_pattern = format!("(?s){}", regex_pattern);
                }
            }

            let regex = Regex::new(&regex_pattern)
                .map_err(|e| SiftError::InvalidQuery(format!("Invalid regex: {}", e)))?;
            
            Ok(regex.is_match(s))
        } else {
            Ok(false)
        }
    }

    /// Get the type of a JSON value as a string
    pub fn get_value_type(value: &Value) -> &'static str {
        match value {
            Value::Null => "null",
            Value::Bool(_) => "boolean",
            Value::Number(n) => {
                if n.is_i64() || n.is_u64() {
                    "number"
                } else {
                    "number"
                }
            }
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_values_equal() {
        assert!(utils::values_equal(&json!(null), &json!(null)));
        assert!(utils::values_equal(&json!(true), &json!(true)));
        assert!(utils::values_equal(&json!(42), &json!(42)));
        assert!(utils::values_equal(&json!("hello"), &json!("hello")));
        assert!(utils::values_equal(&json!([1, 2, 3]), &json!([1, 2, 3])));
        assert!(utils::values_equal(&json!({"a": 1}), &json!({"a": 1})));
        
        assert!(!utils::values_equal(&json!(true), &json!(false)));
        assert!(!utils::values_equal(&json!(42), &json!(43)));
    }

    #[test]
    fn test_get_nested_value() {
        let value = json!({
            "user": {
                "name": "Alice",
                "profile": {
                    "age": 30
                }
            }
        });

        assert_eq!(utils::get_nested_value(&value, "user.name"), Some(&json!("Alice")));
        assert_eq!(utils::get_nested_value(&value, "user.profile.age"), Some(&json!(30)));
        assert_eq!(utils::get_nested_value(&value, "user.invalid"), None);
    }

    #[test]
    fn test_compare_numbers() {
        use std::cmp::Ordering;
        
        assert_eq!(utils::compare_numbers(&json!(1), &json!(2)), Some(Ordering::Less));
        assert_eq!(utils::compare_numbers(&json!(2), &json!(1)), Some(Ordering::Greater)); 
        assert_eq!(utils::compare_numbers(&json!(1), &json!(1)), Some(Ordering::Equal));
        assert_eq!(utils::compare_numbers(&json!(1), &json!("1")), None);
    }
}
