//! Kimi (Moonshot AI) wire types.
//!
//! OpenAI-compatible Chat Completions at `api.moonshot.ai/v1` with Moonshot
//! extensions:
//!
//! - Thinking: `reasoning_content`, `thinking { type, keep }` on K2.x, and
//!   top-level `reasoning_effort` on `kimi-k3`.
//! - Content parts `text` / `image_url` / `video_url`, with the media parts
//!   accepting an object (`{"url": …}`) or a bare URL string.
//! - `prediction`, `stop`, `logprobs` / `top_logprobs`, and the context cache
//!   under `prompt_cache_key` / `prompt_cache_options`.
//! - Native `json_schema` structured output, and `max_completion_tokens` in
//!   place of the deprecated `max_tokens`.
//! - A `kimi-k3` dynamic tool-loading message ([`message::DynamicToolMessage`])
//!   in the `messages` array.
//! - A usage breakdown under `prompt_tokens_details { cached_tokens,
//!   cache_write_tokens }`.
//!
//! Sampling parameters (`temperature`, `top_p`, `n`) are fixed per model and
//! rejected when sent, so they are not part of this surface.
//!
//! See: <https://platform.kimi.ai/docs/api/chat>

mod message;
mod params;
mod request;
mod response;
mod stream;
mod tool;

pub use message::*;
pub use params::*;
pub use request::*;
pub use response::*;
pub use stream::*;
pub use tool::*;

/// Kimi (Moonshot AI) standard API base URL (the `v1` surface).
pub const KIMI_BASE_URL: &str = "https://api.moonshot.ai/v1";
