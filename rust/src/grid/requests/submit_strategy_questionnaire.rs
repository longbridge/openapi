use std::collections::HashMap;

use serde::Serialize;

/// Options for submitting the strategy risk-disclosure questionnaire record.
///
/// This records the user's consent to the strategy risk disclosure required
/// before using grid trading. The default body is
/// `{ "type": "strategy", "items": { "agree": "true" } }`.
#[derive(Debug, Serialize, Clone)]
pub struct SubmitStrategyQuestionnaireOptions {
    r#type: String,
    items: HashMap<String, String>,
}

impl Default for SubmitStrategyQuestionnaireOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl SubmitStrategyQuestionnaireOptions {
    /// Create a new `SubmitStrategyQuestionnaireOptions` with the strategy
    /// consent body.
    #[inline]
    pub fn new() -> Self {
        let mut items = HashMap::new();
        items.insert("agree".to_string(), "true".to_string());
        Self {
            r#type: "strategy".to_string(),
            items,
        }
    }
}
