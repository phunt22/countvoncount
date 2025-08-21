use thiserror::Error;

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Model API error: {0}")]
    ModelError(String),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Invalid input: {0}")]
    InvalidInputError(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Tool execution error in '{tool_name}': {message}")]
    ToolError {
        tool_name: String,
        message: String,
    },

    #[error("Tool not found: '{tool_name}'. Available tools: {available_tools}")]
    ToolNotFoundError {
        tool_name: String,
        available_tools: String,
    },

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Agent conversation exceeded maximum loops ({max_loops})")]
    MaxLoopsExceeded { max_loops: usize },

    #[error("Invalid tool arguments for '{tool_name}': {details}")]
    InvalidToolArguments {
        tool_name: String,
        details: String,
    },
}