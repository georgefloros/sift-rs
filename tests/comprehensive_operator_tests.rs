#[cfg(test)]
mod comprehensive_tests {
    use sift_rs::{sift, create_filter};
    use serde_json::json;

    // Sample data sets for testing
    fn sample_users() -> Vec<serde_json::Value> {
        vec![
            json!({
                "name": "Alice Johnson",
                "age": 28,
                "city": "New York",
                "status": "active",
                "tags": ["developer", "rust", "javascript"],
                "profile": {
                    "experience": 5,
                    "salary": 75000,
                    "rating": 4.5
                }
            }),
            json!({
                "name": "Bob Smith",
                "age": 34,
                "city": "San Francisco",
                "status": "active",
                "tags": ["designer", "ui", "ux"],
                "profile": {
                    "experience": 8,
                    "salary": 85000,
                    "rating": 4.2
                }
            }),
            json!({
                "name": "Charlie Brown",
                "age": 22,
                "city": "Austin",
                "status": "inactive",
                "tags": ["student", "python"],
                "profile": {
                    "experience": 1,
                    "salary": 45000,
                    "rating": 3.8
                }
            }),
            json!({
                "name": "Diana Prince",
                "age": 31,
                "city": "Seattle",
                "status": "active",
                "tags": ["manager", "leadership", "senior"],
                "profile": {
                    "experience": 10,
                    "salary": 95000,
                    "rating": 4.9
                }
            })
        ]
    }

    fn sample_products() -> Vec<serde_json::Value> {
        vec![
            json!({
                "id": "PROD-001",
                "name": "Gaming Laptop",
                "price": 1499.99,
                "category": "electronics",
                "brand": "TechCorp",
                "specs": {
                    "cpu": "Intel i7",
                    "ram": 16,
                    "storage": 512
                },
                "ratings": {
                    "average": 4.5,
                    "count": 127
                },
                "availability": {
                    "in_stock": true,
                    "quantity": 15
                },
                "tags": ["gaming", "high-performance", "portable"],
                "created_at": "2024-01-15T10:30:00Z"
            }),
            json!({
                "id": "PROD-002",
                "name": "Office Keyboard",
                "price": 89.99,
                "category": "accessories",
                "brand": "TypeCorp",
                "specs": {
                    "type": "mechanical",
                    "layout": "QWERTY",
                    "backlit": true
                },
                "ratings": {
                    "average": 4.1,
                    "count": 89
                },
                "availability": {
                    "in_stock": false,
                    "quantity": 0
                },
                "tags": ["office", "productivity"],
                "created_at": "2024-02-20T14:15:00Z"
            })
        ]
    }

    // Test 1: Basic Comparison Operators
    #[test]
    fn test_basic_comparison_operators() {
        let users = sample_users();

        // $eq - equality
        let eq_query = json!({"age": {"$eq": 28}});
        let eq_results: Vec<_> = users.iter()
            .filter(|user| sift(&eq_query, user).unwrap())
            .collect();
        assert_eq!(eq_results.len(), 1);
        assert_eq!(eq_results[0]["name"], "Alice Johnson");

        // $ne - not equal
        let ne_query = json!({"status": {"$ne": "active"}});
        let ne_results: Vec<_> = users.iter()
            .filter(|user| sift(&ne_query, user).unwrap())
            .collect();
        assert_eq!(ne_results.len(), 1);
        assert_eq!(ne_results[0]["name"], "Charlie Brown");

        // $gt - greater than
        let gt_query = json!({"age": {"$gt": 30}});
        let gt_results: Vec<_> = users.iter()
            .filter(|user| sift(&gt_query, user).unwrap())
            .collect();
        assert_eq!(gt_results.len(), 2); // Bob and Diana

        // $gte - greater than or equal
        let gte_query = json!({"age": {"$gte": 30}});
        let gte_results: Vec<_> = users.iter()
            .filter(|user| sift(&gte_query, user).unwrap())
            .collect();
        assert_eq!(gte_results.len(), 2); // Bob and Diana

        // $lt - less than
        let lt_query = json!({"age": {"$lt": 30}});
        let lt_results: Vec<_> = users.iter()
            .filter(|user| sift(&lt_query, user).unwrap())
            .collect();
        assert_eq!(lt_results.len(), 2); // Alice and Charlie

        // $lte - less than or equal
        let lte_query = json!({"age": {"$lte": 28}});
        let lte_results: Vec<_> = users.iter()
            .filter(|user| sift(&lte_query, user).unwrap())
            .collect();
        assert_eq!(lte_results.len(), 2); // Alice and Charlie
    }

