mod agent;
mod gemini_client;
mod test_runner;
mod workspace;

use agent::Agent;
use gemini_client::GeminiClient;
use workspace::Workspace;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("GEMINI_API_KEY")?;
    let model = std::env::var("GEMINI_MODEL")?;

    // Initialize the LLM client.
    let llm = GeminiClient::new(api_key, model)?;

    // Initialize the isolated workspace.
    let workspace = Workspace::initialize()?;

    // Create the agent.
    let agent = Agent::new(llm, workspace);

    // Give the agent a task.
    let task = r#"
        Implement a function called calculate_average.

        Requirements:
        - Accept a slice of f64.
        - Return Option<f64>.
        - Return None for an empty slice.
        - Return the arithmetic average otherwise.
        - Include unit tests for normal input and empty input.
    "#;

    agent.run(task).await?;

    Ok(())
}
