use sift_rs::sift;
use serde_json::json;

fn main() {
    // Test the specific failing case
    let array_data = json!({"items": ["a", "b", "c", "d"], "count": 4});
    let array_query = json!({"$where": "this.items.length === this.count"});
    
    println!("Data: {}", serde_json::to_string_pretty(&array_data).unwrap());
    println!("Query: {}", serde_json::to_string_pretty(&array_query).unwrap());
    
    let result = sift(&array_query, &array_data).unwrap();
    println!("Result: {}", result);
    
    // Let's test a simpler length query
    let simple_query = json!({"$where": "this.items.length === 4"});
    let simple_result = sift(&simple_query, &array_data).unwrap();
    println!("Simple result: {}", simple_result);
}