    // Test 2: Array Operators
    #[test]
    fn test_array_operators() {
        let users = sample_users();

        // $in - value in array
        let in_query = json!({"city": {"$in": ["New York", "San Francisco", "Seattle"]}});
        let in_results: Vec<_> = users.iter()
            .filter(|user| sift(&in_query, user).unwrap())
            .collect();
        assert_eq!(in_results.len(), 3); // Alice, Bob, Diana

        // $nin - value not in array
        let nin_query = json!({"city": {"$nin": ["Austin"]}});
        let nin_results: Vec<_> = users.iter()
            .filter(|user| sift(&nin_query, user).unwrap())
            .collect();
        assert_eq!(nin_results.len(), 3); // Everyone except Charlie

        // $all - array contains all elements
        let all_query = json!({"tags": {"$all": ["developer", "rust"]}});
        let all_results: Vec<_> = users.iter()
            .filter(|user| sift(&all_query, user).unwrap())
            .collect();
        assert_eq!(all_results.len(), 1); // Only Alice

        // $size - array size
        let size_query = json!({"tags": {"$size": 3}});
        let size_results: Vec<_> = users.iter()
            .filter(|user| sift(&size_query, user).unwrap())
            .collect();
        assert_eq!(size_results.len(), 3); // Alice, Bob, and Diana all have 3 tags
    }

    // Test 3: Existence and Type Operators
    #[test]
    fn test_existence_and_type_operators() {
        let test_data = vec![
            json!({"name": "John", "age": 30, "phone": "+1234567890"}),
            json!({"name": "Jane", "age": 25}),
            json!({"name": "Bob", "age": "thirty", "active": true})
        ];

        // $exists true - field exists
        let exists_true_query = json!({"phone": {"$exists": true}});
        let exists_true_results: Vec<_> = test_data.iter()
            .filter(|item| sift(&exists_true_query, item).unwrap())
            .collect();
        assert_eq!(exists_true_results.len(), 1);

        // $exists false - field doesn't exist
        let exists_false_query = json!({"phone": {"$exists": false}});
        let exists_false_results: Vec<_> = test_data.iter()
            .filter(|item| sift(&exists_false_query, item).unwrap())
            .collect();
        assert_eq!(exists_false_results.len(), 2);

        // $type - field type check
        let type_number_query = json!({"age": {"$type": "number"}});
        let type_number_results: Vec<_> = test_data.iter()
            .filter(|item| sift(&type_number_query, item).unwrap())
            .collect();
        assert_eq!(type_number_results.len(), 2); // John and Jane

        let type_string_query = json!({"age": {"$type": "string"}});
        let type_string_results: Vec<_> = test_data.iter()
            .filter(|item| sift(&type_string_query, item).unwrap())
            .collect();
        assert_eq!(type_string_results.len(), 1); // Bob
    }

    // Test 4: Regular Expression Operations
    #[test]
    fn test_regex_operations() {
        let users = sample_users();

        // Email validation pattern
        let test_data = vec![
            json!({"email": "user@example.com", "name": "Valid User"}),
            json!({"email": "invalid-email", "name": "Invalid User"}),
            json!({"email": "another@test.org", "name": "Another Valid"})
        ];

        let email_regex = json!({"email": {"$regex": "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"}});
        let valid_emails: Vec<_> = test_data.iter()
            .filter(|item| sift(&email_regex, item).unwrap())
            .collect();
        assert_eq!(valid_emails.len(), 2);

        // Name pattern matching
        let name_pattern_query = json!({"name": {"$regex": "^[AB]"}});
        let name_results: Vec<_> = users.iter()
            .filter(|user| sift(&name_pattern_query, user).unwrap())
            .collect();
        assert_eq!(name_results.len(), 2); // Alice and Bob
    }

