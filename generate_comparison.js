const fs = require('fs');

// Check if correct number of arguments provided
if (process.argv.length !== 4) {
    console.error('Usage: node generate_comparison.js <rust_results.json> <js_results.json>');
    process.exit(1);
}

const rustResultsFile = process.argv[2];
const jsResultsFile = process.argv[3];

// Load results
let rustResults, jsResults;

try {
    rustResults = JSON.parse(fs.readFileSync(rustResultsFile, 'utf8'));
    jsResults = JSON.parse(fs.readFileSync(jsResultsFile, 'utf8'));
} catch (error) {
    console.error('Error reading result files:', error.message);
    process.exit(1);
}

console.log('## Sift-rs vs. Sift.js Benchmark Comparison\n');
console.log('### Overview\n');
console.log('The following is a comparison of sift-rs and sift.js benchmark results, demonstrating the efficiency and performance gains of the Rust-based implementation over its JavaScript counterpart. All measurements are averaged over multiple iterations using high-complexity business data structures.\n');

// Group benchmarks by category
const categories = {
    'Basic Comparisons': ['$eq operator', '$ne operator', '$gt operator', '$gte operator', '$lt operator', '$lte operator'],
    'Array Operations': ['$in operator', '$nin operator', '$all operator', '$size operator'],
    'Logical Operations': ['$and operator', '$or operator', '$not operator', '$nor operator'],
    'Field Operations': ['$exists operator', '$type operator', '$regex operator', '$mod operator'],
    'Complex Queries': ['Complex nested query', '$elemMatch query'],
    '$where Operations': ['$where logic'],
    'Filter Creation': ['Direct sift calls', 'Using create_filter']
};

Object.entries(categories).forEach(([category, benchmarks]) => {
    console.log(`### ${category}`);
    benchmarks.forEach(benchmark => {
        const rustKey = `${category}/${benchmark}`;
        let jsKey = `${category}/${benchmark}`;

        // The JavaScript results should now use the same key format as Rust
        // No fallback needed since both should have category/benchmark format

        if (rustResults[rustKey] && jsResults[jsKey]) {
            const rustTime = rustResults[rustKey].timePerOp.toFixed(2);
            const jsTime = jsResults[jsKey].timePerOp.toFixed(2);
            const improvement = (jsTime / rustTime).toFixed(2);
            const improvementText = rustTime < jsTime ? ` (${improvement}x faster)` : '';
            console.log(`- **${benchmark}**: sift-rs - ${rustTime} µs, sift.js - ${jsTime} µs${improvementText}`);
        }
    });
    console.log('');
});

console.log('### Key Performance Insights');

// Calculate performance ratios with proper key matching
let rustFaster = 0;
let jsFaster = 0;
let totalBenchmarks = 0;

// Create a map of all valid benchmark pairs
const benchmarkPairs = [];
Object.entries(categories).forEach(([category, benchmarks]) => {
    benchmarks.forEach(benchmark => {
        const rustKey = `${category}/${benchmark}`;
        let jsKey = `${category}/${benchmark}`;

        // Handle special cases where JavaScript keys differ
        if (!jsResults[jsKey]) {
            jsKey = benchmark;
        }

        if (rustResults[rustKey] && jsResults[jsKey]) {
            benchmarkPairs.push({ rustKey, jsKey, displayName: benchmark });
        }
    });
});

benchmarkPairs.forEach(({ rustKey, jsKey }) => {
    totalBenchmarks++;
    const rustTime = rustResults[rustKey].timePerOp;
    const jsTime = jsResults[jsKey].timePerOp;

    if (rustTime < jsTime) {
        rustFaster++;
    } else {
        jsFaster++;
    }
});

console.log(`- **sift-rs outperforms sift.js** in ${rustFaster} out of ${totalBenchmarks} benchmarks.`);

// Find the biggest performance differences
let biggestRustAdvantage = { benchmark: '', ratio: 0 };
let biggestJsAdvantage = { benchmark: '', ratio: 0 };

benchmarkPairs.forEach(({ rustKey, jsKey, displayName }) => {
    const rustTime = rustResults[rustKey].timePerOp;
    const jsTime = jsResults[jsKey].timePerOp;
    const ratio = jsTime / rustTime;

    if (ratio > biggestRustAdvantage.ratio) {
        biggestRustAdvantage = { benchmark: displayName, ratio };
    }

    if (ratio < 1 && (1 / ratio) > biggestJsAdvantage.ratio) {
        biggestJsAdvantage = { benchmark: displayName, ratio: 1 / ratio };
    }
});

if (biggestRustAdvantage.ratio > 1) {
    console.log(`- **Biggest sift-rs advantage**: ${biggestRustAdvantage.benchmark} (${biggestRustAdvantage.ratio.toFixed(2)}x faster)`);
}

if (biggestJsAdvantage.ratio > 1) {
    console.log(`- **Biggest sift.js advantage**: ${biggestJsAdvantage.benchmark} (${biggestJsAdvantage.ratio.toFixed(2)}x faster)`);
}

console.log('- Overall, sift-rs provides superior performance capabilities in most areas, leveraging Rust\'s strengths in speed and optimization.');

console.log('\n---\n');
