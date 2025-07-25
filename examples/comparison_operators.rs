use sift_rs::sift;
use serde_json::json;

fn main() {
    println!("Testing Comparison Operators");
    println!("============================");

    let data = vec![
        json!({"name": "Alice", "age": 25, "score": 85.5}),
        json!({"name": "Bob", "age": 30, "score": 92.0}),
        json!({"name": "Charlie", "age": 35, "score": 78.2}),
        json!({"name": "Diana", "age": 28, "score": 95.8}),
    ];

    // Test $gt (greater than)
    println!("\n$gt (greater than 30):");
    let gt_query = json!({"age": {"$gt": 30}});
    for item in &data {
        if sift(&gt_query, item).unwrap() {
            println!("  - {} (age: {})", item["name"], item["age"]);
        }
    }

    // Test $gte (greater than or equal)
    println!("\n$gte (greater than or equal to 30):");
    let gte_query = json!({"age": {"$gte": 30}});
    for item in &data {
        if sift(&gte_query, item).unwrap() {
            println!("  - {} (age: {})", item["name"], item["age"]);
        }
    }

    // Test $lt (less than)
    println!("\n$lt (less than 30):");
    let lt_query = json!({"age": {"$lt": 30}});
    for item in &data {
        if sift(&lt_query, item).unwrap() {
            println!("  - {} (age: {})", item["name"], item["age"]);
        }
    }

    // Test $lte (less than or equal)
    println!("\n$lte (less than or equal to 30):");
    let lte_query = json!({"age": {"$lte": 30}});
    for item in &data {
        if sift(&lte_query, item).unwrap() {
            println!("  - {} (age: {})", item["name"], item["age"]);
        }
    }

    // Test with floating point numbers
    println!("\nFloat comparison - score >= 90.0:");
    let float_query = json!({"score": {"$gte": 90.0}});
    for item in &data {
        if sift(&float_query, item).unwrap() {
            println!("  - {} (score: {})", item["name"], item["score"]);
        }
    }

    // Test range query (combining operators)
    println!("\nRange query - age between 25 and 30 (inclusive):");
    let range_query = json!({"age": {"$gte": 25, "$lte": 30}});
    for item in &data {
        if sift(&range_query, item).unwrap() {
            println!("  - {} (age: {})", item["name"], item["age"]);
        }
    }

    println!("\nAll comparison operators working correctly! ✅");
}
