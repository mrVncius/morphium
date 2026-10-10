//! OpenAI wire types.
//!
//! # APIs
//!
//! - [`responses`]: Responses API wire types
//! - [`transcribe`]: Transcriptions API wire types
//! - [`realtime`]: Realtime API wire types (WebSocket events, sessions, and
//!   their REST siblings)

pub mod realtime;
pub mod responses;
pub mod transcribe;

/// OpenAI API base URL.
pub const OPENAI_BASE_URL: &str = "https://api.openai.com";
