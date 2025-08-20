use thiserror::Error;


#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Model API error: {0}")]
    ModelError(String),

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Invalid Input: {0}")]
    InvalidInputError(String),

    #[error("Tool execution error in {tool_name}: {message}")]
    ToolError {
        tool_name: String,
        message: String
    }

}