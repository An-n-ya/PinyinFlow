use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize)]
struct OpenAIMessage {
    role: String,
    content: Value,
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    temperature: f32,
    max_completion_tokens: u32,
    top_p: f32,
    response_format: ResponseFormat,
}

#[derive(Serialize)]
#[serde(tag = "type")]
enum ResponseFormat {
    #[serde(rename = "json_schema")]
    JsonSchema { json_schema: Value },
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize)]
struct MessageContent {
    content: Option<String>,
}

pub async fn revise(
    api_key: &str,
    base_url: &str,
    model: &str,
    prompt: &str,
    system_prompt: &str,
    json_schema: Value,
) -> Result<String> {
    let client = Client::new();
    let url = format!("{}/chat/completions", base_url.trim_end_matches('/'));

    let mut messages = Vec::new();
    messages.push(OpenAIMessage {
        role: "system".to_string(),
        content: Value::String(system_prompt.to_string()),
    });

    let content_part = serde_json::json!({
        "type": "text",
        "text": prompt
    });
    messages.push(OpenAIMessage {
        role: "user".to_string(),
        content: Value::Array(vec![content_part]),
    });

    let request = OpenAIRequest {
        model: model.to_string(),
        messages,
        temperature: 0.2,
        max_completion_tokens: 512,
        top_p: 0.8,
        response_format: ResponseFormat::JsonSchema { json_schema },
    };

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(anyhow::anyhow!("Request failed: {}", error_text));
    }

    let response_body: OpenAIResponse = response.json().await?;

    if let Some(choice) = response_body.choices.first() {
        if let Some(content) = &choice.message.content {
            return Ok(content.clone());
        }
    }

    Err(anyhow::anyhow!("No content in response"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_request_serialization() {
        let prompt = "test prompt";
        let system_prompt = "system prompt";
        let json_schema = json!({"type": "object"});
        let model = "gpt-4";

        let mut messages = Vec::new();
        messages.push(OpenAIMessage {
            role: "system".to_string(),
            content: Value::String(system_prompt.to_string()),
        });
        let content_part = serde_json::json!({
            "type": "text",
            "text": prompt
        });
        messages.push(OpenAIMessage {
            role: "user".to_string(),
            content: Value::Array(vec![content_part]),
        });

        let request = OpenAIRequest {
            model: model.to_string(),
            messages,
            temperature: 0.2,
            max_completion_tokens: 512,
            top_p: 0.8,
            response_format: ResponseFormat::JsonSchema { json_schema: json_schema.clone() },
        };

        let json = serde_json::to_string(&request).unwrap();
        println!("{}", json);
        assert!(json.contains("gpt-4"));
        assert!(json.contains("system prompt"));
        assert!(json.contains("test prompt"));
        assert!(json.contains("json_schema"));
    }
}
