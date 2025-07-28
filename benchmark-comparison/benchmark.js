const Benchmark = require('benchmark');
const sift = require('sift');

function generateTestData() {
    return [
        {
            company: {
                id: "COMP-789",
                name: "TechInnovate Inc.",
                industry: "software",
                founded: "2015-03-10T00:00:00Z",
                headquarters: {
                    address: {
                        street: "123 Tech St",
                        city: "San Francisco",
                        state: "CA",
                        country: "USA",
                        postal_code: "94105"
                    },
                    coordinates: {
                        lat: 37.7749,
                        lng: -122.4194
                    }
                },
                employees: [
                    {
                        id: "EMP-001",
                        name: "John Doe",
                        role: "CEO",
                        department: "executive",
                        salary: 250000,
                        start_date: "2015-03-10T00:00:00Z",
                        skills: ["leadership", "strategy", "fundraising"],
                        performance: {
                            rating: 4.8,
                            reviews: 12,
                            goals_met: 0.95
                        },
                        email: "john.doe@techinnovate.com",
                        age: 45
                    },
                    {
                        id: "EMP-002",
                        name: "Jane Smith",
                        role: "CTO",
                        department: "engineering",
                        salary: 220000,
                        start_date: "2015-06-15T00:00:00Z",
                        skills: ["rust", "typescript", "architecture", "leadership"],
                        performance: {
                            rating: 4.9,
                            reviews: 15,
                            goals_met: 0.98
                        },
                        email: "jane.smith@techinnovate.com",
                        age: 38
                    },
                    {
                        id: "EMP-003",
                        name: "Bob Wilson",
                        role: "Senior Developer",
                        department: "engineering",
                        salary: 180000,
                        start_date: "2018-01-20T00:00:00Z",
                        skills: ["rust", "python", "databases"],
                        performance: {
                            rating: 4.6,
                            reviews: 8,
                            goals_met: 0.88
                        },
                        age: 32
                    }
                ],
                projects: [
                    {
                        id: "PROJ-001",
                        name: "AI Platform",
                        status: "active",
                        budget: 2500000,
                        start_date: "2023-01-01T00:00:00Z",
                        end_date: "2024-06-30T00:00:00Z",
                        team_size: 15,
                        technologies: ["rust", "python", "tensorflow", "kubernetes"]
                    },
                    {
                        id: "PROJ-002",
                        name: "Mobile App",
                        status: "completed",
                        budget: 800000,
                        start_date: "2023-03-01T00:00:00Z",
                        end_date: "2023-12-31T00:00:00Z",
                        team_size: 8,
                        technologies: ["react-native", "typescript", "firebase"]
                    }
                ],
                financials: {
                    revenue: {
                        "2023": 15000000,
                        "2022": 12000000,
                        "2021": 8000000
                    },
                    funding_rounds: [
                        {
                            round: "Series C",
                            amount: 50000000,
                            date: "2023-05-15T00:00:00Z",
                            investors: ["VentureCapital Corp", "TechFund Partners"]
                        }
                    ]
                }
            }
        }
    ];
}

const data = generateTestData();
const suite = new Benchmark.Suite();
const results = {};
const outputJson = process.argv.includes('--json');

if (!outputJson) {
    console.log('Starting sift.js Performance Benchmarks...');
    console.log('=============================================\n');
}

suite.add('$eq operator', () => {
    const query = sift({ 'company.employees.0.age': { $eq: 45 } });
    data.filter(query).length;
});

suite.add('$ne operator', () => {
    const query = sift({ 'company.industry': { $ne: 'healthcare' } });
    data.filter(query).length;
});

suite.add('$gt operator', () => {
    const query = sift({ 'company.financials.revenue.2023': { $gt: 10000000 } });
    data.filter(query).length;
});

suite.add('$gte operator', () => {
    const query = sift({ 'company.employees.0.salary': { $gte: 200000 } });
    data.filter(query).length;
});

suite.add('$lt operator', () => {
    const query = sift({ 'company.employees.2.age': { $lt: 40 } });
    data.filter(query).length;
});

suite.add('$lte operator', () => {
    const query = sift({ 'company.projects.0.budget': { $lte: 3000000 } });
    data.filter(query).length;
});

