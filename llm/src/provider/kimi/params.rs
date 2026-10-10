//! Numeric newtypes for request parameters.
//!
//! Each wraps a raw number in a domain-specific type. Kimi fixes `temperature`,
//! `top_p` and `n` per model and rejects any other value, so those are absent:
//! only the tunable knobs are modelled.

use derive_more::{Deref, Display, Into};
use serde::{Deserialize, Serialize};

/// Maximum number of tokens the model is allowed to generate.
///
/// Sent as `max_completion_tokens` (Kimi deprecated `max_tokens`). The default
/// varies by model — `kimi-k3` defaults to 131072 and accepts up to 1048576.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, Into, Deref)]
#[display("{_0}")]
#[serde(transparent)]
pub struct MaxTokens(u32);

impl MaxTokens {
    /// Create a new `MaxTokens`.
    pub fn new(value: u32) -> Self {
        Self(value)
    }
}

impl From<u32> for MaxTokens {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

/// Number of most-likely tokens to return at each position (`0 … 20`).
///
/// Requires `logprobs = true` on the request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, Into, Deref)]
#[display("{_0}")]
#[serde(transparent)]
pub struct TopLogProbs(u8);

impl TopLogProbs {
    /// Minimum allowed value (`0`).
    pub const MIN: Self = Self(0);
    /// Maximum allowed value (`20`).
    pub const MAX: Self = Self(20);

    /// Create a new `TopLogProbs`, clamped to `[0, 20]`.
    pub fn new(value: u8) -> Self {
        Self(value.clamp(Self::MIN.0, Self::MAX.0))
    }
}

impl From<u8> for TopLogProbs {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}
