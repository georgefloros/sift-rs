use crate::core::{Operation, QueryOperator};
use crate::{SiftError, SiftResult};
use serde_json::Value;

/// $size operator - tests array length
pub struct SizeOperator;

impl QueryOperator for SizeOperator {
    fn create_operation(&self, params: &Value, _parent_query: &Value) -> SiftResult<Box<dyn Operation>> {
        if let Some(size) = params.as_u64() {
            Ok(Box::new(SizeOperation { expected_size: size as usize }))
        } else {
            Err(SiftError::InvalidQuery("$size requires a number".to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "$size"
    }
}

struct SizeOperation {
    expected_size: usize,
}

impl Operation for SizeOperation {
    fn test(&self, value: &Value, _key: Option<&str>, _parent: Option<&Value>) -> SiftResult<bool> {
        if let Value::Array(array) = value {
            Ok(array.len() == self.expected_size)
        } else if let Value::String(string) = value {
            Ok(string.len() == self.expected_size)
        } else {
            Ok(false)
        }
    }
}
