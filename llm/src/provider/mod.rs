//! Provider modules: one per vendor's native wire.
//!
//! Each vendor module owns its native request, response, and streaming types.
//! Model identifiers and settings remain part of the native request. Every
//! module is wire-only: no client, no transport, no runtime.
//!
//! Each sub-module covers a distinct API domain — one native wire each:
//!
//! - [`deepseek`] — DeepSeek Chat Completions over SSE, with tool calls and
//!   reasoning control.
//! - [`openai`] — the Responses wire (`input_text`, `input_image`,
//!   `input_file`, native `json_schema`, streaming events) plus its
//!   transcriptions sibling, which ships in the same module because it is
//!   the same key and the same host.
//! - [`ollama`] — Ollama's native `/api/chat`: NDJSON lines rather than SSE,
//!   images as bare base64 rather than data URLs, tool-call arguments that
//!   arrive already parsed, and `keep_alive`.
//! - [`zai`] — Z.AI's (GLM) OpenAI-compatible Chat Completions
//!   (`POST /api/paas/v4/chat/completions`): `thinking` mode and
//!   `reasoning_content`, an envelope that omits `object`, and the prompt
//!   cache under `prompt_tokens_details.cached_tokens`. The same vendor ships
//!   a standalone Web Search wire (`POST /api/paas/v4/web_search`) beside it.
//! - [`kimi`] — Kimi's (Moonshot AI) OpenAI-compatible Chat Completions
//!   (`POST /v1/chat/completions`): `thinking` mode and `reasoning_content`,
//!   native `json_schema` structured output, and a usage breakdown under
//!   `prompt_tokens_details`.

pub mod deepseek;
pub mod kimi;
pub mod ollama;
pub mod openai;
pub mod zai;
