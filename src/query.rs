use crate::core::{CompiledQuery, Operation, QueryContext, utils};
use crate::{SiftError, SiftResult};
use serde_json::{Map, Value};
use std::collections::HashMap;

/// Represents a MongoDB-style query that can be compiled and executed
#[derive(Debug, Clone)]
pub struct Query {
    conditions: HashMap<String, QueryCondition>,
}

/// Represents a condition in a query (either a direct value match or an operation)
#[derive(Debug, Clone)]
pub enum QueryCondition {
    Value(Value),
    Operations(HashMap<String, Value>),
    Mixed {
        value: Option<Value>,
        operations: HashMap<String, Value>,
    },
}

impl Query {
    /// Create a new empty query
    pub fn new() -> Self {
        Query {
            conditions: HashMap::new(),
        }
    }

    /// Create a query from a JSON value
    pub fn from_value(value: &Value) -> SiftResult<Self> {
        match value {
            Value::Object(obj) => Self::from_object(obj),
            _ => {
                // For non-object values, create an equality query
                let mut conditions = HashMap::new();
                conditions.insert("".to_string(), QueryCondition::Value(value.clone()));
                Ok(Query { conditions })
            }
        }
    }

    /// Create a query from a JSON object
    pub fn from_object(obj: &Map<String, Value>) -> SiftResult<Self> {
        let mut conditions = HashMap::new();

        // Check if both $and and $or are present at the top level
        let has_and = obj.contains_key("$and");
        let has_or = obj.contains_key("$or");

        if has_and && has_or {
            // Both $and and $or are present - $or must be at top level with $and nested inside
            // This is our rule: when both operators exist, $or is always the top-level operator
            let mut and_value: Option<Value> = None;
            let mut or_value: Option<Value> = None;
            let mut other_conditions = HashMap::new();

            for (key, value) in obj {
                match key.as_str() {
                    "$and" => {
                        and_value = Some(value.clone());
                    }
                    "$or" => {
                        or_value = Some(value.clone());
                    }
                    _ => {
                        if matches!(key.as_str(), "$not" | "$nor" | "$where") {
                            let mut operations = HashMap::new();
                            operations.insert(key.clone(), value.clone());
                            other_conditions.insert(key.clone(), QueryCondition::Operations(operations));
                        } else {
                            let condition = Self::parse_condition(value)?;
                            other_conditions.insert(key.clone(), condition);
                        }
                    }
                }
            }

            // Always nest $and inside $or when both are present
            let mut nested_and_obj = Map::new();
            nested_and_obj.insert("$and".to_string(), and_value.unwrap());
            
            let mut or_val = or_value.unwrap();
            if let Value::Array(ref mut or_array) = or_val {
                or_array.push(Value::Object(nested_and_obj));
            }
            
            let mut operations = HashMap::new();
            operations.insert("$or".to_string(), or_val);
            conditions.insert("$or".to_string(), QueryCondition::Operations(operations));

            // Add other conditions
            conditions.extend(other_conditions);
        } else {
            // Normal processing when both $and and $or are not present
            for (key, value) in obj {
                if matches!(key.as_str(), "$and" | "$or" | "$not" | "$nor" | "$where") {
                    let mut operations = HashMap::new();
                    operations.insert(key.clone(), value.clone());
                    conditions.insert(key.clone(), QueryCondition::Operations(operations));
                } else {
                    let condition = Self::parse_condition(value)?;
                    conditions.insert(key.clone(), condition);
                }
            }
        }

        Ok(Query { conditions })
    }

