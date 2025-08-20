use crate::error::AgentError;
use crate::traits::Model;
use crate::types::{Message, ModelResponse, ToolSpec};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OpenAITool>>,
}

#[derive(Serialize)]
struct OpenAITool {
    #[serde(rename = "type")]
    tool_type: String,
    function: OpenAIFunction,
}

#[derive(Serialize, Deserialize)]
struct OpenAIFunction {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

pub struct OpenAIModel {
    client: reqwest::Client, 
    api_key: String,
    model_name: String,
}

impl OpenAIModel {
    pub fn new(api_key: String, model_name: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
            model_name: model_name,
        }
    }
}

#[async_trait::async_trait]
impl Model for OpenAIModel {
    async fn generate(
        &self,
        messages: Vec<Message>,
        _tools: Option<Vec<ToolSpec>>,
    ) -> Result<ModelResponse, AgentError> {
        
        let request = serde_json::json!({
            "model": self.model_name,
            "messages": messages,
        });

        let response = self.client.post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send().await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(AgentError::ModelError(format!("OpenAI API Error: {}", error_text)));
        }

        let json: serde_json::Value = response.json().await?;
        let content = json["choices"][0]["message"]["content"].as_str().unwrap_or("No response from AI Model!").to_string();

        Ok(ModelResponse::Text(content))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openai_model() {
        let model = OpenAIModel::new(
            std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set"),
            String::from("TEST_MODEL"),
        );
        // dummy test, model name being passed in correctly
        assert_eq!(model.model_name, "TEST_MODEL");   
    }

    // addtnl. tests here
    // invalid model types, missing key, bad response, etc.
}