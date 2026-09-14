//! API key pattern scanner.
//!
//! Scans environment variable names and values to detect common API keys
//! (OpenAI, AWS, GitHub, etc.) and classify them by provider.

use std::collections::HashMap;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::error::AetherisError;

/// A detected API key candidate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyDetection {
    pub var_name: String,
    pub provider: String,
    pub confidence: DetectionConfidence,
    pub scope: String,
    pub hint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DetectionConfidence {
    High,
    Medium,
    Low,
}

/// Pattern definition for a provider's API key.
struct ProviderPattern {
    pub provider: &'static str,
    pub var_patterns: Vec<Regex>,
    pub value_patterns: Vec<Regex>,
    pub confidence: DetectionConfidence,
}

/// Builds the default set of provider patterns.
fn default_patterns() -> Vec<ProviderPattern> {
    vec![
        ProviderPattern {
            provider: "openai",
            var_patterns: vec![
                Regex::new(r"(?i)OPENAI[_-]?API[_-]?KEY").unwrap(),
                Regex::new(r"(?i)OPENAI[_-]?KEY").unwrap(),
            ],
            value_patterns: vec![Regex::new(r"^sk-[A-Za-z0-9]{20,}$").unwrap()],
            confidence: DetectionConfidence::High,
        },
        ProviderPattern {
            provider: "anthropic",
            var_patterns: vec![Regex::new(r"(?i)ANTHROPIC[_-]?API[_-]?KEY").unwrap()],
            value_patterns: vec![Regex::new(r"^sk-ant-[A-Za-z0-9\-]{20,}$").unwrap()],
            confidence: DetectionConfidence::High,
        },
        ProviderPattern {
            provider: "aws",
            var_patterns: vec![
                Regex::new(r"(?i)AWS[_-]?ACCESS[_-]?KEY[_-]?ID").unwrap(),
                Regex::new(r"(?i)AWS[_-]?SECRET[_-]?ACCESS[_-]?KEY").unwrap(),
            ],
            value_patterns: vec![],
            confidence: DetectionConfidence::High,
        },
        ProviderPattern {
            provider: "github",
            var_patterns: vec![
                Regex::new(r"(?i)GITHUB[_-]?TOKEN").unwrap(),
                Regex::new(r"(?i)GH[_-]?TOKEN").unwrap(),
            ],
            value_patterns: vec![Regex::new(r"^gh[pousr]_[A-Za-z0-9]{36,}$").unwrap()],
            confidence: DetectionConfidence::High,
        },
        ProviderPattern {
            provider: "azure",
            var_patterns: vec![Regex::new(r"(?i)AZURE[_-]?KEY").unwrap()],
            value_patterns: vec![],
            confidence: DetectionConfidence::Medium,
        },
        ProviderPattern {
            provider: "google",
            var_patterns: vec![
                Regex::new(r"(?i)GOOGLE[_-]?API[_-]?KEY").unwrap(),
                Regex::new(r"(?i)GCP[_-]?KEY").unwrap(),
            ],
            value_patterns: vec![],
            confidence: DetectionConfidence::Medium,
        },
        ProviderPattern {
            provider: "stripe",
            var_patterns: vec![Regex::new(r"(?i)STRIPE[_-]?KEY").unwrap()],
            value_patterns: vec![Regex::new(r"^sk_(live|test)_[A-Za-z0-9]{20,}$").unwrap()],
            confidence: DetectionConfidence::High,
        },
        ProviderPattern {
            provider: "generic_api_key",
            var_patterns: vec![Regex::new(r"(?i).*[_-]?API[_-]?KEY$").unwrap()],
            value_patterns: vec![],
            confidence: DetectionConfidence::Medium,
        },
        ProviderPattern {
            provider: "generic_token",
            var_patterns: vec![Regex::new(r"(?i).*[_-]?TOKEN$").unwrap()],
            value_patterns: vec![],
            confidence: DetectionConfidence::Low,
        },
        ProviderPattern {
            provider: "generic_secret",
            var_patterns: vec![Regex::new(r"(?i).*[_-]?SECRET$").unwrap()],
            value_patterns: vec![],
            confidence: DetectionConfidence::Low,
        },
    ]
}

