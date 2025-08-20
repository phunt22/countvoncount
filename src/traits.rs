use crate::error::AgentError;
use crate::types::{Message, ModelResponse, ToolSpec};


#[async_trait::async_trait]
pub trait Model: Send + Sync {
    async fn generate(
        &self,
        messages: Vec<Message>,
        tools: Option<Vec<ToolSpec>>,
    ) -> Result<ModelResponse, AgentError>;
}