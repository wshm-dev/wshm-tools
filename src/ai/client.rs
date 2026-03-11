use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use tracing::debug;

use crate::config::Config;

pub struct AiClient {
    http: reqwest::Client,
    api_key: String,
    model: String,
    provider: String,
}

impl AiClient {
    pub fn new(config: &Config) -> Result<Self> {
        let api_key = config.ai_api_key()?;
        Ok(Self {
            http: reqwest::Client::new(),
            api_key,
            model: config.ai.model.clone(),
            provider: config.ai.provider.clone(),
        })
    }

    pub async fn complete<T: DeserializeOwned>(&self, system: &str, user: &str) -> Result<T> {
        let raw = match self.provider.as_str() {
            "anthropic" => self.call_anthropic(system, user).await?,
            "openai" => self.call_openai(system, user).await?,
            other => anyhow::bail!("Unknown AI provider: {other}"),
        };

        debug!("AI response: {raw}");

        // Extract JSON from response (handle markdown code blocks)
        let json_str = extract_json(&raw);
        serde_json::from_str(json_str)
            .with_context(|| format!("Failed to parse AI response as JSON:\n{raw}"))
    }

    async fn call_anthropic(&self, system: &str, user: &str) -> Result<String> {
        let body = serde_json::json!({
            "model": self.model,
            "max_tokens": 4096,
            "system": system,
            "messages": [
                {"role": "user", "content": user}
            ]
        });

        let response = self
            .http
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to call Anthropic API")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("Anthropic API error ({status}): {text}");
        }

        let resp: serde_json::Value = serde_json::from_str(&text)?;
        let content = resp["content"][0]["text"]
            .as_str()
            .context("Missing text in Anthropic response")?;

        Ok(content.to_string())
    }

    async fn call_openai(&self, system: &str, user: &str) -> Result<String> {
        let body = serde_json::json!({
            "model": self.model,
            "messages": [
                {"role": "system", "content": system},
                {"role": "user", "content": user}
            ],
            "max_tokens": 4096,
            "temperature": 0.1
        });

        let response = self
            .http
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to call OpenAI API")?;

        let status = response.status();
        let text = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("OpenAI API error ({status}): {text}");
        }

        let resp: serde_json::Value = serde_json::from_str(&text)?;
        let content = resp["choices"][0]["message"]["content"]
            .as_str()
            .context("Missing content in OpenAI response")?;

        Ok(content.to_string())
    }
}

fn extract_json(text: &str) -> &str {
    let trimmed = text.trim();
    // Handle ```json ... ``` blocks
    if let Some(start) = trimmed.find("```json") {
        let json_start = start + 7;
        if let Some(end) = trimmed[json_start..].find("```") {
            return trimmed[json_start..json_start + end].trim();
        }
    }
    // Handle ``` ... ``` blocks
    if let Some(start) = trimmed.find("```") {
        let json_start = start + 3;
        if let Some(end) = trimmed[json_start..].find("```") {
            return trimmed[json_start..json_start + end].trim();
        }
    }
    trimmed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_json_plain() {
        let input = r#"{"category": "bug"}"#;
        assert_eq!(extract_json(input), input);
    }

    #[test]
    fn test_extract_json_code_block() {
        let input = "```json\n{\"category\": \"bug\"}\n```";
        assert_eq!(extract_json(input), r#"{"category": "bug"}"#);
    }
}
