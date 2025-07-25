#[cfg(test)]
mod where_debug_tests {
    use sift_rs::sift;
    use serde_json::json;

    #[test]
    fn test_simple_query_first() {
        // First test with a simple operator to see debug output
        let data = json!({"age": 25});
        let query = json!({"age": {"$gt": 18}});
        
        println!("Testing simple query first:");
        let result = sift(&query, &data).unwrap();
        println!("Simple query result: {}", result);
    }

    #[test] 
    fn test_where_registry() {
        // Check if $where is in the registry
        use sift_rs::core::QueryContext;
        let context = QueryContext::new();
        
        let where_op = context.registry.get("$where");
        println!("$where operator found in registry: {}", where_op.is_some());
        
        if let Some(op) = where_op {
            println!("$where operator name: {}", op.name());
        }
    }

    #[test]
    fn test_where_operator_debug() {
        let data = json!({
            "user": {
                "profile": {
                    "level": 5,
                    "experience": 12500
                },
                "achievements": ["first_win", "expert", "marathon"]
            }
        });

        let query = json!({
            "$where": "this.user.profile.experience >= this.user.profile.level && this.user.achievements.includes('expert')"
        });

        println!("Data: {}", serde_json::to_string_pretty(&data).unwrap());
        println!("Query: {}", serde_json::to_string_pretty(&query).unwrap());

        let result = sift(&query, &data).unwrap();
        println!("Result: {}", result);
        
        // Let's break down the query into parts
        let query1 = json!({
            "$where": "this.user.profile.experience >= this.user.profile.level"
        });
        let result1 = sift(&query1, &data).unwrap();
        println!("Part 1 (experience >= level): {}", result1);

        let query2 = json!({
            "$where": "this.user.achievements.includes('expert')"
        });
        let result2 = sift(&query2, &data).unwrap();
        println!("Part 2 (achievements includes expert): {}", result2);

        // Test individual property access
        let query3 = json!({
            "$where": "this.user.profile.experience > 10000"
        });
        let result3 = sift(&query3, &data).unwrap();
        println!("Part 3 (experience > 10000): {}", result3);

        // Test direct property access without operators
        let query4 = json!({
            "$where": "this.user.profile.level == 5"
        });
        let result4 = sift(&query4, &data).unwrap();
        println!("Part 4 (level == 5): {}", result4);

        // Let's debug what the get_nested_value function returns
        use sift_rs::core::utils;
        let level_value = utils::get_nested_value(&data, "user.profile.level");
        println!("Direct property access - user.profile.level: {:?}", level_value);
        
        let exp_value = utils::get_nested_value(&data, "user.profile.experience");
        println!("Direct property access - user.profile.experience: {:?}", exp_value);
        
        let achievements_value = utils::get_nested_value(&data, "user.achievements");
        println!("Direct property access - user.achievements: {:?}", achievements_value);
        
        // assert!(result, "The combined query should return true");
    }
}
