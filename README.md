# Rust Agent Experiments

This project explores how to build agents in Rust that use LLM APIs to complete software tasks. The current experiment gives an agent a function specification, asks Gemini to implement it with unit tests, runs those tests, and sends any compiler or test failures back to the model for a correction.

## Current workflow

The agent in `src/agent.rs` runs up to three attempts:

1. Ask Gemini to generate a complete Rust library source file for the requested function, including unit tests.
2. Write the response to `workspace/src/lib.rs`.
3. Run `cargo test --offline` in the isolated `workspace/` project.
4. If compilation or tests fail, provide the source and command output to Gemini and ask it to fix the implementation.
5. Stop when the tests pass or the attempt limit is reached.

The current task, `calculate_average`, is defined in `src/main.rs`. It asks for the arithmetic average of a slice of `f64`, returning `None` for an empty slice.

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

The generated project is stored in `workspace/`. Each run initializes its Cargo manifest and overwrites the generated library source.

## Exploring next

This is an early experiment in an agent loop: translate a task into code, use test results as feedback, and iterate. The task and retry limit are currently fixed in source; they can evolve as the project explores more flexible tasks, tools, and ways to evaluate generated changes.