    /// Parse a single condition value
    fn parse_condition(value: &Value) -> SiftResult<QueryCondition> {
        match value {
            Value::Object(obj) => {
                let mut operations = HashMap::new();
                let mut has_operators = false;
                let mut regular_fields = HashMap::new();

                for (key, val) in obj {
                    if key.starts_with('$') {
                        has_operators = true;
                        operations.insert(key.clone(), val.clone());
                    } else {
                        regular_fields.insert(key.clone(), val.clone());
                    }
                }

                if has_operators && !regular_fields.is_empty() {
                    // Mixed operators and regular fields - treat regular fields as $eq operations
                    let mut value_obj = Map::new();
                    for (k, v) in regular_fields {
                        value_obj.insert(k, v);
                    }
                    Ok(QueryCondition::Mixed {
                        value: Some(Value::Object(value_obj)),
                        operations,
                    })
                } else if has_operators {
                    Ok(QueryCondition::Operations(operations))
                } else {
                    Ok(QueryCondition::Value(value.clone()))
                }
            }
            _ => Ok(QueryCondition::Value(value.clone())),
        }
    }

    /// Compile the query into an executable form
    pub fn compile(&self) -> SiftResult<CompiledQuery> {
        self.compile_with_context(QueryContext::new())
    }

    /// Compile the query with a specific context
    pub fn compile_with_context(&self, context: QueryContext) -> SiftResult<CompiledQuery> {
        let mut operations: Vec<Box<dyn Operation>> = Vec::new();

        for (field_path, condition) in &self.conditions {
            let field_operations = self.compile_condition(field_path, condition, &context)?;
            operations.extend(field_operations);
        }

        Ok(CompiledQuery::new(operations, context))
    }

    /// Compile a single condition into operations
    fn compile_condition(
        &self,
        field_path: &str,
        condition: &QueryCondition,
        context: &QueryContext,
    ) -> SiftResult<Vec<Box<dyn Operation>>> {
        let mut operations: Vec<Box<dyn Operation>> = Vec::new();

        match condition {
            QueryCondition::Value(value) => {
                // Direct value comparison (implicit $eq)
                if let Some(eq_op) = context.registry.get("$eq") {
                    let operation = eq_op.create_operation(value, &Value::Null)?;
                    operations.push(Box::new(FieldOperation::new(
                        field_path.to_string(),
                        operation,
                    )) as Box<dyn Operation>);
                }
            }
            QueryCondition::Operations(ops) => {
                for (op_name, op_value) in ops {
                    if let Some(operator) = context.registry.get(op_name) {
                        let operation = operator.create_operation(op_value, &Value::Null)?;
                        
                        // Special handling for logical operators that don't operate on specific fields
                        if matches!(
                            op_name.as_str(),
                            "$and" | "$or" | "$nor" | "$where"
                        ) {
                            operations.push(operation);
                        } else {
                            // $not can be both root-level and field-level, apply to field if field_path is not empty
                            operations.push(Box::new(FieldOperation::new(
                                field_path.to_string(),
                                operation,
                            )) as Box<dyn Operation>);
                        }
                    } else {
                        return Err(SiftError::UnsupportedOperation(format!(
                            "Unknown operator: {}",
                            op_name
                        )));
                    }
                }
            }
            QueryCondition::Mixed { value, operations: ops } => {
                // Handle mixed value and operations
                if let Some(val) = value {
                    if let Some(eq_op) = context.registry.get("$eq") {
                        let operation = eq_op.create_operation(val, &Value::Null)?;
                        operations.push(Box::new(FieldOperation::new(
                            field_path.to_string(),
                            operation,
                        )) as Box<dyn Operation>);
                    }
                }

                for (op_name, op_value) in ops {
                    if let Some(operator) = context.registry.get(op_name) {
                        let operation = operator.create_operation(op_value, &Value::Null)?;
                        operations.push(Box::new(FieldOperation::new(
                            field_path.to_string(),
                            operation,
                        )) as Box<dyn Operation>);
                    } else {
                        return Err(SiftError::UnsupportedOperation(format!(
                            "Unknown operator: {}",
                            op_name
                        )));
                    }
                }
            }
        }

        Ok(operations)
    }

    /// Test a value against this query directly (without compilation)
    pub fn test(&self, value: &Value) -> SiftResult<bool> {
        let compiled = self.compile()?;
        compiled.test(value)
    }
}

