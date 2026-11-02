use gemini_rust::prelude::*;

pub struct GeminiClient {
    client: Gemini,
    model: String,
}

impl GeminiClient {
    pub fn new(
        api_key: String,
        model: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            client: Gemini::new(api_key)?,
            model,
        })
    }

    pub async fn generate(
        &self,
        prompt: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let interaction = self.client
            .create_interaction()
            .with_model(&self.model)
            .with_text(prompt)
            .execute()
            .await?;

        Ok(interaction.output_text().to_string())
    }
}
