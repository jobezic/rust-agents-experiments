use crate::gemini_client::GeminiClient;
use crate::test_runner::TestRunner;
use crate::workspace::Workspace;

const MAX_ATTEMPTS: usize = 3;

pub struct Agent {
    llm: GeminiClient,
    workspace: Workspace,
}

impl Agent {
    pub fn new(llm: GeminiClient, workspace: Workspace) -> Self {
        Self { llm, workspace }
    }

    pub async fn run(
        &self,
        task: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut prompt = format!(
            "You are a Rust software engineer.\n\
             Implement the following task:\n\n\
             {task}\n\n\
             Requirements:\n\
             - Generate a complete Rust library source file.\n\
             - Include unit tests using #[cfg(test)].\n\
             - Do not use external dependencies.\n\
             - Return only Rust source code.\n\
             - Do not include Markdown code fences."
        );

        for attempt in 1..=MAX_ATTEMPTS {
            println!("\n=== Agent attempt {attempt}/{MAX_ATTEMPTS} ===");

            // Ask Gemini to generate or fix the implementation.
            let response = self.llm.generate(&prompt).await?;

            // Extract the Rust source.
            let code = extract_code(&response);

            // Write generated code.
            self.workspace.write_source(code)?;

            println!("Generated source written.");

            // Execute the tests.
            let result = TestRunner::run(self.workspace.root())?;

            println!("{}", result.feedback());

            if result.success {
                println!("\nAgent completed successfully!");
                return Ok(());
            }

            if attempt == MAX_ATTEMPTS {
                break;
            }

            // Give the LLM feedback from the compiler/test runner.
            prompt = format!(
                "Your previous Rust implementation failed.\n\n\
                 Original task:\n{task}\n\n\
                 Previous source code:\n\
                 ```rust\n{code}\n```\n\n\
                 Compiler and test feedback:\n{}\n\n\
                 Fix the implementation and return the complete \
                 corrected Rust source file. Include unit tests. \
                 Return only Rust code without Markdown fences.",
                result.feedback()
            );

            println!("Tests failed. Asking Gemini to fix the code...");
        }

        Err("Agent failed after maximum attempts".into())
    }
}

fn extract_code(response: &str) -> &str {
    let response = response.trim();

    if let Some(code) = response.strip_prefix("```rust") {
        return code.strip_suffix("```").unwrap_or(code).trim();
    }

    if let Some(code) = response.strip_prefix("```") {
        return code.strip_suffix("```").unwrap_or(code).trim();
    }

    response
}
