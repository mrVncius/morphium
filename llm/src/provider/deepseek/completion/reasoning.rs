//! Reasoning effort level for thinking mode.

use derive_more::Display;
use serde::{Deserialize, Serialize};

/// Reasoning effort level for thinking mode.
///
/// | Variant | API value | Effect |
/// |---------|-----------|--------|
/// | `High`  | `"high"`  | Default for regular requests |
/// | `Max`   | `"max"`   | For complex agent requests |
///
/// Note: `low`/`medium` are mapped to `high` by the server, and `xhigh` is
/// mapped to `max`. Only `high` and `max` are valid inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum ReasoningEffort {
    /// Default reasoning effort for regular requests.
    #[serde(rename = "high")]
    #[display("high")]
    High,
    /// Maximum reasoning effort for complex agent requests.
    #[serde(rename = "max")]
    #[display("max")]
    Max,
}
