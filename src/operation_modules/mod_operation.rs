use crate::core::{Operation, QueryOperator};
use crate::{SiftError, SiftResult};
use serde_json::Value;

/// $mod operator - tests if number modulo divisor equals remainder
pub struct ModOperator;

impl QueryOperator for ModOperator {
    fn create_operation(&self, params: &Value, _parent_query: &Value) -> SiftResult<Box<dyn Operation>> {
        if let Value::Array(arr) = params {
            if arr.len() == 2 {
                if let (Some(divisor), Some(remainder)) = (arr[0].as_f64(), arr[1].as_f64()) {
                    if divisor == 0.0 {
                        return Err(SiftError::InvalidQuery("$mod divisor cannot be zero".to_string()));
                    }
                    Ok(Box::new(ModOperation { divisor, remainder }))
                } else {
                    Err(SiftError::InvalidQuery("$mod requires numeric divisor and remainder".to_string()))
                }
            } else {
                Err(SiftError::InvalidQuery("$mod requires an array of [divisor, remainder]".to_string()))
            }
        } else {
            Err(SiftError::InvalidQuery("$mod requires an array of [divisor, remainder]".to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "$mod"
    }
}

struct ModOperation {
    divisor: f64,
    remainder: f64,
}

impl Operation for ModOperation {
    fn test(&self, value: &Value, _key: Option<&str>, _parent: Option<&Value>) -> SiftResult<bool> {
        if let Some(num) = value.as_f64() {
            let actual_remainder = num % self.divisor;
            // Use a small epsilon for floating point comparison
            Ok((actual_remainder - self.remainder).abs() < f64::EPSILON)
        } else {
            Ok(false)
        }
    }
}
