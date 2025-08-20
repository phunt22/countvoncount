pub mod error;
pub mod types;
pub mod traits;
pub mod openai;


use error::AgentError;
use openai::OpenAIModel;
use traits::Model;
use types::{Message, ModelResponse, ToolSpec};

pub async fn run_cli(args: Vec<String>) -> Result<String, AgentError>{
    if args.is_empty() {
        return Err(AgentError::InvalidInputError(String::from("Please provide a prompt!")))
    }

    let prompt = args.join(" ");
    let api_key = std::env::var("OPENAI_API_KEY")
        .map_err(|_| AgentError::InvalidInputError(
            String::from("api key not set")
        ))?;
    let model = OpenAIModel::new(api_key, String::from("gpt-4.1-nano"));

    let messages = vec![
        Message::system("You are count von count, a helpful assistant."),
        Message::user(&prompt),
    ];

    match model.generate(messages, None).await? {
        ModelResponse::Text(text) => Ok(text),
        ModelResponse::ToolCalls(_) => {
            Ok(String::from("Tool calls not supported yet, but model wanted to use!"))
        }
    }
}

pub fn run_benchmarks() -> String {
    String::from("Hello, world! Running benchmarks")    
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_cli_with_no_prompt() {
        let r = run_cli(vec![]).await;
        assert!(r.is_err());
        assert_eq!(r.err().unwrap().to_string(), "Invalid Input: Please provide a prompt!");
    }
}