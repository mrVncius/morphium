//! Streaming types for the chat completion endpoint.

use derive_more::Display;
use serde::{Deserialize, Serialize};

use super::message::Role;
use super::response::{FinishReason, LogProbs, Usage};

/// Chunk of a streaming response
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ChatCompletionChunk {
    /// Unique identifier for the completion
    pub id: String,

    /// Object type (always "chat.completion.chunk")
    pub object: String,

    /// Unix timestamp of creation
    pub created: u64,

    /// Model used
    pub model: String,

    /// System fingerprint (for tracking backend configuration)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,

    /// List of choice deltas
    pub choices: Vec<ChoiceDelta>,

    /// Usage statistics (only included in final chunk if stream_options.include_usage=true)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

/// Delta choice in streaming response
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ChoiceDelta {
    /// Index of the choice
    pub index: u32,

    /// Delta update to the message
    pub delta: Delta,

    /// Finish reason (null while streaming)
    #[serde(rename = "finish_reason")]
    pub finish_reason: Option<FinishReason>,

    /// Log probabilities (if requested)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<LogProbs>,
}

/// Delta content in streaming response
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct Delta {
    /// Role (typically only in first chunk)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,

    /// Content delta (text fragment)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Reasoning content (for thinking mode)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,

    /// Tool calls delta
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tool_calls: Vec<ToolCallDelta>,
}

/// Tool call delta in streaming response
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ToolCallDelta {
    /// Index of the tool call (for parallel tool calls)
    pub index: u32,

    /// Tool call ID (typically in first chunk)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Function delta
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function: Option<FunctionCallDelta>,
}

/// Function call delta
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct FunctionCallDelta {
    /// Function name (typically in first chunk)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Arguments fragment (accumulate to get full JSON)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
}
