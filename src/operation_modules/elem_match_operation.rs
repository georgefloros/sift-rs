use crate::core::{Operation, QueryOperator};
use crate::SiftResult;
use serde_json::Value;

/// $elemMatch operator - tests if any array element matches the given query
pub struct ElemMatchOperator;

impl QueryOperator for ElemMatchOperator {
    fn create_operation(&self, params: &Value, _parent_query: &Value) -> SiftResult<Box<dyn Operation>> {
        let query = crate::query::Query::from_value(&params)?;
        Ok(Box::new(ElemMatchOperation { query }))
    }
    
    fn name(&self) -> &'static str {
        "$elemMatch"
    }
}

struct ElemMatchOperation {
    query: crate::query::Query,
}

impl Operation for ElemMatchOperation {
    fn test(&self, value: &Value, _key: Option<&str>, _parent: Option<&Value>) -> SiftResult<bool> {
        if let Value::Array(array) = value {
            for item in array {
                if self.query.test(item)? {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}
