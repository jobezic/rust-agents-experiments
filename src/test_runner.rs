use std::process::{Command, Output};

pub struct TestRunner;

pub struct TestResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

impl TestRunner {
    pub fn run(workspace: &str) -> Result<TestResult, std::io::Error> {
        let output: Output = Command::new("cargo")
            .args(["test", "--offline", "--color", "never"])
            .current_dir(workspace)
            .output()?;

        Ok(TestResult {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }
}

impl TestResult {
    pub fn feedback(&self) -> String {
        format!(
            "Test execution result:\n\
             Success: {}\n\
             STDOUT:\n{}\n\
             STDERR:\n{}",
            self.success,
            self.stdout,
            self.stderr
        )
    }
}
