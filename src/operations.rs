use crate::core::{Operation, QueryOperator, utils};
use crate::{SiftError, SiftResult};
use serde_json::Value;
use std::cmp::Ordering;

// Import operations from modules - these will be added separately to lib.rs

// Basic operations for MongoDB query operators

/// $eq operator - tests for equality
pub struct EqOperator;

impl QueryOperator for EqOperator {
    fn create_operation(&self, params: &Value, _parent_query: &Value) -> SiftResult<Box<dyn Operation>> {
        Ok(Box::new(EqOperation { expected: params.clone() }))
    }
    
    fn name(&self) -> &'static str {
        "$eq"
    }
}

struct EqOperation {
    expected: Value,
}

impl Operation for EqOperation {
    fn test(&self, value: &Value, _key: Option<&str>, _parent: Option<&Value>) -> SiftResult<bool> {
        Ok(utils::values_equal(value, &self.expected))
    }
}

/// $ne operator - tests for inequality
pub struct NeOperator;

impl QueryOperator for NeOperator {
    fn create_operation(&self, params: &Value, _parent_query: &Value) -> SiftResult<Box<dyn Operation>> {
        Ok(Box::new(NeOperation { expected: params.clone() }))
    }
    
    fn name(&self) -> &'static str {
        "$ne"
    }
}

struct NeOperation {
    expected: Value,
}

impl Operation for NeOperation {
    fn test(&self, value: &Value, _key: Option<&str>, _parent: Option<&Value>) -> SiftResult<bool> {
        Ok(!utils::values_equal(value, &self.expected))
    }
}

/// $gt operator - greater than
pub struct GtOperator;

impl QueryOperator for GtOperator {
    fn create_operation(&self, params: &Value, _parent_query: &Value) -> SiftResult<Box<dyn Operation>> {
        Ok(Box::new(GtOperation { threshold: params.clone() }))
    }
    
    fn name(&self) -> &'static str {
        "$gt"
    }
}

struct GtOperation {
    threshold: Value,
}

impl Operation for GtOperation {
    fn test(&self, value: &Value, _key: Option<&str>, _parent: Option<&Value>) -> SiftResult<bool> {
        Ok(crate::core::utils::compare_values(value, &self.threshold)
            .map(|ord| ord == std::cmp::Ordering::Greater)
            .unwrap_or(false))
    }
}


/// $gte operator - greater than or equal
pub struct GteOperator;

impl QueryOperator for GteOperator {
    fn create_operation(&self, params: &Value, _parent_query: &Value) -> SiftResult<Box<dyn Operation>> {
        Ok(Box::new(GteOperation { threshold: params.clone() }))
    }
    
    fn name(&self) -> &'static str {
        "$gte"
    }
}

struct GteOperation {
    threshold: Value,
}

impl Operation for GteOperation {
    fn test(&self, value: &Value, _key: Option<&str>, _parent: Option<&Value>) -> SiftResult<bool> {
        match utils::compare_values(value, &self.threshold) {
            Some(Ordering::Greater) | Some(Ordering::Equal) => Ok(true),
            _ => Ok(false),
        }
    }
}

/// $lt operator - less than
pub struct LtOperator;

impl QueryOperator for LtOperator {
    fn create_operation(&self, params: &Value, _parent_query: &Value) -> SiftResult<Box<dyn Operation>> {
        Ok(Box::new(LtOperation { threshold: params.clone() }))
    }
    
    fn name(&self) -> &'static str {
        "$lt"
    }
}

struct LtOperation {
    threshold: Value,
}

impl Operation for LtOperation {
    fn test(&self, value: &Value, _key: Option<&str>, _parent: Option<&Value>) -> SiftResult<bool> {
        match utils::compare_values(value, &self.threshold) {
            Some(Ordering::Less) => Ok(true),
            _ => Ok(false),
        }
    }
}

/// $lte operator - less than or equal
pub struct LteOperator;

impl QueryOperator for LteOperator {
    fn create_operation(&self, params: &Value, _parent_query: &Value) -> SiftResult<Box<dyn Operation>> {
        Ok(Box::new(LteOperation { threshold: params.clone() }))
    }
    
    fn name(&self) -> &'static str {
        "$lte"
    }
}

struct LteOperation {
    threshold: Value,
}

impl Operation for LteOperation {
    fn test(&self, value: &Value, _key: Option<&str>, _parent: Option<&Value>) -> SiftResult<bool> {
        match utils::compare_values(value, &self.threshold) {
            Some(Ordering::Less) | Some(Ordering::Equal) => Ok(true),
            _ => Ok(false),
        }
    }
}

/// $in operator - tests if value is in the given array
pub struct InOperator;

impl QueryOperator for InOperator {
    fn create_operation(&self, params: &Value, _parent_query: &Value) -> SiftResult<Box<dyn Operation>> {
        if let Value::Array(values) = params {
            Ok(Box::new(InOperation { values: values.clone() }))
        } else {
            Err(SiftError::InvalidQuery("$in requires an array".to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "$in"
    }
}

struct InOperation {
    values: Vec<Value>,
}

impl Operation for InOperation {
    fn test(&self, value: &Value, _key: Option<&str>, _parent: Option<&Value>) -> SiftResult<bool> {
        // If the field value is an array, check if any element of the array is in the expected values
        if let Value::Array(array) = value {
            for item in array {
                for expected in &self.values {
                    if utils::values_equal(item, expected) {
                        return Ok(true);
                    }
                }
            }
            return Ok(false);
        }
        
        // For non-array values, check if the value itself is in the expected values
        for expected in &self.values {
            if utils::values_equal(value, expected) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

/// $nin operator - tests if value is not in the given array
pub struct NinOperator;

impl QueryOperator for NinOperator {
    fn create_operation(&self, params: &Value, _parent_query: &Value) -> SiftResult<Box<dyn Operation>> {
        if let Value::Array(values) = params {
            Ok(Box::new(NinOperation { values: values.clone() }))
        } else {
            Err(SiftError::InvalidQuery("$nin requires an array".to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "$nin"
    }
}

struct NinOperation {
    values: Vec<Value>,
}

impl Operation for NinOperation {
    fn test(&self, value: &Value, _key: Option<&str>, _parent: Option<&Value>) -> SiftResult<bool> {
        // If the field value is an array, check if any element of the array is in the excluded values
        if let Value::Array(array) = value {
            for item in array {
                for expected in &self.values {
                    if utils::values_equal(item, expected) {
                        return Ok(false); // Found a match, so $nin fails
                    }
                }
            }
            return Ok(true); // No matches found, so $nin succeeds
        }
        
        // For non-array values, check if the value itself is in the excluded values
        for expected in &self.values {
            if utils::values_equal(value, expected) {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

/// $all operator - tests if array contains all specified values
pub struct AllOperator;

impl QueryOperator for AllOperator {
    fn create_operation(&self, params: &Value, _parent_query: &Value) -> SiftResult<Box<dyn Operation>> {
        if let Value::Array(values) = params {
            Ok(Box::new(AllOperation { required_values: values.clone() }))
        } else {
            Err(SiftError::InvalidQuery("$all requires an array".to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "$all"
    }
}

struct AllOperation {
    required_values: Vec<Value>,
}

impl Operation for AllOperation {
    fn test(&self, value: &Value, _key: Option<&str>, _parent: Option<&Value>) -> SiftResult<bool> {
        if let Value::Array(array) = value {
            for required in &self.required_values {
                let mut found = false;
                for item in array {
                    if utils::values_equal(item, required) {
                        found = true;
                        break;
                    }
                }
                if !found {
                    return Ok(false);
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