    // Test 5: Mathematical Operations
    #[test]
    fn test_mathematical_operations() {
        let test_data = vec![
            json!({"id": 12, "name": "Even ID"}),
            json!({"id": 13, "name": "Odd ID"}),
            json!({"id": 20, "name": "Another Even"}),
            json!({"id": 15, "name": "Another Odd"})
        ];

        // $mod - modulo operation for even numbers
        let even_query = json!({"id": {"$mod": [2, 0]}});
        let even_results: Vec<_> = test_data.iter()
            .filter(|item| sift(&even_query, item).unwrap())
            .collect();
        assert_eq!(even_results.len(), 2);

        // $mod - modulo operation for odd numbers
        let odd_query = json!({"id": {"$mod": [2, 1]}});
        let odd_results: Vec<_> = test_data.iter()
            .filter(|item| sift(&odd_query, item).unwrap())
            .collect();
        assert_eq!(odd_results.len(), 2);
    }

    // Test 6: Date Comparisons
    #[test]
    fn test_date_comparisons() {
        let documents = vec![
            json!({
                "created_at": "2024-01-15T10:30:00Z",
                "name": "Document A"
            }),
            json!({
                "created_at": "2024-03-15T10:30:00Z",
                "name": "Recent Document"
            }),
            json!({
                "created_at": "2023-12-31T23:59:59Z",
                "name": "Old Document"
            })
        ];

        // Date equality
        let date_eq_query = json!({"created_at": "2024-01-15T10:30:00Z"});
        let date_eq_results: Vec<_> = documents.iter()
            .filter(|doc| sift(&date_eq_query, doc).unwrap())
            .collect();
        assert_eq!(date_eq_results.len(), 1);

        // Date range (after a specific date)
        let date_after_query = json!({"created_at": {"$gte": "2024-01-01T00:00:00Z"}});
        let date_after_results: Vec<_> = documents.iter()
            .filter(|doc| sift(&date_after_query, doc).unwrap())
            .collect();
        assert_eq!(date_after_results.len(), 2);

        // Date range (before a specific date)
        let date_before_query = json!({"created_at": {"$lt": "2024-01-01T00:00:00Z"}});
        let date_before_results: Vec<_> = documents.iter()
            .filter(|doc| sift(&date_before_query, doc).unwrap())
            .collect();
        assert_eq!(date_before_results.len(), 1);
    }

    // Test 7: Logical Operators
    #[test]
    fn test_logical_operators() {
        let users = sample_users();

        // $and - all conditions must be true
        let and_query = json!({
            "$and": [
                {"age": {"$gte": 25}},
                {"status": "active"},
                {"profile.salary": {"$gt": 80000}}
            ]
        });
        let and_results: Vec<_> = users.iter()
            .filter(|user| sift(&and_query, user).unwrap())
            .collect();
        assert_eq!(and_results.len(), 2); // Bob and Diana

        // $or - at least one condition must be true
        let or_query = json!({
            "$or": [
                {"age": {"$lt": 25}},
                {"status": "inactive"},
                {"profile.salary": {"$gte": 90000}}
            ]
        });
        let or_results: Vec<_> = users.iter()
            .filter(|user| sift(&or_query, user).unwrap())
            .collect();
        assert_eq!(or_results.len(), 2); // Charlie and Diana

        // $not - condition must not be true
        let not_query = json!({"status": {"$not": {"$eq": "active"}}});
        let not_results: Vec<_> = users.iter()
            .filter(|user| sift(&not_query, user).unwrap())
            .collect();
        assert_eq!(not_results.len(), 1); // Charlie

        // $nor - none of the conditions should be true
        let nor_query = json!({
            "$nor": [
                {"status": "inactive"},
                {"age": {"$lt": 25}}
            ]
        });
        let nor_results: Vec<_> = users.iter()
            .filter(|user| sift(&nor_query, user).unwrap())
            .collect();
        assert_eq!(nor_results.len(), 3); // Alice, Bob, Diana
    }

