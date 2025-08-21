use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use crate::error::AgentError;
use crate::tools::calculator::CalculatorTool;
use crate::tools::datetime::DatetimeTool;
use crate::types::ToolSpec;

pub mod calculator;
pub mod datetime;

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn json_schema(&self) -> serde_json::Value;
    async fn run(&self, args: HashMap<String, Value>) -> Result<String, AgentError>;
}

#[derive(Clone)]
pub struct ToolRegistry {
    tools: HashMap<&'static str, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    pub fn register(&mut self, tool: Arc<dyn Tool>) -> &mut Self {
        self.tools.insert(tool.name(), tool);
        self
    }

    pub fn get(&self, name: &str) -> Option<&Arc<dyn Tool>> {
        self.tools.get(name)
    }

    pub fn to_tool_specs(&self) -> Vec<ToolSpec> {
        self.tools.values().map(|tool| ToolSpec {
            name: tool.name().to_string(),
            description: tool.description().to_string(),
            parameters: tool.json_schema(),
        }).collect()
    }

    pub async fn execute_tool(
        &self,
        name: &str,
        args: HashMap<String, Value>,
    ) -> Result<String, AgentError> {
        let tool = self.tools.get(name).ok_or_else(|| {
            AgentError::ToolNotFoundError {
                tool_name: name.to_string(),
                available_tools: self.tool_names().join(", "),
            }
        })?;
        
        tool.run(args).await.map_err(|e| match e {
            AgentError::ToolError { .. } => e,
            other => AgentError::ToolError {
                tool_name: name.to_string(),
                message: other.to_string(),
            }
        })
    }

    pub fn tool_names(&self) -> Vec<String> {
        self.tools.keys().map(|&k| k.to_string()).collect()
    }

    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }
}

pub fn create_default_registry() -> ToolRegistry {
    let mut registry = ToolRegistry::new();
    registry
        .register(Arc::new(CalculatorTool::new()))
        .register(Arc::new(DatetimeTool::new()));
    registry
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = create_default_registry();
        assert!(!registry.is_empty());
        assert!(registry.get("calculator").is_some());
        assert!(registry.get("datetime").is_some());
    }

    #[test]
    fn test_tool_specs_generation() {
        let registry = create_default_registry();
        let specs = registry.to_tool_specs();
        assert_eq!(specs.len(), 2);
        assert!(specs.iter().any(|s| s.name == "calculator"));
        assert!(specs.iter().any(|s| s.name == "datetime"));
    }
}