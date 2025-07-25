use crate::core::{Operation, QueryOperator};
use crate::{SiftError, SiftResult};
use serde_json::Value;

/// $regex operator - tests if string matches regular expression
pub struct RegexOperator;

impl QueryOperator for RegexOperator {
    fn create_operation(&self, params: &Value, _parent_query: &Value) -> SiftResult<Box<dyn Operation>> {
        if let Some(pattern) = params.as_str() {
            match regex::Regex::new(pattern) {
                Ok(regex) => Ok(Box::new(RegexOperation { regex })),
                Err(e) => Err(SiftError::InvalidQuery(format!("Invalid regex pattern: {}", e))),
            }
        } else {
            Err(SiftError::InvalidQuery("$regex requires a string pattern".to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "$regex"
    }
}

struct RegexOperation {
    regex: regex::Regex,
}

impl Operation for RegexOperation {
    fn test(&self, value: &Value, _key: Option<&str>, _parent: Option<&Value>) -> SiftResult<bool> {
        if let Some(string_value) = value.as_str() {
            Ok(self.regex.is_match(string_value))
        } else {
            Ok(false)
        }
    }
}
