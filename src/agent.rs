use gemini_rust::prelude::*;
use serde_json::Value;

use crate::{tools::AgentTools, workspace::Workspace};

const MAX_TOOL_CALLS: usize = 10;

pub struct Agent {
    client: Gemini,
    model: String,
    workspace: Workspace,
}

impl Agent {
    pub fn new(
        client: Gemini,
        model: String,
        workspace: Workspace,
    ) -> Self {
        Self {
            client,
            model,
            workspace,
        }
    }

    pub async fn run(
        &self,
        task: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let tools = AgentTools::definitions();

        let mut previous_id: Option<String> = None;

        let mut user_input = InteractionInput::text(task);

        for iteration in 1..=MAX_TOOL_CALLS {
            println!("\n--- Agent iteration {iteration} ---");

            let mut builder = self.client
                .create_interaction()
                .with_model(&self.model)
                .with_system_instruction(
                    "You are a Rust coding agent.

                     You have access to two tools:
                     - write_file: write Rust source code
                     - run_tests: execute the Rust tests

                     Workflow:
                     1. Understand the user's task.
                     2. Generate a complete Rust implementation,
                        including unit tests.
                     3. Call write_file.
                     4. Call run_tests.
                     5. If tests fail, fix the implementation
                        and repeat.
                     6. If tests pass, provide a final summary.

                     Do not claim success unless run_tests succeeds."
                );

            for tool in &tools {
                builder = builder.with_tool(tool.clone());
            }

            if let Some(ref id) = previous_id {
                builder = builder.with_previous_interaction(id);
            }

            builder = match &user_input {
                InteractionInput::Text(text) => {
                    builder.with_text(text.clone())
                }
                InteractionInput::StepArray(steps) => {
                    builder.with_step_input(steps.clone())
                }
                _ => builder,
            };

            let interaction = builder.execute().await?;

            let function_calls: Vec<_> = interaction.steps
                .iter()
                .filter_map(|step| {
                    if let Step::FunctionCall {
                        name,
                        arguments,
                        id,
                    } = step {
                        Some((
                            name.clone(),
                            arguments.clone(),
                            id.clone(),
                        ))
                    } else {
                        None
                    }
                })
                .collect();

            // No function calls means Gemini has finished.
            if function_calls.is_empty() {
                println!(
                    "\nAgent final response:\n{}",
                    interaction.output_text()
                );
                return Ok(());
            }

            let mut results = Vec::new();

            for (name, args, call_id) in function_calls {
                println!("Tool requested: {name}");
                println!("Arguments: {args}");

                let result: Value = match AgentTools::execute(
                    &name,
                    &args,
                    &self.workspace,
                ) {
                    Ok(value) => value,
                    Err(error) => serde_json::json!({
                        "success": false,
                        "error": error
                    }),
                };

                println!("Tool result: {result}");

                results.push(Step::FunctionResult {
                    name: Some(name),
                    call_id,
                    result: StepResult::from_json(result),
                    is_error: None,
                });
            }

            // Preserve the interaction context.
            previous_id = interaction.id()
                .map(|id| id.to_string());

            // Send the tool results back to Gemini.
            user_input = InteractionInput::step_array(results);
        }

        Err("Maximum tool-call iterations exceeded".into())
    }
}
