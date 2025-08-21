pub mod error;
pub mod types;
pub mod traits;
pub mod openai;
pub mod tools;
pub mod agent;
pub mod cli;
pub mod benchmark;

pub use cli::{run_cli, run_cli_no_tools};
pub use benchmark::run_benchmarks;
pub use error::AgentError;

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_cli_with_empty_prompt() {
        unsafe { std::env::set_var("OPENAI_API_KEY", "test-key"); }
        let result = run_cli("".to_string(), false).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("non-empty prompt"));
    }

    #[tokio::test]
    async fn test_cli_no_tools_with_empty_prompt() {
        unsafe { std::env::set_var("OPENAI_API_KEY", "test-key"); }
        let result = run_cli_no_tools("".to_string()).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("non-empty prompt"));
    }
}