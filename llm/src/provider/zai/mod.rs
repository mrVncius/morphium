//! Z.AI (GLM) wire types.
//!
//! OpenAI-compatible Chat Completions with GLM extensions: `thinking` mode,
//! `reasoning_content`, and an envelope that omits `object`, adds `request_id`,
//! and reports the prompt cache under `prompt_tokens_details.cached_tokens`.
//! The standalone Web Search request and response types are exported alongside
//! the Chat Completions types.
//!
//! See: <https://docs.z.ai/api-reference/llm/chat-completion>

mod message;
mod params;
mod request;
mod response;
mod search;
mod stream;
mod tool;

pub use message::*;
pub use params::*;
pub use request::*;
pub use response::*;
pub use search::*;
pub use stream::*;
pub use tool::*;

/// Z.AI standard API base URL (the `paas/v4` surface).
pub const ZAI_BASE_URL: &str = "https://api.z.ai/api/paas/v4";
