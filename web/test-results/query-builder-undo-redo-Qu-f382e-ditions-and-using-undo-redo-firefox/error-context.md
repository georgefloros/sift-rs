# Page snapshot

```yaml
- heading "Sift-rs MongoDB Query Validator" [level=1]
- paragraph: Build and test MongoDB queries against JSON data in real-time using the sift-rs validation engine.
- heading "JSON Input" [level=2]
- paragraph: Enter the JSON object to validate against
- code:
  - textbox "Editor content"
- heading "MongoDB Query Builder" [level=2]
- paragraph: Build your query using available fields
- heading "Query Builder" [level=3]
- button "Undo (Ctrl+Z)"
- button "Redo (Ctrl+Y)" [disabled]
- paragraph: "Current Conditions:"
- button "Clear All"
- text: AND name $eq "John"
- button "Edit condition"
- button "Delete condition"
- button "Add New Condition"
- paragraph: "Generated MongoDB Query:"
- text: "{ \"$and\": [ { \"name\": { \"$eq\": \"John\" } } ], \"$or\": [ { \"age\": { \"$gt\": 25 } } ] }"
- button "Validate Query"
- paragraph: Validation Error
- paragraph: "JSON.parse: unexpected character at line 1 column 1 of the JSON data"
- alert
- alert
- alert
```