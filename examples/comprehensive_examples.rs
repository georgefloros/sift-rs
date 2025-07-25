use serde_json::json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Comprehensive Sift-rs API Examples ===\n");

    let client = reqwest::Client::new();
    let base_url = "http://localhost:3000";

    // Health check first
    println!("🔍 Health Check");
    println!("================");
    let health_response = client.get(&format!("{}/health", base_url)).send().await?;
    if !health_response.status().is_success() {
        println!("❌ Server not available. Please start the server first with: cargo run");
        return Ok(());
    }
    println!("✅ Server is running\n");

    // Test 1: Basic Comparison Operators
    println!("1️⃣  Basic Comparison Operators");
    println!("==============================");

    let basic_comparisons = json!([
        // $eq (equality)
        {
            "input": {"age": 25, "name": "Alice"},
            "query": {"age": {"$eq": 25}}
        },
        // $ne (not equal)
        {
            "input": {"age": 30, "name": "Bob"},
            "query": {"age": {"$ne": 25}}
        },
        // $gt (greater than)
        {
            "input": {"score": 85, "name": "Charlie"},
            "query": {"score": {"$gt": 80}}
        },
        // $gte (greater than or equal)
        {
            "input": {"score": 80, "name": "Diana"},
            "query": {"score": {"$gte": 80}}
        },
        // $lt (less than)
        {
            "input": {"age": 20, "name": "Eve"},
            "query": {"age": {"$lt": 25}}
        },
        // $lte (less than or equal)
        {
            "input": {"age": 25, "name": "Frank"},
            "query": {"age": {"$lte": 25}}
        }
    ]);

    let response = client
        .post(&format!("{}/validate", base_url))
        .json(&basic_comparisons)
        .send()
        .await?;
    print_results("Basic Comparisons", response).await?;

    // Test 2: Array Operators
    println!("\n2️⃣  Array Operators");
    println!("===================");

    let array_operators = json!([
        // $in (value in array)
        {
            "input": {"category": "fruits", "name": "Apple"},
            "query": {"category": {"$in": ["fruits", "vegetables", "grains"]}}
        },
        // $nin (value not in array)
        {
            "input": {"category": "meat", "name": "Beef"},
            "query": {"category": {"$nin": ["fruits", "vegetables"]}}
        },
        // $all (array contains all elements)
        {
            "input": {"tags": ["red", "sweet", "fresh"], "name": "Strawberry"},
            "query": {"tags": {"$all": ["red", "sweet"]}}
        },
        // $size (array size)
        {
            "input": {"items": ["laptop", "mouse", "keyboard"], "total": 3},
            "query": {"items": {"$size": 3}}
        }
    ]);

    let response = client
        .post(&format!("{}/validate", base_url))
        .json(&array_operators)
        .send()
        .await?;
    print_results("Array Operators", response).await?;

    // Test 3: Existence and Type Operators
    println!("\n3️⃣  Existence and Type Operators");
    println!("================================");

    let existence_type = json!([
        // $exists (field exists)
        {
            "input": {"name": "John", "age": 30, "phone": "+1234567890"},
            "query": {"phone": {"$exists": true}}
        },
        // $exists false (field doesn't exist)
        {
            "input": {"name": "Jane", "age": 25},
            "query": {"phone": {"$exists": false}}
        },
        // $type (field type check)
        {
            "input": {"age": 30, "name": "Alice"},
            "query": {"age": {"$type": "number"}}
        },
        {
            "input": {"age": "thirty", "name": "Bob"},
            "query": {"age": {"$type": "string"}}
        }
    ]);

    let response = client
        .post(&format!("{}/validate", base_url))
        .json(&existence_type)
        .send()
        .await?;
    print_results("Existence and Type", response).await?;

    // Test 4: Regular Expression and String Operations
    println!("\n4️⃣  Regular Expression and String Operations");
    println!("===========================================");

    let regex_operations = json!([
        // Email validation
        {
            "input": {"email": "user@example.com", "name": "Valid User"},
            "query": {"email": {"$regex": "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"}}
        },
        // Invalid email
        {
            "input": {"email": "invalid-email", "name": "Invalid User"},
            "query": {"email": {"$regex": "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"}}
        },
        // Phone number pattern
        {
            "input": {"phone": "+1-555-123-4567", "name": "US User"},
            "query": {"phone": {"$regex": "^\\+1-\\d{3}-\\d{3}-\\d{4}$"}}
        },
        // Case insensitive search
        {
            "input": {"name": "Alice Johnson", "role": "Developer"},
            "query": {"name": {"$regex": "alice", "$options": "i"}}
        }
    ]);

    let response = client
        .post(&format!("{}/validate", base_url))
        .json(&regex_operations)
        .send()
        .await?;
    print_results("Regex Operations", response).await?;

    // Test 5: Mathematical and Modulo Operations
    println!("\n5️⃣  Mathematical and Modulo Operations");
    println!("=====================================");

    let math_operations = json!([
        // $mod (modulo operation)
        {
            "input": {"id": 12, "name": "Even ID"},
            "query": {"id": {"$mod": [2, 0]}}  // Even numbers
        },
        {
            "input": {"id": 13, "name": "Odd ID"},
            "query": {"id": {"$mod": [2, 1]}}  // Odd numbers
        },
        // Multiple conditions on numbers
        {
            "input": {"score": 85, "attempts": 3},
            "query": {
                "score": {"$gte": 80, "$lte": 90},
                "attempts": {"$lte": 5}
            }
        }
    ]);

    let response = client
        .post(&format!("{}/validate", base_url))
        .json(&math_operations)
        .send()
        .await?;
    print_results("Math Operations", response).await?;

    // Test 6: Date Comparisons
    println!("\n6️⃣  Date Comparisons");
    println!("====================");

    let date_operations = json!([
        // Date equality
        {
            "input": {
                "created_at": "2024-01-15T10:30:00Z",
                "name": "Document A"
            },
            "query": {"created_at": "2024-01-15T10:30:00Z"}
        },
        // Date range (after a specific date)
        {
            "input": {
                "created_at": "2024-03-15T10:30:00Z",
                "name": "Recent Document"
            },
            "query": {"created_at": {"$gte": "2024-01-01T00:00:00Z"}}
        },
        // Date range (between dates)
        {
            "input": {
                "created_at": "2024-02-15T10:30:00Z",
                "updated_at": "2024-02-20T15:45:00Z",
                "name": "February Document"
            },
            "query": {
                "$and": [
                    {"created_at": {"$gte": "2024-02-01T00:00:00Z"}},
                    {"created_at": {"$lt": "2024-03-01T00:00:00Z"}}
                ]
            }
        },
        // Complex date logic
        {
            "input": {
                "birth_date": "1990-05-15T00:00:00Z",
                "registration_date": "2024-01-01T12:00:00Z",
                "name": "User Profile"
            },
            "query": {
                "$and": [
                    {"birth_date": {"$lt": "2000-01-01T00:00:00Z"}},  // Born before 2000
                    {"registration_date": {"$gte": "2024-01-01T00:00:00Z"}}  // Registered in 2024
                ]
            }
        }
    ]);

    let response = client
        .post(&format!("{}/validate", base_url))
        .json(&date_operations)
        .send()
        .await?;
    print_results("Date Operations", response).await?;

    // Test 7: Logical Operators
    println!("\n7️⃣  Logical Operators");
    println!("=====================");

    let logical_operations = json!([
        // $and
        {
            "input": {"age": 28, "status": "active", "score": 85},
            "query": {
                "$and": [
                    {"age": {"$gte": 25}},
                    {"status": "active"},
                    {"score": {"$gt": 80}}
                ]
            }
        },
        // $or
        {
            "input": {"age": 20, "status": "premium", "score": 95},
            "query": {
                "$or": [
                    {"age": {"$gte": 25}},
                    {"status": "premium"},
                    {"score": {"$gte": 90}}
                ]
            }
        },
        // $not
        {
            "input": {"status": "active", "banned": false},
            "query": {"status": {"$not": {"$eq": "banned"}}}
        },
        // $nor (none of the conditions are true)
        {
            "input": {"status": "active", "age": 30},
            "query": {
                "$nor": [
                    {"status": "banned"},
                    {"status": "suspended"},
                    {"age": {"$lt": 18}}
                ]
            }
        }
    ]);

    let response = client
        .post(&format!("{}/validate", base_url))
        .json(&logical_operations)
        .send()
        .await?;
    print_results("Logical Operations", response).await?;

    // Test 8: Element Match for Arrays of Objects
    println!("\n8️⃣  Element Match for Arrays of Objects");
    println!("======================================");

    let element_match = json!([
        // $elemMatch - array contains object matching criteria
        {
            "input": {
                "name": "Product A",
                "reviews": [
                    {"rating": 4, "author": "Alice", "verified": true},
                    {"rating": 5, "author": "Bob", "verified": false},
                    {"rating": 3, "author": "Charlie", "verified": true}
                ]
            },
            "query": {
                "reviews": {
                    "$elemMatch": {
                        "rating": {"$gte": 4},
                        "verified": true
                    }
                }
            }
        },
        // Complex $elemMatch with nested conditions
        {
            "input": {
                "name": "Order #123",
                "items": [
                    {"product": "Laptop", "price": 1200, "category": "electronics", "warranty": true},
                    {"product": "Mouse", "price": 25, "category": "electronics", "warranty": false},
                    {"product": "Book", "price": 15, "category": "books", "warranty": false}
                ]
            },
            "query": {
                "items": {
                    "$elemMatch": {
                        "$and": [
                            {"price": {"$gte": 1000}},
                            {"category": "electronics"},
                            {"warranty": true}
                        ]
                    }
                }
            }
        }
    ]);

    let response = client
        .post(&format!("{}/validate", base_url))
        .json(&element_match)
        .send()
        .await?;
    print_results("Element Match", response).await?;

    // Test 9: Medium Complexity Objects and Queries
    println!("\n9️⃣  Medium Complexity Objects and Queries");
    println!("=========================================");

    let medium_complexity = json!([
        // E-commerce product with multiple criteria
        {
            "input": {
                "id": "PROD-001",
                "name": "Gaming Laptop",
                "price": 1499.99,
                "category": "electronics",
                "subcategory": "computers",
                "brand": "TechCorp",
                "specs": {
                    "cpu": "Intel i7",
                    "ram": 16,
                    "storage": 512,
                    "gpu": "RTX 3070"
                },
                "ratings": {
                    "average": 4.5,
                    "count": 127
                },
                "availability": {
                    "in_stock": true,
                    "quantity": 15,
                    "warehouse": "US-WEST"
                },
                "tags": ["gaming", "high-performance", "portable"],
                "created_at": "2024-01-15T10:30:00Z"
            },
            "query": {
                "$and": [
                    {"category": "electronics"},
                    {"price": {"$gte": 1000, "$lte": 2000}},
                    {"specs.ram": {"$gte": 16}},
                    {"ratings.average": {"$gte": 4.0}},
                    {"availability.in_stock": true},
                    {"tags": {"$in": ["gaming", "professional"]}}
                ]
            }
        },
        // User profile with complex matching
        {
            "input": {
                "id": "USER-456",
                "profile": {
                    "name": "Alice Johnson",
                    "age": 29,
                    "location": {
                        "country": "USA",
                        "state": "CA",
                        "city": "San Francisco"
                    },
                    "preferences": {
                        "newsletter": true,
                        "notifications": {
                            "email": true,
                            "sms": false,
                            "push": true
                        }
                    }
                },
                "subscription": {
                    "tier": "premium",
                    "expires_at": "2024-12-31T23:59:59Z",
                    "features": ["advanced_analytics", "priority_support", "api_access"]
                },
                "activity": {
                    "last_login": "2024-01-20T14:30:00Z",
                    "total_sessions": 156,
                    "total_time_hours": 89.5
                }
            },
            "query": {
                "$and": [
                    {"profile.age": {"$gte": 25, "$lte": 35}},
                    {"profile.location.country": "USA"},
                    {"subscription.tier": {"$in": ["premium", "enterprise"]}},
                    {"subscription.features": {"$all": ["api_access"]}},
                    {"activity.total_sessions": {"$gte": 100}}
                ]
            }
        }
    ]);

    let response = client
        .post(&format!("{}/validate", base_url))
        .json(&medium_complexity)
        .send()
        .await?;
    print_results("Medium Complexity", response).await?;

    // Test 10: High Complexity Objects and Queries
    println!("\n🔟 High Complexity Objects and Queries");
    println!("======================================");

    let high_complexity = json!([
        // Complex business data with nested arrays and objects
        {
            "input": {
                "company": {
                    "id": "COMP-789",
                    "name": "TechInnovate Inc.",
                    "industry": "software",
                    "founded": "2015-03-10T00:00:00Z",
                    "headquarters": {
                        "address": {
                            "street": "123 Tech St",
                            "city": "San Francisco",
                            "state": "CA",
                            "country": "USA",
                            "postal_code": "94105"
                        },
                        "coordinates": {
                            "lat": 37.7749,
                            "lng": -122.4194
                        }
                    },
                    "employees": [
                        {
                            "id": "EMP-001",
                            "name": "John Doe",
                            "role": "CEO",
                            "department": "executive",
                            "salary": 250000,
                            "start_date": "2015-03-10T00:00:00Z",
                            "skills": ["leadership", "strategy", "fundraising"],
                            "performance": {
                                "rating": 4.8,
                                "reviews": 12,
                                "goals_met": 0.95
                            }
                        },
                        {
                            "id": "EMP-002",
                            "name": "Jane Smith",
                            "role": "CTO",
                            "department": "engineering",
                            "salary": 220000,
                            "start_date": "2015-06-15T00:00:00Z",
                            "skills": ["rust", "typescript", "architecture", "leadership"],
                            "performance": {
                                "rating": 4.9,
                                "reviews": 15,
                                "goals_met": 0.98
                            }
                        },
                        {
                            "id": "EMP-003",
                            "name": "Bob Wilson",
                            "role": "Senior Developer",
                            "department": "engineering",
                            "salary": 180000,
                            "start_date": "2018-01-20T00:00:00Z",
                            "skills": ["rust", "python", "databases"],
                            "performance": {
                                "rating": 4.6,
                                "reviews": 8,
                                "goals_met": 0.88
                            }
                        }
                    ],
                    "projects": [
                        {
                            "id": "PROJ-001",
                            "name": "AI Platform",
                            "status": "active",
                            "budget": 2500000,
                            "start_date": "2023-01-01T00:00:00Z",
                            "end_date": "2024-06-30T00:00:00Z",
                            "team_size": 15,
                            "technologies": ["rust", "python", "tensorflow", "kubernetes"]
                        },
                        {
                            "id": "PROJ-002",
                            "name": "Mobile App",
                            "status": "completed",
                            "budget": 800000,
                            "start_date": "2023-03-01T00:00:00Z",
                            "end_date": "2023-12-31T00:00:00Z",
                            "team_size": 8,
                            "technologies": ["react-native", "typescript", "firebase"]
                        }
                    ],
                    "financials": {
                        "revenue": {
                            "2023": 15000000,
                            "2022": 12000000,
                            "2021": 8000000
                        },
                        "funding_rounds": [
                            {
                                "round": "Series C",
                                "amount": 50000000,
                                "date": "2023-05-15T00:00:00Z",
                                "investors": ["VentureCapital Corp", "TechFund Partners"]
                            }
                        ]
                    }
                }
            },
            "query": {
                "$and": [
                    {"company.industry": "software"},
                    {"company.employees": {
                        "$elemMatch": {
                            "$and": [
                                {"department": "engineering"},
                                {"salary": {"$gte": 200000}},
                                {"skills": {"$in": ["rust", "leadership"]}},
                                {"performance.rating": {"$gte": 4.5}}
                            ]
                        }
                    }},
                    {"company.projects": {
                        "$elemMatch": {
                            "$and": [
                                {"status": "active"},
                                {"budget": {"$gte": 1000000}},
                                {"technologies": {"$all": ["rust"]}}
                            ]
                        }
                    }},
                    {"company.financials.revenue.2023": {"$gte": 10000000}}
                ]
            }
        }
    ]);

    let response = client
        .post(&format!("{}/validate", base_url))
        .json(&high_complexity)
        .send()
        .await?;
    print_results("High Complexity", response).await?;

    // Test 11: $where Examples with JavaScript-like Expressions
    println!("\n1️⃣1️⃣ $where Examples (JavaScript-like Expressions)");
    println!("=================================================");

    let where_examples = json!([
        // Simple $where condition
        {
            "input": {"a": 10, "b": 5},
            "query": {"$where": "this.a > this.b"}
        },
        // Complex mathematical condition
        {
            "input": {"x": 15, "y": 20, "z": 25},
            "query": {"$where": "this.x + this.y === this.z + 10"}
        },
        // String manipulation
        {
            "input": {"first_name": "John", "last_name": "Doe", "full_name": "John Doe"},
            "query": {"$where": "this.first_name + ' ' + this.last_name === this.full_name"}
        },
        // Array length condition
        {
            "input": {"items": ["a", "b", "c", "d"], "count": 4},
            "query": {"$where": "this.items.length === this.count"}
        },
        // Complex business logic
        {
            "input": {
                "order": {
                    "subtotal": 100,
                    "tax_rate": 0.08,
                    "discount": 10,
                    "total": 98
                }
            },
            "query": {"$where": "(this.order.subtotal * (1 + this.order.tax_rate)) - this.order.discount === this.order.total"}
        },
        // Date-based calculation
        {
            "input": {
                "birth_year": 1990,
                "current_year": 2024,
                "is_adult": true
            },
            "query": {"$where": "this.current_year - this.birth_year >= 18 && this.is_adult"}
        },
        // Advanced array operations
        {
            "input": {
                "scores": [85, 92, 78, 96, 88],
                "average": 87.8,
                "passed": true
            },
            "query": {"$where": "this.scores.reduce((sum, score) => sum + score, 0) / this.scores.length >= 80 && this.passed"}
        },
        // Nested object validation
        {
            "input": {
                "user": {
                    "profile": {
                        "level": 5,
                        "experience": 12500
                    },
                    "achievements": ["first_win", "expert", "marathon"]
                }
            },
            "query": {"$where": "this.user.profile.experience >= this.user.profile.level * 2000 && this.user.achievements.includes('expert')"}
        },
        // Complex validation with multiple conditions
        {
            "input": {
                "product": {
                    "price": 299.99,
                    "discount_percentage": 15,
                    "final_price": 254.99,
                    "category": "electronics",
                    "rating": 4.5,
                    "reviews_count": 150
                }
            },
            "query": {"$where": "Math.round(this.product.price * (1 - this.product.discount_percentage / 100) * 100) / 100 === this.product.final_price && this.product.rating >= 4.0 && this.product.reviews_count >= 100"}
        },
        // Financial calculation validation
        {
            "input": {
                "loan": {
                    "principal": 100000,
                    "annual_rate": 0.05,
                    "years": 30,
                    "monthly_payment": 536.82
                }
            },
            "query": {"$where": "Math.round((this.loan.principal * (this.loan.annual_rate / 12) * Math.pow(1 + this.loan.annual_rate / 12, this.loan.years * 12)) / (Math.pow(1 + this.loan.annual_rate / 12, this.loan.years * 12) - 1) * 100) / 100 === this.loan.monthly_payment"}
        }
    ]);

    let response = client
        .post(&format!("{}/validate", base_url))
        .json(&where_examples)
        .send()
        .await?;
    print_results("$where Examples", response).await?;

    println!("\n🎉 All comprehensive examples completed!");
    println!("=======================================");
    println!("This demonstration covered:");
    println!("✅ All basic comparison operators ($eq, $ne, $gt, $gte, $lt, $lte)");
    println!("✅ Array operators ($in, $nin, $all, $size)");
    println!("✅ Existence and type checking ($exists, $type)");
    println!("✅ Regular expressions with various patterns");
    println!("✅ Mathematical operations ($mod)");
    println!("✅ Date comparisons and ranges");
    println!("✅ Logical operators ($and, $or, $not, $nor)");
    println!("✅ Element matching for arrays of objects ($elemMatch)");
    println!("✅ Medium complexity nested objects and queries");
    println!("✅ High complexity business data structures");
    println!("✅ Complex $where expressions with JavaScript-like logic");

    Ok(())
}

async fn print_results(
    test_name: &str,
    response: reqwest::Response,
) -> Result<(), Box<dyn Error>> {
    if response.status().is_success() {
        let results: serde_json::Value = response.json().await?;
        println!("✅ {}: SUCCESS", test_name);

        // Count true/false results
        if let serde_json::Value::Array(arr) = &results {
            let true_count = arr
                .iter()
                .filter(|v| v.get("valid").and_then(|v| v.as_bool()).unwrap_or(false))
                .count();
            let false_count = arr.len() - true_count;
            println!(
                "   📊 Results: {} ✅ true, {} ❌ false",
                true_count, false_count
            );

            // Show individual results
            for (i, result) in arr.iter().enumerate() {
                if let Some(valid) = result.get("valid").and_then(|v| v.as_bool()) {
                    println!("   {}: {}", i + 1, if valid { "✅" } else { "❌" });
                }
            }
        }
    } else {
        let error_text = response.text().await?;
        println!("❌ {}: FAILED - {}", test_name, error_text);
    }
    Ok(())
}
