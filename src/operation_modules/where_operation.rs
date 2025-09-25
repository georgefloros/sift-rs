// Conditional compilation for using Boa (WASM compatible) vs rustyscript
#[cfg(feature = "server")]
use crate::core::{Operation, QueryOperator};
#[cfg(feature = "server")]
use crate::{SiftError, SiftResult};
#[cfg(feature = "server")]
use serde_json::Value;
#[cfg(feature = "server")]
use rustyscript::{Runtime, RuntimeOptions};
#[cfg(feature = "server")]
use std::sync::mpsc;
#[cfg(feature = "server")]
use std::thread;

// Conditional compilation for WASM-enabled where using Boa
#[cfg(feature = "wasm")]
use crate::core::{Operation, QueryOperator};
#[cfg(feature = "wasm")]
use crate::{SiftError, SiftResult};
#[cfg(feature = "wasm")]
use serde_json::Value;

/// $where operator - evaluates JavaScript-like expressions
#[cfg(any(feature = "server", feature = "wasm"))]
pub struct WhereOperator;

// Server implementation using rustyscript
#[cfg(feature = "server")]
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

// WASM implementation using Boa
#[cfg(feature = "wasm")]
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

// Server implementation with rustyscript
#[cfg(feature = "server")]
struct WhereOperation {
    expression: String,
}

#[cfg(feature = "server")]
impl Operation for WhereOperation {
    fn test(&self, value: &Value, _key: Option<&str>, _parent: Option<&Value>) -> SiftResult<bool> {
        // Use RustyScript to evaluate the JavaScript expression
        self.evaluate_expression(&self.expression, value)
    }
}

#[cfg(feature = "server")]
impl WhereOperation {
    fn evaluate_expression(&self, expr: &str, value: &Value) -> SiftResult<bool> {
        let expr = expr.to_string();
        let value = value.clone();
        
        // Run the JavaScript evaluation in a separate thread to avoid async context issues
        let (tx, rx) = mpsc::channel();
        
        thread::spawn(move || {
            let result = Self::evaluate_js_in_thread(&expr, &value);
            let _ = tx.send(result);
        });
        
        rx.recv().map_err(|_| SiftError::EvaluationError("Thread communication failed".to_string()))?
    }
    
    fn evaluate_js_in_thread(expr: &str, value: &Value) -> SiftResult<bool> {
        // Create a new runtime for each evaluation
        let mut runtime = Runtime::new(RuntimeOptions::default())
            .map_err(|e| SiftError::EvaluationError(format!("Failed to initialize RustyScript: {}", e)))?;

        // Convert the JSON value to a JavaScript object string
        let js_object = serde_json::to_string(value)
            .map_err(|e| SiftError::EvaluationError(format!("Failed to serialize JSON: {}", e)))?;
        
        // Create a script that sets 'this' to our JSON object and evaluates the expression
        let script_code = format!(
            "const thisObj = {}; (function() {{ return {}; }}).call(thisObj);",
            js_object, expr
        );

        // Execute the JavaScript expression
        let result = runtime.eval::<serde_json::Value>(&script_code)
            .map_err(|e| SiftError::EvaluationError(format!("Script execution error: {}", e)))?;

        // Convert the result to a boolean
        match result {
            Value::Bool(b) => Ok(b),
            Value::Number(n) => Ok(n.as_f64().unwrap_or(0.0) != 0.0),
            Value::String(s) => Ok(!s.is_empty()),
            Value::Null => Ok(false),
            Value::Array(arr) => Ok(!arr.is_empty()),
            Value::Object(obj) => Ok(!obj.is_empty()),
        }
    }
}

// WASM implementation with Boa
#[cfg(feature = "wasm")]
struct WhereOperation {
    expression: String,
}

#[cfg(feature = "wasm")]
impl Operation for WhereOperation {
    fn test(&self, value: &Value, _key: Option<&str>, _parent: Option<&Value>) -> SiftResult<bool> {
        // Use Boa to evaluate the JavaScript expression
        self.evaluate_expression(&self.expression, value)
    }
}

#[cfg(feature = "wasm")]
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

// Provide a stub implementation when neither server nor wasm features are enabled
#[cfg(not(any(feature = "server", feature = "wasm")))]
use crate::core::{Operation, QueryOperator};
#[cfg(not(any(feature = "server", feature = "wasm")))]
use crate::{SiftError, SiftResult};
#[cfg(not(any(feature = "server", feature = "wasm")))]
use serde_json::Value;

#[cfg(not(any(feature = "server", feature = "wasm")))]
pub struct WhereOperator;

#[cfg(not(any(feature = "server", feature = "wasm")))]
impl QueryOperator for WhereOperator {
    fn create_operation(&self, _params: &Value, _parent_query: &Value) -> SiftResult<Box<dyn Operation>> {
        Err(SiftError::UnsupportedOperation("$where operator is not available in this build".to_string()))
    }
    
    fn name(&self) -> &'static str {
        "$where"
    }
}
