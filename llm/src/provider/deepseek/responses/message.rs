//! Stateless conversation items and content parts.

use serde::{Deserialize, Serialize};

/// Plain text or the complete conversation history.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Input {
    /// A single user message.
    Text(String),
    /// Messages, reasoning, calls, and tool results in conversation order.
    Items(Vec<InputItem>),
}

/// A message or tool-result body.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Content {
    /// Plain text.
    Text(String),
    /// Typed content parts.
    Parts(Vec<ContentPart>),
}

/// Supported message roles. DeepSeek treats developer messages as user messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    /// System instructions.
    System,
    /// Developer input.
    Developer,
    /// User input.
    User,
    /// Assistant output.
    Assistant,
}

/// Image resolution hint; ignored for uploaded images.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageDetail {
    /// Downsample to 512 by 512.
    Low,
    /// Preserve original resolution.
    High,
    /// Preserve original resolution.
    Original,
    /// Let the model choose.
    Auto,
}

/// A text or image content part.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentPart {
    /// Input text.
    InputText {
        /// Text content.
        text: String,
    },
    /// Generated text, also accepted when replaying history.
    OutputText {
        /// Text content.
        text: String,
        /// Output annotations.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        annotations: Vec<serde_json::Value>,
        /// Token log probabilities.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        logprobs: Option<Vec<serde_json::Value>>,
    },
    /// Plain-text chain of thought.
    ReasoningText {
        /// Reasoning content.
        text: String,
    },
    /// Image input, supported only by deepseek-flash in user/developer messages or tool results.
    /// Supply exactly one of `image_url` and `file_id`.
    InputImage {
        /// HTTP(S) URL or base64 data URL.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        image_url: Option<String>,
        /// Uploaded image ID.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        file_id: Option<String>,
        /// Resolution hint.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        detail: Option<ImageDetail>,
    },
}

/// Generation status of an output item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemStatus {
    /// Still generating.
    InProgress,
    /// Complete.
    Completed,
    /// Truncated.
    Incomplete,
}

/// Conversation item accepted in `input` and produced in `output`.
/// Output items can be replayed directly in the next stateless request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputItem {
    /// A conversation message.
    Message {
        /// Server-assigned item ID, optional for input.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        /// Message role.
        role: Role,
        /// Message content.
        content: Content,
        /// Generation status, when returned by the server.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        status: Option<ItemStatus>,
    },
    /// Plain-text reasoning merged into the adjacent assistant message on replay.
    Reasoning {
        /// Server-assigned item ID.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        /// Chain-of-thought content.
        #[serde(default)]
        content: Vec<ContentPart>,
        /// Generation status.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        status: Option<ItemStatus>,
    },
    /// A generated function call; validate arguments before executing it.
    FunctionCall {
        /// Server-assigned item ID.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        /// Identifier pairing the call with its result.
        call_id: String,
        /// Function name.
        name: String,
        /// JSON-encoded arguments.
        arguments: String,
        /// Generation status.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        status: Option<ItemStatus>,
    },
    /// A function result, optionally including images.
    FunctionCallOutput {
        /// Identifier of the corresponding call.
        call_id: String,
        /// Tool result.
        output: Content,
    },
    /// A custom apply_patch call.
    CustomToolCall {
        /// Server-assigned item ID.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        /// Identifier pairing the call with its result.
        call_id: String,
        /// Custom tool name (apply_patch).
        name: String,
        /// Raw tool input.
        input: String,
        /// Generation status.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        status: Option<ItemStatus>,
    },
    /// A custom tool result, optionally including images.
    CustomToolCallOutput {
        /// Identifier of the corresponding call.
        call_id: String,
        /// Tool result.
        output: Content,
    },
}