impl Default for Query {
    fn default() -> Self {
        Self::new()
    }
}

/// Wraps an operation to apply it to a specific field path
pub struct FieldOperation {
    field_path: String,
    operation: Box<dyn Operation>,
}

impl FieldOperation {
    pub fn new(field_path: String, operation: Box<dyn Operation>) -> Self {
        FieldOperation {
            field_path,
            operation,
        }
    }
}

impl Operation for FieldOperation {
    fn test(&self, value: &Value, _key: Option<&str>, _parent: Option<&Value>) -> SiftResult<bool> {
        if self.field_path.is_empty() {
            // Root level operation
            return self.operation.test(value, None, None);
        }

        // Handle dot notation field paths
        if self.field_path.contains('.') {
            let mut found_match = false;
            utils::walk_array_values(value, &self.field_path, |field_value| {
                if let Ok(result) = self.operation.test(field_value, None, None) {
                    if result {
                        found_match = true;
                        return true; // Stop walking
                    }
                }
                false // Continue walking
            });
            return Ok(found_match);
        }

        // Simple field access
        match value {
            Value::Object(obj) => {
                if let Some(field_value) = obj.get(&self.field_path) {
                    self.operation.test(field_value, Some(&self.field_path), Some(value))
                } else {
                    // Field doesn't exist - let the operation decide how to handle this
                    self.operation.test(&Value::Null, Some(&self.field_path), Some(value))
                }
            }
            Value::Array(arr) => {
                // For arrays, check if any element matches when treated as an object
                for element in arr.iter() {
                    if let Value::Object(obj) = element {
                        if let Some(field_value) = obj.get(&self.field_path) {
                            if self.operation.test(field_value, Some(&self.field_path), Some(element))? {
                                return Ok(true);
                            }
                        }
                    }
                }
                Ok(false)
            }
            _ => {
                // For primitive values, only match if field_path is empty or the operation handles nulls
                self.operation.test(&Value::Null, Some(&self.field_path), Some(value))
            }
        }
    }

    fn reset(&mut self) {
        self.operation.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_query_from_value() {
        let query_value = json!({
            "name": "Alice",
            "age": {"$gte": 18},
            "status": {"$in": ["active", "pending"]}
        });

        let query = Query::from_value(&query_value).unwrap();
        assert_eq!(query.conditions.len(), 3);
    }

    #[test]
    fn test_simple_equality_query() {
        let query = Query::from_value(&json!({"name": "Alice"})).unwrap();
        let value = json!({"name": "Alice", "age": 30});
        
        assert!(query.test(&value).unwrap());
    }

    #[test]
    fn test_comparison_query() {
        let query = Query::from_value(&json!({"age": {"$gte": 18}})).unwrap();
        
        assert!(query.test(&json!({"age": 25})).unwrap());
        assert!(!query.test(&json!({"age": 15})).unwrap());
    }

    #[test]
    fn test_in_query() {
        let query = Query::from_value(&json!({"status": {"$in": ["active", "pending"]}})).unwrap();
        
        assert!(query.test(&json!({"status": "active"})).unwrap());
        assert!(query.test(&json!({"status": "pending"})).unwrap());
        assert!(!query.test(&json!({"status": "inactive"})).unwrap());
    }

    #[test]
    fn test_nested_field_access() {
        let query = Query::from_value(&json!({"user.name": "Alice"})).unwrap();
        let value = json!({
            "user": {
                "name": "Alice",
                "age": 30
            }
        });
        
        assert!(query.test(&value).unwrap());
    }

    #[test]
    fn test_logical_operators() {
        let query = Query::from_value(&json!({
            "$and": [
                {"age": {"$gte": 18}},
                {"status": "active"}
            ]
        })).unwrap();
        
        let value = json!({"age": 25, "status": "active"});
        assert!(query.test(&value).unwrap());
        
        let value2 = json!({"age": 15, "status": "active"});
        assert!(!query.test(&value2).unwrap());
    }
}
