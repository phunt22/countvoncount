use std::collections::HashMap;
use async_trait::async_trait;
use chrono::{DateTime, Utc, Local};
use serde_json::{json, Value};
use crate::error::AgentError;
use crate::tools::Tool;

pub struct DatetimeTool;

impl DatetimeTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for DatetimeTool {
    fn name(&self) -> &'static str {
        "datetime"
    }

    fn description(&self) -> &'static str {
        "Get the current date and time in various formats. Args: { format: string }"
    }

    fn json_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "format": {
                    "type": "string",
                    "description": "Output format: 'timestamp' (Unix timestamp), 'iso' (ISO 8601), 'human' (human readable), or 'local' (local timezone)",
                    "default": "iso",
                    "enum": ["timestamp", "iso", "human", "local"]
                }
            },
            "required": []
        })
    }

    async fn run(&self, args: HashMap<String, Value>) -> Result<String, AgentError> {
        
        let format = args.get("format")
            .and_then(|v| v.as_str())
            .unwrap_or("iso");

        let now_utc: DateTime<Utc> = Utc::now();

        let result = match format {
            "timestamp" => now_utc.timestamp().to_string(),
            "iso" => now_utc.to_rfc3339(),
            "human" => now_utc.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
            "local" => {
                let local_time = now_utc.with_timezone(&Local);
                local_time.format("%Y-%m-%d %H:%M:%S %Z").to_string()
            },
            _ => {
                return Err(AgentError::ToolError {
                    tool_name: self.name().to_string(),
                    message: format!("Invalid format '{}'. Use: timestamp, iso, human, or local", format)
                });
            }
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_datetime_iso_format() {
        let dt = DatetimeTool::new();
        let mut args = HashMap::new();
        args.insert("format".to_string(), Value::String("iso".to_string()));
        
        let result = dt.run(args).await.unwrap();
        // ISO format should contain T and either Z or +/-
        assert!(result.contains("T") && (result.contains("Z") || result.contains("+") || result.contains("-")));
    }

    #[tokio::test]
    async fn test_datetime_timestamp() {
        let dt = DatetimeTool::new();
        let mut args = HashMap::new();
        args.insert("format".to_string(), Value::String("timestamp".to_string()));
        
        let result = dt.run(args).await.unwrap();
        let timestamp: i64 = result.parse().unwrap();
        assert!(timestamp > 1600000000); // After 2020
    }

    #[tokio::test]
    async fn test_datetime_validation_fails() {
        let dt = DatetimeTool::new();
        let mut args = HashMap::new();
        args.insert("format".to_string(), Value::String("invalid_format".to_string()));
        
        let result = dt.run(args).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_datetime_default_format() {
        let dt = DatetimeTool::new();
        let args = HashMap::new(); // No format specified
        
        let result = dt.run(args).await.unwrap();
        assert!(result.contains("T")); // Should be ISO format by default
    }
}