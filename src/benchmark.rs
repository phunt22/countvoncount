use crate::cli::{run_cli, run_cli_no_tools};
use crate::error::AgentError;
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Clone, Deserialize)]
pub struct TestCaseFile {
    pub schema: Vec<String>,
    pub tests: Vec<serde_yaml::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub prompt: String,
    pub expected_output: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TestResult {
    pub prompt: String,
    pub expected_output: String,
    pub with_tools: String,
    pub without_tools: String,
    pub timestamp: String,
    pub with_tools_duration_ms: u64,
    pub without_tools_duration_ms: u64,
}

#[derive(Debug, Serialize)]
pub struct BenchmarkSummary {
    pub total_tests: usize,
    pub timestamp: String,
    pub results_file: String,
    pub test_results: Vec<TestResult>,
}

pub async fn run_benchmarks() -> Result<String, AgentError> {
    let test_cases = load_test_cases().await?;
    
    let mut results = Vec::new();
    let mut output_lines = vec![
        "=== Count von Count Tool Benchmarks ===".to_string(),
        format!("Loaded {} test cases from YAML", test_cases.len()),
        "".to_string(),
    ];
    
    for (i, test_case) in test_cases.iter().enumerate() {
        println!("Running benchmark {}/{}: {}", i + 1, test_cases.len(), test_case.prompt);
        
        let with_tools_start = std::time::Instant::now();
        let with_tools_response = run_cli(
            test_case.prompt.clone(), 
            false // verbose = false for benchmarks
        ).await.unwrap_or_else(|e| format!("Error: {}", e));
        let with_tools_duration = with_tools_start.elapsed();
        
        let without_tools_start = std::time::Instant::now();
        let without_tools_response = run_cli_no_tools(
            test_case.prompt.clone()
        ).await.unwrap_or_else(|e| format!("Error: {}", e));
        let without_tools_duration = without_tools_start.elapsed();
        
        let result = TestResult {
            prompt: test_case.prompt.clone(),
            expected_output: test_case.expected_output.clone(),
            with_tools: with_tools_response.clone(),
            without_tools: without_tools_response.clone(),
            timestamp: Utc::now().to_rfc3339(),
            with_tools_duration_ms: with_tools_duration.as_millis() as u64,
            without_tools_duration_ms: without_tools_duration.as_millis() as u64,
        };
        
        results.push(result);
        
        let with_tools_display = truncate_string(&with_tools_response, 100);
        let without_tools_display = truncate_string(&without_tools_response, 100);
        
        output_lines.push(format!("Benchmark: {}", test_case.prompt));
        output_lines.push(format!("  With Tools:    {} ({}ms)", with_tools_display, with_tools_duration.as_millis()));
        output_lines.push(format!("  Without Tools: {} ({}ms)", without_tools_display, without_tools_duration.as_millis()));
        output_lines.push("".to_string());
    }
    
    let summary = save_benchmark_results(results).await?;
    output_lines.push(format!("Results saved to: {}", summary.results_file));
    
    Ok(output_lines.join("\n"))
}

async fn load_test_cases() -> Result<Vec<TestCase>, AgentError> {
    let yaml_content = tokio::fs::read_to_string("test_cases.yaml").await
        .map_err(|e| AgentError::InvalidInputError(format!("Failed to read test_cases.yaml: {}", e)))?;
    
    let test_file: TestCaseFile = serde_yaml::from_str(&yaml_content)
        .map_err(|e| AgentError::InvalidInputError(format!("Failed to parse test_cases.yaml: {}", e)))?;
    
    let mut test_cases = Vec::new();
    
    for test_data in test_file.tests {
        if let Some(test_array) = test_data.as_sequence() {
            if test_array.len() >= 2 {
                let prompt = test_array[0].as_str().unwrap_or("").to_string();
                let expected = test_array[1].as_str().unwrap_or("").to_string();
                
                test_cases.push(TestCase {
                    prompt,
                    expected_output: expected,
                });
            }
        }
    }
    
    Ok(test_cases)
}

async fn save_benchmark_results(results: Vec<TestResult>) -> Result<BenchmarkSummary, AgentError> {
    tokio::fs::create_dir_all("results").await
        .map_err(|e| AgentError::InvalidInputError(format!("Could not create results directory: {}", e)))?;
    
    let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
    let results_file = format!("results/benchmark_{}.jsonl", timestamp);
    
    let mut jsonl_lines = Vec::new();
    for result in &results {
        let json_line = serde_json::to_string(result)
            .map_err(|e| AgentError::InvalidInputError(format!("Failed to serialize result: {}", e)))?;
        jsonl_lines.push(json_line);
    }
    
    tokio::fs::write(&results_file, jsonl_lines.join("\n")).await
        .map_err(|e| AgentError::InvalidInputError(format!("Could not save results to {}: {}", results_file, e)))?;
    
    Ok(BenchmarkSummary {
        total_tests: results.len(),
        timestamp: Utc::now().to_rfc3339(),
        results_file: results_file.clone(),
        test_results: results,
    })
}

fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len])
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("short", 10), "short");
        assert_eq!(truncate_string("this is a very long string", 10), "this is a ...");
    }

    #[tokio::test]
    async fn test_load_test_cases() {
        if std::path::Path::new("test_cases.yaml").exists() {
            let result = load_test_cases().await;
            assert!(result.is_ok());
            let test_cases = result.unwrap();
            assert!(!test_cases.is_empty());
        }
    }
}
