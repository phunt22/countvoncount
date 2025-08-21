use crate::error::AgentError;
use crate::traits::Model;
use crate::types::{Message, ModelResponse, ToolSpec, ToolCall, ToolFunction};
use serde::{Deserialize, Serialize};


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
            model_name,
        }
    }
}

#[async_trait::async_trait]
impl Model for OpenAIModel {
    async fn generate(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<ToolSpec>>,
    ) -> Result<ModelResponse, AgentError> {
        
        let mut request = serde_json::json!({
            "model": self.model_name,
            "messages": messages,
            "temperature": 0.0 // for debugging
        });

        if let Some(tool_specs) = tools {
            let openai_tools: Vec<OpenAITool> = tool_specs.into_iter().map(|spec| {
                OpenAITool {
                    tool_type: "function".to_string(),
                    function: OpenAIFunction {
                        name: spec.name,
                        description: spec.description,
                        parameters: spec.parameters,
                    },
                }
            }).collect();
            
            request["tools"] = serde_json::json!(openai_tools);
        }

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
        let message = &json["choices"][0]["message"];
                
        if let Some(tool_calls) = message["tool_calls"].as_array() {
            let mut parsed_tool_calls = Vec::new();
            
            for tool_call in tool_calls {
                let id = tool_call["id"].as_str().unwrap_or("unknown").to_string();
                let call_type = tool_call["type"].as_str().unwrap_or("function").to_string();
                let name = tool_call["function"]["name"].as_str().unwrap_or("unknown").to_string();
                let arguments = tool_call["function"]["arguments"].as_str().unwrap_or("{}").to_string();
                
                
                parsed_tool_calls.push(ToolCall { 
                    id, 
                    call_type,
                    function: ToolFunction { name, arguments }
                });
            }
            
            return Ok(ModelResponse::ToolCalls(parsed_tool_calls));
        }
        
        // Otherwise, return text content
        let content = message["content"].as_str().unwrap_or("No response from AI Model!").to_string();
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