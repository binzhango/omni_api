use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::canonical::ProviderSelection;
use crate::types::{ProviderId, ProviderReason, ReasonClass, ReasonCode};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoutingDecision {
    pub selected_provider: Option<ProviderId>,
    pub fallback_candidates: Vec<ProviderId>,
    pub provider_reasons: HashMap<ProviderId, ProviderReason>,
}

pub fn route_availability_first(selection: &ProviderSelection) -> RoutingDecision {
    let mut usable = Vec::new();
    let mut provider_reasons = HashMap::new();

    for provider in &selection.preferred {
        match selection.availability.get(provider) {
            Some(status) if !status.available => {
                provider_reasons.insert(
                    provider.clone(),
                    ProviderReason {
                        class: ReasonClass::Unavailable,
                        code: ReasonCode::HostMarkedUnavailable,
                        retryable: true,
                        detail: json!({
                            "source": "host_snapshot",
                            "reason": status.reason,
                            "last_seen_healthy_at": status.last_seen_healthy_at,
                        }),
                    },
                );
            }
            Some(status) if status.compatible == Some(false) => {
                provider_reasons.insert(
                    provider.clone(),
                    ProviderReason {
                        class: ReasonClass::Incompatible,
                        code: ReasonCode::UnsupportedParam,
                        retryable: false,
                        detail: json!({
                            "source": "host_snapshot",
                            "reason": status.reason,
                        }),
                    },
                );
            }
            _ => usable.push(provider.clone()),
        }
    }

    let selected_provider = usable.first().cloned();
    let fallback_candidates = if usable.len() > 1 {
        usable[1..].to_vec()
    } else {
        Vec::new()
    };

    RoutingDecision {
        selected_provider,
        fallback_candidates,
        provider_reasons,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::canonical::{ProviderAvailability, ProviderSelection};
    use crate::types::ProviderId;

    use super::route_availability_first;

    #[test]
    fn selects_first_usable_provider() {
        let input = ProviderSelection {
            preferred: vec![ProviderId::OpenAi, ProviderId::Gemini],
            availability: HashMap::new(),
        };

        let decision = route_availability_first(&input);
        assert_eq!(decision.selected_provider, Some(ProviderId::OpenAi));
        assert_eq!(decision.fallback_candidates, vec![ProviderId::Gemini]);
    }

    #[test]
    fn skips_unavailable_provider() {
        let mut availability = HashMap::new();
        availability.insert(
            ProviderId::OpenAi,
            ProviderAvailability {
                available: false,
                compatible: None,
                reason: Some("healthcheck down".to_string()),
                last_seen_healthy_at: None,
            },
        );

        let input = ProviderSelection {
            preferred: vec![ProviderId::OpenAi, ProviderId::Gemini],
            availability,
        };

        let decision = route_availability_first(&input);
        assert_eq!(decision.selected_provider, Some(ProviderId::Gemini));
        assert_eq!(decision.fallback_candidates, Vec::<ProviderId>::new());
        assert!(decision.provider_reasons.contains_key(&ProviderId::OpenAi));
    }

    #[test]
    fn skips_incompatible_provider() {
        let mut availability = HashMap::new();
        availability.insert(
            ProviderId::OpenAi,
            ProviderAvailability {
                available: true,
                compatible: Some(false),
                reason: Some("missing feature".to_string()),
                last_seen_healthy_at: None,
            },
        );

        let input = ProviderSelection {
            preferred: vec![ProviderId::OpenAi, ProviderId::Gemini],
            availability,
        };

        let decision = route_availability_first(&input);
        assert_eq!(decision.selected_provider, Some(ProviderId::Gemini));
        assert!(decision.provider_reasons.contains_key(&ProviderId::OpenAi));
    }

    #[test]
    fn handles_full_exhaustion() {
        let mut availability = HashMap::new();
        availability.insert(
            ProviderId::OpenAi,
            ProviderAvailability {
                available: false,
                compatible: None,
                reason: Some("down".to_string()),
                last_seen_healthy_at: None,
            },
        );
        availability.insert(
            ProviderId::Gemini,
            ProviderAvailability {
                available: false,
                compatible: None,
                reason: Some("down".to_string()),
                last_seen_healthy_at: None,
            },
        );

        let input = ProviderSelection {
            preferred: vec![ProviderId::OpenAi, ProviderId::Gemini],
            availability,
        };

        let decision = route_availability_first(&input);
        assert_eq!(decision.selected_provider, None);
        assert_eq!(decision.fallback_candidates, Vec::<ProviderId>::new());
    }
}
