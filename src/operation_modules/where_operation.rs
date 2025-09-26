use crate::core::{Operation, QueryOperator};
use crate::{SiftError, SiftResult};
use serde_json::Value;

/// $where operator - evaluates JavaScript-like expressions
pub struct WhereOperator;

impl QueryOperator for WhereOperator {
    fn create_operation(&self, params: &Value, _parent_query: &Value) -> SiftResult<Box<dyn Operation>> {
        if let Some(expression) = params.as_str() {
            Ok(Box::new(WhereOperation { expression: expression.to_string() }))
        } else {
            Err(SiftError::InvalidQuery("$where requires a JavaScript expression string".to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "$where"
    }
}

struct WhereOperation {
    expression: String,
}

impl Operation for WhereOperation {
    fn test(&self, value: &Value, _key: Option<&str>, _parent: Option<&Value>) -> SiftResult<bool> {
        // Use Boa to evaluate the JavaScript expression
        self.evaluate_expression(&self.expression, value)
    }
}

impl WhereOperation {
    fn evaluate_expression(&self, expr: &str, value: &Value) -> SiftResult<bool> {
        use boa_engine::{Context, JsValue, Source};
        
        // Create a JavaScript context
        let mut context = Context::default();
        
        // Convert the JSON value to a JavaScript object string
        let js_object_str = serde_json::to_string(value)
            .map_err(|e| SiftError::EvaluationError(format!("Failed to serialize JSON: {}", e)))?;
        
        // Create a script that sets 'this' to our JSON object and evaluates the expression
        let script_code = format!(
            "const thisObj = {}; (function() {{ return {}; }}).call(thisObj);",
            js_object_str, expr
        );

        // Evaluate the script
        let result = context.eval(Source::from_bytes(&script_code))
            .map_err(|e| SiftError::EvaluationError(format!("JavaScript execution error: {:?}", e)))?;

        // Convert the result to a boolean
        match result {
            JsValue::Boolean(b) => Ok(b),
            JsValue::Integer(n) => Ok(n != 0),
            JsValue::Rational(n) => Ok(n != 0.0),
            JsValue::String(s) => {
                let s_str = s.as_str();
                Ok(!s_str.is_empty())
            },
            JsValue::Null | JsValue::Undefined => Ok(false),
            JsValue::Object(_) => Ok(true), // Objects are truthy in JavaScript
            _ => Ok(true), // Other types are generally truthy
        }
    }
}
