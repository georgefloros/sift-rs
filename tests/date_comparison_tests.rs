use sift_rs::{sift, create_filter};
use serde_json::json;

#[cfg(test)]
mod date_comparison_tests {
    use super::*;

    #[test]
    fn test_date_greater_than() {
        let query = json!({
            "date": { "$gt": "2023-01-01T00:00:00Z" }
        });
        
        let data1 = json!({
            "date": "2023-06-15T12:30:00Z"
        });
        
        let data2 = json!({
            "date": "2022-12-31T23:59:59Z"
        });
        
        assert!(sift(&query, &data1).unwrap());
        assert!(!sift(&query, &data2).unwrap());
    }

    #[test]
    fn test_date_greater_than_or_equal() {
        let query = json!({
            "date": { "$gte": "2023-01-01T00:00:00Z" }
        });
        
        let data1 = json!({
            "date": "2023-01-01T00:00:00Z"
        });
        
        let data2 = json!({
            "date": "2023-06-15T12:30:00Z"
        });
        
        let data3 = json!({
            "date": "2022-12-31T23:59:59Z"
        });
        
        assert!(sift(&query, &data1).unwrap());
        assert!(sift(&query, &data2).unwrap());
        assert!(!sift(&query, &data3).unwrap());
    }

    #[test]
    fn test_date_less_than() {
        let query = json!({
            "date": { "$lt": "2023-06-01T00:00:00Z" }
        });
        
        let data1 = json!({
            "date": "2023-01-15T10:30:00Z"
        });
        
        let data2 = json!({
            "date": "2023-07-01T12:00:00Z"
        });
        
        assert!(sift(&query, &data1).unwrap());
        assert!(!sift(&query, &data2).unwrap());
    }

    #[test]
    fn test_date_less_than_or_equal() {
        let query = json!({
            "date": { "$lte": "2023-06-01T00:00:00Z" }
        });
        
        let data1 = json!({
            "date": "2023-06-01T00:00:00Z"
        });
        
        let data2 = json!({
            "date": "2023-01-15T10:30:00Z"
        });
        
        let data3 = json!({
            "date": "2023-07-01T12:00:00Z"
        });
        
        assert!(sift(&query, &data1).unwrap());
        assert!(sift(&query, &data2).unwrap());
        assert!(!sift(&query, &data3).unwrap());
    }

    #[test]
    fn test_date_range_query() {
        let query = json!({
            "date": {
                "$gte": "2023-01-01T00:00:00Z",
                "$lt": "2024-01-01T00:00:00Z"
            }
        });
        
        let data1 = json!({
            "date": "2023-06-15T12:30:00Z"
        });
        
        let data2 = json!({
            "date": "2022-12-31T23:59:59Z"
        });
        
        let data3 = json!({
            "date": "2024-01-01T00:00:00Z"
        });
        
        assert!(sift(&query, &data1).unwrap());
        assert!(!sift(&query, &data2).unwrap());
        assert!(!sift(&query, &data3).unwrap());
    }

    #[test]
    fn test_mixed_numeric_and_date_comparisons() {
        // Test that numeric comparisons still work
        let numeric_query = json!({
            "value": { "$gt": 10 }
        });
        
        let numeric_data = json!({
            "value": 15
        });
        
        assert!(sift(&numeric_query, &numeric_data).unwrap());
        
        // Test date comparisons in the same test
        let date_query = json!({
            "timestamp": { "$gt": "2023-01-01T00:00:00Z" }
        });
        
        let date_data = json!({
            "timestamp": "2023-06-15T12:30:00Z"
        });
        
        assert!(sift(&date_query, &date_data).unwrap());
    }

    #[test]
    fn test_create_filter_with_dates() {
        let query = json!({
            "created_at": { "$gte": "2023-01-01T00:00:00Z" }
        });
        
        let filter = create_filter(&query).unwrap();
        
        let data = vec![
            json!({ "created_at": "2023-06-15T12:30:00Z", "name": "record1" }),
            json!({ "created_at": "2022-11-20T10:00:00Z", "name": "record2" }),
            json!({ "created_at": "2023-03-10T08:45:00Z", "name": "record3" }),
        ];
        
        let results: Vec<_> = data.iter().filter(|&item| filter(item)).collect();
        
        assert_eq!(results.len(), 2);
        assert_eq!(results[0]["name"], "record1");
        assert_eq!(results[1]["name"], "record3");
    }

    #[test]
    fn test_invalid_date_fallback_to_string_comparison() {
        let query = json!({
            "date": { "$gt": "not-a-date" }
        });
        
        let data = json!({
            "date": "z-string-that-comes-after"
        });
        
        // Should fall back to string comparison
        assert!(sift(&query, &data).unwrap());
    }

    #[test]
    fn test_different_date_formats() {
        // Test various ISO8601 formats
        let queries_and_data = vec![
            // With timezone offset
            ("2023-01-01T00:00:00+00:00", "2023-06-15T12:30:00+00:00"),
            // With different timezone
            ("2023-01-01T00:00:00-05:00", "2023-06-15T12:30:00-05:00"),
            // Date only (should parse with default time)
            ("2023-01-01", "2023-06-15"),
        ];
        
        for (threshold, test_date) in queries_and_data {
            let query = json!({
                "date": { "$gt": threshold }
            });
            
            let data = json!({
                "date": test_date
            });
            
            assert!(sift(&query, &data).unwrap(), 
                "Failed for threshold: {} and test_date: {}", threshold, test_date);
        }
    }
}
