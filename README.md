# Count von Count 🧛‍♂️

This projects adds custom tools in Rust (calculator and datetime) to OpenAI's API. The goal of this project was to learn how AI Agents can interact with tools and see how these tools can make systems more reliable.

### Results

gpt-4.1-nano shows substantial improvements on my benchmark set when using tooling (shocking, I know).

[RESULTS_GO_HERE]

### Installation

```bash
# Clone the repository
git clone https://github.com/phunt22/countvoncount.git
cd countvoncount

# Set up .env file

### Example .env file
#### (note: if MODEL_NAME is ommitted, then gpt-4.1-nano will be used)
OPENAI_API_KEY=sk-<YOUR-KEY-HERE>
MODEL_NAME=gpt-4.1-nano

# Build and install
cargo build --release
cargo install --path .
```

### Usage

```bash
# Ask questions with tools (default)
cvc "How many days until Thanksgiving?"

# Ask without tools (for comparison)
cvc --no-tools "How many days until Thanksgiving?"

# Run benchmarks
cvc --combine

# Enable verbose output (to track tool calling behavior)
cvc --verbose "Complex calculation: (25 + 75) * 2 / 4"
```

## Benchmarking

`test_cases.yaml` contains 100 test cases. To run the benchmark, simply run:

```bash
cvc --combine
```

Results will be saved to a `.jsonl` file in the `results` directory, with the prompt, expected output, and results with and without tools. To evaluate, I did it by hand, but using LLM-as-a-judge would probably be a good next step.

### Custom Test Cases

```yaml
schema: [prompt, expected_output]
tests:
  - ["How many days unitl thanksgiving?", "88"]
  - ["What time is it?", "current_time"]
```

## Adding New Tools

Ability to add new tools was a consideration in development, and should (hopefully) be pretty easy to do. Simply:

1. Create a new file in `src/tools/<your_tool>.rs`
2. Implement the `Tool` trait
3. Register it in `src/tools/mod.rs`
4. Edit the system prompt to tell the model it has that tool in `src/cli.rs`
5. Add tests

Example:

```rust
use async_trait::async_trait;
use crate::tools::Tool;

pub struct <YourTool>;

#[async_trait]
impl Tool for <YourTool> {
    fn name(&self) -> &'static str { "your_tool" }
    fn description(&self) -> &'static str { "Description" }
    fn json_schema(&self) -> serde_json::Value { /* schema */ }
    async fn run(&self, args: HashMap<String, Value>) -> Result<String, AgentError> {
        // YOUR IMPLEMENTATION HERE
    }
}
```

## Acknowledgments

- `OpenAI` for the API
- `evalexpr` crate for expression evalutaion
- `chrono` for datetime handling
- `clap` for CLI argument parsing
