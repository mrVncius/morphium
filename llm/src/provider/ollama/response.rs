//! Response types for Ollama's native `/api/chat`.
//!
//! One shape covers both paths: without streaming the body is a single
//! object, and with streaming it is the same object once per NDJSON line,
//! the last of which carries `done` and the token counts.

use serde::{Deserialize, Serialize};

use super::request::ChatMessage;

/// One chat response — a whole one, or one line of the stream.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    /// The model that answered. Absent on most streamed lines.
    #[serde(default)]
    pub model: String,
    /// The assistant's message. Under streaming this is the delta.
    #[serde(default)]
    pub message: ChatMessage,
    /// Whether this object ends the response.
    #[serde(default)]
    pub done: bool,
    /// Why generation stopped (`"stop"`, `"length"`, …).
    #[serde(default)]
    pub done_reason: Option<String>,
    /// Prompt tokens.
    #[serde(default)]
    pub prompt_eval_count: Option<u64>,
    /// Generated tokens.
    #[serde(default)]
    pub eval_count: Option<u64>,
}
