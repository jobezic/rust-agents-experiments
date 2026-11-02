use gemini_rust::prelude::*;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

const OUTPUT_DIR: &str = "workspace";
const OUTPUT_FILE: &str = "workspace/generated.rs";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("GEMINI_API_KEY")?;
    let model = std::env::var("GEMINI_MODEL")?;

    let client = Gemini::new(api_key)?;

    let task = "Implement a public Rust function named hello_world
                that returns the String \"Hello, world!\".
                Return only the Rust source code, without Markdown fences.";

    println!("Task: {task}");
    println!("Generating code with Gemini...");

    let interaction = client
        .create_interaction()
        .with_model(&model)
        .with_text(task)
        .execute()
        .await?;

    let generated_code = interaction.output_text();

    println!("\nGenerated response:\n{generated_code}");

    // Extract Rust code, including support for accidental Markdown fences.
    let code = extract_rust_code(&generated_code);

    // Create the isolated output directory.
    fs::create_dir_all(OUTPUT_DIR)?;

    // Save the generated source.
    fs::write(OUTPUT_FILE, code)?;

    println!("\nCode saved to {OUTPUT_FILE}");

    // Check the generated file before reporting success.
    validate_generated_file(Path::new(OUTPUT_FILE))?;

    println!("Generated file exists and is not empty.");

    Ok(())
}

fn extract_rust_code(response: &str) -> &str {
    let trimmed = response.trim();

    if let Some(rest) = trimmed.strip_prefix("```rust") {
        return rest
            .strip_suffix("```")
            .unwrap_or(rest)
            .trim();
    }

    if let Some(rest) = trimmed.strip_prefix("```") {
        return rest
            .strip_suffix("```")
            .unwrap_or(rest)
            .trim();
    }

    trimmed
}

fn validate_generated_file(path: &Path) -> Result<(), Box<dyn Error>> {
    let metadata = fs::metadata(path)?;

    if !metadata.is_file() || metadata.len() == 0 {
        return Err("Generated source file is missing or empty".into());
    }

    Ok(())
}

// Keep this available for the next step, when we add compilation.
#[allow(dead_code)]
fn run_cargo_check() -> Result<(), Box<dyn Error>> {
    let status = Command::new("cargo")
        .args(["check", "--offline"])
        .current_dir(OUTPUT_DIR)
        .status()?;

    if !status.success() {
        return Err("Generated Rust code did not compile successfully".into());
    }

    Ok(())
}