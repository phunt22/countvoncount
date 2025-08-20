use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn user(content: &str) -> Self {
        Self {
            role: String::from("user"),
            content: String::from(content),
        }
    }

    pub fn assistant(content: &str) -> Self {
        Self {
            role: String::from("assistant"),
            content: String::from(content),
        }
    }

    pub fn system(content: &str) -> Self {
        Self {
            role: String::from("system"),
            content: String::from(content),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub args: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
pub enum ModelResponse {
    Text(String),
    ToolCalls(Vec<ToolCall>),
}

pub struct ToolSpec {
    pub name: String,
    pub description: String,
    pub parameters: HashMap<String, serde_json::Value>,
}
