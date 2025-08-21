use std::collections::HashMap;
use async_trait::async_trait;
use evalexpr::eval;
use serde_json::{json, Value};
use crate::error::AgentError;
use crate::tools::Tool;

pub struct CalculatorTool;

impl CalculatorTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for CalculatorTool {
    fn name(&self) -> &'static str {
        "calculator"
    }

    fn description(&self) -> &'static str {
        "Evaluate arithmetic expressions, e.g., '2 + 2 * (3 - 1)'. Args: { expression: string }"
    }

    fn json_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "expression": {
                    "type": "string",
                    "description": "The mathematical expression to evaluate"
                }
            },
            "required": ["expression"]
        })
    }

    async fn run(&self, args: HashMap<String, Value>) -> Result<String, AgentError> {
        
        let expression = args.get("expression")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AgentError::InvalidInputError("Missing 'expression' parameter".to_string()))?;

        if expression.trim().is_empty() {
            return Err(AgentError::InvalidInputError("Expression cannot be empty".to_string()));
        }

        match eval(&expression) {
            Ok(result) => {
                let result_str = match result {
                    evalexpr::Value::Int(i) => i.to_string(),
                    evalexpr::Value::Float(f) => {
                        if f.fract() == 0.0 {
                            (f as i64).to_string()
                        } else {
                            // more friendly format for LLM
                            format!("{:.6}", f).trim_end_matches('0').trim_end_matches('.').to_string()
                        }
                    },
                    evalexpr::Value::Boolean(b) => b.to_string(),
                    _ => result.to_string(),
                };
                Ok(result_str)
            },
            Err(e) => Err(AgentError::ToolError {
                tool_name: self.name().to_string(),
                message: format!("Failed to evaluate expression '{}': {}", expression, e)
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_calculator_basic() {
        let calc = CalculatorTool::new();
        let mut args = HashMap::new();
        args.insert("expression".to_string(), Value::String("2 + 2".to_string()));
        
        let result = calc.run(args).await.unwrap();
        assert_eq!(result, "4");
    }

    #[tokio::test]
    async fn test_calculator_complex() {
        let calc = CalculatorTool::new();
        let mut args = HashMap::new();
        args.insert("expression".to_string(), Value::String("15 * 7 + 23".to_string()));
        
        let result = calc.run(args).await.unwrap();
        assert_eq!(result, "128");
    }

    #[tokio::test]
    async fn test_calculator_validation_fails() {
        let calc = CalculatorTool::new();
        let mut args = HashMap::new();
        args.insert("expression".to_string(), Value::String("delete * from users".to_string()));
        
        let result = calc.run(args).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_calculator_empty_expression() {
        let calc = CalculatorTool::new();
        let mut args = HashMap::new();
        args.insert("expression".to_string(), Value::String("".to_string())); 
        
        let result = calc.run(args).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[tokio::test]
    async fn test_calculator_whitespace_expression() {
        let calc = CalculatorTool::new();
        let mut args = HashMap::new();
        args.insert("expression".to_string(), Value::String("   ".to_string()));
        
        let result = calc.run(args).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }
}