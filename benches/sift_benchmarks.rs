use criterion::{criterion_group, criterion_main, Criterion, black_box};
use sift_rs::{sift, create_filter};
use serde_json::json;

// Generate test data based on high complexity examples
fn generate_test_data() -> Vec<serde_json::Value> {
    vec![
        // High complexity business data
        json!({
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
                        },
                        "email": "john.doe@techinnovate.com",
                        "age": 45
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
                        },
                        "email": "jane.smith@techinnovate.com",
                        "age": 38
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
                        },
                        "age": 32
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
        }),
    ]
}

fn benchmark_where_operations(c: &mut Criterion) {
    let data = generate_test_data();

    let mut group = c.benchmark_group("$where Operations");

    // Sample $where query
    let where_query = json!({ "$where": "this.company.employees.length > 1" });
    group.bench_function("$where logic", |b| {
        b.iter(|| {
            let count = data.iter()
                .filter(|item| sift(black_box(&where_query), black_box(item)).unwrap_or(false))
                .count();
            black_box(count)
        })
    });

    group.finish();
}

fn benchmark_basic_comparisons(c: &mut Criterion) {
    let data = generate_test_data();
    
    let mut group = c.benchmark_group("Basic Comparisons");
    
    // Test $eq operator
    let eq_query = json!({"company.employees.0.age": {"$eq": 45}});
    group.bench_function("$eq operator", |b| {
        b.iter(|| {
            let count = data.iter()
                .filter(|item| sift(black_box(&eq_query), black_box(item)).unwrap_or(false))
                .count();
            black_box(count)
        })
    });
    
    // Test $ne operator
    let ne_query = json!({"company.industry": {"$ne": "healthcare"}});
    group.bench_function("$ne operator", |b| {
        b.iter(|| {
            let count = data.iter()
                .filter(|item| sift(black_box(&ne_query), black_box(item)).unwrap_or(false))
                .count();
            black_box(count)
        })
    });
    
    // Test $gt operator
    let gt_query = json!({"company.financials.revenue.2023": {"$gt": 10000000}});
    group.bench_function("$gt operator", |b| {
        b.iter(|| {
            let count = data.iter()
                .filter(|item| sift(black_box(&gt_query), black_box(item)).unwrap_or(false))
                .count();
            black_box(count)
        })
    });
    
    // Test $gte operator
    let gte_query = json!({"company.employees.0.salary": {"$gte": 200000}});
    group.bench_function("$gte operator", |b| {
        b.iter(|| {
            let count = data.iter()
                .filter(|item| sift(black_box(&gte_query), black_box(item)).unwrap_or(false))
                .count();
            black_box(count)
        })
    });
    
    // Test $lt operator
    let lt_query = json!({"company.employees.2.age": {"$lt": 40}});
    group.bench_function("$lt operator", |b| {
        b.iter(|| {
            let count = data.iter()
                .filter(|item| sift(black_box(&lt_query), black_box(item)).unwrap_or(false))
                .count();
            black_box(count)
        })
    });
    
    // Test $lte operator
    let lte_query = json!({"company.projects.0.budget": {"$lte": 3000000}});
    group.bench_function("$lte operator", |b| {
        b.iter(|| {
            let count = data.iter()
                .filter(|item| sift(black_box(&lte_query), black_box(item)).unwrap_or(false))
                .count();
            black_box(count)
        })
    });
    
    group.finish();
}

fn benchmark_array_operations(c: &mut Criterion) {
    let data = generate_test_data();
    
    let mut group = c.benchmark_group("Array Operations");
    
    // Test $in operator
    let in_query = json!({"company.projects.0.status": {"$in": ["active", "pending", "completed"]}});
    group.bench_function("$in operator", |b| {
        b.iter(|| {
            let count = data.iter()
                .filter(|item| sift(black_box(&in_query), black_box(item)).unwrap_or(false))
                .count();
            black_box(count)
        })
    });
    
    // Test $nin operator
    let nin_query = json!({"company.projects.0.status": {"$nin": ["cancelled", "suspended"]}});
    group.bench_function("$nin operator", |b| {
        b.iter(|| {
            let count = data.iter()
                .filter(|item| sift(black_box(&nin_query), black_box(item)).unwrap_or(false))
                .count();
            black_box(count)
        })
    });
    
    // Test $all operator
    let all_query = json!({"company.employees.0.skills": {"$all": ["leadership", "strategy"]}});
    group.bench_function("$all operator", |b| {
        b.iter(|| {
            let count = data.iter()
                .filter(|item| sift(black_box(&all_query), black_box(item)).unwrap_or(false))
                .count();
            black_box(count)
        })
    });
    
    // Test $size operator
    let size_query = json!({"company.employees": {"$size": 3}});
    group.bench_function("$size operator", |b| {
        b.iter(|| {
            let count = data.iter()
                .filter(|item| sift(black_box(&size_query), black_box(item)).unwrap_or(false))
                .count();
            black_box(count)
        })
    });
    
    group.finish();
}

