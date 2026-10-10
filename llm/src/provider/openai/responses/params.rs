//! Numeric newtypes for request parameters.

use derive_more::{Deref, Display, Into};
use serde::{Deserialize, Serialize};

/// Sampling temperature (0.0 to 2.0). Higher values make output more random.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Display, Into, Deref)]
#[serde(transparent)]
#[display("{_0}")]
pub struct Temperature(f32);

impl Temperature {
    /// Minimum allowed value (0.0).
    pub const MIN: Self = Self(0.0);
    /// Maximum allowed value (2.0).
    pub const MAX: Self = Self(2.0);
    /// Default value (1.0).
    pub const DEFAULT: Self = Self(1.0);

    /// Create a new Temperature, clamped to [0, 2].
    pub fn new(value: f32) -> Self {
        Self(value.clamp(Self::MIN.0, Self::MAX.0))
    }
}

impl From<f32> for Temperature {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

/// Nucleus sampling parameter (0.0 to 1.0).
///
/// The model considers tokens with `top_p` probability mass only.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Display, Into, Deref)]
#[serde(transparent)]
#[display("{_0}")]
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

/// Number of most-likely tokens to return at each position (0 … 20).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, Into, Deref)]
#[serde(transparent)]
#[display("{_0}")]
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

/// Maximum number of output tokens the model is allowed to generate.
///
/// Per the OpenAI Responses API, the minimum is 16.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, Into, Deref)]
#[serde(transparent)]
#[display("{_0}")]
pub struct MaxOutputTokens(u32);

impl MaxOutputTokens {
    /// Minimum allowed value (16).
    pub const MIN: Self = Self(16);

    /// Create a new `MaxOutputTokens`, floored at 16.
    pub fn new(value: u32) -> Self {
        Self(value.max(Self::MIN.0))
    }
}

impl From<u32> for MaxOutputTokens {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}
