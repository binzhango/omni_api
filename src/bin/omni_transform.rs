use std::io::{self, Read};

use omni_api::canonical::CanonicalEnvelope;
use omni_api::engine::transform;
use omni_api::types::{Diagnostics, ErrorCode, OmniError, TransformResult};
use serde_json::json;

fn main() {
    let mut input = String::new();
    if let Err(err) = io::stdin().read_to_string(&mut input) {
        emit_error(OmniError::new(
            ErrorCode::InternalMappingError,
            "Failed to read stdin",
            json!({"error": err.to_string()}),
            true,
        ));
        return;
    }

    let envelope: CanonicalEnvelope = match serde_json::from_str(&input) {
        Ok(value) => value,
        Err(err) => {
            emit_error(OmniError::new(
                ErrorCode::InvalidCanonicalRequest,
                "Failed to deserialize canonical request",
                json!({"error": err.to_string()}),
                false,
            ));
            return;
        }
    };

    let result = transform(envelope);
    println!(
        "{}",
        serde_json::to_string_pretty(&result).expect("result serialization should not fail")
    );
}

fn emit_error(error: OmniError) {
    let result = TransformResult::failure(error, Vec::new(), Vec::new(), Diagnostics::default());
    println!(
        "{}",
        serde_json::to_string_pretty(&result).expect("result serialization should not fail")
    );
}
