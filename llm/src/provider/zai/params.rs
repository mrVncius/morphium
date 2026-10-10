//! Numeric newtypes for request parameters.
//!
//! Each wraps a raw number in a domain-specific type so callers cannot
//! accidentally pass, say, a temperature where `top_p` is expected. The
//! ranges are Z.AI's own (`temperature` in `[0, 1]`, `top_p` in `[0.01, 1]`).

use derive_more::{Deref, Display, Into};
use serde::{Deserialize, Serialize};

/// Sampling temperature (`0.0 … 1.0`).
///
/// Higher values make output more random, lower values more deterministic.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Display, Into, Deref)]
#[display("{_0}")]
#[serde(transparent)]
pub struct Temperature(f32);

impl Temperature {
    /// Minimum allowed value (`0.0`).
    pub const MIN: Self = Self(0.0);
    /// Maximum allowed value (`1.0`).
    pub const MAX: Self = Self(1.0);
    /// Z.AI's default (`1.0`).
    pub const DEFAULT: Self = Self(1.0);

    /// Create a new `Temperature`, clamped to `[0, 1]`.
    pub fn new(value: f32) -> Self {
        Self(value.clamp(Self::MIN.0, Self::MAX.0))
    }
}

impl From<f32> for Temperature {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

/// Nucleus sampling parameter (`0.01 … 1.0`).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Display, Into, Deref)]
#[display("{_0}")]
#[serde(transparent)]
pub struct TopP(f32);

impl TopP {
    /// Minimum allowed value (`0.01`).
    pub const MIN: Self = Self(0.01);
    /// Maximum allowed value (`1.0`).
    pub const MAX: Self = Self(1.0);
    /// Z.AI's default (`0.95`).
    pub const DEFAULT: Self = Self(0.95);

    /// Create a new `TopP`, clamped to `[0.01, 1]`.
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
