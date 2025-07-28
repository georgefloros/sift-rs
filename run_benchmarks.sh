#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# temp folder
TEMP_FOLDER='./tmp'

echo -e "${BLUE}=== Running Sift-rs vs. Sift.js Benchmarks ===${NC}\n"

# Create a temporary directory for results
if [ -d "${TEMP_FOLDER}" ]; then
    echo -e "${YELLOW}Temporary directory '${TEMP_FOLDER}' already exists. Using existing directory.${NC}"
else
    echo -e "${YELLOW}Creating temporary directory '${TEMP_FOLDER}' for benchmark results...${NC}"
    mkdir -p ${TEMP_FOLDER}
fi

# Output files
RUST_RESULTS_FILE="${TEMP_FOLDER}/sift-rs-results.json"
JS_RESULTS_FILE="${TEMP_FOLDER}/sift-js-results.json"

echo -e "${YELLOW}Step 1: Running Rust benchmarks...${NC}"

# Run Rust benchmarks and capture the full output
cargo bench --bench sift_benchmarks 2>&1 > ${TEMP_FOLDER}/rust_full_output.txt

# Create a script to parse the benchmark results
cat > ${TEMP_FOLDER}/parse_rust_benchmarks.awk << 'EOF'
BEGIN { 
    benchmark_name = ""
}

# Match lines that show benchmark names (standalone lines, not "Benchmarking" lines)
# Include $ symbol for $where Operations
/^[A-Za-z$].*\/.*$/ && !/^Benchmarking/ {
    benchmark_name = $0
    next
}

# Match lines with timing results
/^[ \t]*time:.*\[.*\]/ {
    # Extract the middle timing value and unit using a simpler approach
    gsub(/.*\[/, "", $0)  # Remove everything before [
    gsub(/\].*/, "", $0)  # Remove everything after ]
    
    # Now $0 should contain something like "1.6378 µs 1.6405 µs 1.6432 µs"
    # Split by space and take the middle values (2nd and 3rd elements)
    split($0, arr, " ")
    time_str = arr[3]  # Middle value
    unit = arr[4]      # Unit for middle value
    
    # Convert to microseconds
    if (unit == "ns") {
        time_us = time_str / 1000
    } else if (unit == "µs" || unit == "μs" || unit == "us") {
        time_us = time_str
    } else if (unit == "ms") {
        time_us = time_str * 1000
    } else if (unit == "s") {
        time_us = time_str * 1000000
    } else {
        time_us = time_str  # assume microseconds if unclear
    }
    
    if (benchmark_name != "") {
        printf "{\"%s\": {\"timePerOp\": %.2f}}\n", benchmark_name, time_us
        benchmark_name = ""  # Reset for next benchmark
    }
}
EOF

# Parse the benchmark results
awk -f ${TEMP_FOLDER}/parse_rust_benchmarks.awk ${TEMP_FOLDER}/rust_full_output.txt > ${TEMP_FOLDER}/rust_raw.json

# Convert to proper JSON format
echo "{" > "$RUST_RESULTS_FILE"
first=true
while IFS= read -r line; do
    if [ "$first" = true ]; then
        first=false
    else
        echo "," >> "$RUST_RESULTS_FILE"
    fi
    echo "$line" | sed 's/^{//' | sed 's/}$//' >> "$RUST_RESULTS_FILE"
done < ${TEMP_FOLDER}/rust_raw.json
echo "}" >> "$RUST_RESULTS_FILE"

echo -e "${GREEN}Rust benchmarks completed. Results saved to ${RUST_RESULTS_FILE}${NC}\n"

echo -e "${YELLOW}Step 2: Running JavaScript benchmarks...${NC}"

# Run JavaScript benchmarks
cd benchmark-comparison
node benchmark.js --json > "../${JS_RESULTS_FILE}"
cd ..

echo -e "${GREEN}JavaScript benchmarks completed. Results saved to ${JS_RESULTS_FILE}${NC}\n"

echo -e "${YELLOW}Step 3: Generating comparison table...${NC}"

# Run the comparison script
node generate_comparison.js "$RUST_RESULTS_FILE" "$JS_RESULTS_FILE"

echo -e "${YELLOW}Cleaning up JSON files...${NC}"
rm -rf ${TEMP_FOLDER}

echo -e "\n${GREEN}=== Benchmark comparison completed! ===${NC}"
