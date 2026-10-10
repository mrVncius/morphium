//! DeepSeek Chat Completions and stateless Responses wire types.
//!
//! Chat types live in [`completion`]; Responses types live in [`responses`].
//! Responses callers send full history on every turn; there is no server-side
//! conversation storage.
//!
//! See: <https://api-docs.deepseek.com/>

pub mod completion;
pub mod responses;

/// DeepSeek standard API base URL.
pub const DEEPSEEK_BASE_URL: &str = "https://api.deepseek.com";
