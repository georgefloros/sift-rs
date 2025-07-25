use crate::core::{Operation, QueryOperator};
use crate::{SiftError, SiftResult};
use serde_json::Value;

/// $exists operator - tests if field exists
pub struct ExistsOperator;

impl QueryOperator for ExistsOperator {
    fn create_operation(&self, params: &Value, _parent_query: &Value) -> SiftResult<Box<dyn Operation>> {
        if let Some(should_exist) = params.as_bool() {
            Ok(Box::new(ExistsOperation { should_exist }))
        } else {
            Err(SiftError::InvalidQuery("$exists requires a boolean value".to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "$exists"
    }
}

struct ExistsOperation {
    should_exist: bool,
}

impl Operation for ExistsOperation {
    fn test(&self, value: &Value, key: Option<&str>, parent: Option<&Value>) -> SiftResult<bool> {
        // Check if the field actually exists in the parent object
        let exists = if let (Some(field_name), Some(parent_obj)) = (key, parent) {
            match parent_obj {
                Value::Object(obj) => obj.contains_key(field_name),
                _ => false,
            }
        } else {
            // If no parent context, just check if value is not null
            !value.is_null()
        };
        
        Ok(exists == self.should_exist)
    }
}
