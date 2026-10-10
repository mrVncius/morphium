//! Response types for the chat completion endpoint.
//!
//! The envelope differs from OpenAI's: `object` is omitted, `request_id` is
//! added, `finish_reason` carries a wider set of values than the classic four,
//! and the prompt cache count lives under `prompt_tokens_details.cached_tokens`.
//! Every field the vendor may omit is `#[serde(default)]`.

use serde::{Deserialize, Serialize};

use super::message::Message;

/// Chat completion response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    /// Task ID.
    #[serde(default)]
    pub id: String,

    /// Request ID echoed by the platform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,

    /// Object type; omitted by Z.AI but present on compatible endpoints.
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

    /// Why generation stopped (`stop`, `tool_calls`, `length`, …).
    #[serde(rename = "finish_reason", default)]
    pub finish_reason: Option<String>,
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

    /// Breakdown of the prompt tokens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_tokens_details: Option<PromptTokensDetails>,
}

/// Breakdown of the prompt tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTokensDetails {
    /// Tokens served from the context cache.
    #[serde(default)]
    pub cached_tokens: u32,
}
