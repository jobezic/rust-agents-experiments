use std::fs;

pub struct Workspace;

impl Workspace {
    pub fn initialize() -> Result<Self, Box<dyn std::error::Error>> {
        fs::create_dir_all("workspace/src")?;

        let manifest = r#"
[package]
name = "generated_code"
version = "0.1.0"
edition = "2024"

[dependencies]
"#;

        fs::write("workspace/Cargo.toml", manifest)?;

        Ok(Self)
    }

    pub fn write_source(
        &self,
        code: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        fs::write("workspace/src/lib.rs", code)?;
        Ok(())
    }

    pub fn root(&self) -> &str {
        "workspace"
    }
}
