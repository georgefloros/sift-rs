use crate::core::{Operation, QueryOperator};
use crate::{SiftError, SiftResult};
use serde_json::Value;

/// $type operator - tests the BSON type of the value
pub struct TypeOperator;

impl QueryOperator for TypeOperator {
    fn create_operation(&self, params: &Value, _parent_query: &Value) -> SiftResult<Box<dyn Operation>> {
        if let Some(type_name) = params.as_str() {
            Ok(Box::new(TypeOperation { expected_type: type_name.to_string() }))
        } else if let Some(type_number) = params.as_u64() {
            // MongoDB also supports BSON type numbers
            let type_name = match type_number {
                1 => "double",
                2 => "string", 
                3 => "object",
                4 => "array",
                8 => "bool",
                10 => "null",
                16 => "int",
                18 => "long",
                _ => return Err(SiftError::InvalidQuery(format!("Unknown BSON type number: {}", type_number))),
            };
            Ok(Box::new(TypeOperation { expected_type: type_name.to_string() }))
        } else {
            Err(SiftError::InvalidQuery("$type requires a string type name or numeric BSON type".to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "$type"
    }
}

struct TypeOperation {
    expected_type: String,
}

impl Operation for TypeOperation {
    fn test(&self, value: &Value, _key: Option<&str>, _parent: Option<&Value>) -> SiftResult<bool> {
        let actual_type = match value {
            Value::Null => "null",
            Value::Bool(_) => {
                if self.expected_type == "bool" || self.expected_type == "boolean" {
                    return Ok(true);
                }
                "bool"
            },
            Value::Number(n) => {
                // Handle different number type expectations
                match self.expected_type.as_str() {
                    "number" => return Ok(true), // Generic number type
                    "double" => {
                        if n.is_f64() { return Ok(true); }
                        "double"
                    },
                    "int" | "integer" => {
                        if n.is_i64() || n.is_u64() { return Ok(true); }
                        "int"
                    },
                    "long" => {
                        if n.is_i64() { return Ok(true); }
                        "long"
                    },
                    _ => "number"
                }
            },
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
        };
        
        Ok(actual_type == self.expected_type)
    }
}