suite.add('$in operator', () => {
    const query = sift({ 'company.projects.0.status': { $in: ['active', 'pending', 'completed'] } });
    data.filter(query).length;
});

suite.add('$nin operator', () => {
    const query = sift({ 'company.projects.0.status': { $nin: ['cancelled', 'suspended'] } });
    data.filter(query).length;
});

suite.add('$all operator', () => {
    const query = sift({ 'company.employees.0.skills': { $all: ['leadership', 'strategy'] } });
    data.filter(query).length;
});

suite.add('$size operator', () => {
    const query = sift({ 'company.employees': { $size: 3 } });
    data.filter(query).length;
});

suite.add('$and operator', () => {
    const query = sift({
        $and: [
            { 'company.industry': 'software' },
            { 'company.employees.0.age': { $gte: 40 } }
        ]
    });
    data.filter(query).length;
});

suite.add('$or operator', () => {
    const query = sift({
        $or: [
            { 'company.employees.0.age': { $gte: 50 } },
            { 'company.projects.0.status': 'active' }
        ]
    });
    data.filter(query).length;
});

suite.add('$not operator', () => {
    const query = sift({ 'company.industry': { $not: { $eq: 'healthcare' } } });
    data.filter(query).length;
});

suite.add('$nor operator', () => {
    const query = sift({
        $nor: [
            { 'company.industry': 'healthcare' },
            { 'company.employees.0.age': { $lt: 30 } }
        ]
    });
    data.filter(query).length;
});

suite.add('$exists operator', () => {
    const query = sift({ 'company.employees.0.email': { $exists: true } });
    data.filter(query).length;
});

suite.add('$type operator', () => {
    const query = sift({ 'company.employees.0.age': { $type: 'number' } });
    data.filter(query).length;
});

suite.add('$regex operator', () => {
    const query = sift({ 'company.employees.0.email': { $regex: /@techinnovate\.com$/ } });
    data.filter(query).length;
});

suite.add('$mod operator', () => {
    const query = sift({ 'company.employees.0.age': { $mod: [5, 0] } });
    data.filter(query).length;
});

suite.add('Complex nested query', () => {
    const query = sift({
        $and: [
            { 'category': 'electronics' },
            { 'price': { $gte: 1000, $lte: 2000 } },
            { 'specs.ram': { $gte: 16 } },
            { 'ratings.average': { $gte: 4.0 } },
            { 'availability.in_stock': true },
            { 'tags': { $in: ['gaming', 'professional'] } }
        ]
    });
    data.filter(query).length;
});

suite.add('$elemMatch query', () => {
    const query = sift({
        $and: [
            { 'company.industry': 'software' },
            {
                'company.employees': {
                    $elemMatch: {
                        $and: [
                            { 'department': 'engineering' },
                            { 'salary': { $gte: 200000 } },
                            { 'skills': { $in: ['rust', 'leadership'] } },
                            { 'performance.rating': { $gte: 4.5 } }
                        ]
                    }
                }
            },
            { 'company.financials.revenue.2023': { $gte: 10000000 } }
        ]
    });
    data.filter(query).length;
});

suite.add('Query compilation', () => {
    const query = sift({
        $and: [
            { 'category': 'electronics' },
            { 'price': { $gte: 1000 } },
            { 'specs.ram': { $gte: 16 } }
        ]
    });
});

suite.on('cycle', (event) => {
    const benchmark = event.target;
    const opsPerSec = benchmark.hz;
    const timePerOp = (1 / opsPerSec) * 1000000;
    
    results[benchmark.name] = {
        opsPerSec: opsPerSec,
        timePerOp: parseFloat(timePerOp.toFixed(2))
    };
    
    if (!outputJson) {
        console.log(`${benchmark.name}: ${opsPerSec.toFixed(0).padStart(10)} ops/sec (${timePerOp.toFixed(2)} µs/op)`);
    }
});

suite.on('complete', () => {
    if (outputJson) {
        console.log(JSON.stringify(results, null, 2));
    } else {
        console.log('\n=== sift.js Benchmark Results Summary ===');
        console.log('All benchmarks completed successfully!');
    }
});

suite.run({ async: true });

if (!outputJson) {
    console.log('\nNote: These results will be compared with sift-rs benchmarks to demonstrate the performance advantages of the Rust implementation.\n');
}