/// Scans a collection of env var names/values and returns detected API keys.
///
/// # Errors
///
/// Returns `AetherisError` on scan failure.
///
/// # Example
///
/// ```
/// use aetheris_core::env::scanner::scan_for_api_keys;
/// let detections = scan_for_api_keys(&[("OPENAI_API_KEY".into(), "sk-proj-xyz".into())]).unwrap();
/// assert_eq!(detections.len(), 1);
/// assert_eq!(detections[0].provider, "openai");
/// ```
pub fn scan_for_api_keys(vars: &[(String, String)]) -> Result<Vec<ApiKeyDetection>, AetherisError> {
    let patterns = default_patterns();
    let mut detections = Vec::new();

    for (name, value) in vars {
        for pattern in &patterns {
            let var_match = pattern.var_patterns.iter().any(|re| re.is_match(name));
            let val_match = if value.len() > 8 && value.len() < 512 {
                pattern.value_patterns.iter().any(|re| re.is_match(value))
            } else {
                false
            };

            if var_match || val_match {
                let confidence = if val_match {
                    DetectionConfidence::High
                } else {
                    pattern.confidence.clone()
                };
                let hint = format!(
                    "Detected {} API key in var '{}'",
                    pattern.provider, name
                );
                detections.push(ApiKeyDetection {
                    var_name: name.clone(),
                    provider: pattern.provider.to_string(),
                    confidence,
                    scope: "unknown".to_string(),
                    hint,
                });
                break; // Only report highest-priority match per var
            }
        }
    }

    Ok(detections)
}

/// Scans only env var names (no values) for API key patterns.
pub fn scan_names_only(names: &[String]) -> Result<Vec<ApiKeyDetection>, AetherisError> {
    let tuples: Vec<(String, String)> = names.iter().map(|n| (n.clone(), String::new())).collect();
    scan_for_api_keys(&tuples)
}

/// Scans a single .env file content for API key assignments.
pub fn scan_env_content(content: &str) -> Result<Vec<ApiKeyDetection>, AetherisError> {
    let line_re = Regex::new(r"(?m)^([A-Za-z_][A-Za-z0-9_]*)\s*=\s*(.+?)\s*$").map_err(|e| {
        AetherisError::CryptoError(format!("regex build: {e}"))
    })?;

    let vars: Vec<(String, String)> = line_re
        .captures_iter(content)
        .filter_map(|cap| {
            let name = cap.get(1)?.as_str().to_string();
            let value = cap.get(2)?.as_str().trim_matches('"').trim_matches('\'').to_string();
            Some((name, value))
        })
        .collect();

    scan_for_api_keys(&vars)
}

/// Returns a map of provider -> count for detections.
pub fn summarize_by_provider(detections: &[ApiKeyDetection]) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for d in detections {
        *map.entry(d.provider.clone()).or_insert(0) += 1;
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_openai_key() {
        let vars = vec![("OPENAI_API_KEY".into(), "sk-proj-abcdef123456".into())];
        let detections = scan_for_api_keys(&vars).unwrap();
        assert_eq!(detections.len(), 1);
        assert_eq!(detections[0].provider, "openai");
    }

    #[test]
    fn detects_github_token() {
        let vars = vec![("GITHUB_TOKEN".into(), "ghp_abc123".into())];
        let detections = scan_for_api_keys(&vars).unwrap();
        assert_eq!(detections.len(), 1);
        assert_eq!(detections[0].provider, "github");
    }

    #[test]
    fn no_detection_for_plain_vars() {
        let vars = vec![
            ("PATH".into(), "C:\\Windows".into()),
            ("HOME".into(), "/home/user".into()),
        ];
        let detections = scan_for_api_keys(&vars).unwrap();
        assert!(detections.is_empty());
    }

    #[test]
    fn scan_env_content_finds_keys() {
        let content = r#"
DATABASE_URL=postgres://localhost
OPENAI_API_KEY=sk-proj-xyz
DEBUG=true
"#;
        let detections = scan_env_content(content).unwrap();
        assert_eq!(detections.len(), 1);
        assert_eq!(detections[0].var_name, "OPENAI_API_KEY");
    }

    #[test]
    fn summarize_by_provider_counts() {
        let detections = vec![
            ApiKeyDetection {
                var_name: "A".into(),
                provider: "openai".into(),
                confidence: DetectionConfidence::High,
                scope: "user".into(),
                hint: "".into(),
            },
            ApiKeyDetection {
                var_name: "B".into(),
                provider: "openai".into(),
                confidence: DetectionConfidence::High,
                scope: "user".into(),
                hint: "".into(),
            },
            ApiKeyDetection {
                var_name: "C".into(),
                provider: "github".into(),
                confidence: DetectionConfidence::High,
                scope: "user".into(),
                hint: "".into(),
            },
        ];
        let summary = summarize_by_provider(&detections);
        assert_eq!(summary.get("openai"), Some(&2));
        assert_eq!(summary.get("github"), Some(&1));
    }
}
