//! Ollama's native chat API wire types.
//!
//! The native API exposes `keep_alive` for model residency control and
//! streams responses as NDJSON. Its endpoint paths have no `/v1` prefix.

mod request;
mod response;

pub use request::{
    CHAT_PATH, CalledFunction, ChatMessage, ChatRequest, Options, Tool, ToolCall, ToolFunction,
};
pub use response::ChatResponse;

/// Ollama's default base URL on a local machine — the **native** API, which
/// carries no `/v1` prefix.
pub const OLLAMA_BASE_URL: &str = "http://localhost:11434";
