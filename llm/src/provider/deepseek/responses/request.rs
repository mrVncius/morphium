//! Supported DeepSeek Responses request parameters.

use serde::{Deserialize, Serialize};

use super::{Input, MaxOutputTokens, Temperature, Tool, ToolChoice, TopLogProbs, TopP};

/// Request to `POST /responses`.
/// At least one of `input` and `instructions` is required. Send the complete
/// conversation on each turn; server-side response storage is not supported.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponsesRequest {
    /// Model ID: deepseek-flash or deepseek-v4-pro.
    pub model: String,
    /// Plain text or conversation history.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<Input>,
    /// Instructions inserted as the first system message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Thinking mode and effort.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningConfig>,
    /// Upper bound including visible and reasoning tokens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<MaxOutputTokens>,
    /// Enable semantic SSE events (no `[DONE]` marker).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Sampling temperature in [0, 2]; ignored in thinking mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Temperature>,
    /// Nucleus sampling in [0, 1]; effective range [0.95, 1] in thinking mode.
    /// Fixed at 1 in non-thinking mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_p: Option<TopP>,
    /// Text output format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<ResponseTextConfig>,
    /// Available function or apply_patch tools.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tool>,
    /// Tool selection policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    /// Number of most likely tokens returned per position, in [0, 20].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<TopLogProbs>,
    /// End-user identifier: [a-zA-Z0-9_-], at most 512 characters; no private data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Thinking mode configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReasoningConfig {
    /// Effort; omitted uses the model's default (thinking enabled).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effort: Option<ReasoningEffort>,
    /// Accepted for compatibility, but no summary is generated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

/// Native effort levels and accepted compatibility spellings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningEffort {
    /// Disable thinking.
    None,
    /// Low effort.
    Low,
    /// High effort.
    High,
    /// Maximum effort.
    Max,
    /// Compatibility spelling for low.
    Minimal,
    /// Compatibility spelling for high.
    Medium,
    /// Compatibility spelling for high.
    Xhigh,
}

/// Text output configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResponseTextConfig {
    /// Output format; defaults to plain text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<ResponseFormat>,
    /// Accepted for compatibility, but ignored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<String>,
}

/// Plain text, JSON mode, or structured JSON output.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResponseFormat {
    /// Plain text.
    Text,
    /// JSON mode.
    JsonObject,
    /// Structured output conforming to a named JSON schema.
    JsonSchema {
        /// Schema name.
        name: String,
        /// JSON schema definition.
        schema: serde_json::Value,
        /// Schema description.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Strict schema adherence.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        strict: Option<bool>,
    },
}
