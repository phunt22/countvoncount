# Count Von Count - Rust Agent with Tools and Evals

This project is a minimal, model-agnostic AI Agent implemented as a Rust CLI. It focuses on robust tool use (calculator, Python) and quick evals to demonstrate improvement when tools are enabled.

## Goals

- Build a clean, swappable model abstraction so different backends (OpenAI, Anthropic, local/Ollama) can be plugged in later.
- Implement a small set of tools:
  - Calculator: arithmetic expressions via `evalexpr`.
  - Python: execute a short snippet using `python3 -c`.
- Provide a simple CLI with two commands:
  - `chat`: run a single prompt through the agent and print the output.
  - `evals`: run quick tasks to measure tool vs non-tool performance.
- Document assumptions, requirements, and steps to extend toward MCP servers later.

## Quickstart

```bash
cargo build
cargo run -- chat "calc: 2 + 2 * 3"  # => 8 (via tool)
cargo run -- chat "python: print(6 * 7)"  # => 42 (via tool)
cargo run -- chat "how many r's in strawberry"  # => 2 (built-in heuristic)

# Run evals
cargo run -- evals

# Disable tools to compare
cargo run -- --no-tools chat "calc: 2 + 2 * 3"  # => tool calls disabled
```

## Architecture

- `models::Model` trait: single `generate(messages, tools)` returning either assistant text or a list of tool calls. The `HeuristicModel` is a placeholder that detects `calc:` and `python:` prefixes and emits tool calls. Replace with real LLM backends later.
- `tools::Tool` trait: `name()`, `description()`, and async `run(args)` returning JSON. Registered in a `ToolRegistry`.
- `agent::Agent`: orchestrates the loop of model → tool calls → tool results → model, with a bounded number of tool iterations.
- `evals`: a small harness that runs canned cases and reports pass/fail.

## Planned Backends (swappable models)

Implement each by satisfying `models::Model`:

- OpenAI (Responses API / Chat Completions)
- Anthropic (Messages)
- Ollama (local models)

Feature flags (`openai`, `anthropic`, `ollama`) are scaffolded in `Cargo.toml` for future modules.

## Security Notes

- Python tool executes arbitrary code via `python3 -c`. For safety, keep it to short, trusted snippets. In a real agent, sandboxing is recommended.

## Clarifications

1. First integration openAI model calling
2. Agent does not need output streaming, that can be implemented later

## Requirements (initial)

- Model must be swappable behind `models::Model`.
- Tools registered through `ToolRegistry`; new tools must not require changes to the agent core.
- CLI provides `chat` and `evals` subcommands.
- Evals should show improvement with tools enabled vs disabled on tasks that require external computation.

## Next Steps

- Add a real LLM backend (OpenAI), binding their tool-calling format to `ToolCallSpec`.
- Expand eval set (date math, string transforms, small code tasks).
- Add logging and traces of tool calls and outputs.
- Start a basic MCP server skeleton exposing these tools.