fn benchmark_logical_operations(c: &mut Criterion) {
    let data = generate_test_data();
    
    let mut group = c.benchmark_group("Logical Operations");
    
    // Test $and operator
    let and_query = json!({
        "$and": [
            {"company.industry": "software"},
            {"company.employees.0.age": {"$gte": 40}}
        ]
    });
    group.bench_function("$and operator", |b| {
        b.iter(|| {
            let count = data.iter()
                .filter(|item| sift(black_box(&and_query), black_box(item)).unwrap_or(false))
                .count();
            black_box(count)
        })
    });
    
    // Test $or operator
    let or_query = json!({
        "$or": [
            {"company.employees.0.age": {"$gte": 50}},
            {"company.projects.0.status": "active"}
        ]
    });
    group.bench_function("$or operator", |b| {
        b.iter(|| {
            let count = data.iter()
                .filter(|item| sift(black_box(&or_query), black_box(item)).unwrap_or(false))
                .count();
            black_box(count)
        })
    });
    
    // Test $not operator
    let not_query = json!({"company.industry": {"$not": {"$eq": "healthcare"}}});
    group.bench_function("$not operator", |b| {
        b.iter(|| {
            let count = data.iter()
                .filter(|item| sift(black_box(&not_query), black_box(item)).unwrap_or(false))
                .count();
            black_box(count)
        })
    });
    
    // Test $nor operator
    let nor_query = json!({
        "$nor": [
            {"company.industry": "healthcare"},
            {"company.employees.0.age": {"$lt": 30}}
        ]
    });
    group.bench_function("$nor operator", |b| {
        b.iter(|| {
            let count = data.iter()
                .filter(|item| sift(black_box(&nor_query), black_box(item)).unwrap_or(false))
                .count();
            black_box(count)
        })
    });
    
    group.finish();
}

fn benchmark_field_operations(c: &mut Criterion) {
    let data = generate_test_data();
    
    let mut group = c.benchmark_group("Field Operations");
    
    // Test $exists operator
    let exists_query = json!({"company.employees.0.email": {"$exists": true}});
    group.bench_function("$exists operator", |b| {
        b.iter(|| {
            let count = data.iter()
                .filter(|item| sift(black_box(&exists_query), black_box(item)).unwrap_or(false))
                .count();
            black_box(count)
        })
    });
    
    // Test $type operator
    let type_query = json!({"company.employees.0.age": {"$type": "number"}});
    group.bench_function("$type operator", |b| {
        b.iter(|| {
            let count = data.iter()
                .filter(|item| sift(black_box(&type_query), black_box(item)).unwrap_or(false))
                .count();
            black_box(count)
        })
    });
    
    // Test $regex operator
    let regex_query = json!({"company.employees.0.email": {"$regex": "@techinnovate\\.com$"}});
    group.bench_function("$regex operator", |b| {
        b.iter(|| {
            let count = data.iter()
                .filter(|item| sift(black_box(&regex_query), black_box(item)).unwrap_or(false))
                .count();
            black_box(count)
        })
    });
    
    // Test $mod operator
    let mod_query = json!({"company.employees.0.age": {"$mod": [5, 0]}});
    group.bench_function("$mod operator", |b| {
        b.iter(|| {
            let count = data.iter()
                .filter(|item| sift(black_box(&mod_query), black_box(item)).unwrap_or(false))
                .count();
            black_box(count)
        })
    });
    
    group.finish();
}

fn benchmark_complex_queries(c: &mut Criterion) {
    let data = generate_test_data();
    
    let mut group = c.benchmark_group("Complex Queries");
    
    // Test nested object query
    let nested_query = json!({
        "$and": [
            {"category": "electronics"},
            {"price": {"$gte": 1000, "$lte": 2000}},
            {"specs.ram": {"$gte": 16}},
            {"ratings.average": {"$gte": 4.0}},
            {"availability.in_stock": true},
            {"tags": {"$in": ["gaming", "professional"]}}
        ]
    });
    group.bench_function("Complex nested query", |b| {
        b.iter(|| {
            let count = data.iter()
                .filter(|item| sift(black_box(&nested_query), black_box(item)).unwrap_or(false))
                .count();
            black_box(count)
        })
    });
    
    // Test $elemMatch query
    let elem_match_query = json!({
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
            {"company.financials.revenue.2023": {"$gte": 10000000}}
        ]
    });
    group.bench_function("$elemMatch query", |b| {
        b.iter(|| {
            let count = data.iter()
                .filter(|item| sift(black_box(&elem_match_query), black_box(item)).unwrap_or(false))
                .count();
            black_box(count)
        })
    });
    
    group.finish();
}

fn benchmark_filter_creation(c: &mut Criterion) {
    let data = generate_test_data();
    
    let mut group = c.benchmark_group("Filter Creation");
    
    let query = json!({"age": {"$gte": 25}});
    
    // Benchmark create_filter vs direct sift calls
    group.bench_function("Direct sift calls", |b| {
        b.iter(|| {
            let count = data.iter()
                .filter(|item| sift(black_box(&query), black_box(item)).unwrap_or(false))
                .count();
            black_box(count)
        })
    });
    
    group.bench_function("Using create_filter", |b| {
        let filter = create_filter(&query).unwrap();
        b.iter(|| {
            let count = data.iter()
                .filter(|item| filter(black_box(item)))
                .count();
            black_box(count)
        })
    });
    
    group.finish();
}

fn benchmark_memory_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Memory Allocation");
    
    // Benchmark data generation (memory allocation)
    group.bench_function("Generate test data", |b| {
        b.iter(|| {
            let data = generate_test_data();
            black_box(data)
        })
    });
    
    // Benchmark query parsing
    let query_json = json!({
        "$and": [
            {"category": "electronics"},
            {"price": {"$gte": 1000}},
            {"specs.ram": {"$gte": 16}}
        ]
    });
    
    group.bench_function("Query parsing", |b| {
        b.iter(|| {
            let filter = create_filter(black_box(&query_json)).unwrap();
            black_box(filter)
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    benchmark_basic_comparisons,
    benchmark_array_operations,
    benchmark_logical_operations,
    benchmark_field_operations,
    benchmark_complex_queries,
    benchmark_filter_creation,
    benchmark_memory_allocation,
    benchmark_where_operations
);
criterion_main!(benches);
