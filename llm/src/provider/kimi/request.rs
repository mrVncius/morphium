//! Request types for the chat completion endpoint.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::message::InputMessage;
use super::params::{MaxTokens, TopLogProbs};
use super::tool::{Tool, ToolChoice};

/// Chat completion request body (`POST /v1/chat/completions`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionRequest {
    /// Model ID to use.
    pub model: String,

    /// Conversation messages.
    pub messages: Vec<InputMessage>,

    /// Tools the model may call.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tool>,

    /// Tool-selection strategy. `required` is accepted by `kimi-k3` only;
    /// `kimi-k2.6` and `kimi-k2.7-code` reject it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,

    /// Thinking-mode configuration (K2.x models). `kimi-k3` always reasons and
    /// is configured through [`Self::reasoning_effort`] instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thinking: Option<ThinkingConfig>,

    /// Reasoning effort (`kimi-k3`): `low`, `high`, or `max` (default `max`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ReasoningEffort>,

    /// Maximum tokens to generate (Kimi's current name for `max_tokens`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<MaxTokens>,

    /// Output format: text, JSON object, or a JSON schema.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,

    /// Stop words (at most 5, each ≤ 32 bytes); the matched word is dropped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop: Option<StopSequence>,

    /// Whether to stream the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Streaming options (e.g. `include_usage`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,

    /// Whether to return the log probability of each output token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,

    /// Number of most-likely tokens to return at each position (`0 … 20`).
    /// Requires `logprobs = true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<TopLogProbs>,

    /// Predicted-output configuration, to speed up regeneration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prediction: Option<Prediction>,

    /// Cache key for similar requests, to raise the prefix-cache hit rate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,

    /// Context-cache write options. Omitted, the server writes to the `5m`
    /// tier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_cache_options: Option<PromptCacheOptions>,

    /// Stable end-user identifier, hashed to avoid sending identifying data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub safety_identifier: Option<String>,
}

/// Thinking-mode configuration (K2.x models).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThinkingConfig {
    /// Whether thinking is `enabled` or `disabled`.
    #[serde(rename = "type")]
    pub thinking_type: ThinkingType,

    /// Preserved Thinking: `"all"` keeps historical `reasoning_content` in
    /// context; omitted means not kept.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keep: Option<ThinkingKeep>,
}

/// Thinking-mode type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThinkingType {
    /// Use thinking mode.
    Enabled,
    /// Use non-thinking mode (`kimi-k2.7-code` rejects this).
    Disabled,
}

/// Preserved Thinking setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThinkingKeep {
    /// Keep historical `reasoning_content` in context.
    All,
}

/// Reasoning effort level (`kimi-k3`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffort {
    /// Low reasoning effort.
    Low,
    /// High reasoning effort.
    High,
    /// Maximum reasoning effort (default).
    Max,
}

/// Streaming options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamOptions {
    /// Whether to include usage statistics in the final chunk.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_usage: Option<bool>,
}

/// Stop words: a single string or an array of up to 5 strings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StopSequence {
    /// A single stop word.
    Single(String),
    /// Up to 5 stop words.
    Multiple(Vec<String>),
}

/// Response format configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFormat {
    /// Output format type.
    #[serde(rename = "type")]
    pub format_type: ResponseFormatType,

    /// Schema definition, required when `format_type` is `JsonSchema`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<JsonSchema>,
}

/// Response format type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormatType {
    /// Plain text output.
    Text,
    /// JSON-object output.
    JsonObject,
    /// Structured output constrained to a JSON schema.
    JsonSchema,
}

/// A named JSON schema for structured output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonSchema {
    /// Schema name for identification.
    pub name: String,

    /// Whether to strictly enforce the schema (the vendor defaults it to
    /// `true`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,

    /// The JSON schema definition.
    pub schema: Value,
}

/// Predicted-output configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prediction {
    /// The kind of predicted content (always `content`).
    #[serde(rename = "type")]
    pub prediction_type: PredictionType,

    /// The static predicted output.
    pub content: PredictionContent,
}

/// Predicted-output kind (only `content` is supported).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PredictionType {
    /// Static predicted content.
    Content,
}

/// Predicted-output content: a string or an array of text parts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PredictionContent {
    /// Plain text.
    Text(String),
    /// An array of text-only content parts.
    Parts(Vec<PredictionPart>),
}

/// A text content part inside a predicted output.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PredictionPart {
    /// Text content.
    #[serde(rename = "text")]
    Text {
        /// The text.
        text: String,
    },
}

/// Context-cache write options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptCacheOptions {
    /// Cache write mode (only `implicit` is supported).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<CacheMode>,

    /// Lifetime of the written cache.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<CacheTtl>,
}

/// Cache write mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CacheMode {
    /// Automatically write the request prefix to the cache.
    Implicit,
}

/// Cache lifetime tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CacheTtl {
    /// 5 minutes.
    #[serde(rename = "5m")]
    FiveMinutes,
    /// 1 hour.
    #[serde(rename = "1h")]
    OneHour,
}
