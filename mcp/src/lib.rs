//! MCP (Model Context Protocol) executor.
//!
//! Provides [`McpExecutor`], a
//! [`ToolExecutor`](morphium::tool::ToolExecutor) that connects to one or
//! more MCP servers over streamable HTTP or stdio.
//!
//! Each server's tools are exposed as `mcp__<server>__<tool>` (sanitized and
//! length-capped), a server's `instructions` are available for the host to
//! place in the model's context, and a call is routed by ownership, not by
//! name shape: `run()` only reaches
//! a server that advertised the tool in its `tools/list` reply, so a name no
//! server advertises is [`ToolError::UnknownTool`](morphium::tool::ToolError::UnknownTool)
//! and an unreachable server is
//! [`Unavailable`](morphium::tool::ToolError::Unavailable) — both of which a
//! host-side composite treats as "not mine, try elsewhere". The wire answers
//! the same way: a server's `-32602` (unknown tool) reply is `UnknownTool`, its
//! `-32601` (no `tools/call`) and `-32022` (version rejected) are `Unavailable`,
//! and a `-32021` (capability this bridge does not declare) is a `Transport`
//! failure that surfaces rather than being absorbed.
//!
//! Discovery is asynchronous while the kernel's [`tools`](morphium::tool::ToolExecutor::tools)
//! is synchronous. The host calls [`McpExecutor::refresh`] before a turn and
//! `tools()` answers from the resulting snapshot; a refresh that fails leaves
//! the previous snapshot in place.

pub mod client;
pub mod error;
pub mod executor;

pub use executor::{McpExecutor, McpServerConfig, McpTransport};
