//! Streaming types for the chat completion endpoint.

use serde::{Deserialize, Serialize};

use super::message::Role;
use super::response::Usage;

/// A chunk of a streaming response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionChunk {
    /// Unique identifier for the completion.
    #[serde(default)]
    pub id: String,

    /// Object type; omitted by Z.AI but present on compatible endpoints.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,

    /// Unix timestamp of creation.
    #[serde(default)]
    pub created: u64,

    /// Model used.
    #[serde(default)]
    pub model: String,

    /// Choice deltas.
    #[serde(default)]
    pub choices: Vec<ChoiceDelta>,

    /// Usage statistics (only in the final chunk with `include_usage`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
}

/// A choice delta in a streaming response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceDelta {
    /// Index of the choice.
    #[serde(default)]
    pub index: u32,

    /// Delta update to the message.
    pub delta: Delta,

    /// Finish reason, `None` while streaming.
    #[serde(rename = "finish_reason", default)]
    pub finish_reason: Option<String>,
}

/// Delta content in a streaming response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delta {
    /// Role (typically only in the first chunk).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,

    /// Content delta (text fragment).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Reasoning content (for thinking mode).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,

    /// Tool call deltas.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tool_calls: Vec<ToolCallDelta>,
}

/// A tool call delta in a streaming response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallDelta {
    /// Index of the tool call (for parallel tool calls).
    #[serde(default)]
    pub index: u32,

    /// Tool call ID (typically in the first chunk).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Function delta.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub function: Option<FunctionCallDelta>,
}

/// Function call delta.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallDelta {
    /// Function name (typically in the first chunk).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Arguments fragment (accumulate to get the full JSON).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
}
