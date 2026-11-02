use gemini_rust::prelude::*;
use serde_json::{json, Value};

use crate::test_runner::TestRunner;
use crate::workspace::Workspace;

pub struct AgentTools;

impl AgentTools {
    pub fn definitions() -> Vec<InteractionTool> {
        let write_file = InteractionTool::function(
            "write_file",
            "Write Rust source code to the generated project. Only src/lib.rs is allowed.",
            json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "enum": ["src/lib.rs"]
                    },
                    "content": {
                        "type": "string",
                        "description": "Complete Rust source code"
                    }
                },
                "required": ["path", "content"]
            }),
        );

        let run_tests = InteractionTool::function(
            "run_tests",
            "Run cargo test in the generated Rust project.",
            json!({
                "type": "object",
                "properties": {},
                "required": []
            }),
        );

        vec![write_file, run_tests]
    }

    pub fn execute(
        name: &str,
        args: &Value,
        workspace: &Workspace,
    ) -> Result<Value, String> {
        match name {
            "write_file" => {
                let path = args["path"]
                    .as_str()
                    .ok_or("Missing path")?;

                let content = args["content"]
                    .as_str()
                    .ok_or("Missing content")?;

                // Enforce the path restriction independently
                // of the model-provided arguments.
                if path != "src/lib.rs" {
                    return Err("Only src/lib.rs is allowed".into());
                }

                workspace.write_source(content)
                    .map_err(|e| e.to_string())?;

                Ok(json!({
                    "success": true,
                    "message": "Source file written successfully"
                }))
            }

            "run_tests" => {
                let result = TestRunner::run(workspace.root())
                    .map_err(|e| e.to_string())?;

                Ok(json!({
                    "success": result.success,
                    "stdout": result.stdout,
                    "stderr": result.stderr
                }))
            }

            _ => Err(format!("Unknown tool: {name}")),
        }
    }
}
