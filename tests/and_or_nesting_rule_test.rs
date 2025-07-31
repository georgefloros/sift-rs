use sift_rs::Query;
use serde_json::json;

#[cfg(test)]
mod and_or_nesting_tests {
    use super::*;

    #[test]
    fn test_or_at_top_level_when_both_and_or_present() {
        // Test case: query with both $and and $or should nest $and inside $or
        let query_json = json!({
            "$and": [
                {"name": "Alice"},
                {"age": {"$gte": 18}}
            ],
            "$or": [
                {"status": "active"},
                {"priority": "high"}
            ]
        });
        
        let query = Query::from_value(&query_json).unwrap();
        
        // Test with a document that matches $or condition (status: active) 
        // but doesn't match all $and conditions (name is different)
        let test_doc_or_match = json!({
            "name": "Bob",  // Doesn't match $and requirement (name: Alice)
            "age": 25,      // Matches $and requirement (age >= 18)
            "status": "active",  // Matches $or requirement
            "priority": "low"
        });
        
        // Since $or is at top level, this should match because status is "active"
        let result = query.test(&test_doc_or_match).unwrap();
        assert!(result, "Document should match because $or is at top level and status is 'active'");
        
        // Test with a document that matches nested $and conditions
        let test_doc_and_match = json!({
            "name": "Alice",  // Matches $and requirement
            "age": 25,        // Matches $and requirement
            "status": "inactive",  // Doesn't match $or direct conditions
            "priority": "low"      // Doesn't match $or direct conditions
        });
        
        // This should match because the nested $and conditions are satisfied
        let result2 = query.test(&test_doc_and_match).unwrap();
        assert!(result2, "Document should match because nested $and conditions are satisfied");
        
        // Test with a document that matches neither $or nor nested $and
        let test_doc_no_match = json!({
            "name": "Carol",      // Doesn't match $and requirement (name: Alice)
            "age": 16,            // Doesn't match $and requirement (age >= 18)
            "status": "inactive", // Doesn't match $or requirement
            "priority": "low"     // Doesn't match $or requirement
        });
        
        // This should not match
        let result3 = query.test(&test_doc_no_match).unwrap();
        assert!(!result3, "Document should not match because it satisfies neither $or nor nested $and conditions");
    }

    #[test]
    fn test_only_and_conditions() {
        // Test case: query with only $and should work normally
        let query_json = json!({
            "$and": [
                {"name": "Alice"},
                {"age": {"$gte": 18}}
            ]
        });
        
        let query = Query::from_value(&query_json).unwrap();
        
        let test_doc = json!({
            "name": "Alice",
            "age": 25
        });
        
        let result = query.test(&test_doc).unwrap();
        assert!(result, "Document should match $and conditions");
        
        let test_doc2 = json!({
            "name": "Bob",
            "age": 25
        });
        
        let result2 = query.test(&test_doc2).unwrap();
        assert!(!result2, "Document should not match $and conditions");
    }

    #[test]
    fn test_only_or_conditions() {
        // Test case: query with only $or should work normally
        let query_json = json!({
            "$or": [
                {"status": "active"},
                {"priority": "high"}
            ]
        });
        
        let query = Query::from_value(&query_json).unwrap();
        
        let test_doc = json!({
            "status": "active",
            "priority": "low"
        });
        
        let result = query.test(&test_doc).unwrap();
        assert!(result, "Document should match $or conditions");
        
        let test_doc2 = json!({
            "status": "inactive",
            "priority": "low"
        });
        
        let result2 = query.test(&test_doc2).unwrap();
        assert!(!result2, "Document should not match $or conditions");
    }

    #[test]
    fn test_complex_nested_case() {
        // Test a more complex case with additional fields
        let query_json = json!({
            "$and": [
                {"department": "Engineering"},
                {"experience": {"$gte": 2}}
            ],
            "$or": [
                {"role": "Senior"},
                {"salary": {"$gte": 80000}}
            ],
            "active": true  // Additional field outside logical operators
        });
        
        let query = Query::from_value(&query_json).unwrap();
        
        // Should match because role is "Senior" (satisfies $or) and active is true
        let test_doc = json!({
            "department": "Marketing",  // Doesn't match nested $and
            "experience": 1,            // Doesn't match nested $and
            "role": "Senior",           // Matches $or
            "salary": 70000,
            "active": true
        });
        
        let result = query.test(&test_doc).unwrap();
        assert!(result, "Document should match because it satisfies $or condition and active field");
    }
}
