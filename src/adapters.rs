use std::collections::HashMap;
use std::sync::Arc;

use serde_json::{Value, json};

use crate::canonical::{
    ChatMessage, ChatRequest, ContentPart, MessageRole, ResponseFormatType, ToolChoice,
};
use crate::types::{
    AdapterVersion, ErrorCode, OmniError, ProviderId, ProviderReason, ReasonClass, ReasonCode,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AdapterKey {
    pub provider_id: ProviderId,
    pub capability: crate::types::Capability,
    pub adapter_version: AdapterVersion,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SupportReport {
    pub supported: bool,
    pub reason: Option<ProviderReason>,
}

impl SupportReport {
    pub fn supported() -> Self {
        Self {
            supported: true,
            reason: None,
        }
    }

    pub fn unsupported(reason: ProviderReason) -> Self {
        Self {
            supported: false,
            reason: Some(reason),
        }
    }
}

pub trait ProviderAdapter: Send + Sync {
    fn provider_id(&self) -> ProviderId;
    fn capability(&self) -> crate::types::Capability;
    fn adapter_version(&self) -> AdapterVersion;
    fn supports(&self, request: &ChatRequest) -> SupportReport;
    fn build_payload(&self, request: &ChatRequest) -> Result<Value, OmniError>;
}

#[derive(Default)]
pub struct AdapterRegistry {
    entries: HashMap<AdapterKey, Arc<dyn ProviderAdapter>>,
}

impl AdapterRegistry {
    pub fn register(&mut self, adapter: Arc<dyn ProviderAdapter>) {
        let key = AdapterKey {
            provider_id: adapter.provider_id(),
            capability: adapter.capability(),
            adapter_version: adapter.adapter_version(),
        };
        self.entries.insert(key, adapter);
    }

    pub fn get(&self, key: &AdapterKey) -> Option<Arc<dyn ProviderAdapter>> {
        self.entries.get(key).cloned()
    }
}

#[derive(Default)]
pub struct OpenAiChatAdapter;

#[derive(Default)]
pub struct GeminiChatAdapter;

#[derive(Default)]
pub struct OllamaChatAdapter;

impl ProviderAdapter for OpenAiChatAdapter {
    fn provider_id(&self) -> ProviderId {
        ProviderId::OpenAi
    }

    fn capability(&self) -> crate::types::Capability {
        crate::types::Capability::Chat
    }

    fn adapter_version(&self) -> AdapterVersion {
        AdapterVersion::default()
    }

    fn supports(&self, _request: &ChatRequest) -> SupportReport {
        SupportReport::supported()
    }

    fn build_payload(&self, request: &ChatRequest) -> Result<Value, OmniError> {
        let messages = request
            .messages
            .iter()
            .map(map_openai_message)
            .collect::<Result<Vec<_>, _>>()?;

        let mut payload = json!({
            "model": request.model,
            "messages": messages,
            "temperature": request.generation.temperature,
            "top_p": request.generation.top_p,
            "max_tokens": request.generation.max_tokens,
            "stream": request.stream,
        });

        if let Some(response_format) = &request.response_format
            && response_format.format_type == ResponseFormatType::JsonSchema
        {
            payload["response_format"] = json!({
                "type": "json_schema",
                "json_schema": response_format.json_schema,
            });
        }

        Ok(payload)
    }
}

impl ProviderAdapter for GeminiChatAdapter {
    fn provider_id(&self) -> ProviderId {
        ProviderId::Gemini
    }

    fn capability(&self) -> crate::types::Capability {
        crate::types::Capability::Chat
    }

    fn adapter_version(&self) -> AdapterVersion {
        AdapterVersion::default()
    }

    fn supports(&self, request: &ChatRequest) -> SupportReport {
        if request
            .response_format
            .as_ref()
            .map(|f| f.format_type == ResponseFormatType::JsonSchema)
            .unwrap_or(false)
        {
            return SupportReport::unsupported(ProviderReason {
                class: ReasonClass::Incompatible,
                code: ReasonCode::UnsupportedResponseFormat,
                retryable: false,
                detail: json!({ "feature": "response_format.json_schema" }),
            });
        }

        SupportReport::supported()
    }

    fn build_payload(&self, request: &ChatRequest) -> Result<Value, OmniError> {
        let contents = request
            .messages
            .iter()
            .map(map_gemini_content)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(json!({
            "model": request.model,
            "contents": contents,
            "generationConfig": {
                "temperature": request.generation.temperature,
                "topP": request.generation.top_p,
                "maxOutputTokens": request.generation.max_tokens,
            }
        }))
    }
}

impl ProviderAdapter for OllamaChatAdapter {
    fn provider_id(&self) -> ProviderId {
        ProviderId::Ollama
    }

    fn capability(&self) -> crate::types::Capability {
        crate::types::Capability::Chat
    }

    fn adapter_version(&self) -> AdapterVersion {
        AdapterVersion::default()
    }

    fn supports(&self, request: &ChatRequest) -> SupportReport {
        if request.tools.len() > 0 {
            return SupportReport::unsupported(ProviderReason {
                class: ReasonClass::Incompatible,
                code: ReasonCode::UnsupportedToolCalling,
                retryable: false,
                detail: json!({ "feature": "tools" }),
            });
        }

        if request
            .response_format
            .as_ref()
            .map(|f| f.format_type == ResponseFormatType::JsonSchema)
            .unwrap_or(false)
        {
            return SupportReport::unsupported(ProviderReason {
                class: ReasonClass::Incompatible,
                code: ReasonCode::UnsupportedResponseFormat,
                retryable: false,
                detail: json!({ "feature": "response_format.json_schema" }),
            });
        }

        if request
            .messages
            .iter()
            .flat_map(|m| m.content.iter())
            .any(|part| matches!(part, ContentPart::ImageUrl { .. }))
        {
            return SupportReport::unsupported(ProviderReason {
                class: ReasonClass::Incompatible,
                code: ReasonCode::UnsupportedMultimodalContent,
                retryable: false,
                detail: json!({ "feature": "image_url" }),
            });
        }

        SupportReport::supported()
    }

    fn build_payload(&self, request: &ChatRequest) -> Result<Value, OmniError> {
        let messages = request
            .messages
            .iter()
            .map(map_ollama_message)
            .collect::<Result<Vec<_>, _>>()?;

        let mut payload = json!({
            "model": request.model,
            "messages": messages,
            "stream": request.stream,
            "options": {
                "temperature": request.generation.temperature,
                "top_p": request.generation.top_p,
                "num_predict": request.generation.max_tokens,
            }
        });

        if let Some(ToolChoice::Named { name }) = &request.tool_choice {
            payload["tool_choice"] = json!(name);
        }

        Ok(payload)
    }
}

fn map_openai_message(message: &ChatMessage) -> Result<Value, OmniError> {
    let role = map_role(message.role.clone());

    let content_parts = message
        .content
        .iter()
        .map(|part| match part {
            ContentPart::Text { text } => Ok(json!({ "type": "text", "text": text })),
            ContentPart::ImageUrl { url } => Ok(json!({
                "type": "image_url",
                "image_url": {"url": url},
            })),
        })
        .collect::<Result<Vec<_>, OmniError>>()?;

    Ok(json!({ "role": role, "content": content_parts }))
}

fn map_gemini_content(message: &ChatMessage) -> Result<Value, OmniError> {
    let role = match message.role {
        MessageRole::Assistant => "model",
        _ => "user",
    };

    let parts = message
        .content
        .iter()
        .map(|part| match part {
            ContentPart::Text { text } => Ok(json!({ "text": text })),
            ContentPart::ImageUrl { url } => Ok(json!({
                "file_data": {"mime_type": "image/*", "file_uri": url}
            })),
        })
        .collect::<Result<Vec<_>, OmniError>>()?;

    Ok(json!({ "role": role, "parts": parts }))
}

fn map_ollama_message(message: &ChatMessage) -> Result<Value, OmniError> {
    let content = flatten_text_content(&message.content)?;
    Ok(json!({
        "role": map_role(message.role.clone()),
        "content": content,
    }))
}

fn flatten_text_content(parts: &[ContentPart]) -> Result<String, OmniError> {
    let mut texts = Vec::new();
    for part in parts {
        match part {
            ContentPart::Text { text } => texts.push(text.clone()),
            ContentPart::ImageUrl { .. } => {
                return Err(OmniError::new(
                    ErrorCode::ProviderPayloadBuildFailed,
                    "Ollama text-only payload build failed",
                    json!({ "reason": "image content is not supported for text flattening" }),
                    true,
                ));
            }
        }
    }
    Ok(texts.join("\n"))
}

fn map_role(role: MessageRole) -> &'static str {
    match role {
        MessageRole::System => "system",
        MessageRole::User => "user",
        MessageRole::Assistant => "assistant",
        MessageRole::Tool => "tool",
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::canonical::{
        ChatMessage, ChatRequest, ContentPart, GenerationConfig, MessageRole, ResponseFormat,
        ResponseFormatType,
    };
    use crate::types::{ReasonClass, ReasonCode};

    use super::{GeminiChatAdapter, OllamaChatAdapter, OpenAiChatAdapter, ProviderAdapter};

    fn sample_request() -> ChatRequest {
        ChatRequest {
            model: "model-x".to_string(),
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
        }
    }

    #[test]
    fn openai_adapter_builds_payload() {
        let adapter = OpenAiChatAdapter;
        let payload = adapter.build_payload(&sample_request()).expect("payload");
        assert_eq!(payload["model"], json!("model-x"));
        assert!(payload["messages"].is_array());
    }

    #[test]
    fn gemini_rejects_json_schema_response_format() {
        let adapter = GeminiChatAdapter;
        let mut request = sample_request();
        request.response_format = Some(ResponseFormat {
            format_type: ResponseFormatType::JsonSchema,
            json_schema: Some(json!({"type": "object"})),
        });

        let support = adapter.supports(&request);
        assert!(!support.supported);
        let reason = support.reason.expect("reason");
        assert_eq!(reason.class, ReasonClass::Incompatible);
        assert_eq!(reason.code, ReasonCode::UnsupportedResponseFormat);
    }

    #[test]
    fn ollama_rejects_multimodal_content() {
        let adapter = OllamaChatAdapter;
        let mut request = sample_request();
        request.messages[0].content = vec![ContentPart::ImageUrl {
            url: "https://example.com/image.png".to_string(),
        }];

        let support = adapter.supports(&request);
        assert!(!support.supported);
        let reason = support.reason.expect("reason");
        assert_eq!(reason.code, ReasonCode::UnsupportedMultimodalContent);
    }
}
