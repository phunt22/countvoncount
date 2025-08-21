use countvoncount::{run_cli, run_cli_no_tools};

#[tokio::test]
async fn test_empty_prompt_handling() {
    unsafe { std::env::set_var("OPENAI_API_KEY", "test-key"); }
    let result = run_cli("".to_string(), false).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("non-empty prompt"));
}

#[tokio::test]
async fn test_whitespace_only_prompt() {
    unsafe { std::env::set_var("OPENAI_API_KEY", "test-key"); }
    let result = run_cli("   \n\t  ".to_string(), false).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("non-empty prompt"));
}

#[tokio::test]
async fn test_no_tools_variant() {
    unsafe { std::env::set_var("OPENAI_API_KEY", "test-key"); }
    let result = run_cli_no_tools("test prompt".to_string()).await;
    assert!(result.is_err());
}
