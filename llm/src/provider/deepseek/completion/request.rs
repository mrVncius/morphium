//! Request types for the chat completion endpoint.

use derive_more::Display;
use serde::{Deserialize, Serialize};

use super::params::{MaxTokens, Temperature, TopLogProbs, TopP};
use super::reasoning::ReasoningEffort;

use super::message::Message;
use super::tool::{Tool, ToolChoice};

/// Chat completion request body
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ChatCompletionRequest {
    /// Model ID to use
    pub model: String,

    /// List of messages in the conversation
    pub messages: Vec<Message>,

    /// List of tools the model can call
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tool>,

    /// Controls which tool to call ("none", "auto", "required", or specific)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,

    /// Allow the model to request several tools in one turn.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    /// Thinking mode configuration (for reasoning models)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thinking: Option<ThinkingConfig>,

    /// Sampling temperature [0, 2] — higher = more random
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Temperature>,

    /// Nucleus sampling — consider tokens with top_p probability mass
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_p: Option<TopP>,

    /// Maximum tokens to generate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<MaxTokens>,

    /// Number of completions to generate (default 1)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>,

    /// Whether to stream the response
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Streaming options (e.g., include_usage)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,

    /// Up to 16 sequences where the API will stop generating
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopSequence>,

    /// Presence penalty [-2.0, 2.0] - DEPRECATED (no longer supported by DeepSeek)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,

    /// Frequency penalty [-2.0, 2.0] - DEPRECATED (no longer supported by DeepSeek)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,

    /// Response format configuration (text or json_object)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,

    /// Whether to return log probabilities
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,

    /// Number of top logprobs to return (0 … 20, requires logprobs=true)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<TopLogProbs>,

    /// A custom user_id for distinguishing users
    /// Allowed: [a-zA-Z0-9\-_], max length 512
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// Thinking mode configuration
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ThinkingConfig {
    /// Type of thinking mode: "enabled" or "disabled"
    #[serde(rename = "type")]
    pub thinking_type: ThinkingType,

    /// Reasoning effort of the model.
    /// Default is `High` for regular requests; `Max` for complex agent requests.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,
}

/// Thinking mode type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "lowercase")]
pub enum ThinkingType {
    /// If set to `enabled`, then use thinking mode.
    #[display("enabled")]
    Enabled,
    /// If set to `disabled`, then use non-thinking mode.
    #[display("disabled")]
    Disabled,
}

/// Streaming options
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct StreamOptions {
    /// Whether to include usage statistics in the final chunk
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_usage: Option<bool>,
}

/// Stop sequence (can be a single string or array of up to 16 strings)
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
#[serde(untagged)]
pub enum StopSequence {
    /// A single stop sequence.
    Single(String),
    /// An array of up to 16 stop sequences.
    Multiple(Vec<String>),
}

/// Response format configuration
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ResponseFormat {
    /// Response format type: "text" or "json_object"
    #[serde(rename = "type")]
    pub format_type: ResponseFormatType,
}

/// Response format type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormatType {
    /// Plain text output.
    #[display("text")]
    Text,
    /// Enables JSON Output, which guarantees the message the model generates is valid JSON.
    #[display("json_object")]
    JsonObject,
}
