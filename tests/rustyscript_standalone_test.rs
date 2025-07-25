use rustyscript::{Runtime, RuntimeOptions};

#[test]
fn test_rustyscript_math_evaluation() {
    // Test 1: Simple math - check for floating point precision issue
    let mut runtime1 = Runtime::new(RuntimeOptions::default()).unwrap();
    let simple_math = "100 * (1 + 0.1)";
    println!("Testing calculation: {}", simple_math);
    let result1 = runtime1.eval::<serde_json::Value>(simple_math).unwrap();
    println!("Calculation result: {:?}", result1);
    
    let comparison = "100 * (1 + 0.1) === 110";
    println!("Testing comparison: {}", comparison);
    let result1b = runtime1.eval::<serde_json::Value>(comparison).unwrap();
    println!("Comparison result: {:?}", result1b);
    
    // Test 2: With object context
    let mut runtime2 = Runtime::new(RuntimeOptions::default()).unwrap();
    let obj_test = r#"
        const obj = {"base_price": 100, "tax_rate": 0.1, "final_price": 110};
        const calculated = obj.base_price * (1 + obj.tax_rate);
        console.log("Calculated:", calculated, "Expected:", obj.final_price);
        calculated === obj.final_price
    "#;
    println!("Testing with object: {}", obj_test);
    let result2 = runtime2.eval::<serde_json::Value>(obj_test).unwrap();
    println!("Object result: {:?}", result2);
    
    // Test 3: Using function call with this context (similar to our implementation)
    let mut runtime3 = Runtime::new(RuntimeOptions::default()).unwrap();
    let function_test = r#"
        const thisObj = {"product":{"base_price":100,"tax_rate":0.1,"final_price":110}};
        (function() { 
            const calculated = this.product.base_price * (1 + this.product.tax_rate);
            console.log("Function calculated:", calculated, "Expected:", this.product.final_price);
            return calculated === this.product.final_price;
        }).call(thisObj);
    "#;
    println!("Testing with function call: {}", function_test);
    let result3 = runtime3.eval::<serde_json::Value>(function_test).unwrap();
    println!("Function result: {:?}", result3);
    
    // Test 4: Check for floating point precision issues with tolerance
    let mut runtime4 = Runtime::new(RuntimeOptions::default()).unwrap();
    let precision_test = r#"
        const thisObj = {"product":{"base_price":100,"tax_rate":0.1,"final_price":110}};
        const calculated = thisObj.product.base_price * (1 + thisObj.product.tax_rate);
        const diff = Math.abs(calculated - thisObj.product.final_price);
        console.log("Calculated:", calculated, "Expected:", thisObj.product.final_price, "Diff:", diff);
        diff < 0.0001;
    "#;
    println!("Testing with precision tolerance: {}", precision_test);
    let result4 = runtime4.eval::<serde_json::Value>(precision_test).unwrap();
    println!("Precision result: {:?}", result4);
}
