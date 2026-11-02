mod agent;
mod test_runner;
mod tools;
mod workspace;

use gemini_rust::prelude::*;

use agent::Agent;
use workspace::Workspace;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("GEMINI_API_KEY")?;
    let model = std::env::var("GEMINI_MODEL")?;

    let client = Gemini::new(api_key)?;

    let workspace = Workspace::initialize()?;

    let agent = Agent::new(
        client,
        model,
        workspace,
    );

    let task = r#"
        Implement a function called calculate_average.

        Requirements:
        - Accept a slice of f64.
        - Return Option<f64>.
        - Return None for an empty slice.
        - Return the arithmetic average otherwise.
        - Include unit tests.
    "#;

    agent.run(task).await?;

    Ok(())
}
