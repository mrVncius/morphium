//! Request types for the chat completion endpoint.

use serde::{Deserialize, Serialize};

use super::message::Message;
use super::params::{MaxTokens, Temperature, TopP};
use super::tool::Tool;

/// Chat completion request body (`POST /paas/v4/chat/completions`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    /// Model ID to use.
    pub model: String,

    /// Conversation messages.
    pub messages: Vec<Message>,

    /// Tools the model may call.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tool>,

    /// Tool-selection strategy. Z.AI currently accepts only `"auto"`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<String>,

    /// Thinking-mode configuration (control of the chain of thought).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thinking: Option<ThinkingConfig>,

    /// Sampling temperature (`0.0 … 1.0`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Temperature>,

    /// Nucleus sampling (`0.01 … 1.0`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_p: Option<TopP>,

    /// Maximum tokens to generate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<MaxTokens>,

    /// Whether to stream the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Streaming options (e.g. `include_usage`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,

    /// Output format: `text` or `json_object`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,

    /// Per-end-user identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// Thinking-mode configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThinkingConfig {
    /// Whether the chain of thought is `enabled` or `disabled`.
    #[serde(rename = "type")]
    pub thinking_type: ThinkingType,
}

/// Thinking-mode type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThinkingType {
    /// Use thinking mode.
    Enabled,
    /// Use non-thinking mode.
    Disabled,
}

/// Streaming options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamOptions {
    /// Whether to include usage statistics in the final chunk.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_usage: Option<bool>,
}

/// Response format configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFormat {
    /// Output format type.
    #[serde(rename = "type")]
    pub format_type: ResponseFormatType,
}

/// Response format type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormatType {
    /// Plain text output.
    Text,
    /// JSON-object output.
    JsonObject,
}
