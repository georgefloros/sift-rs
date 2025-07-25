use serde_json::Value;

/// Utility functions for working with serde_json Values
pub fn is_numeric(value: &Value) -> bool {
    matches!(value, Value::Number(_))
}

/// Check if a value is truthy in a MongoDB context
pub fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64().map_or(false, |f| f != 0.0),
        Value::String(s) => !s.is_empty(),
        Value::Array(arr) => !arr.is_empty(),
        Value::Object(obj) => !obj.is_empty(),
    }
}

/// Convert a Value to a string representation for comparison
pub fn value_to_string(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => s.clone(),
        Value::Array(_) => "[Array]".to_string(),
        Value::Object(_) => "[Object]".to_string(),
    }
}

/// Check if a value represents an array-like structure
pub fn is_array_like(value: &Value) -> bool {
    match value {
        Value::Array(_) => true,
        Value::String(_) => true, // Strings can be indexed
        _ => false,
    }
}

/// Get the length of an array-like value
pub fn get_length(value: &Value) -> Option<usize> {
    match value {
        Value::Array(arr) => Some(arr.len()),
        Value::String(s) => Some(s.len()),
        _ => None,
    }
}

/// Deep clone a Value (serde_json::Value already implements Clone, but this is explicit)
pub fn deep_clone(value: &Value) -> Value {
    value.clone()
}

/// Check if two values are of the same type
pub fn same_type(a: &Value, b: &Value) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

/// Merge two JSON objects
pub fn merge_objects(a: &Value, b: &Value) -> Value {
    match (a, b) {
        (Value::Object(obj_a), Value::Object(obj_b)) => {
            let mut result = obj_a.clone();
            for (key, value) in obj_b {
                result.insert(key.clone(), value.clone());
            }
            Value::Object(result)
        }
        _ => b.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_is_truthy() {
        assert!(!is_truthy(&json!(null)));
        assert!(!is_truthy(&json!(false)));
        assert!(is_truthy(&json!(true)));
        assert!(!is_truthy(&json!(0)));
        assert!(is_truthy(&json!(1)));
        assert!(!is_truthy(&json!("")));
        assert!(is_truthy(&json!("hello")));
        assert!(!is_truthy(&json!([])));
        assert!(is_truthy(&json!([1])));
        assert!(!is_truthy(&json!({})));
        assert!(is_truthy(&json!({"a": 1})));
    }

    #[test]
    fn test_get_length() {
        assert_eq!(get_length(&json!([1, 2, 3])), Some(3));
        assert_eq!(get_length(&json!("hello")), Some(5));
        assert_eq!(get_length(&json!(123)), None);
        assert_eq!(get_length(&json!({})), None);
    }

    #[test]
    fn test_same_type() {
        assert!(same_type(&json!(1), &json!(2)));
        assert!(same_type(&json!("a"), &json!("b")));
        assert!(!same_type(&json!(1), &json!("1")));
        assert!(same_type(&json!([]), &json!([1, 2])));
    }
}
