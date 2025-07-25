use sift_rs::sift;
use serde_json::json;

#[test]
fn debug_where_math_test() {
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
    
    println!("Testing data: {:#}", math_data);
    println!("Testing query: {:#}", math_query);
    
    match sift(&math_query, &math_data) {
        Ok(result) => {
            println!("Sift result: {}", result);
            assert!(result);
        }
        Err(e) => {
            println!("Sift error: {}", e);
            panic!("Sift returned an error: {}", e);
        }
    }
}

#[test]
fn debug_simple_where_test() {
    let simple_data = json!({"a": 10, "b": 5});
    let simple_query = json!({"$where": "this.a > this.b"});
    
    println!("Simple test data: {:#}", simple_data);
    println!("Simple test query: {:#}", simple_query);
    
    match sift(&simple_query, &simple_data) {
        Ok(result) => {
            println!("Simple sift result: {}", result);
            assert!(result);
        }
        Err(e) => {
            println!("Simple sift error: {}", e);
            panic!("Simple sift returned an error: {}", e);
        }
    }
}