    // Test 8: Element Match for Arrays of Objects
    #[test]
    fn test_element_match() {
        let products_with_reviews = vec![
            json!({
                "name": "Product A",
                "reviews": [
                    {"rating": 4, "author": "Alice", "verified": true},
                    {"rating": 5, "author": "Bob", "verified": false},
                    {"rating": 3, "author": "Charlie", "verified": true}
                ]
            }),
            json!({
                "name": "Product B",
                "reviews": [
                    {"rating": 2, "author": "David", "verified": true},
                    {"rating": 3, "author": "Eve", "verified": false}
                ]
            })
        ];

        // $elemMatch - array contains object matching criteria
        let elem_match_query = json!({
            "reviews": {
                "$elemMatch": {
                    "rating": {"$gte": 4},
                    "verified": true
                }
            }
        });
        let elem_match_results: Vec<_> = products_with_reviews.iter()
            .filter(|product| sift(&elem_match_query, product).unwrap())
            .collect();
        assert_eq!(elem_match_results.len(), 1); // Only Product A

        // Complex $elemMatch with nested conditions
        let complex_data = vec![
            json!({
                "name": "Order #123",
                "items": [
                    {"product": "Laptop", "price": 1200, "category": "electronics", "warranty": true},
                    {"product": "Mouse", "price": 25, "category": "electronics", "warranty": false},
                    {"product": "Book", "price": 15, "category": "books", "warranty": false}
                ]
            }),
            json!({
                "name": "Order #456",
                "items": [
                    {"product": "Tablet", "price": 800, "category": "electronics", "warranty": true},
                    {"product": "Case", "price": 50, "category": "accessories", "warranty": false}
                ]
            })
        ];

        let complex_elem_match_query = json!({
            "items": {
                "$elemMatch": {
                    "$and": [
                        {"price": {"$gte": 1000}},
                        {"category": "electronics"},
                        {"warranty": true}
                    ]
                }
            }
        });
        let complex_results: Vec<_> = complex_data.iter()
            .filter(|order| sift(&complex_elem_match_query, order).unwrap())
            .collect();
        assert_eq!(complex_results.len(), 1); // Only Order #123
    }

    // Test 9: $where Operator Tests
    #[test]
    fn test_where_operator() {
        // Simple property comparison
        let simple_data = json!({"a": 10, "b": 5});
        let simple_query = json!({"$where": "this.a > this.b"});
        assert!(sift(&simple_query, &simple_data).unwrap());

        // Array length condition
        let array_data = json!({"items": ["a", "b", "c", "d"], "count": 4});
        let array_query = json!({"$where": "this.items.length === this.count"});
        assert!(sift(&array_query, &array_data).unwrap());

        // Array includes condition
        let includes_data = json!({"tags": ["red", "sweet", "fresh"], "target": "sweet"});
        let includes_query = json!({"$where": "this.tags.includes('sweet')"});
        assert!(sift(&includes_query, &includes_data).unwrap());

        // Complex business logic
        let order_data = json!({
            "user": {
                "profile": {
                    "level": 5,
                    "experience": 12500
                },
                "achievements": ["first_win", "expert", "marathon"]
            }
        });
        let complex_query = json!({
            "$where": "this.user.profile.experience >= this.user.profile.level && this.user.achievements.includes('expert')"
        });
        assert!(sift(&complex_query, &order_data).unwrap());

        // Property-to-property comparison with mathematical operations
        let math_data = json!({
            "product": {
                "base_price": 100,
                "tax_amount": 10,
                "final_price": 110
            }
        });
        let math_query = json!({
            "$where": "this.product.base_price + this.product.tax_amount === this.product.final_price"
        });
        assert!(sift(&math_query, &math_data).unwrap());
    }

    // Test 10: Complex Nested Objects
    #[test]
    fn test_complex_nested_objects() {
        let products = sample_products();

        // Multi-level nested query
        let complex_query = json!({
            "$and": [
                {"category": "electronics"},
                {"price": {"$gte": 1000, "$lte": 2000}},
                {"specs.ram": {"$gte": 16}},
                {"ratings.average": {"$gte": 4.0}},
                {"availability.in_stock": true},
                {"tags": {"$in": ["gaming", "professional"]}}
            ]
        });
        let complex_results: Vec<_> = products.iter()
            .filter(|product| sift(&complex_query, product).unwrap())
            .collect();
        assert_eq!(complex_results.len(), 1); // Gaming Laptop

        // Nested field access with multiple conditions
        let nested_query = json!({
            "specs.cpu": "Intel i7",
            "ratings.count": {"$gte": 100}
        });
        let nested_results: Vec<_> = products.iter()
            .filter(|product| sift(&nested_query, product).unwrap())
            .collect();
        assert_eq!(nested_results.len(), 1);
    }

