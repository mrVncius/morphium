//! Request types for Ollama's native `/api/chat`.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The path of the native chat endpoint.
pub const CHAT_PATH: &str = "/api/chat";

/// A chat request on Ollama's native wire.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    /// Model identifier, as Ollama knows it (`qwen3:8b`).
    pub model: String,
    /// The conversation.
    pub messages: Vec<ChatMessage>,
    /// Whether the server streams NDJSON lines. `None` leaves it to the
    /// endpoint's own default, which is to stream.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// How long to keep the model resident after this call: seconds, `0` to
    /// unload, or `-1` to keep it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<String>,
    /// Sampling knobs, as Ollama names them.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Options>,
    /// Tools the model may call.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tool>,
}

/// One message on the native wire.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMessage {
    /// "system", "user", "assistant", or "tool".
    #[serde(default)]
    pub role: String,
    /// The text.
    #[serde(default)]
    pub content: String,
    /// Images supplied by the caller as **bare base64**, without a data-URL wrapper.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<String>,
    /// Tool calls the assistant asked for (responses only).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tool_calls: Vec<ToolCall>,
}

/// Sampling knobs, as Ollama names them.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Options {
    /// Sampling temperature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Maximum tokens to generate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_predict: Option<u32>,
}

/// A tool definition, in the OpenAI shape Ollama accepts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// Always `"function"`.
    #[serde(rename = "type")]
    pub tool_type: String,
    /// The function.
    pub function: ToolFunction,
}

/// A callable function's manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolFunction {
    /// The name the model calls.
    pub name: String,
    /// What it does.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// JSON Schema for its arguments.
    pub parameters: Value,
}

/// A tool call the model asked for.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// The function and its arguments.
    pub function: CalledFunction,
}

/// A called function. The native wire hands the arguments back as an
/// **object**, not as a JSON string, so no re-parsing is needed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalledFunction {
    /// The function's name.
    pub name: String,
    /// Its arguments.
    #[serde(default)]
    pub arguments: Value,
}
