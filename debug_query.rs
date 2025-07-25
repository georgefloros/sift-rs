use serde_json::json;
use std::fs;

fn main() {
    // Load the library - assuming it's built as a library
    let test_data = json!({
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
    
    println!("Testing data: {}", serde_json::to_string_pretty(&test_data).unwrap());
    println!("Query: {}", serde_json::to_string_pretty(&query).unwrap());
    
    // Let's manually trace what should happen:
    // 1. this.user.profile.experience should be 12500
    // 2. this.user.profile.level should be 5  
    // 3. 12500 >= 5 should be true
    // 4. this.user.achievements should be ["first_win", "expert", "marathon"]
    // 5. this.user.achievements.includes('expert') should be true
    // 6. true && true should be true
    
    println!("\n--- Manual verification ---");
    println!("user.profile.experience: {}", test_data["user"]["profile"]["experience"]);
    println!("user.profile.level: {}", test_data["user"]["profile"]["level"]);
    println!("12500 >= 5: {}", 12500 >= 5);
    println!("user.achievements: {}", test_data["user"]["achievements"]);
    
    // Check if "expert" is in achievements
    let achievements = test_data["user"]["achievements"].as_array().unwrap();
    let has_expert = achievements.iter().any(|v| v.as_str() == Some("expert"));
    println!("achievements includes 'expert': {}", has_expert);
    println!("Expected result: {}", true && has_expert);
}
