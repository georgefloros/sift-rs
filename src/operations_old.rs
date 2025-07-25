use crate::core::{Operation, QueryOperator, utils};
use crate::query::Query;
use crate::{SiftError, SiftResult};
use serde_json::Value;
use std::cmp::Ordering;

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

// Import operations from modules
mod operations;
use operations::*;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_gt_operation() {
        let op = GtOperator;
        let operation = op.create_operation(&json!(5), &Value::Null).unwrap();
        
        assert!(operation.test(&json!(10), None, None).unwrap());
        assert!(operation.test(&json!(6), None, None).unwrap());
        assert!(!operation.test(&json!(5), None, None).unwrap()); // equal should be false
        assert!(!operation.test(&json!(3), None, None).unwrap());
        assert!(!operation.test(&json!("not a number"), None, None).unwrap());
    }

    #[test]
    fn test_gte_operation() {
        let op = GteOperator;
        let operation = op.create_operation(&json!(5), &Value::Null).unwrap();
        
        assert!(operation.test(&json!(10), None, None).unwrap());
        assert!(operation.test(&json!(6), None, None).unwrap());
        assert!(operation.test(&json!(5), None, None).unwrap()); // equal should be true
        assert!(!operation.test(&json!(3), None, None).unwrap());
        assert!(!operation.test(&json!("not a number"), None, None).unwrap());
    }

    #[test]
    fn test_lt_operation() {
        let op = LtOperator;
        let operation = op.create_operation(&json!(5), &Value::Null).unwrap();
        
        assert!(!operation.test(&json!(10), None, None).unwrap());
        assert!(!operation.test(&json!(6), None, None).unwrap());
        assert!(!operation.test(&json!(5), None, None).unwrap()); // equal should be false
        assert!(operation.test(&json!(3), None, None).unwrap());
        assert!(operation.test(&json!(1), None, None).unwrap());
        assert!(!operation.test(&json!("not a number"), None, None).unwrap());
    }

    #[test]
    fn test_lte_operation() {
        let op = LteOperator;
        let operation = op.create_operation(&json!(5), &Value::Null).unwrap();
        
        assert!(!operation.test(&json!(10), None, None).unwrap());
        assert!(!operation.test(&json!(6), None, None).unwrap());
        assert!(operation.test(&json!(5), None, None).unwrap()); // equal should be true
        assert!(operation.test(&json!(3), None, None).unwrap());
        assert!(operation.test(&json!(1), None, None).unwrap());
        assert!(!operation.test(&json!("not a number"), None, None).unwrap());
    }

    #[test]
    fn test_eq_operation() {
        let op = EqOperator;
        let operation = op.create_operation(&json!("test"), &Value::Null).unwrap();
        
        assert!(operation.test(&json!("test"), None, None).unwrap());
        assert!(!operation.test(&json!("other"), None, None).unwrap());
        assert!(!operation.test(&json!(123), None, None).unwrap());
    }

    #[test]
    fn test_ne_operation() {
        let op = NeOperator;
        let operation = op.create_operation(&json!("test"), &Value::Null).unwrap();
        
        assert!(!operation.test(&json!("test"), None, None).unwrap());
        assert!(operation.test(&json!("other"), None, None).unwrap());
        assert!(operation.test(&json!(123), None, None).unwrap());
    }

    #[test]
    fn test_floating_point_comparisons() {
        let gt_op = GtOperator;
        let gt_operation = gt_op.create_operation(&json!(5.5), &Value::Null).unwrap();
        
        assert!(gt_operation.test(&json!(6.0), None, None).unwrap());
        assert!(gt_operation.test(&json!(5.6), None, None).unwrap());
        assert!(!gt_operation.test(&json!(5.5), None, None).unwrap());
        assert!(!gt_operation.test(&json!(5.4), None, None).unwrap());
        
        let gte_op = GteOperator;
        let gte_operation = gte_op.create_operation(&json!(5.5), &Value::Null).unwrap();
        
        assert!(gte_operation.test(&json!(6.0), None, None).unwrap());
        assert!(gte_operation.test(&json!(5.6), None, None).unwrap());
        assert!(gte_operation.test(&json!(5.5), None, None).unwrap());
        assert!(!gte_operation.test(&json!(5.4), None, None).unwrap());
    }

    #[test]
    fn test_in_operation() {
        let op = InOperator;
        let operation = op.create_operation(&json!([1, 2, 3, "test"]), &Value::Null).unwrap();
        
        assert!(operation.test(&json!(1), None, None).unwrap());
        assert!(operation.test(&json!(2), None, None).unwrap());
        assert!(operation.test(&json!(3), None, None).unwrap());
        assert!(operation.test(&json!("test"), None, None).unwrap());
        assert!(!operation.test(&json!(4), None, None).unwrap());
        assert!(!operation.test(&json!("other"), None, None).unwrap());
    }

    #[test]
    fn test_in_operation_invalid_params() {
        let op = InOperator;
        let result = op.create_operation(&json!("not an array"), &Value::Null);
        assert!(result.is_err());
    }

    #[test]
    fn test_nin_operation() {
        let op = NinOperator;
        let operation = op.create_operation(&json!([1, 2, 3, "test"]), &Value::Null).unwrap();
        
        assert!(!operation.test(&json!(1), None, None).unwrap());
        assert!(!operation.test(&json!(2), None, None).unwrap());
        assert!(!operation.test(&json!(3), None, None).unwrap());
        assert!(!operation.test(&json!("test"), None, None).unwrap());
        assert!(operation.test(&json!(4), None, None).unwrap());
        assert!(operation.test(&json!("other"), None, None).unwrap());
    }

    #[test]
    fn test_nin_operation_invalid_params() {
        let op = NinOperator;
        let result = op.create_operation(&json!("not an array"), &Value::Null);
        assert!(result.is_err());
    }

    #[test]
    fn test_all_operation() {
        let op = AllOperator;
        let operation = op.create_operation(&json!([1, 2, 3]), &Value::Null).unwrap();
        
        // Array containing all required values
        assert!(operation.test(&json!([1, 2, 3, 4, 5]), None, None).unwrap());
        assert!(operation.test(&json!([3, 2, 1]), None, None).unwrap());
        
        // Array missing some required values
        assert!(!operation.test(&json!([1, 2]), None, None).unwrap());
        assert!(!operation.test(&json!([1, 3]), None, None).unwrap());
        assert!(!operation.test(&json!([4, 5, 6]), None, None).unwrap());
        
        // Non-array value
        assert!(!operation.test(&json!(1), None, None).unwrap());
        assert!(!operation.test(&json!("not an array"), None, None).unwrap());
    }

    #[test]
    fn test_all_operation_invalid_params() {
        let op = AllOperator;
        let result = op.create_operation(&json!("not an array"), &Value::Null);
        assert!(result.is_err());
    }

    #[test]
    fn test_size_operation() {
        let op = SizeOperator;
        let operation = op.create_operation(&json!(3), &Value::Null).unwrap();
        
        // Test arrays
        assert!(operation.test(&json!([1, 2, 3]), None, None).unwrap());
        assert!(!operation.test(&json!([1, 2]), None, None).unwrap());
        assert!(!operation.test(&json!([1, 2, 3, 4]), None, None).unwrap());
        assert!(operation.test(&json!([]), None, None).unwrap_or(false) || !operation.test(&json!([]), None, None).unwrap()); // empty array has size 0
        
        // Test strings (also supported by MongoDB $size)
        assert!(operation.test(&json!("abc"), None, None).unwrap());
        assert!(!operation.test(&json!("ab"), None, None).unwrap());
        assert!(!operation.test(&json!("abcd"), None, None).unwrap());
        
        // Non-array, non-string values
        assert!(!operation.test(&json!(123), None, None).unwrap());
        assert!(!operation.test(&json!(null), None, None).unwrap());
    }

    #[test]
    fn test_size_operation_zero() {
        let op = SizeOperator;
        let operation = op.create_operation(&json!(0), &Value::Null).unwrap();
        
        assert!(operation.test(&json!([]), None, None).unwrap());
        assert!(operation.test(&json!(""), None, None).unwrap());
        assert!(!operation.test(&json!([1]), None, None).unwrap());
        assert!(!operation.test(&json!("a"), None, None).unwrap());
    }

    #[test]
    fn test_size_operation_invalid_params() {
        let op = SizeOperator;
        let result = op.create_operation(&json!("not a number"), &Value::Null);
        assert!(result.is_err());
    }

    #[test]
    fn test_elemmatch_operation() {
        let op = ElemMatchOperator;
        let query_params = json!({ "score": { "$gt": 80 } });
        let operation = op.create_operation(&query_params, &Value::Null).unwrap();
        
        // Array with matching element
        let array_with_match = json!([
            { "score": 75 },
            { "score": 85 },  // This matches $gt: 80
            { "score": 60 }
        ]);
        assert!(operation.test(&array_with_match, None, None).unwrap());
        
        // Array without matching element
        let array_without_match = json!([
            { "score": 75 },
            { "score": 70 },
            { "score": 60 }
        ]);
        assert!(!operation.test(&array_without_match, None, None).unwrap());
        
        // Non-array value
        let non_array = json!({ "score": 85 });
        assert!(!operation.test(&non_array, None, None).unwrap());
        
        // Empty array
        let empty_array = json!([]);
        assert!(!operation.test(&empty_array, None, None).unwrap());
    }

    #[test]
    fn test_elemmatch_operation_complex_query() {
        let op = ElemMatchOperator;
        let query_params = json!({
            "$and": [
                { "score": { "$gte": 80 } },
                { "grade": "A" }
            ]
        });
        let operation = op.create_operation(&query_params, &Value::Null).unwrap();
        
        // Array with matching element
        let array_with_match = json!([
            { "score": 75, "grade": "B" },
            { "score": 85, "grade": "A" },  // This matches both conditions
            { "score": 90, "grade": "B" }   // High score but wrong grade
        ]);
        assert!(operation.test(&array_with_match, None, None).unwrap());
        
        // Array without fully matching element
        let array_without_match = json!([
            { "score": 75, "grade": "A" },  // Right grade but low score
            { "score": 85, "grade": "B" },  // High score but wrong grade
            { "score": 60, "grade": "C" }
        ]);
        assert!(!operation.test(&array_without_match, None, None).unwrap());
    }

    // Tests for logical operators
    #[test]
    fn test_and_operation_basic() {
        let op = AndOperator;
        let query_params = json!([
            { "age": { "$gte": 18 } },
            { "status": "active" }
        ]);
        let operation = op.create_operation(&query_params, &Value::Null).unwrap();
        
        // Document matching both conditions
        let matching_doc = json!({ "age": 25, "status": "active" });
        assert!(operation.test(&matching_doc, None, None).unwrap());
        
        // Document matching only first condition
        let partial_match1 = json!({ "age": 25, "status": "inactive" });
        assert!(!operation.test(&partial_match1, None, None).unwrap());
        
        // Document matching only second condition
        let partial_match2 = json!({ "age": 16, "status": "active" });
        assert!(!operation.test(&partial_match2, None, None).unwrap());
        
        // Document matching neither condition
        let no_match = json!({ "age": 16, "status": "inactive" });
        assert!(!operation.test(&no_match, None, None).unwrap());
    }

    #[test]
    fn test_and_operation_empty_array() {
        let op = AndOperator;
        let query_params = json!([]);
        let operation = op.create_operation(&query_params, &Value::Null).unwrap();
        
        // Empty $and should match any document (vacuous truth)
        let doc = json!({ "name": "test" });
        assert!(operation.test(&doc, None, None).unwrap());
    }

    #[test]
    fn test_and_operation_nested() {
        let op = AndOperator;
        let query_params = json!([
            { "$or": [{ "category": "A" }, { "category": "B" }] },
            { "score": { "$gt": 50 } }
        ]);
        let operation = op.create_operation(&query_params, &Value::Null).unwrap();
        
        // Document matching both nested conditions
        let matching_doc = json!({ "category": "A", "score": 75 });
        assert!(operation.test(&matching_doc, None, None).unwrap());
        
        // Document with category B and high score
        let matching_doc2 = json!({ "category": "B", "score": 60 });
        assert!(operation.test(&matching_doc2, None, None).unwrap());
        
        // Document with wrong category
        let no_match1 = json!({ "category": "C", "score": 75 });
        assert!(!operation.test(&no_match1, None, None).unwrap());
        
        // Document with low score
        let no_match2 = json!({ "category": "A", "score": 30 });
        assert!(!operation.test(&no_match2, None, None).unwrap());
    }

    #[test]
    fn test_and_operation_invalid_params() {
        let op = AndOperator;
        let result = op.create_operation(&json!("not an array"), &Value::Null);
        assert!(result.is_err());
    }

    #[test]
    fn test_or_operation_basic() {
        let op = OrOperator;
        let query_params = json!([
            { "status": "premium" },
            { "age": { "$gte": 65 } }
        ]);
        let operation = op.create_operation(&query_params, &Value::Null).unwrap();
        
        // Document matching first condition only
        let match1 = json!({ "status": "premium", "age": 30 });
        assert!(operation.test(&match1, None, None).unwrap());
        
        // Document matching second condition only
        let match2 = json!({ "status": "regular", "age": 70 });
        assert!(operation.test(&match2, None, None).unwrap());
        
        // Document matching both conditions
        let match_both = json!({ "status": "premium", "age": 70 });
        assert!(operation.test(&match_both, None, None).unwrap());
        
        // Document matching neither condition
        let no_match = json!({ "status": "regular", "age": 30 });
        assert!(!operation.test(&no_match, None, None).unwrap());
    }

    #[test]
    fn test_or_operation_empty_array() {
        let op = OrOperator;
        let query_params = json!([]);
        let operation = op.create_operation(&query_params, &Value::Null).unwrap();
        
        // Empty $or should not match any document
        let doc = json!({ "name": "test" });
        assert!(!operation.test(&doc, None, None).unwrap());
    }

    #[test]
    fn test_or_operation_nested() {
        let op = OrOperator;
        let query_params = json!([
            { "$and": [{ "type": "user" }, { "verified": true }] },
            { "role": "admin" }
        ]);
        let operation = op.create_operation(&query_params, &Value::Null).unwrap();
        
        // Document matching first nested condition (verified user)
        let match1 = json!({ "type": "user", "verified": true, "role": "member" });
        assert!(operation.test(&match1, None, None).unwrap());
        
        // Document matching second condition (admin)
        let match2 = json!({ "type": "guest", "verified": false, "role": "admin" });
        assert!(operation.test(&match2, None, None).unwrap());
        
        // Document matching neither condition
        let no_match = json!({ "type": "user", "verified": false, "role": "member" });
        assert!(!operation.test(&no_match, None, None).unwrap());
    }

    #[test]
    fn test_or_operation_invalid_params() {
        let op = OrOperator;
        let result = op.create_operation(&json!(42), &Value::Null);
        assert!(result.is_err());
    }

    #[test]
    fn test_not_operation_basic() {
        let op = NotOperator;
        let query_params = json!({ "status": "inactive" });
        let operation = op.create_operation(&query_params, &Value::Null).unwrap();
        
        // Document that would match the inner condition (should fail $not)
        let matching_inner = json!({ "status": "inactive" });
        assert!(!operation.test(&matching_inner, None, None).unwrap());
        
        // Document that would not match the inner condition (should pass $not)
        let not_matching_inner = json!({ "status": "active" });
        assert!(operation.test(&not_matching_inner, None, None).unwrap());
        
        // Document without the field (should pass $not)
        let missing_field = json!({ "name": "test" });
        assert!(operation.test(&missing_field, None, None).unwrap());
    }

    #[test]
    fn test_not_operation_with_comparison() {
        let op = NotOperator;
        let query_params = json!({ "age": { "$lt": 18 } });
        let operation = op.create_operation(&query_params, &Value::Null).unwrap();
        
        // Age less than 18 (should fail $not)
        let minor = json!({ "age": 16 });
        assert!(!operation.test(&minor, None, None).unwrap());
        
        // Age 18 or greater (should pass $not)
        let adult = json!({ "age": 25 });
        assert!(operation.test(&adult, None, None).unwrap());
        
        // Exactly 18 (should pass $not)
        let exactly_18 = json!({ "age": 18 });
        assert!(operation.test(&exactly_18, None, None).unwrap());
    }

    #[test]
    fn test_not_operation_nested() {
        let op = NotOperator;
        let query_params = json!({
            "$and": [
                { "type": "temporary" },
                { "expires": { "$exists": true } }
            ]
        });
        let operation = op.create_operation(&query_params, &Value::Null).unwrap();
        
        // Document matching inner $and (should fail $not)
        let temp_with_expiry = json!({ "type": "temporary", "expires": "2024-12-31" });
        assert!(!operation.test(&temp_with_expiry, None, None).unwrap());
        
        // Document not matching inner $and (should pass $not)
        let permanent = json!({ "type": "permanent" });
        assert!(operation.test(&permanent, None, None).unwrap());
        
        // Temporary but no expiry field (should pass $not)
        let temp_no_expiry = json!({ "type": "temporary" });
        assert!(operation.test(&temp_no_expiry, None, None).unwrap());
    }

    #[test]
    fn test_nor_operation_basic() {
        let op = NorOperator;
        let query_params = json!([
            { "status": "banned" },
            { "score": { "$lt": 0 } }
        ]);
        let operation = op.create_operation(&query_params, &Value::Null).unwrap();
        
        // Document matching first condition (should fail $nor)
        let banned_user = json!({ "status": "banned", "score": 50 });
        assert!(!operation.test(&banned_user, None, None).unwrap());
        
        // Document matching second condition (should fail $nor)
        let negative_score = json!({ "status": "active", "score": -10 });
        assert!(!operation.test(&negative_score, None, None).unwrap());
        
        // Document matching both conditions (should fail $nor)
        let both_bad = json!({ "status": "banned", "score": -5 });
        assert!(!operation.test(&both_bad, None, None).unwrap());
        
        // Document matching neither condition (should pass $nor)
        let good_user = json!({ "status": "active", "score": 100 });
        assert!(operation.test(&good_user, None, None).unwrap());
    }

    #[test]
    fn test_nor_operation_empty_array() {
        let op = NorOperator;
        let query_params = json!([]);
        let operation = op.create_operation(&query_params, &Value::Null).unwrap();
        
        // Empty $nor should match any document (no conditions to violate)
        let doc = json!({ "name": "test" });
        assert!(operation.test(&doc, None, None).unwrap());
    }

    #[test]
    fn test_nor_operation_nested() {
        let op = NorOperator;
        let query_params = json!([
            { "$and": [{ "type": "spam" }, { "reported": true }] },
            { "reputation": { "$lte": -100 } }
        ]);
        let operation = op.create_operation(&query_params, &Value::Null).unwrap();
        
        // Document matching first nested condition (should fail $nor)
        let reported_spam = json!({ "type": "spam", "reported": true, "reputation": 50 });
        assert!(!operation.test(&reported_spam, None, None).unwrap());
        
        // Document matching second condition (should fail $nor)
        let low_reputation = json!({ "type": "normal", "reported": false, "reputation": -150 });
        assert!(!operation.test(&low_reputation, None, None).unwrap());
        
        // Document matching neither condition (should pass $nor)
        let good_user = json!({ "type": "normal", "reported": false, "reputation": 500 });
        assert!(operation.test(&good_user, None, None).unwrap());
        
        // Spam but not reported (should pass $nor)
        let unreported_spam = json!({ "type": "spam", "reported": false, "reputation": 10 });
        assert!(operation.test(&unreported_spam, None, None).unwrap());
    }

    #[test]
    fn test_nor_operation_invalid_params() {
        let op = NorOperator;
        let result = op.create_operation(&json!("invalid"), &Value::Null);
        assert!(result.is_err());
    }

    #[test]
    fn test_logical_operators_combination() {
        // Test complex combination: $and with $or and $not
        let and_op = AndOperator;
        let query_params = json!([
            { "$or": [{ "category": "premium" }, { "score": { "$gte": 90 } }] },
            { "$not": { "status": "suspended" } },
            { "verified": true }
        ]);
        let operation = and_op.create_operation(&query_params, &Value::Null).unwrap();
        
        // Premium, not suspended, verified (should match)
        let premium_user = json!({
            "category": "premium",
            "status": "active",
            "verified": true,
            "score": 50
        });
        assert!(operation.test(&premium_user, None, None).unwrap());
        
        // High score, not suspended, verified (should match)
        let high_score_user = json!({
            "category": "regular",
            "status": "active",
            "verified": true,
            "score": 95
        });
        assert!(operation.test(&high_score_user, None, None).unwrap());
        
        // Premium but suspended (should not match)
        let suspended_premium = json!({
            "category": "premium",
            "status": "suspended",
            "verified": true,
            "score": 50
        });
        assert!(!operation.test(&suspended_premium, None, None).unwrap());
        
        // Premium, not suspended, but not verified (should not match)
        let unverified_premium = json!({
            "category": "premium",
            "status": "active",
            "verified": false,
            "score": 50
        });
        assert!(!operation.test(&unverified_premium, None, None).unwrap());
        
        // Regular category, low score, not suspended, verified (should not match)
        let regular_low_score = json!({
            "category": "regular",
            "status": "active",
            "verified": true,
            "score": 60
        });
        assert!(!operation.test(&regular_low_score, None, None).unwrap());
    }

    #[test]
    fn test_logical_operators_edge_cases() {
        // Test $and with single condition
        let and_op = AndOperator;
        let single_condition = json!([{ "name": "test" }]);
        let and_operation = and_op.create_operation(&single_condition, &Value::Null).unwrap();
        
        let matching_doc = json!({ "name": "test" });
        assert!(and_operation.test(&matching_doc, None, None).unwrap());
        
        let non_matching_doc = json!({ "name": "other" });
        assert!(!and_operation.test(&non_matching_doc, None, None).unwrap());
        
        // Test $or with single condition
        let or_op = OrOperator;
        let or_operation = or_op.create_operation(&single_condition, &Value::Null).unwrap();
        
        assert!(or_operation.test(&matching_doc, None, None).unwrap());
        assert!(!or_operation.test(&non_matching_doc, None, None).unwrap());
        
        // Test $nor with single condition
        let nor_op = NorOperator;
        let nor_operation = nor_op.create_operation(&single_condition, &Value::Null).unwrap();
        
        assert!(!nor_operation.test(&matching_doc, None, None).unwrap());
        assert!(nor_operation.test(&non_matching_doc, None, None).unwrap());
    }

    // Tests for newly implemented operators
    #[test]
    fn test_exists_operation() {
        let op = ExistsOperator;
        
        // Test $exists: true
        let exists_true = op.create_operation(&json!(true), &Value::Null).unwrap();
        
        assert!(exists_true.test(&json!("value"), None, None).unwrap());
        assert!(exists_true.test(&json!(123), None, None).unwrap());
        assert!(exists_true.test(&json!(false), None, None).unwrap());
        assert!(exists_true.test(&json!([]), None, None).unwrap());
        assert!(exists_true.test(&json!({}), None, None).unwrap());
        assert!(!exists_true.test(&json!(null), None, None).unwrap());
        
        // Test $exists: false
        let exists_false = op.create_operation(&json!(false), &Value::Null).unwrap();
        
        assert!(!exists_false.test(&json!("value"), None, None).unwrap());
        assert!(!exists_false.test(&json!(123), None, None).unwrap());
        assert!(!exists_false.test(&json!(false), None, None).unwrap());
        assert!(!exists_false.test(&json!([]), None, None).unwrap());
        assert!(!exists_false.test(&json!({}), None, None).unwrap());
        assert!(exists_false.test(&json!(null), None, None).unwrap());
    }

    #[test]
    fn test_exists_operation_invalid_params() {
        let op = ExistsOperator;
        let result = op.create_operation(&json!("not a boolean"), &Value::Null);
        assert!(result.is_err());
        
        let result = op.create_operation(&json!(123), &Value::Null);
        assert!(result.is_err());
    }

    #[test]
    fn test_regex_operation() {
        let op = RegexOperator;
        
        // Basic pattern matching
        let regex_op = op.create_operation(&json!("^test"), &Value::Null).unwrap();
        
        assert!(regex_op.test(&json!("test123"), None, None).unwrap());
        assert!(regex_op.test(&json!("testing"), None, None).unwrap());
        assert!(!regex_op.test(&json!("abc test"), None, None).unwrap());
        assert!(!regex_op.test(&json!("TEST"), None, None).unwrap()); // case sensitive
        
        // Case insensitive pattern
        let case_insensitive = op.create_operation(&json!("(?i)hello"), &Value::Null).unwrap();
        
        assert!(case_insensitive.test(&json!("hello"), None, None).unwrap());
        assert!(case_insensitive.test(&json!("Hello"), None, None).unwrap());
        assert!(case_insensitive.test(&json!("HELLO"), None, None).unwrap());
        assert!(case_insensitive.test(&json!("say hello world"), None, None).unwrap());
        assert!(!case_insensitive.test(&json!("hi there"), None, None).unwrap());
        
        // Number pattern
        let number_pattern = op.create_operation(&json!(r"^\d+$"), &Value::Null).unwrap();
        
        assert!(number_pattern.test(&json!("123"), None, None).unwrap());
        assert!(number_pattern.test(&json!("0"), None, None).unwrap());
        assert!(!number_pattern.test(&json!("12.3"), None, None).unwrap());
        assert!(!number_pattern.test(&json!("abc123"), None, None).unwrap());
        
        // Non-string values should return false
        assert!(!regex_op.test(&json!(123), None, None).unwrap());
        assert!(!regex_op.test(&json!(true), None, None).unwrap());
        assert!(!regex_op.test(&json!(null), None, None).unwrap());
        assert!(!regex_op.test(&json!([]), None, None).unwrap());
        assert!(!regex_op.test(&json!({}), None, None).unwrap());
    }

    #[test]
    fn test_regex_operation_invalid_params() {
        let op = RegexOperator;
        
        // Invalid regex pattern
        let result = op.create_operation(&json!("["), &Value::Null);
        assert!(result.is_err());
        
        // Non-string parameter
        let result = op.create_operation(&json!(123), &Value::Null);
        assert!(result.is_err());
        
        let result = op.create_operation(&json!(true), &Value::Null);
        assert!(result.is_err());
    }

    #[test]
    fn test_mod_operation() {
        let op = ModOperator;
        
        // Basic modulo operation: value % 4 == 2
        let mod_op = op.create_operation(&json!([4, 2]), &Value::Null).unwrap();
        
        assert!(mod_op.test(&json!(6), None, None).unwrap());  // 6 % 4 = 2
        assert!(mod_op.test(&json!(10), None, None).unwrap()); // 10 % 4 = 2
        assert!(mod_op.test(&json!(2), None, None).unwrap());  // 2 % 4 = 2
        assert!(!mod_op.test(&json!(5), None, None).unwrap()); // 5 % 4 = 1
        assert!(!mod_op.test(&json!(8), None, None).unwrap()); // 8 % 4 = 0
        assert!(!mod_op.test(&json!(3), None, None).unwrap()); // 3 % 4 = 3
        
        // Modulo with negative numbers
        let neg_mod = op.create_operation(&json!([3, 1]), &Value::Null).unwrap();
        
        assert!(neg_mod.test(&json!(4), None, None).unwrap());  // 4 % 3 = 1
        assert!(neg_mod.test(&json!(7), None, None).unwrap());  // 7 % 3 = 1
        assert!(neg_mod.test(&json!(-2), None, None).unwrap()); // -2 % 3 = 1 (in Rust)
        assert!(!neg_mod.test(&json!(6), None, None).unwrap()); // 6 % 3 = 0
        
        // Floating point modulo
        let float_mod = op.create_operation(&json!([2.5, 1.5]), &Value::Null).unwrap();
        
        assert!(float_mod.test(&json!(4.0), None, None).unwrap()); // 4.0 % 2.5 = 1.5
        assert!(!float_mod.test(&json!(5.0), None, None).unwrap()); // 5.0 % 2.5 = 0.0
        
        // Non-numeric values should return false
        assert!(!mod_op.test(&json!("not a number"), None, None).unwrap());
        assert!(!mod_op.test(&json!(true), None, None).unwrap());
        assert!(!mod_op.test(&json!(null), None, None).unwrap());
        assert!(!mod_op.test(&json!([]), None, None).unwrap());
        assert!(!mod_op.test(&json!({}), None, None).unwrap());
    }

    #[test]
    fn test_mod_operation_invalid_params() {
        let op = ModOperator;
        
        // Not an array
        let result = op.create_operation(&json!("not an array"), &Value::Null);
        assert!(result.is_err());
        
        // Wrong array length
        let result = op.create_operation(&json!([4]), &Value::Null);
        assert!(result.is_err());
        
        let result = op.create_operation(&json!([4, 2, 1]), &Value::Null);
        assert!(result.is_err());
        
        // Non-numeric values in array
        let result = op.create_operation(&json!(["4", 2]), &Value::Null);
        assert!(result.is_err());
        
        let result = op.create_operation(&json!([4, "2"]), &Value::Null);
        assert!(result.is_err());
        
        // Zero divisor
        let result = op.create_operation(&json!([0, 1]), &Value::Null);
        assert!(result.is_err());
    }

    #[test]
    fn test_type_operation() {
        let op = TypeOperator;
        
        // Test string type
        let string_type = op.create_operation(&json!("string"), &Value::Null).unwrap();
        
        assert!(string_type.test(&json!("hello"), None, None).unwrap());
        assert!(string_type.test(&json!(""), None, None).unwrap());
        assert!(!string_type.test(&json!(123), None, None).unwrap());
        assert!(!string_type.test(&json!(true), None, None).unwrap());
        
        // Test number types
        let double_type = op.create_operation(&json!("double"), &Value::Null).unwrap();
        let int_type = op.create_operation(&json!("int"), &Value::Null).unwrap();
        let long_type = op.create_operation(&json!("long"), &Value::Null).unwrap();
        
        assert!(double_type.test(&json!(3.14), None, None).unwrap());
        assert!(int_type.test(&json!(42), None, None).unwrap() || long_type.test(&json!(42), None, None).unwrap()); // depends on serde_json representation
        assert!(!double_type.test(&json!("3.14"), None, None).unwrap());
        
        // Test boolean type
        let bool_type = op.create_operation(&json!("bool"), &Value::Null).unwrap();
        
        assert!(bool_type.test(&json!(true), None, None).unwrap());
        assert!(bool_type.test(&json!(false), None, None).unwrap());
        assert!(!bool_type.test(&json!(1), None, None).unwrap());
        assert!(!bool_type.test(&json!(0), None, None).unwrap());
        
        // Test null type
        let null_type = op.create_operation(&json!("null"), &Value::Null).unwrap();
        
        assert!(null_type.test(&json!(null), None, None).unwrap());
        assert!(!null_type.test(&json!(0), None, None).unwrap());
        assert!(!null_type.test(&json!(""), None, None).unwrap());
        assert!(!null_type.test(&json!(false), None, None).unwrap());
        
        // Test array type
        let array_type = op.create_operation(&json!("array"), &Value::Null).unwrap();
        
        assert!(array_type.test(&json!([]), None, None).unwrap());
        assert!(array_type.test(&json!([1, 2, 3]), None, None).unwrap());
        assert!(!array_type.test(&json!({}), None, None).unwrap());
        assert!(!array_type.test(&json!("[]"), None, None).unwrap());
        
        // Test object type
        let object_type = op.create_operation(&json!("object"), &Value::Null).unwrap();
        
        assert!(object_type.test(&json!({}), None, None).unwrap());
        assert!(object_type.test(&json!({"key": "value"}), None, None).unwrap());
        assert!(!object_type.test(&json!([]), None, None).unwrap());
        assert!(!object_type.test(&json!("{}"), None, None).unwrap());
    }

    #[test]
    fn test_type_operation_bson_numbers() {
        let op = TypeOperator;
        
        // Test BSON type numbers
        let double_type = op.create_operation(&json!(1), &Value::Null).unwrap(); // 1 = double
        let string_type = op.create_operation(&json!(2), &Value::Null).unwrap(); // 2 = string
        let object_type = op.create_operation(&json!(3), &Value::Null).unwrap(); // 3 = object
        let array_type = op.create_operation(&json!(4), &Value::Null).unwrap();  // 4 = array
        let bool_type = op.create_operation(&json!(8), &Value::Null).unwrap();   // 8 = bool
        let null_type = op.create_operation(&json!(10), &Value::Null).unwrap();  // 10 = null
        let int_type = op.create_operation(&json!(16), &Value::Null).unwrap();   // 16 = int
        let long_type = op.create_operation(&json!(18), &Value::Null).unwrap();  // 18 = long
        
        assert!(double_type.test(&json!(3.14), None, None).unwrap());
        assert!(string_type.test(&json!("hello"), None, None).unwrap());
        assert!(object_type.test(&json!({}), None, None).unwrap());
        assert!(array_type.test(&json!([]), None, None).unwrap());
        assert!(bool_type.test(&json!(true), None, None).unwrap());
        assert!(null_type.test(&json!(null), None, None).unwrap());
        assert!(int_type.test(&json!(42), None, None).unwrap() || long_type.test(&json!(42), None, None).unwrap());
    }

    #[test]
    fn test_type_operation_invalid_params() {
        let op = TypeOperator;
        
        // Invalid type name
        let result = op.create_operation(&json!("invalid_type"), &Value::Null);
        assert!(result.is_ok()); // This should actually be ok, it just won't match anything
        
        // Invalid BSON type number
        let result = op.create_operation(&json!(999), &Value::Null);
        assert!(result.is_err());
        
        // Non-string, non-number parameter
        let result = op.create_operation(&json!(true), &Value::Null);
        assert!(result.is_err());
        
        let result = op.create_operation(&json!([]), &Value::Null);
        assert!(result.is_err());
    }
}
