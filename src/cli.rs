use crate::agent::Agent;
use crate::error::AgentError;
use crate::openai::OpenAIModel;
use crate::tools::create_default_registry;
use crate::types::Message;

pub async fn run_cli(prompt: String, verbose: bool) -> Result<String, AgentError> {
    if prompt.trim().is_empty() {
        return Err(AgentError::InvalidInputError(
            "Please provide a non-empty prompt".to_string()
        ));
    }

    let api_key = std::env::var("OPENAI_API_KEY")
        .map_err(|_| AgentError::ConfigurationError(
            "OPENAI_API_KEY not set".to_string()
        ))?;

    let model_name = std::env::var("MODEL_NAME")
        .unwrap_or_else(|_| "gpt-4.1-nano".to_string());

    let model = Box::new(OpenAIModel::new(api_key, model_name));
    let tool_registry = create_default_registry();
    let agent = Agent::new(model, tool_registry);

    let system_message = Message::system(
        "You are Count von Count, a helpful assistant who loves counting and numbers! 

        Available tools:
        1. 'calculator' - Evaluates arithmetic expressions like '2 + 2', '15 * 7 + 23', etc. Cannot handle dates or date arithmetic.
        2. 'datetime' - Gets the current date/time only. Cannot calculate differences between dates.

        For date calculations (like 'days until X'), use your built-in knowledge to calculate manually after getting the current date. Do not try to subtract dates with the calculator tool.

        Respond naturally and enthusiastically as Count von Count."
    );

    let messages = vec![
        system_message,
        Message::user(&prompt),
    ];

    agent.run_conversation(messages, true, verbose).await
}

pub async fn run_cli_no_tools(prompt: String) -> Result<String, AgentError> {
    if prompt.trim().is_empty() {
        return Err(AgentError::InvalidInputError(
            "Please provide a non-empty prompt".to_string()
        ));
    }

    let api_key = std::env::var("OPENAI_API_KEY")
        .map_err(|_| AgentError::ConfigurationError(
            "OPENAI_API_KEY not set".to_string()
        ))?;

    let model_name = std::env::var("MODEL_NAME")
        .unwrap_or_else(|_| "gpt-4o-mini".to_string());

    let model = Box::new(OpenAIModel::new(api_key, model_name));
    let tool_registry = crate::tools::ToolRegistry::new(); // Empty registry
    let agent = Agent::new(model, tool_registry);

    let system_message = Message::system("You are Count von Count, a helpful assistant who loves counting and numbers! You do NOT have access to any tools - answer using only your built-in knowledge.");

    let messages = vec![
        system_message,
        Message::user(&prompt),
    ];

    agent.run_conversation(messages, false, false).await
}
