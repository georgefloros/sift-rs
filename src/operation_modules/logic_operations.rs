use crate::core::{Operation, QueryOperator};
use crate::{SiftError, SiftResult};
use serde_json::Value;

/// $and operator - all queries must match
pub struct AndOperator;

impl QueryOperator for AndOperator {
    fn create_operation(&self, params: &Value, _parent_query: &Value) -> SiftResult<Box<dyn Operation>> {
        if let Value::Array(queries) = params {
            Ok(Box::new(AndOperation { queries: queries.clone() }))
        } else {
            Err(SiftError::InvalidQuery("$and requires an array of queries".to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "$and"
    }
}

struct AndOperation {
    queries: Vec<Value>,
}

impl Operation for AndOperation {
    fn test(&self, value: &Value, _key: Option<&str>, _parent: Option<&Value>) -> SiftResult<bool> {
        for query_value in &self.queries {
            // Create a temporary query for each sub-query
            let query = crate::query::Query::from_value(query_value)?;
            if !query.test(value)? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

/// $or operator - at least one query must match
pub struct OrOperator;

impl QueryOperator for OrOperator {
    fn create_operation(&self, params: &Value, _parent_query: &Value) -> SiftResult<Box<dyn Operation>> {
        if let Value::Array(queries) = params {
            Ok(Box::new(OrOperation { queries: queries.clone() }))
        } else {
            Err(SiftError::InvalidQuery("$or requires an array of queries".to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "$or"
    }
}

struct OrOperation {
    queries: Vec<Value>,
}

impl Operation for OrOperation {
    fn test(&self, value: &Value, _key: Option<&str>, _parent: Option<&Value>) -> SiftResult<bool> {
        for query_value in &self.queries {
            let query = crate::query::Query::from_value(query_value)?;
            if query.test(value)? {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

/// $nor operator - none of the queries must match
pub struct NorOperator;

impl QueryOperator for NorOperator {
    fn create_operation(&self, params: &Value, _parent_query: &Value) -> SiftResult<Box<dyn Operation>> {
        if let Value::Array(queries) = params {
            Ok(Box::new(NorOperation { queries: queries.clone() }))
        } else {
            Err(SiftError::InvalidQuery("$nor requires an array of queries".to_string()))
        }
    }
    
    fn name(&self) -> &'static str {
        "$nor"
    }
}

struct NorOperation {
    queries: Vec<Value>,
}

impl Operation for NorOperation {
    fn test(&self, value: &Value, _key: Option<&str>, _parent: Option<&Value>) -> SiftResult<bool> {
        for query_value in &self.queries {
            let query = crate::query::Query::from_value(query_value)?;
            if query.test(value)? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

/// $not operator - the query must not match
pub struct NotOperator;

impl QueryOperator for NotOperator {
    fn create_operation(&self, params: &Value, _parent_query: &Value) -> SiftResult<Box<dyn Operation>> {
        Ok(Box::new(NotOperation { query: params.clone() }))
    }
    
    fn name(&self) -> &'static str {
        "$not"
    }
}

struct NotOperation {
    query: Value,
}

impl Operation for NotOperation {
    fn test(&self, value: &Value, _key: Option<&str>, _parent: Option<&Value>) -> SiftResult<bool> {
        // Check if the query is a field-level operation (contains operators like $eq, $gt, etc.)
        if let Value::Object(obj) = &self.query {
            // If it's a single operation (like {"$eq": "active"}), apply it directly to the value
            if obj.len() == 1 && obj.keys().next().unwrap().starts_with('$') {
                let (op_name, op_value) = obj.iter().next().unwrap();
                
                // Get the operator from the registry
                let context = crate::core::QueryContext::new();
                if let Some(operator) = context.registry.get(op_name) {
                    let operation = operator.create_operation(op_value, &Value::Null)?;
                    let result = operation.test(value, _key, _parent)?;
                    return Ok(!result);
                }
            }
        }
        
        // Fall back to treating it as a full query (for complex nested queries)
        let query = crate::query::Query::from_value(&self.query)?;
        let result = query.test(value)?;
        Ok(!result)
    }
}
