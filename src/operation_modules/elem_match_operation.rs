use crate::core::{Operation, QueryOperator, CompiledQuery};
use crate::SiftResult;
use serde_json::Value;

/// $elemMatch operator - tests if any array element matches the given query
pub struct ElemMatchOperator;

impl QueryOperator for ElemMatchOperator {
    fn create_operation(&self, params: &Value, _parent_query: &Value) -> SiftResult<Box<dyn Operation>> {
        let query = crate::query::Query::from_value(&params)?;
        let compiled_subquery = query.compile()?;
        Ok(Box::new(ElemMatchOperation { compiled_subquery }))
    }
    
    fn name(&self) -> &'static str {
        "$elemMatch"
    }
}

struct ElemMatchOperation {
    compiled_subquery: CompiledQuery,
}

impl Operation for ElemMatchOperation {
    fn test(&self, value: &Value, _key: Option<&str>, _parent: Option<&Value>) -> SiftResult<bool> {
        if let Value::Array(array) = value {
            for item in array {
                if self.compiled_subquery.test(item)? {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}
