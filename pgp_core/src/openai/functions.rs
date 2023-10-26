use async_openai::types::{ChatCompletionFunctions, ChatCompletionFunctionsArgs};
use serde_json::json;

pub fn list_functions() -> [ChatCompletionFunctions; 1] {
    [ChatCompletionFunctionsArgs::default()
        .name("run_sql_query")
        .description("Get data from database using SQL query")
        .parameters(json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "PostgreSQL query to run",
                }
            },
            "required": ["query"],
        }))
        .build()
        .unwrap()]
}
