//! Domain-specific numeric request parameters.

use derive_more::{Deref, Display, Into};
use serde::{Deserialize, Serialize};

/// Sampling temperature in [0, 2].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Display, Into, Deref)]
#[display("{_0}")]
#[serde(transparent)]
pub struct Temperature(f32);

impl Temperature {
    /// Minimum temperature.
    pub const MIN: Self = Self(0.0);
    /// Maximum temperature.
    pub const MAX: Self = Self(2.0);
    /// Default temperature.
    pub const DEFAULT: Self = Self(1.0);

    /// Clamp to the supported range.
    pub fn new(value: f32) -> Self {
        Self(value.clamp(Self::MIN.0, Self::MAX.0))
    }
}

impl From<f32> for Temperature {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

/// Nucleus sampling in [0, 1]; DeepSeek uses [0.95, 1] in thinking mode.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Display, Into, Deref)]
#[display("{_0}")]
#[serde(transparent)]
pub struct TopP(f32);

impl TopP {
    /// Minimum value.
    pub const MIN: Self = Self(0.0);
    /// Maximum value.
    pub const MAX: Self = Self(1.0);
    /// Default value.
    pub const DEFAULT: Self = Self(1.0);

    /// Clamp to the supported request range.
    pub fn new(value: f32) -> Self {
        Self(value.clamp(Self::MIN.0, Self::MAX.0))
    }
}

impl From<f32> for TopP {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

/// Maximum generated tokens, including reasoning tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, Into, Deref)]
#[display("{_0}")]
#[serde(transparent)]
pub struct MaxOutputTokens(u32);

impl MaxOutputTokens {
    /// Create a token limit.
    pub fn new(value: u32) -> Self {
        Self(value)
    }
}

impl From<u32> for MaxOutputTokens {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

/// Number of most likely tokens returned per position, in [0, 20].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, Into, Deref)]
#[display("{_0}")]
#[serde(transparent)]
pub struct TopLogProbs(u8);

impl TopLogProbs {
    /// Minimum value.
    pub const MIN: Self = Self(0);
    /// Maximum value.
    pub const MAX: Self = Self(20);

    /// Clamp to the supported range.
    pub fn new(value: u8) -> Self {
        Self(value.clamp(Self::MIN.0, Self::MAX.0))
    }
}

impl From<u8> for TopLogProbs {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}
