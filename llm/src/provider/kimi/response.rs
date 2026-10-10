//! Response types for the chat completion endpoint.

use serde::{Deserialize, Serialize};

use super::message::Message;

/// Chat completion response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    /// Unique identifier for the completion.
    #[serde(default)]
    pub id: String,

    /// Object type (always `chat.completion`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,

    /// Unix timestamp of creation.
    #[serde(default)]
    pub created: u64,

    /// Model that served the completion.
    #[serde(default)]
    pub model: String,

    /// Completion choices.
    #[serde(default)]
    pub choices: Vec<Choice>,

    /// Token usage statistics.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

/// A completion choice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    /// Index of the choice.
    #[serde(default)]
    pub index: u32,

    /// The generated message.
    pub message: Message,

    /// Why generation stopped.
    #[serde(rename = "finish_reason")]
    pub finish_reason: FinishReason,

    /// Per-token log probabilities, present when the request set `logprobs`.
    /// The vendor docs do not specify the shape, so it is carried untyped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<serde_json::Value>,
}

/// Reason why the generation finished.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinishReason {
    /// The model completed the message naturally.
    Stop,
    /// The maximum token limit was reached.
    Length,
    /// The model called one or more tools.
    ToolCalls,
}

/// Token usage statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    /// Tokens in the prompt.
    #[serde(rename = "prompt_tokens", default)]
    pub prompt_tokens: u32,

    /// Tokens in the completion.
    #[serde(rename = "completion_tokens", default)]
    pub completion_tokens: u32,

    /// Total tokens (prompt + completion).
    #[serde(rename = "total_tokens", default)]
    pub total_tokens: u32,

    /// Tokens served from the cache (also mirrored in `prompt_tokens_details`).
    #[serde(default)]
    pub cached_tokens: u32,

    /// Breakdown of the prompt tokens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_tokens_details: Option<PromptTokensDetails>,
}

/// Breakdown of the prompt tokens.
///
/// `cached_tokens`, `cache_write_tokens` and the uncached remainder are
/// mutually exclusive and sum to `prompt_tokens`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTokensDetails {
    /// Tokens served from the cache.
    #[serde(default)]
    pub cached_tokens: u32,

    /// Tokens written to the cache by this request.
    #[serde(default)]
    pub cache_write_tokens: u32,
}
