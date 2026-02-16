use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Capability {
    Chat,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProviderId {
    #[serde(rename = "openai")]
    OpenAi,
    #[serde(rename = "gemini")]
    Gemini,
    #[serde(rename = "ollama")]
    Ollama,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AdapterVersion(pub String);

impl Default for AdapterVersion {
    fn default() -> Self {
        Self("v1".to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReasonClass {
    Unavailable,
    Incompatible,
    AdapterFailure,
    ConfigError,
    PolicyBlocked,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReasonCode {
    HealthcheckDown,
    HostMarkedUnavailable,
    CircuitOpen,
    RateLimitCooldown,
    TimeoutRecent,
    UnsupportedModel,
    UnsupportedToolCalling,
    UnsupportedResponseFormat,
    UnsupportedMultimodalContent,
    UnsupportedParam,
    PayloadBuildError,
    CanonicalToProviderMappingFailed,
    SerializationError,
    AdapterVersionMismatch,
    InternalAdapterError,
    MissingProviderConfig,
    InvalidProviderConfig,
    MissingAdapterRegistration,
    InvalidRoutingConfig,
    InvalidApiVersionTarget,
    ProviderNotAllowed,
    ModelNotAllowed,
    FeatureNotAllowed,
    DataClassificationBlocked,
    RegionRestricted,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProviderReason {
    pub class: ReasonClass,
    pub code: ReasonCode,
    pub retryable: bool,
    #[serde(default)]
    pub detail: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    InvalidCanonicalRequest,
    MissingRequiredField,
    UnsupportedFeatureForProvider,
    ProviderPayloadBuildFailed,
    NoProviderAvailable,
    RoutingConfigInvalid,
    InternalMappingError,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OmniError {
    pub code: ErrorCode,
    pub message: String,
    #[serde(default)]
    pub details: Value,
    pub retryable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Warning {
    pub code: String,
    pub message: String,
    #[serde(default)]
    pub detail: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Diagnostics {
    #[serde(default)]
    pub dropped_fields: Vec<String>,
    #[serde(default)]
    pub coercions: Vec<String>,
    #[serde(default)]
    pub mapping_trace: Vec<String>,
    #[serde(default)]
    pub provider_reasons: HashMap<ProviderId, ProviderReason>,
    #[serde(default)]
    pub attempted_providers: Vec<ProviderId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransformResult {
    pub ok: bool,
    pub selected_provider: Option<ProviderId>,
    pub provider_payload: Option<Value>,
    #[serde(default)]
    pub fallback_candidates: Vec<ProviderId>,
    #[serde(default)]
    pub warnings: Vec<Warning>,
    pub diagnostics: Diagnostics,
    pub error: Option<OmniError>,
}

impl TransformResult {
    pub fn success(
        selected_provider: ProviderId,
        provider_payload: Value,
        fallback_candidates: Vec<ProviderId>,
        warnings: Vec<Warning>,
        diagnostics: Diagnostics,
    ) -> Self {
        Self {
            ok: true,
            selected_provider: Some(selected_provider),
            provider_payload: Some(provider_payload),
            fallback_candidates,
            warnings,
            diagnostics,
            error: None,
        }
    }

    pub fn failure(
        error: OmniError,
        fallback_candidates: Vec<ProviderId>,
        warnings: Vec<Warning>,
        diagnostics: Diagnostics,
    ) -> Self {
        Self {
            ok: false,
            selected_provider: None,
            provider_payload: None,
            fallback_candidates,
            warnings,
            diagnostics,
            error: Some(error),
        }
    }
}

impl OmniError {
    pub fn new(
        code: ErrorCode,
        message: impl Into<String>,
        details: Value,
        retryable: bool,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            details,
            retryable,
        }
    }
}