    // Test 11: Range Queries
    #[test]
    fn test_range_queries() {
        let users = sample_users();

        // Age range
        let age_range_query = json!({"age": {"$gte": 25, "$lte": 35}});
        let age_range_results: Vec<_> = users.iter()
            .filter(|user| sift(&age_range_query, user).unwrap())
            .collect();
        assert_eq!(age_range_results.len(), 3); // Alice, Bob, Diana

        // Salary range with nested field
        let salary_range_query = json!({"profile.salary": {"$gte": 70000, "$lt": 90000}});
        let salary_range_results: Vec<_> = users.iter()
            .filter(|user| sift(&salary_range_query, user).unwrap())
            .collect();
        assert_eq!(salary_range_results.len(), 2); // Alice and Bob

        // Multiple range conditions
        let multi_range_query = json!({
            "age": {"$gte": 25, "$lt": 35},
            "profile.experience": {"$gte": 5, "$lte": 10}
        });
        let multi_range_results: Vec<_> = users.iter()
            .filter(|user| sift(&multi_range_query, user).unwrap())
            .collect();
        assert_eq!(multi_range_results.len(), 3); // Alice, Bob, and Diana
    }

    // Test 12: create_filter Function
    #[test]
    fn test_create_filter_function() {
        let users = sample_users();

        // Create reusable filters
        let senior_filter = create_filter(&json!({"age": {"$gte": 30}})).unwrap();
        let active_filter = create_filter(&json!({"status": "active"})).unwrap();
        let high_salary_filter = create_filter(&json!({"profile.salary": {"$gte": 80000}})).unwrap();

        let senior_count = users.iter().filter(|user| senior_filter(user)).count();
        let active_count = users.iter().filter(|user| active_filter(user)).count();
        let high_salary_count = users.iter().filter(|user| high_salary_filter(user)).count();

        assert_eq!(senior_count, 2); // Bob and Diana
        assert_eq!(active_count, 3); // Alice, Bob, Diana
        assert_eq!(high_salary_count, 2); // Bob and Diana

        // Combine filters
        let combined_count = users.iter()
            .filter(|user| active_filter(user) && high_salary_filter(user))
            .count();
        assert_eq!(combined_count, 2); // Bob and Diana
    }

    // Test 13: Edge Cases and Error Handling
    #[test]
    fn test_edge_cases() {
        // Empty arrays
        let empty_array_data = json!({"tags": [], "count": 0});
        let empty_array_query = json!({"tags": {"$size": 0}});
        assert!(sift(&empty_array_query, &empty_array_data).unwrap());

        // Null values
        let null_data = json!({"name": "John", "middle_name": null, "age": 30});
        let null_query = json!({"middle_name": null});
        assert!(sift(&null_query, &null_data).unwrap());

        // Non-existent fields
        let missing_field_query = json!({"non_existent_field": {"$exists": false}});
        assert!(sift(&missing_field_query, &null_data).unwrap());

        // Mixed data types
        let mixed_data = json!({"values": [1, "two", 3.0, true, null]});
        let mixed_query = json!({"values": {"$size": 5}});
        assert!(sift(&mixed_query, &mixed_data).unwrap());
    }

    // Test 14: Float/Decimal Precision
    #[test]
    fn test_float_precision() {
        let users = sample_users();

        // Test floating point comparisons
        let float_query = json!({"profile.rating": {"$gte": 4.5}});
        let float_results: Vec<_> = users.iter()
            .filter(|user| sift(&float_query, user).unwrap())
            .collect();
        assert_eq!(float_results.len(), 2); // Alice and Diana

        // Test exact float matching
        let exact_float_query = json!({"profile.rating": 4.2});
        let exact_float_results: Vec<_> = users.iter()
            .filter(|user| sift(&exact_float_query, user).unwrap())
            .collect();
        assert_eq!(exact_float_results.len(), 1); // Bob
    }

    // Test 15: String Operations
    #[test]
    fn test_string_operations() {
        let users = sample_users();

        // Case sensitive string matching
        let case_query = json!({"name": "Alice Johnson"});
        let case_results: Vec<_> = users.iter()
            .filter(|user| sift(&case_query, user).unwrap())
            .collect();
        assert_eq!(case_results.len(), 1);

        // String inequality
        let string_ne_query = json!({"city": {"$ne": "Austin"}});
        let string_ne_results: Vec<_> = users.iter()
            .filter(|user| sift(&string_ne_query, user).unwrap())
            .collect();
        assert_eq!(string_ne_results.len(), 3); // Everyone except Charlie
    }
}
