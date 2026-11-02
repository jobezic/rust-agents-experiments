# Rust Agent Experiments

This project explores building software agents in Rust that use LLM APIs. The current experiment gives Gemini a Rust function task and a small set of tools. Gemini can write the generated source, run its tests, inspect the results, and request further changes through the same interaction.

## Current experiment

The task in `src/main.rs` asks for a `calculate_average` function that accepts a slice of `f64`, returns `None` for an empty slice, and otherwise returns the arithmetic average. The agent is instructed to include unit tests.

The agent loop in `src/agent.rs` provides two tools:

- `write_file` writes the implementation to `workspace/src/lib.rs`. The tool only permits this path.
- `run_tests` runs `cargo test --offline` in the generated project and returns its output to Gemini.

Gemini decides when to call each tool. Rust executes the requested tool and sends the result back as part of the ongoing interaction, so Gemini can revise the source and run the tests again. The agent allows up to 10 interaction iterations.

The generated Cargo project lives in `workspace/`. Starting the program creates its directories and writes a Cargo manifest; generated source is written there by the `write_file` tool.

## Configure

You need Rust and Cargo, plus a Gemini API key and model name supported by your account. Set `GEMINI_API_KEY` and `GEMINI_MODEL` in your environment, or create a `.env` file in the project root:

```dotenv
GEMINI_API_KEY=your_api_key
GEMINI_MODEL=your_model_name
```

The application loads `.env` automatically. Keep your API key private and do not commit the `.env` file.

## Run

```sh
cargo run
```

The task is currently defined in `src/main.rs`. The generated project uses offline Cargo testing, so its dependencies must already be available locally; the generated manifest currently has no dependencies.

## Current limitation

The system instruction asks Gemini to finish only after tests pass, but the Rust loop currently treats a response with no tool calls as completion. It does not independently verify that `run_tests` was called successfully before returning. The 10-iteration limit is also a cap on interaction rounds, not a fixed number of test retries.

This is an early experiment in tool use, feedback loops, and connecting model decisions to Rust code. The task, tools, and completion checks can evolve as the agent grows.
