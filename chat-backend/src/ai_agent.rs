use anyhow::{Context, Result};
use rig::completion::Prompt;
use rig::prelude::*;
use rig::providers::groq;
use std::env;
use tracing::{info, warn};

use crate::models::AgentResponse;

const SYSTEM_MESSAGE_TEMPLATE: &str = r#"
You are an AI assistant for the Sift-rs Query Builder.
Your goal is to help users build MongoDB-style queries by understanding their natural language requests.

Instructions:
1.  **Analyze the user's request** to understand their intent.
2.  **Generate a MongoDB query** in JSON format that matches the request.
3.  **Use ONLY the fields that exist in the provided data structure**.
4.  **Provide a clear, concise explanation** of how the query works.
5.  **Respond in JSON format** with the query and explanation.
6.  **If the request is ambiguous**, ask clarifying questions.
7.  **If the request references fields not in the schema**, suggest alternative fields.
8.  **If the request is not a query**, respond as a helpful assistant.
9.  **Use the $where operator ONLY if the user explicitly asks for it or mentions JavaScript-like expressions**.

{schema_context}

MongoDB Query Operators Reference (prioritize these over $where):
- $eq: Equal to
- $ne: Not equal to
- $gt: Greater than
- $gte: Greater than or equal to
- $lt: Less than
- $lte: Less than or equal to
- $in: Value in array
- $nin: Value not in array
- $exists: Field exists
- $regex: Regular expression match
- $and: Logical AND
- $or: Logical OR
- $not: Logical NOT
- $all: Array contains all values
- $size: Array has specific length
- $elemMatch: Array element matches query
- $where: JavaScript expression (use ONLY when explicitly requested)

Example Response Format:

```json
{{
  "query": {{"age": {{"$gte": 21}}}},
  "explanation": "This query finds documents where the 'age' field is greater than or equal to 21."
}}
```
"#;

#[derive(Clone)]
pub struct ChatAgent {
    client: groq::Client,
    model: String,
}

impl ChatAgent {
    pub async fn new() -> Result<Self> {
        // Get API key
        let api_key = env::var("GROQ_API_KEY").context("GROQ_API_KEY must be set")?;

        // Get model name from environment or use a default
        let model = env::var("AI_MODEL").unwrap_or_else(|_| "llama3-70b-8192".to_string());

        info!("Initializing Groq client");
        info!("Using model: {}", model);

        // Create Groq client
        let client = groq::Client::new(&api_key);

        Ok(Self { client, model })
    }

    pub async fn process_message(
        &self,
        message: &str,
        schema: Option<&serde_json::Value>,
        sample_data: Option<&serde_json::Value>,
    ) -> Result<AgentResponse> {
        info!("Processing message with AI agent: '{}'", message);

        // Build schema context
        let schema_context = self.build_schema_context(schema, sample_data);

        // Create the system message with schema context
        let system_message = SYSTEM_MESSAGE_TEMPLATE.replace("{schema_context}", &schema_context);

        let agent = self
            .client
            .agent(&self.model)
            .preamble(&system_message)
            .build();

        match agent.prompt(message).await {
            Ok(response) => {
                let response_str = response.to_string();
                info!("AI agent response: {}", response_str);

                // Attempt to parse the response as JSON
                if let Ok(json_response) = serde_json::from_str::<serde_json::Value>(&response_str)
                {
                    let query = json_response.get("query").and_then(|v| {
                        if v.is_string() {
                            v.as_str().map(|s| s.to_string())
                        } else {
                            Some(v.to_string())
                        }
                    });
                    let explanation = json_response
                        .get("explanation")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());

                    if query.is_some() || explanation.is_some() {
                        return Ok(AgentResponse {
                            message: response_str,
                            query,
                            explanation,
                        });
                    }
                }

                // If not a valid query/explanation JSON, return the raw response
                Ok(AgentResponse {
                    message: response_str,
                    query: None,
                    explanation: None,
                })
            }
            Err(e) => {
                warn!("Error from AI agent: {}", e);
                Err(e.into())
            }
        }
    }

    /// Build schema context for the AI prompt
    fn build_schema_context(
        &self,
        schema: Option<&serde_json::Value>,
        sample_data: Option<&serde_json::Value>,
    ) -> String {
        let mut context = String::new();

        if let Some(sample) = sample_data {
            context.push_str("**Data Structure:**\n");
            context.push_str(&format!(
                "```json\n{}\n```\n\n",
                serde_json::to_string_pretty(sample).unwrap_or_else(|_| sample.to_string())
            ));

            // Extract available fields from sample data
            if let Some(fields) = self.extract_fields(sample) {
                context.push_str("**Available Fields:**\n");
                for field in fields {
                    context.push_str(&format!("- {}\n", field));
                }
                context.push('\n');
            }
        } else if let Some(schema_val) = schema {
            context.push_str("**Schema:**\n");
            context.push_str(&format!(
                "```json\n{}\n```\n\n",
                serde_json::to_string_pretty(schema_val)
                    .unwrap_or_else(|_| schema_val.to_string())
            ));
        } else {
            context.push_str(
                "**No schema provided** - Please be careful to only reference fields that exist in the user's data.\n\n",
            );
        }

        context.push_str("**Important:** Only use fields that exist in the provided data structure. If a user asks about a field that doesn't exist, suggest the closest available field or ask for clarification.\n\n");

        context
    }

    /// Extract field names from a JSON value recursively
    fn extract_fields(&self, value: &serde_json::Value) -> Option<Vec<String>> {
        match value {
            serde_json::Value::Object(map) => {
                let mut fields = Vec::new();
                self.extract_fields_recursive(map, "", &mut fields);
                if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                }
            }
            _ => None,
        }
    }

    /// Recursively extract field names with dot notation for nested objects
    fn extract_fields_recursive(
        &self,
        map: &serde_json::Map<String, serde_json::Value>,
        prefix: &str,
        fields: &mut Vec<String>,
    ) {
        for (key, value) in map {
            let field_name = if prefix.is_empty() {
                key.clone()
            } else {
                format!("{}.{}", prefix, key)
            };

            fields.push(field_name.clone());

            // Recursively process nested objects
            if let serde_json::Value::Object(nested_map) = value {
                self.extract_fields_recursive(nested_map, &field_name, fields);
            }
        }
    }
}
