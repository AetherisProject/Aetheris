//! Core NLU engine for Aetheris.

use super::NluEngine;
use transformers::pipeline::TextClassificationPipeline;
use anyhow::Result;
use serde_json::json;
use serde_json::Value;

/// A struct to encapsulate the Hugging Face model for NLU.
pub struct NluModel {
    model: TextClassificationPipeline,
}

impl NluModel {
    /// Initialize the model with a Hugging Face model.
    pub fn new() -> Self {
        let model = TextClassificationPipeline::new("bert-base-uncased").unwrap();
        Self { model }
    }
    
    /// Classify a text query using the model.
    pub fn classify(&self, text: &str) -> Result<String> {
        let result = self.model.classify(text)?;
        Ok(result.to_string())
    }
    
    /// Parse and classify a user query using the Hugging Face model.
    /// 
    /// Returns a structured response with classification details.
    pub fn parse_query(&self, query: &str) -> Result<String> {
        let classification = self.classify(query)?;
        let classification_data: Value = json!({
            "query": query,
            "classification": classification,
            "intent": self.detect_intent(query)?,
        });
        Ok(classification_data.to_string())
    }
    
    /// Detect the intent of a user query.
    /// 
    /// Uses a simple keyword-based approach for demonstration.
    fn detect_intent(&self, query: &str) -> Result<String> {
        let keywords = [
            "vault", "password", "key", "secret",
            "login", "authenticate", "recover",
        ];
        for keyword in keywords.iter() {
            if query.contains(keyword) {
                return Ok(format!("Intent: {}", "Vault/Authentication"));
            }
        }
        Ok(format!("Intent: {}", "General"))
    }
}