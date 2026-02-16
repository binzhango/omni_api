use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::types::{Capability, ErrorCode, OmniError, ProviderId};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CanonicalEnvelope {
    pub capability: Capability,
    pub provider: ProviderSelection,
    pub request: ChatRequest,
    #[serde(default)]
    pub metadata: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProviderSelection {
    pub preferred: Vec<ProviderId>,
    #[serde(default)]
    pub availability: HashMap<ProviderId, ProviderAvailability>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProviderAvailability {
    pub available: bool,
    #[serde(default)]
    pub compatible: Option<bool>,
    #[serde(default)]
    pub reason: Option<String>,
    #[serde(default)]
    pub last_seen_healthy_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(default)]
    pub tools: Vec<ToolDefinition>,
    #[serde(default)]
    pub tool_choice: Option<ToolChoice>,
    #[serde(default)]
    pub response_format: Option<ResponseFormat>,
    #[serde(default)]
    pub generation: GenerationConfig,
    #[serde(default)]
    pub stream: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: Vec<ContentPart>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub tool_call_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentPart {
    Text { text: String },
    ImageUrl { url: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolDefinition {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub input_schema: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ToolChoice {
    Mode(ToolChoiceMode),
    Named { name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ToolChoiceMode {
    Auto,
    None,
    Required,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponseFormat {
    #[serde(rename = "type")]
    pub format_type: ResponseFormatType,
    #[serde(default)]
    pub json_schema: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormatType {
    Text,
    JsonSchema,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GenerationConfig {
    #[serde(default = "default_temperature")]
    pub temperature: f32,
    #[serde(default = "default_top_p")]
    pub top_p: f32,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    #[serde(default)]
    pub stop: Vec<String>,
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            temperature: default_temperature(),
            top_p: default_top_p(),
            max_tokens: default_max_tokens(),
            stop: Vec::new(),
        }
    }
}

fn default_temperature() -> f32 {
    0.7
}

fn default_top_p() -> f32 {
    1.0
}

fn default_max_tokens() -> u32 {
    1024
}

pub fn validate_envelope(envelope: &CanonicalEnvelope) -> Result<(), OmniError> {
    let mut violations = Vec::new();

    if envelope.provider.preferred.is_empty() {
        violations.push("provider.preferred must contain at least one provider".to_string());
    }

    if envelope.request.model.trim().is_empty() {
        violations.push("request.model must not be empty".to_string());
    }

    if envelope.request.messages.is_empty() {
        violations.push("request.messages must contain at least one message".to_string());
    }

    for (msg_idx, message) in envelope.request.messages.iter().enumerate() {
        if message.content.is_empty() {
            violations.push(format!(
                "request.messages[{msg_idx}].content must contain at least one part"
            ));
        }

        for (part_idx, part) in message.content.iter().enumerate() {
            match part {
                ContentPart::Text { text } if text.trim().is_empty() => violations.push(format!(
                    "request.messages[{msg_idx}].content[{part_idx}] text must not be empty"
                )),
                ContentPart::ImageUrl { url } if url.trim().is_empty() => violations.push(format!(
                    "request.messages[{msg_idx}].content[{part_idx}] url must not be empty"
                )),
                _ => {}
            }
        }
    }

    if !(0.0..=2.0).contains(&envelope.request.generation.temperature) {
        violations.push("request.generation.temperature must be between 0.0 and 2.0".to_string());
    }

    if !(0.0..=1.0).contains(&envelope.request.generation.top_p) {
        violations.push("request.generation.top_p must be between 0.0 and 1.0".to_string());
    }

    if envelope.request.generation.max_tokens == 0 {
        violations.push("request.generation.max_tokens must be greater than 0".to_string());
    }

    if violations.is_empty() {
        Ok(())
    } else {
        Err(OmniError::new(
            ErrorCode::InvalidCanonicalRequest,
            "Canonical request validation failed",
            json!({ "violations": violations }),
            false,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Capability;

    fn valid_envelope() -> CanonicalEnvelope {
        CanonicalEnvelope {
            capability: Capability::Chat,
            provider: ProviderSelection {
                preferred: vec![ProviderId::OpenAi],
                availability: HashMap::new(),
            },
            request: ChatRequest {
                model: "gpt-4o-mini".to_string(),
                messages: vec![ChatMessage {
                    role: MessageRole::User,
                    content: vec![ContentPart::Text {
                        text: "hello".to_string(),
                    }],
                    name: None,
                    tool_call_id: None,
                }],
                tools: Vec::new(),
                tool_choice: None,
                response_format: None,
                generation: GenerationConfig::default(),
                stream: false,
            },
            metadata: json!({}),
        }
    }

    #[test]
    fn validates_valid_envelope() {
        let envelope = valid_envelope();
        assert!(validate_envelope(&envelope).is_ok());
    }

    #[test]
    fn fails_for_empty_messages() {
        let mut envelope = valid_envelope();
        envelope.request.messages.clear();
        let err = validate_envelope(&envelope).expect_err("expected validation error");
        assert_eq!(err.code, ErrorCode::InvalidCanonicalRequest);
        assert!(
            err.details
                .to_string()
                .contains("request.messages must contain at least one message")
        );
    }

    #[test]
    fn fails_for_empty_text_part() {
        let mut envelope = valid_envelope();
        envelope.request.messages[0].content = vec![ContentPart::Text {
            text: "   ".to_string(),
        }];
        let err = validate_envelope(&envelope).expect_err("expected validation error");
        assert!(err.details.to_string().contains("text must not be empty"));
    }
}
