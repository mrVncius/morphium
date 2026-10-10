//! OpenAI Realtime API wire types.
//!
//! Wire-only: no client, no transport, no runtime. Each sub-module owns one
//! slice of the Realtime resource; every type maps to a documented schema.
//!
//! - [`session`]: session configuration — audio config, formats, turn
//!   detection, noise reduction, truncation, tracing, reasoning, and the
//!   transcription-session variant.
//! - [`message`]: conversation items and their content parts.
//! - [`tool`]: function and MCP tools, tool choice, and MCP result types.
//! - [`prompt`]: prompt templates and their variable content parts.
//! - [`response`]: the response object, creation params, status, usage, and
//!   error types.
//! - [`client_event`]: the client→server WebSocket event union.
//! - [`server_event`]: the server→client WebSocket event union.
//! - [`rest`]: session, transcription-session, and client-secret REST wire.
//! - [`translation`]: the realtime translations wire.
//! - [`call`]: SIP call control.
//!
//! See: <https://developers.openai.com/api/reference/resources/realtime/client-events>
//! and <https://developers.openai.com/api/reference/resources/realtime/server-events>

pub mod call;
pub mod client_event;
pub mod message;
pub mod prompt;
pub mod response;
pub mod rest;
pub mod server_event;
pub mod session;
pub mod tool;
pub mod translation;
