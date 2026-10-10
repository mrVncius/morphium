//! Numeric newtypes for request parameters.
//!
//! Each wraps a raw number in a domain-specific type so callers cannot
//! accidentally pass, say, a temperature where top_p is expected.

use derive_more::{Deref, Display, Into};
use serde::{Deserialize, Serialize};

/// Sampling temperature (0.0 … 2.0).
///
/// Higher values make output more random, lower values more deterministic.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Display, Into, Deref)]
#[display("{_0}")]
#[serde(transparent)]
pub struct Temperature(f32);

impl Temperature {
    /// Minimum allowed value (0.0).
    pub const MIN: Self = Self(0.0);
    /// Maximum allowed value (2.0).
    pub const MAX: Self = Self(2.0);
    /// Default value (1.0).
    pub const DEFAULT: Self = Self(1.0);

    /// Create a new `Temperature`, clamped to [0, 2].
    pub fn new(value: f32) -> Self {
        Self(value.clamp(Self::MIN.0, Self::MAX.0))
    }
}

impl From<f32> for Temperature {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

/// Nucleus sampling parameter (0.0 … 1.0).
///
/// The model considers the tokens with `top_p` probability mass only.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Display, Into, Deref)]
#[display("{_0}")]
#[serde(transparent)]
pub struct TopP(f32);

impl TopP {
    /// Minimum allowed value (0.0).
    pub const MIN: Self = Self(0.0);
    /// Maximum allowed value (1.0).
    pub const MAX: Self = Self(1.0);
    /// Default value (1.0 = consider all tokens).
    pub const DEFAULT: Self = Self(1.0);

    /// Create a new `TopP`, clamped to [0, 1].
    pub fn new(value: f32) -> Self {
        Self(value.clamp(Self::MIN.0, Self::MAX.0))
    }
}

impl From<f32> for TopP {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

/// Maximum number of tokens the model is allowed to generate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, Into, Deref)]
#[display("{_0}")]
#[serde(transparent)]
pub struct MaxTokens(u32);

impl MaxTokens {
    /// Default value (4096).
    pub const DEFAULT: Self = Self(4096);

    /// Create a new `MaxTokens`.
    pub fn new(value: u32) -> Self {
        Self(value)
    }
}

impl From<u32> for MaxTokens {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

/// Number of most-likely tokens to return at each position (0 … 20).
///
/// Requires `logprobs = true` on the request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, Into, Deref)]
#[display("{_0}")]
#[serde(transparent)]
pub struct TopLogProbs(u8);

impl TopLogProbs {
    /// Minimum allowed value (0).
    pub const MIN: Self = Self(0);
    /// Maximum allowed value (20).
    pub const MAX: Self = Self(20);

    /// Create a new `TopLogProbs`, clamped to [0, 20].
    pub fn new(value: u8) -> Self {
        Self(value.clamp(Self::MIN.0, Self::MAX.0))
    }
}

impl From<u8> for TopLogProbs {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}
