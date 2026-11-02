# Rust Agent Experiments

A small project for exploring how to build agents in Rust and connect them to large language model APIs. It starts with a simple Gemini interaction: the program asks a model to write a Rust function, saves the generated source, and checks the output file. This is a first step for experimenting with agent behavior, tool use, and the surrounding Rust application structure.

## Current experiment

The application currently:

1. Sends a code-generation prompt to the configured Gemini model.
2. Extracts Rust source from the response, including responses wrapped in Markdown fences.
3. Saves the source to `workspace/generated.rs` and checks that the file is non-empty.

Compilation of the generated project is a possible next step; it is not currently run by the application.

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

The prompt is currently defined in `src/main.rs`, so each run performs the same experiment. The generated source is written under `workspace/`.

## Direction

This repository is an evolving sandbox for learning and experimenting with Rust agents and LLM APIs. The initial Gemini example provides a working place to explore the request/response loop; future experiments can build on it with configurable tasks, tools, state, and validation.
