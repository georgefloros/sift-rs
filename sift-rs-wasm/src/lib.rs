use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Re-export the main sift-rs functionality with WASM bindings
use sift_rs::{sift as sift_impl, create_filter, SiftError};

// Define a serializable error type for JavaScript
#[derive(Serialize, Deserialize)]
pub struct JsSiftError {
    pub message: String,
    pub error_type: String,
}

// Convert internal SiftError to a JavaScript-compatible error
impl From<SiftError> for JsSiftError {
    fn from(err: SiftError) -> Self {
        let error_type = match err {
            SiftError::InvalidQuery(_) => "InvalidQuery".to_string(),
            SiftError::InvalidValue(_) => "InvalidValue".to_string(),
            SiftError::UnsupportedOperation(_) => "UnsupportedOperation".to_string(),
            SiftError::SerializationError(_) => "SerializationError".to_string(),
            SiftError::EvaluationError(_) => "EvaluationError".to_string(),
        };
        
        JsSiftError {
            message: err.to_string(),
            error_type,
        }
    }
}

// A wrapper that implements the conversion to JsValue
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "Error")]
    type JSError;

    #[wasm_bindgen(constructor)]
    fn new(message: &str) -> JSError;
}

// Convert internal SiftError to a JsValue for JavaScript exceptions
fn error_to_js_value(err: SiftError) -> JsValue {
    let js_error = JsSiftError::from(err);
    // Convert to a JavaScript Error object
    let error_msg = format!("SiftError: {}", js_error.message);
    JsValue::from(JSError::new(&error_msg))
}

/// Main sift function that tests if a value matches a query
/// 
/// # Arguments
/// 
/// * `query` - The MongoDB-style query as a stringified JSON
/// * `value` - The value to test against the query as a stringified JSON
/// 
/// # Returns
/// 
/// Returns `true` if the value matches the query, `false` otherwise, or throws an error
#[wasm_bindgen]
pub fn sift(query: &str, value: &str) -> Result<bool, JsValue> {
    let query_json: Value = serde_json::from_str(query)
        .map_err(|e| JsValue::from_str(&format!("Invalid query JSON: {}", e)))?;
    
    let value_json: Value = serde_json::from_str(value)
        .map_err(|e| JsValue::from_str(&format!("Invalid value JSON: {}", e)))?;
    
    match sift_impl(&query_json, &value_json) {
        Ok(result) => Ok(result),
        Err(e) => Err(error_to_js_value(e)),
    }
}

/// Tests multiple values against a query
/// 
/// # Arguments
/// 
/// * `query` - The MongoDB-style query as a stringified JSON
/// * `values` - An array of values to test against the query as a stringified JSON array
/// 
/// # Returns
/// 
/// Returns an array of booleans indicating whether each value matches the query
#[wasm_bindgen]
pub fn sift_many(query: &str, values: &str) -> Result<String, JsValue> {
    let query_json: Value = serde_json::from_str(query)
        .map_err(|e| JsValue::from_str(&format!("Invalid query JSON: {}", e)))?;
    
    let values_json: Vec<Value> = serde_json::from_str(values)
        .map_err(|e| JsValue::from_str(&format!("Invalid values JSON: {}", e)))?;
    
    let mut results = Vec::new();
    for value in values_json {
        match sift_impl(&query_json, &value) {
            Ok(result) => results.push(result),
            Err(e) => {
                // Log error but continue processing other values
                web_sys::console::error_1(&format!("Error filtering value: {:?}", e).into());
                results.push(false)
            }
        }
    }
    
    serde_json::to_string(&results)
        .map_err(|e| JsValue::from_str(&format!("Error serializing results: {}", e)))
}

/// Creates a filter function that can be used to test multiple values against a query
/// 
/// # Arguments
/// 
/// * `query` - The MongoDB-style query as a stringified JSON
/// 
/// # Returns
/// 
/// Returns a FilterFunction that can be used to test values
#[wasm_bindgen]
pub fn create_filter_fn(query: &str) -> Result<FilterFunction, JsValue> {
    let query_json: Value = serde_json::from_str(query)
        .map_err(|e| JsValue::from_str(&format!("Invalid query JSON: {}", e)))?;
    
    match sift_rs::Query::from_value(&query_json) {
        Ok(query_obj) => {
            Ok(FilterFunction { 
                query_str: query.to_string(),
                query_obj: Some(query_obj), // Store the parsed query object
            })
        },
        Err(e) => Err(error_to_js_value(e)),
    }
}

/// A filter function that can be used to test multiple values against a stored query
#[wasm_bindgen]
pub struct FilterFunction {
    query_str: String,
    // We'll store the query object to avoid re-parsing
    #[wasm_bindgen(skip)]
    query_obj: Option<sift_rs::Query>,
}

#[wasm_bindgen]
impl FilterFunction {
    /// Test a value against the stored query
    pub fn test(&self, value: &str) -> Result<bool, JsValue> {
        let value_json: Value = serde_json::from_str(value)
            .map_err(|e| JsValue::from_str(&format!("Invalid value JSON: {}", e)))?;
        
        // Use the cached query object if available, otherwise parse from string
        if let Some(ref query_obj) = self.query_obj {
            match query_obj.test(&value_json) {
                Ok(result) => Ok(result),
                Err(e) => Err(error_to_js_value(e)),
            }
        } else {
            // Fallback to parsing from string if for some reason query_obj is None
            let query_json: Value = serde_json::from_str(&self.query_str)
                .map_err(|e| JsValue::from_str(&format!("Invalid query JSON: {}", e)))?;
            
            match sift_impl(&query_json, &value_json) {
                Ok(result) => Ok(result),
                Err(e) => Err(error_to_js_value(e)),
            }
        }
    }
}

/// Validates a query string to ensure it's a valid MongoDB-style query
/// 
/// # Arguments
/// 
/// * `query` - The query as a stringified JSON
/// 
/// # Returns
/// 
/// Returns `true` if the query is valid, `false` otherwise
#[wasm_bindgen]
pub fn validate_query(query: &str) -> Result<bool, JsValue> {
    let query_json: Value = serde_json::from_str(query)
        .map_err(|e| JsValue::from_str(&format!("Invalid query JSON: {}", e)))?;
    
    match sift_rs::Query::from_value(&query_json) {
        Ok(_) => Ok(true),
        Err(e) => Err(error_to_js_value(e)),
    }
}