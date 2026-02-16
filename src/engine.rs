use std::sync::Arc;

use serde_json::json;

use crate::adapters::{
    AdapterKey, AdapterRegistry, GeminiChatAdapter, OllamaChatAdapter, OpenAiChatAdapter,
};
use crate::canonical::{CanonicalEnvelope, validate_envelope};
use crate::routing::route_availability_first;
use crate::types::{
    AdapterVersion, Capability, Diagnostics, ErrorCode, OmniError, ProviderReason, ReasonClass,
    ReasonCode, TransformResult, Warning,
};

pub struct TransformEngine {
    registry: AdapterRegistry,
    default_adapter_version: AdapterVersion,
}

impl Default for TransformEngine {
    fn default() -> Self {
        let mut registry = AdapterRegistry::default();
        registry.register(Arc::new(OpenAiChatAdapter));
        registry.register(Arc::new(GeminiChatAdapter));
        registry.register(Arc::new(OllamaChatAdapter));

        Self {
            registry,
            default_adapter_version: AdapterVersion::default(),
        }
    }
}

impl TransformEngine {
    pub fn transform(&self, envelope: CanonicalEnvelope) -> TransformResult {
        if let Err(validation_error) = validate_envelope(&envelope) {
            return TransformResult::failure(
                validation_error,
                Vec::new(),
                Vec::new(),
                Diagnostics::default(),
            );
        }

        let routing = route_availability_first(&envelope.provider);
        let mut diagnostics = Diagnostics {
            provider_reasons: routing.provider_reasons.clone(),
            ..Diagnostics::default()
        };
        let warnings: Vec<Warning> = Vec::new();

        let mut candidates = Vec::new();
        if let Some(primary) = routing.selected_provider.clone() {
            candidates.push(primary);
            candidates.extend(routing.fallback_candidates.clone());
        }

        for (idx, provider) in candidates.iter().enumerate() {
            diagnostics.attempted_providers.push(provider.clone());

            let key = AdapterKey {
                provider_id: provider.clone(),
                capability: Capability::Chat,
                adapter_version: self.default_adapter_version.clone(),
            };

            let Some(adapter) = self.registry.get(&key) else {
                diagnostics.provider_reasons.insert(
                    provider.clone(),
                    ProviderReason {
                        class: ReasonClass::ConfigError,
                        code: ReasonCode::MissingAdapterRegistration,
                        retryable: false,
                        detail: json!({ "provider": provider }),
                    },
                );
                continue;
            };

            let support = adapter.supports(&envelope.request);
            if !support.supported {
                if let Some(reason) = support.reason {
                    diagnostics
                        .provider_reasons
                        .insert(provider.clone(), reason);
                }
                continue;
            }

            match adapter.build_payload(&envelope.request) {
                Ok(payload) => {
                    let fallback_candidates = if idx + 1 < candidates.len() {
                        candidates[idx + 1..].to_vec()
                    } else {
                        Vec::new()
                    };

                    return TransformResult::success(
                        provider.clone(),
                        payload,
                        fallback_candidates,
                        warnings,
                        diagnostics,
                    );
                }
                Err(err) => {
                    diagnostics.provider_reasons.insert(
                        provider.clone(),
                        ProviderReason {
                            class: ReasonClass::AdapterFailure,
                            code: ReasonCode::PayloadBuildError,
                            retryable: true,
                            detail: json!({
                                "error_code": err.code,
                                "message": err.message,
                            }),
                        },
                    );
                }
            }
        }

        let error = OmniError::new(
            ErrorCode::NoProviderAvailable,
            "No configured providers are currently available",
            json!({
                "attempted_providers": diagnostics.attempted_providers,
                "reasons": diagnostics.provider_reasons,
            }),
            true,
        );

        TransformResult::failure(error, Vec::new(), warnings, diagnostics)
    }
}

pub fn transform(envelope: CanonicalEnvelope) -> TransformResult {
    TransformEngine::default().transform(envelope)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::json;

    use crate::canonical::{
        CanonicalEnvelope, ChatMessage, ChatRequest, ContentPart, GenerationConfig, MessageRole,
        ProviderAvailability, ProviderSelection,
    };
    use crate::types::{Capability, ErrorCode, ProviderId};

    use super::TransformEngine;

    fn base_envelope() -> CanonicalEnvelope {
        let mut availability = HashMap::new();
        availability.insert(
            ProviderId::OpenAi,
            ProviderAvailability {
                available: true,
                compatible: Some(true),
                reason: None,
                last_seen_healthy_at: Some("2026-02-15T00:00:00Z".to_string()),
            },
        );
        availability.insert(
            ProviderId::Gemini,
            ProviderAvailability {
                available: true,
                compatible: Some(true),
                reason: None,
                last_seen_healthy_at: Some("2026-02-15T00:00:00Z".to_string()),
            },
        );

        CanonicalEnvelope {
            capability: Capability::Chat,
            provider: ProviderSelection {
                preferred: vec![ProviderId::OpenAi, ProviderId::Gemini],
                availability,
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
    fn normal_path_returns_success() {
        let engine = TransformEngine::default();
        let result = engine.transform(base_envelope());
        assert!(result.ok);
        assert_eq!(result.selected_provider, Some(ProviderId::OpenAi));
        assert!(result.provider_payload.is_some());
    }

    #[test]
    fn fallback_path_selects_next_provider() {
        let engine = TransformEngine::default();
        let mut envelope = base_envelope();
        envelope.provider.availability.insert(
            ProviderId::OpenAi,
            ProviderAvailability {
                available: false,
                compatible: Some(true),
                reason: Some("down".to_string()),
                last_seen_healthy_at: None,
            },
        );

        let result = engine.transform(envelope);
        assert!(result.ok);
        assert_eq!(result.selected_provider, Some(ProviderId::Gemini));
        assert!(
            result
                .diagnostics
                .provider_reasons
                .contains_key(&ProviderId::OpenAi)
        );
    }

    #[test]
    fn no_provider_available_returns_error() {
        let engine = TransformEngine::default();
        let mut envelope = base_envelope();
        envelope.provider.availability.insert(
            ProviderId::OpenAi,
            ProviderAvailability {
                available: false,
                compatible: Some(true),
                reason: Some("down".to_string()),
                last_seen_healthy_at: None,
            },
        );
        envelope.provider.availability.insert(
            ProviderId::Gemini,
            ProviderAvailability {
                available: false,
                compatible: Some(true),
                reason: Some("down".to_string()),
                last_seen_healthy_at: None,
            },
        );

        let result = engine.transform(envelope);
        assert!(!result.ok);
        let error = result.error.expect("error expected");
        assert_eq!(error.code, ErrorCode::NoProviderAvailable);
    }
}
