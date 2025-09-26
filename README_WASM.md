# Sift-rs WASM Implementation

This document describes how to use the sift-rs library as a WebAssembly (WASM) module for both web and Java applications.

## Web Usage

### Setup

First, get the WASM package by building it:

```bash
cd sift-rs-wasm
wasm-pack build --target web --out-dir pkg
```

This creates a `pkg` directory containing the WASM module and JavaScript bindings.

### In the Browser

```html
<!DOCTYPE html>
<html>
  <head>
    <title>Sift-rs WASM Demo</title>
  </head>
  <body>
    <script type="module">
      import init, { sift, validate_query, sift_many } from './pkg/sift_rs_wasm.js';
      
      async function run() {
        // Initialize the WASM module
        await init();
        
        // Example query and value
        const query = '{"age": {"$gte": 30}}';
        const value = '{"name": "Alice", "age": 32}';
        
        // Test if value matches the query
        const result = sift(query, value);
        console.log('Sift result:', result); // true
        
        // Test multiple values
        const valuesArray = '[{"name": "Alice", "age": 32}, {"name": "Bob", "age": 25}]';
        const results = sift_many(query, valuesArray);
        console.log('Sift many results:', results); // [true, false]
        
        // Validate a query
        const isValid = validate_query(query);
        console.log('Query is valid:', isValid); // true
      }
      
      run();
    </script>
  </body>
</html>
```

### API Functions

- `sift(query: string, value: string) -> Result<bool, JsValue>`: Tests if a single value matches the query
- `sift_many(query: string, values: string) -> Result<string, JsValue>`: Tests multiple values against a query
- `validate_query(query: string) -> Result<bool, JsValue>`: Validates if a query is syntactically correct
- `create_filter_fn(query: string) -> Result<FilterFunction, JsValue>`: Creates a reusable filter function

## Java Usage via WASM

### Option 1: Using Wasmer Java

Wasmer provides Java bindings for WebAssembly:

1. Add the Wasmer Java dependency to your project:

For Maven:
```xml
<dependency>
    <groupId>org.wasmer</groupId>
    <artifactId>wasmer-java</artifactId>
    <version>3.3.0</version>
</dependency>
```

For Gradle:
```gradle
implementation 'org.wasmer:wasmer-java:3.3.0'
```

2. Load and use the WASM module:

```java
import org.wasmer.*;
import java.io.File;
import java.util.Map;

public class SiftWasmExample {
    public static void main(String[] args) {
        // Load the WASM module
        byte[] wasmBytes = ...; // Load your sift-rs WASM module
        
        try (Instance instance = new Instance(new Module(wasmBytes), new Imports())) {
            // Assuming you've exported the functions in your WASM build
            Function siftFunction = instance.exports().getFunction("sift");
            
            // Example usage
            String query = "{\"age\": {\"$gte\": 30}}";
            String value = "{\"name\": \"Alice\", \"age\": 32}";
            
            // Call the sift function (this would require proper type conversion)
            Object result = siftFunction.call(query, value);
            
            System.out.println("Sift result: " + result);
        }
    }
}
```

### Option 2: Using Wasm4j

Another option is Wasm4j:

1. Add to your dependencies:
```xml
<dependency>
    <groupId>io.github.green4j</groupId>
    <artifactId>wasm4j</artifactId>
    <version>0.1.0</version>
</dependency>
```

2. Load and use the WASM module:
```java
import io.github.green4j.jwasm.*;
import static io.github.green4j.jwasm.Parameters.*;

public class SiftWasmExample {
    public static void main(String[] args) {
        try (WasmRuntime runtime = WasmRuntime.newInstance()) {
            // Load the WASM module
            byte[] wasmBytes = ...;
            WasmModule module = runtime.loadModule("sift", wasmBytes);
            
            // Get the exported function
            WasmFunction sift = module.getFunction("sift");
            
            // Call the function (parameters need to be properly formatted)
            Parameters params = Parameters.create("...");  // Query JSON
            Parameters params2 = Parameters.create("..."); // Value JSON
            // Call would need proper string handling
            
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}
```

