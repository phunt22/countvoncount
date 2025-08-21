use crate::error::AgentError;
use crate::traits::Model;
use crate::types::{Message, ModelResponse, ToolCall};
use crate::tools::ToolRegistry;
use std::collections::HashMap;

pub struct Agent {
    model: Box<dyn Model>,
    tool_registry: ToolRegistry,
    max_loops: usize,
}

impl Agent {
    pub fn new(model: Box<dyn Model>, tool_registry: ToolRegistry) -> Self {
        Self {
            model,
            tool_registry,
            max_loops: 5,
        }
    }

    pub fn with_max_loops(mut self, max_loops: usize) -> Self {
        self.max_loops = max_loops;
        self
    }

    pub async fn run_conversation(
        &self,
        mut messages: Vec<Message>,
        use_tools: bool,
        verbose: bool,
    ) -> Result<String, AgentError> {
        let tool_specs = if use_tools && !self.tool_registry.is_empty() {
            Some(self.tool_registry.to_tool_specs())
        } else {
            None
        };

        let mut loop_count = 0;
        
        loop {
            loop_count += 1;
            if loop_count > self.max_loops {
                return Err(AgentError::MaxLoopsExceeded { 
                    max_loops: self.max_loops 
                });
            }

            if verbose {
                eprintln!("[DEBUG] Step {}: Sending request to model", loop_count);
                if let Some(last_msg) = messages.last() {
                    let content_preview = last_msg.content.as_ref()
                        .map(|c| if c.len() > 60 { format!("{}...", &c[..60]) } else { c.clone() })
                        .unwrap_or_else(|| "None".to_string());
                    eprintln!("[DEBUG]   Last message: role={:?}, content={}", last_msg.role, content_preview);
                }
            }
            
            let result = self.model.generate(messages.clone(), tool_specs.clone()).await?;
            
            match result {
                ModelResponse::Text(text) => {
                    if verbose {
                        eprintln!("[DEBUG] Step {}: Model returned final text response", loop_count);
                    }
                    return Ok(text);
                },
                ModelResponse::ToolCalls(tool_calls) => {
                    if verbose {
                        eprintln!("[DEBUG] Step {}: Model requested {} tool(s)", loop_count, tool_calls.len());
                        for tool_call in &tool_calls {
                            let args_preview = if tool_call.function.arguments.len() > 50 {
                                format!("{}...", &tool_call.function.arguments[..50])
                            } else {
                                tool_call.function.arguments.clone()
                            };
                            eprintln!("[DEBUG]   -> {} with args: {}", tool_call.function.name, args_preview);
                        }
                    }
                    
                    let tool_result_messages = self.execute_tool_calls(tool_calls.clone(), verbose).await?;
                    
                    messages.push(Message::assistant_with_tool_calls(tool_calls));
                    messages.extend(tool_result_messages);
                    
                    if verbose {
                        eprintln!("[DEBUG] Step {}: Processing tool results, continuing...", loop_count);
                    }
                }
            }
        }
    }

    async fn execute_tool_calls(
        &self,
        tool_calls: Vec<ToolCall>,
        verbose: bool,
    ) -> Result<Vec<Message>, AgentError> {
        let mut result_messages = Vec::new();
        
        for tool_call in tool_calls {
            // Parse the JSON arguments string
            let args: HashMap<String, serde_json::Value> = 
                serde_json::from_str(&tool_call.function.arguments)
                    .map_err(|e| AgentError::InvalidInputError(
                        format!("Failed to parse tool arguments: {}", e)
                    ))?;
            
            let result = self.tool_registry
                .execute_tool(&tool_call.function.name, args)
                .await?;
            
            if verbose {
                let result_preview = if result.len() > 40 { 
                    format!("{}...", &result[..40]) 
                } else { 
                    result.clone() 
                };
                eprintln!("[DEBUG]   <- {} returned: {}", tool_call.function.name, result_preview);
            }
            
            result_messages.push(Message::tool_result(&result, &tool_call.id));
        }
        
        Ok(result_messages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::create_default_registry;
    use crate::openai::OpenAIModel;

    #[tokio::test]
    async fn test_agent_creation() {
        let model = Box::new(OpenAIModel::new("TESTKEY".to_string(), "TESTMODEL".to_string()));
        let registry = create_default_registry();
        
        let agent = Agent::new(model, registry);
        assert_eq!(agent.max_loops, 5);
    }

    #[tokio::test]
    async fn test_agent_with_custom_max_loops() {
        let model = Box::new(OpenAIModel::new("test-key".to_string(), "test-model".to_string()));
        let registry = create_default_registry();
        
        let agent = Agent::new(model, registry).with_max_loops(10);
        assert_eq!(agent.max_loops, 10);
    }
}
