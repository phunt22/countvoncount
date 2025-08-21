use countvoncount::tools::create_default_registry;
use serde_json::json;
use std::collections::HashMap;

#[tokio::test]
async fn test_calculator_tool_execution() {
    let registry = create_default_registry();
    
    let mut args = HashMap::new();
    args.insert("expression".to_string(), json!("2 + 2"));
    
    let result = registry.execute_tool("calculator", args).await.unwrap();
    assert_eq!(result, "4");
}

#[tokio::test]
async fn test_calculator_complex_expression() {
    let registry = create_default_registry();
    
    let mut args = HashMap::new();
    args.insert("expression".to_string(), json!("15 * 7 + 23"));
    
    let result = registry.execute_tool("calculator", args).await.unwrap();
    assert_eq!(result, "128");
}

#[tokio::test]
async fn test_calculator_parentheses() {
    let registry = create_default_registry();
    
    let mut args = HashMap::new();
    args.insert("expression".to_string(), json!("(25 + 75) / 4"));
    
    let result = registry.execute_tool("calculator", args).await.unwrap();
    assert_eq!(result, "25");
}

#[tokio::test]
async fn test_datetime_tool_execution() {
    let registry = create_default_registry();
    
    let mut args = HashMap::new();
    args.insert("format".to_string(), json!("timestamp"));
    
    let result = registry.execute_tool("datetime", args).await.unwrap();
    let timestamp: i64 = result.parse().unwrap();
    assert!(timestamp > 1600000000); // After 2020
}

#[tokio::test]
async fn test_datetime_iso_format() {
    let registry = create_default_registry();
    
    let mut args = HashMap::new();
    args.insert("format".to_string(), json!("iso"));
    
    let result = registry.execute_tool("datetime", args).await.unwrap();
    // ISO format should contain T and either Z or +/-
    assert!(result.contains("T") && (result.contains("Z") || result.contains("+") || result.contains("-")));
}

#[tokio::test]
async fn test_datetime_default_format() {
    let registry = create_default_registry();
    
    let args = HashMap::new(); // No format specified
    
    let result = registry.execute_tool("datetime", args).await.unwrap();
    assert!(result.contains("T")); // Should be ISO format by default
}

#[tokio::test]
async fn test_tool_not_found() {
    let registry = create_default_registry();
    
    let args = HashMap::new();
    let result = registry.execute_tool("nonexistent", args).await;
    
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Tool not found"));
    assert!(error_msg.contains("calculator") && error_msg.contains("datetime"));
}

#[tokio::test]
async fn test_invalid_calculator_args() {
    let registry = create_default_registry();
    
    let mut args = HashMap::new();
    args.insert("expression".to_string(), json!("delete * from users"));
    
    let result = registry.execute_tool("calculator", args).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_invalid_datetime_format() {
    let registry = create_default_registry();
    
    let mut args = HashMap::new();
    args.insert("format".to_string(), json!("invalid_format"));
    
    let result = registry.execute_tool("datetime", args).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_missing_calculator_expression() {
    let registry = create_default_registry();
    
    let args = HashMap::new(); // Missing expression
    
    let result = registry.execute_tool("calculator", args).await;
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("Missing 'expression' parameter"));
}

#[tokio::test]
async fn test_empty_calculator_expression() {
    let registry = create_default_registry();
    
    let mut args = HashMap::new();
    args.insert("expression".to_string(), json!(""));
    
    let result = registry.execute_tool("calculator", args).await;
    assert!(result.is_err());
    let error_msg = result.unwrap_err().to_string();
    assert!(error_msg.contains("cannot be empty"));
}