### Option 3: Execute via Command Line and Inter-Process Communication

Since direct WASM execution from Java is still maturing, another approach is to run the WASM module via a command-line tool and communicate through stdin/stdout or a network interface:

1. Create a simple Node.js wrapper:
```javascript
// sift-runner.js
import init, { sift, sift_many, validate_query } from './pkg/sift_rs_wasm.js';

async function run() {
  await init();
  
  // Read from stdin
  let input = '';
  process.stdin.setEncoding('utf8');
  process.stdin.on('readable', () => {
    let chunk;
    while ((chunk = process.stdin.read()) !== null) {
      input += chunk;
    }
  });
  
  process.stdin.on('end', () => {
    try {
      const { func, args } = JSON.parse(input);
      let result;
      
      switch(func) {
        case 'sift':
          result = { result: sift(args[0], args[1]) };
          break;
        case 'sift_many':
          result = { result: JSON.parse(sift_many(args[0], args[1])) };
          break;
        case 'validate_query':
          result = { result: validate_query(args[0]) };
          break;
        default:
          result = { error: 'Unknown function' };
      }
      
      console.log(JSON.stringify(result));
    } catch (e) {
      console.error(JSON.stringify({ error: e.message }));
    }
  });
}

run();
```

2. Execute from Java:
```java
import java.io.*;
import java.util.concurrent.TimeUnit;

public class SiftJavaClient {
    public static String executeSift(String query, String value) throws IOException, InterruptedException {
        ProcessBuilder pb = new ProcessBuilder("node", "sift-runner.js");
        Process process = pb.start();
        
        // Send input
        String input = "{\"func\":\"sift\",\"args\":[\"" + 
                      query.replace("\"", "\\\"") + "\",\"" + 
                      value.replace("\"", "\\\"") + "\"]}";
        
        try (OutputStreamWriter writer = new OutputStreamWriter(process.getOutputStream())) {
            writer.write(input);
            writer.flush();
        }
        
        // Read result
        StringBuilder output = new StringBuilder();
        try (BufferedReader reader = new BufferedReader(new InputStreamReader(process.getInputStream()))) {
            String line;
            while ((line = reader.readLine()) != null) {
                output.append(line);
            }
        }
        
        process.waitFor(5, TimeUnit.SECONDS);
        return output.toString();
    }
    
    public static void main(String[] args) throws Exception {
        String result = executeSift("{\"age\": {\"$gte\": 30}}", "{\"name\": \"Alice\", \"age\": 32}");
        System.out.println("Result: " + result);
    }
}
```

## Building for Different Targets

### Browser
```bash
wasm-pack build --target web --out-dir pkg
```

### Node.js
```bash
wasm-pack build --target nodejs --out-dir pkg-node
```

### Bundler (for Webpack, etc.)
```bash
wasm-pack build --target bundler --out-dir pkg-bundler
```

## Features and Limitations

### Features
- MongoDB-style query filtering in WASM
- Support for most MongoDB operators ($eq, $ne, $gt, $gte, $lt, $lte, $in, $nin, $exists, $regex, $and, $or, $not, $all, $size, $mod, $type, $elemMatch)
- NEW: $where operator now available in WASM builds using Boa JavaScript engine
- Type-safe query construction
- Good performance for Rust

### Limitations
- Requires JavaScript environment to run
- Larger bundle size due to WASM runtime

## Testing

To test the WASM module:

1. Build the module: `cd sift-rs-wasm && wasm-pack build --target web --out-dir pkg`
2. Open `example-web-app/index.html` in a browser
3. Use the interface to test queries

## Implementation Notes

The $where operator implementation now uses Boa JavaScript engine for all builds, both server and WASM. This provides consistent functionality across all platforms and removes the dependency on rustyscript. The implementation now uses a single JavaScript engine across all platforms:

- Server builds: Boa JavaScript engine
- WASM builds: Boa JavaScript engine