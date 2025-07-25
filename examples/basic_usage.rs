use sift_rs::{sift, create_filter};
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Sift-rs Example Usage ===\n");

    // Sample data - a collection of user records
    let users = vec![
        json!({
            "name": "Alice Johnson",
            "age": 28,
            "city": "New York",
            "status": "active",
            "tags": ["developer", "rust", "javascript"],
            "profile": {
                "experience": 5,
                "salary": 75000
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
                "salary": 85000
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
                "salary": 45000
            }
        }),
        json!({
            "name": "Diana Prince",
            "age": 31,
            "city": "Seattle",
            "status": "active",
            "tags": ["manager", "leadership"],
            "profile": {
                "experience": 10,
                "salary": 95000
            }
        })
    ];

    // Example 1: Simple equality queries
    println!("1. Simple Equality Queries");
    println!("==========================");
    
    let active_users_query = json!({"status": "active"});
    let active_users: Vec<_> = users.iter()
        .filter(|user| sift(&active_users_query, user).unwrap())
        .collect();
    
    println!("Active users count: {}", active_users.len());
    for user in &active_users {
        println!("  - {}", user["name"].as_str().unwrap());
    }

    // Example 2: Comparison operators
    println!("\n2. Comparison Operators");
    println!("=======================");
    
    let senior_users_query = json!({"age": {"$gte": 30}});
    let senior_users: Vec<_> = users.iter()
        .filter(|user| sift(&senior_users_query, user).unwrap())
        .collect();
    
    println!("Users 30 or older:");
    for user in &senior_users {
        println!("  - {} (age: {})", 
                 user["name"].as_str().unwrap(), 
                 user["age"].as_u64().unwrap());
    }

    // Example 3: Array operations ($in)
    println!("\n3. Array Operations ($in)");
    println!("=========================");
    
    let tech_cities_query = json!({"city": {"$in": ["New York", "San Francisco", "Seattle"]}});
    let tech_city_users: Vec<_> = users.iter()
        .filter(|user| sift(&tech_cities_query, user).unwrap())
        .collect();
    
    println!("Users in tech cities:");
    for user in &tech_city_users {
        println!("  - {} from {}", 
                 user["name"].as_str().unwrap(), 
                 user["city"].as_str().unwrap());
    }

    // Example 4: Nested field queries
    println!("\n4. Nested Field Queries");
    println!("=======================");
    
    let high_salary_query = json!({"profile.salary": {"$gt": 80000}});
    let high_earners: Vec<_> = users.iter()
        .filter(|user| sift(&high_salary_query, user).unwrap())
        .collect();
    
    println!("High earners (>$80,000):");
    for user in &high_earners {
        println!("  - {} earns ${}", 
                 user["name"].as_str().unwrap(), 
                 user["profile"]["salary"].as_u64().unwrap());
    }

    // Example 5: Logical operators ($and)
    println!("\n5. Logical Operators ($and)");
    println!("============================");
    
    let experienced_active_query = json!({
        "$and": [
            {"status": "active"},
            {"profile.experience": {"$gte": 5}}
        ]
    });
    
    let experienced_active: Vec<_> = users.iter()
        .filter(|user| sift(&experienced_active_query, user).unwrap())
        .collect();
    
    println!("Experienced active users:");
    for user in &experienced_active {
        println!("  - {} ({} years experience)", 
                 user["name"].as_str().unwrap(), 
                 user["profile"]["experience"].as_u64().unwrap());
    }

    // Example 6: Array element matching
    println!("\n6. Array Element Matching");
    println!("=========================");
    
    let developers_query = json!({"tags": {"$in": ["developer", "rust"]}});
    let developers: Vec<_> = users.iter()
        .filter(|user| sift(&developers_query, user).unwrap())
        .collect();
    
    println!("Users with developer or rust tags:");
    for user in &developers {
        println!("  - {} has tags: {:?}", 
                 user["name"].as_str().unwrap(), 
                 user["tags"].as_array().unwrap());
    }

    // Example 7: Complex query with multiple conditions
    println!("\n7. Complex Multi-Condition Query");
    println!("================================");
    
    let complex_query = json!({
        "status": "active",
        "age": {"$gte": 25, "$lt": 35},
        "profile.salary": {"$gte": 70000},
        "city": {"$nin": ["Austin"]}
    });
    
    let filtered_users: Vec<_> = users.iter()
        .filter(|user| sift(&complex_query, user).unwrap())
        .collect();
    
    println!("Users matching complex criteria:");
    println!("(active, age 25-34, salary ≥$70k, not in Austin)");
    for user in &filtered_users {
        println!("  - {} (age: {}, salary: ${}, city: {})", 
                 user["name"].as_str().unwrap(),
                 user["age"].as_u64().unwrap(),
                 user["profile"]["salary"].as_u64().unwrap(),
                 user["city"].as_str().unwrap());
    }

    // Example 8: Using create_filter for reusable filters
    println!("\n8. Reusable Filters");
    println!("===================");
    
    let senior_filter = create_filter(&json!({"age": {"$gte": 30}}))?;
    let active_filter = create_filter(&json!({"status": "active"}))?;
    
    let senior_count = users.iter().filter(|user| senior_filter(user)).count();
    let active_count = users.iter().filter(|user| active_filter(user)).count();
    
    println!("Senior users (≥30): {}", senior_count);
    println!("Active users: {}", active_count);

    // Example 9: Regular expressions
    println!("\n9. Regular Expression Matching");
    println!("==============================");
    
    let name_pattern_query = json!({"name": {"$regex": "^[AB]"}});
    let users_ab: Vec<_> = users.iter()
        .filter(|user| sift(&name_pattern_query, user).unwrap())
        .collect();
    
    println!("Users whose names start with A or B:");
    for user in &users_ab {
        println!("  - {}", user["name"].as_str().unwrap());
    }

    // Example 10: Existence checks
    println!("\n10. Field Existence Checks");
    println!("==========================");
    
    let has_profile_query = json!({"profile": {"$exists": true}});
    let users_with_profile: Vec<_> = users.iter()
        .filter(|user| sift(&has_profile_query, user).unwrap())
        .collect();
    
    println!("Users with profile field: {}", users_with_profile.len());

    println!("\n=== End of Examples ===");
    Ok(())
}
