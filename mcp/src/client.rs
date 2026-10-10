use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use http::{HeaderName, HeaderValue};
use rmcp::{
    model::{
        CallToolRequest, CallToolRequestParams, CallToolResponse, CancelledNotificationParam,
        ClientCapabilities, ClientConfig, ClientRequest, ElicitationCapability,
        FormElicitationCapability, Implementation, InputResponses, ProtocolVersion, RequestId,
        ServerResult, ToolAnnotations,
    },
    service::{
        ClientLifecycleMode, ClientServiceExt, Peer, PeerRequestOptions, RoleClient, RunningService,
    },
    transport::{
        StreamableHttpClientTransport, TokioChildProcess,
        streamable_http_client::StreamableHttpClientTransportConfig,
    },
};
use serde_json::Value;

use crate::error::Error;
use crate::executor::McpTransport;

/// A tool as advertised by an MCP server — no category or policy attached
/// (those are applied by the consumer via [`McpServerConfig`](super::executor::McpServerConfig)).
#[derive(Debug, Clone)]
pub struct RawTool {
    /// The tool's unique name.
    pub name: String,
    /// A human-readable description of what this tool does.
    pub description: Option<String>,
    /// A human-readable title for the tool.
    ///
    /// The kernel's [`Tool`](morphium::tool::Tool) has no display-name field, so
    /// the executor uses this only to backfill a missing
    /// [`description`](Self::description) — a title is never shown in place of
    /// a real description.
    pub title: Option<String>,
    /// JSON Schema describing the expected input arguments.
    pub input_schema: Value,
    /// JSON Schema describing the tool's structured output, when the server
    /// publishes one. A client SHOULD validate `structuredContent` against it.
    pub output_schema: Option<Value>,
    /// The server's annotations — **untrusted hints**, never promises.
    pub annotations: Option<ToolAnnotations>,
}

/// The result returned after executing an MCP tool.
#[derive(Debug, Clone)]
pub struct ToolResult {
    /// The raw JSON value returned by the server.
    pub raw: Value,
}

impl ToolResult {
    /// Extract the model-facing text from an MCP tool result.
    ///
    /// The same render `render_content` applies to the raw value; this is
    /// kept so an error path can name the result it read.
    pub fn text(&self) -> Option<String> {
        render_content(&self.raw)
    }

    /// Returns `true` if the server marked this result as an error.
    pub fn is_error(&self) -> bool {
        self.raw
            .get("isError")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    }
}

/// The model-facing text of an MCP `CallToolResult`, or `None` when it has
/// none.
///
/// Pure, and the single definition of "what the model reads" — both
/// [`ToolResult::text`] and [`McpExecutor`](crate::executor::McpExecutor)'s
/// model render call it, so they cannot drift.
///
/// Preference order:
/// 1. the `content[]` entries whose `type` is `"text"`, joined with newlines;
/// 2. otherwise the `structuredContent` value, compactly serialized;
/// 3. otherwise `None` — the caller falls back to the raw JSON envelope
///    (an image-only result carries no text a model can read).
pub(crate) fn render_content(raw: &Value) -> Option<String> {
    if let Some(content) = raw.get("content").and_then(Value::as_array) {
        let chunks: Vec<&str> = content
            .iter()
            .filter_map(|item| {
                let is_text = item.get("type").and_then(Value::as_str) == Some("text");
                is_text
                    .then(|| item.get("text").and_then(Value::as_str))
                    .flatten()
            })
            .collect();
        if !chunks.is_empty() {
            return Some(chunks.join("\n"));
        }
    }
    raw.get("structuredContent")
        .filter(|v| !v.is_null())
        .map(Value::to_string)
}

/// Answers one round of a server's mid-call input request (SEP-2322 MRTR).
///
/// Receives the round's `inputRequests` exactly as the server sent them and
/// returns the `inputResponses` map to send back, keyed the same way. `None`
/// means the host declined to answer, and the call fails rather than guessing.
/// The bridge is only a conduit — an eased `ElicitResult`, a sampled message,
/// and a roots list are all just values the host produces.
pub type InputHandler =
    Arc<dyn Fn(Value) -> Pin<Box<dyn Future<Output = Option<Value>> + Send>> + Send + Sync>;

/// What a client advertises, and how it answers a server mid-call.
#[derive(Clone, Default)]
pub struct ClientOptions {
    /// The handler for SEP-2322 input rounds.
    ///
    /// `None` declares **no** elicitation capability, so a conformant server
    /// will not ask; a server that asks anyway gets an error naming the gap
    /// rather than a silent decline.
    pub input: Option<InputHandler>,
}

/// How many `tools/call` rounds one call may take before giving up — the cap
/// the SDK's own MRTR helper uses.
const MRTR_MAX_ROUNDS: usize = 10;

/// A client connected to an MCP server over HTTP or stdio.
pub struct Client {
    service: RunningService<RoleClient, ClientConfig>,
    input: Option<InputHandler>,
}

impl Client {
    /// Connect to an MCP server using the given transport, answering nothing.
    ///
    /// Starts with the modern `Auto` lifecycle: `server/discover` (the
    /// stateless 2026-07-28 protocol) when the server supports it, with
    /// a legacy `initialize` handshake as fallback.
    pub async fn connect(transport: &McpTransport) -> Result<Self, Error> {
        Self::connect_with(transport, ClientOptions::default()).await
    }

    /// Connect with an explicit [`ClientOptions`] — in particular the handler
    /// that answers a server's mid-call input request.
    ///
    /// The elicitation capability is advertised **only** when a handler is
    /// installed: a capability is a promise, and a server should not ask for
    /// something this client cannot do.
    pub async fn connect_with(
        transport: &McpTransport,
        options: ClientOptions,
    ) -> Result<Self, Error> {
        let config = client_config(options.input.is_some());
        let service = match transport {
            McpTransport::Http { url, headers } => {
                let http = StreamableHttpClientTransportConfig::with_uri(url.clone());
                let http = if !headers.is_empty() {
                    http.custom_headers(convert_headers(headers.clone()))
                } else {
                    http
                };
                let transport = StreamableHttpClientTransport::from_config(http);
                config
                    .serve_with_lifecycle(transport, auto_lifecycle())
                    .await?
            }
            McpTransport::Stdio {
                command,
                args,
                env,
                cwd,
            } => {
                // Spawns with piped stdin/stdout (stderr is inherited);
                // dropping the transport kills the child process.
                let mut cmd = tokio::process::Command::new(command);
                cmd.args(args);
                if !env.is_empty() {
                    cmd.envs(env);
                }
                if let Some(cwd) = cwd {
                    cmd.current_dir(cwd);
                }
                let transport = TokioChildProcess::new(cmd)?;
                config
                    .serve_with_lifecycle(transport, auto_lifecycle())
                    .await?
            }
        };
        Ok(Self {
            service,
            input: options.input,
        })
    }

    /// The protocol version the handshake settled on, once connected.
    ///
    /// `Auto` prefers the stateless `2026-07-28` revision and falls back to
    /// the legacy `initialize` handshake, so this is the only way to tell
    /// which path a peer actually took — a host that logs it, and the live
    /// round-trip test that asserts it.
    pub fn protocol_version(&self) -> Option<ProtocolVersion> {
        self.service
            .peer()
            .peer_info()
            .map(|info| info.protocol_version.clone())
    }

    /// The server-wide guidance this server advertised, if any.
    ///
    /// Under both the stateless `server/discover` reply and the legacy
    /// `initialize` reply the value is the same `instructions` field; a server
    /// that sent none yields `None` rather than an empty string.
    pub fn instructions(&self) -> Option<String> {
        self.service
            .peer()
            .peer_info()
            .and_then(|info| info.instructions.clone())
    }

    /// List all tools advertised by this MCP server.
    pub async fn list_tools(&self) -> Result<Vec<RawTool>, Error> {
        let tools = self.service.peer().list_all_tools().await?;
        let mut result = Vec::with_capacity(tools.len());
        for t in tools {
            result.push(RawTool {
                name: t.name.to_string(),
                description: t.description.map(|d| d.into_owned()),
                title: t.title,
                input_schema: Value::Object((*t.input_schema).clone()),
                output_schema: t.output_schema.map(|s| Value::Object((*s).clone())),
                annotations: t.annotations,
            });
        }
        Ok(result)
    }

    /// Execute a tool by name with the given JSON arguments.
    ///
    /// `arguments` must be a JSON object (`{}`). Drives the SEP-2322 MRTR
    /// rounds itself rather than delegating to the SDK's `call_tool`, for two
    /// reasons: a server that needs input mid-call is answered by the
    /// installed [`InputHandler`], and the in-flight request carries an id so
    /// a caller that abandons this future tells the server to stop.
    pub async fn call(&self, name: &str, arguments: Value) -> Result<ToolResult, Error> {
        let args = arguments
            .as_object()
            .cloned()
            .ok_or(Error::InvalidArguments)?;
        let mut params = CallToolRequestParams::new(name.to_string()).with_arguments(args);
        let mut state_only_rounds = 0u32;

        for _round in 0..MRTR_MAX_ROUNDS {
            let required = match self.call_once(&params).await? {
                CallToolResponse::Complete(result) => {
                    return Ok(ToolResult {
                        raw: serde_json::to_value(result)?,
                    });
                }
                CallToolResponse::InputRequired(required) => required,
                // SEP-2663: a task handle this bridge never asked for.
                _ => return Err(Error::UnsupportedResponse),
            };

            let requests = required.input_requests.clone().filter(|r| !r.is_empty());
            if requests.is_none() && required.request_state.is_none() {
                // Neither a question nor state to resume from: nothing this
                // bridge can act on.
                return Err(Error::UnsupportedResponse);
            }

            if requests.is_none() {
                // A state-only round is the server still working. Back off
                // briefly, the way the SDK does (50ms·2ⁿ, capped at 250ms).
                let backoff = 50u64 << state_only_rounds.min(3);
                tokio::time::sleep(Duration::from_millis(backoff.min(250))).await;
                state_only_rounds += 1;
            } else {
                state_only_rounds = 0;
            }

            let responses = match (&requests, &self.input) {
                (Some(requests), Some(handler)) => {
                    let asked = serde_json::to_value(requests)?;
                    match handler(asked).await {
                        Some(answered) => Some(serde_json::from_value::<InputResponses>(answered)?),
                        None => return Err(Error::InputDeclined),
                    }
                }
                (Some(_), None) => return Err(Error::InputHandlerMissing),
                (None, _) => None,
            };

            params.input_responses = responses;
            params.request_state = required.request_state.clone();
        }
        Err(Error::InputRoundsExceeded {
            max_rounds: MRTR_MAX_ROUNDS,
        })
    }

    /// One `tools/call` round whose request can be cancelled if the caller
    /// abandons it.
    ///
    /// The SDK's `call_tool` helpers hide the JSON-RPC id and, on drop, leave
    /// the request pending on the wire; only
    /// [`Peer::send_cancellable_request`] hands the id back, which is what
    /// lets [`CancelOnDrop`] tell the server to stop.
    async fn call_once(&self, params: &CallToolRequestParams) -> Result<CallToolResponse, Error> {
        let peer = self.service.peer().clone();
        let handle = peer
            .send_cancellable_request(
                ClientRequest::CallToolRequest(CallToolRequest::new(params.clone())),
                PeerRequestOptions::no_options(),
            )
            .await?;
        let mut cancel = CancelOnDrop {
            peer,
            id: handle.id.clone(),
            armed: true,
        };
        let response = handle.await_response().await;
        // The round answered on its own: nothing left to cancel.
        cancel.armed = false;

        match response? {
            ServerResult::CallToolResult(result) => Ok(CallToolResponse::Complete(result)),
            ServerResult::InputRequiredResult(result) => {
                Ok(CallToolResponse::InputRequired(result))
            }
            ServerResult::CreateTaskResult(result) => Ok(CallToolResponse::Task(result)),
            _ => Err(Error::UnsupportedResponse),
        }
    }
}

/// Tells a server to stop working on a request its caller abandoned.
///
/// Armed while a round is in flight, defused the moment it answers. The
/// notification must be sent from a spawned task — `Drop` cannot await — and
/// the serve loop resolves the pending round with `Cancelled` once it lands.
struct CancelOnDrop {
    peer: Peer<RoleClient>,
    id: RequestId,
    armed: bool,
}

impl Drop for CancelOnDrop {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        // Without a runtime (shutdown, or someone driving the future by hand)
        // there is nobody left to send the notification; panicking inside
        // `drop` would be worse than a server finishing work nobody wants.
        let Ok(handle) = tokio::runtime::Handle::try_current() else {
            return;
        };
        let peer = self.peer.clone();
        let id = self.id.clone();
        handle.spawn(async move {
            let _ = peer
                .notify_cancelled(CancelledNotificationParam::new(
                    Some(id),
                    Some("the caller abandoned the tool call".to_string()),
                ))
                .await;
        });
    }
}

/// This bridge's client identity, as every server will see it.
///
/// Not [`ClientConfig::default()`]: that names the SDK
/// (`Implementation::from_build_env()` → `"rmcp"`), so each server would log
/// the library as its caller.
///
/// Capabilities stay empty — the legal tools-only position — unless an
/// [`InputHandler`] is installed, in which case elicitation is declared
/// because the bridge can now answer it.
fn client_config(elicit: bool) -> ClientConfig {
    let mut capabilities = ClientCapabilities::default();
    if elicit {
        capabilities.elicitation =
            Some(ElicitationCapability::new().with_form(FormElicitationCapability::new()));
    }
    ClientConfig::new(
        capabilities,
        Implementation::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
            .with_title("morphium MCP bridge"),
    )
}

/// The `Auto` lifecycle: probe `server/discover` (the stateless
/// 2026-07-28 protocol) and fall back to the legacy `initialize`
/// handshake when the peer is an older server.
fn auto_lifecycle() -> ClientLifecycleMode {
    ClientLifecycleMode::Auto {
        preferred_versions: vec![ProtocolVersion::V_2026_07_28],
        legacy_version: Some(ProtocolVersion::V_2025_11_25),
    }
}

/// Convert a `HashMap<String, String>` of raw headers into an `http::HeaderMap`.
fn convert_headers(headers: HashMap<String, String>) -> HashMap<HeaderName, HeaderValue> {
    headers
        .into_iter()
        .filter_map(|(name, value)| {
            let hn = HeaderName::from_bytes(name.as_bytes()).ok()?;
            let hv = HeaderValue::from_str(&value).ok()?;
            Some((hn, hv))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn text_content_renders_joined_text_not_the_envelope() {
        // Text parts around a non-text part: the model reads the joined text,
        // never the JSON envelope that carries it.
        let raw = json!({
            "content": [
                {"type": "text", "text": "first"},
                {"type": "image", "data": "…", "mimeType": "image/png"},
                {"type": "text", "text": "second"}
            ],
            "isError": false
        });
        assert_eq!(render_content(&raw).as_deref(), Some("first\nsecond"));
    }

    #[test]
    fn structured_only_result_renders_its_json() {
        // No text parts at all: `structuredContent` is what the model reads.
        let raw = json!({"content": [], "structuredContent": {"rows": 2}});
        assert_eq!(render_content(&raw).as_deref(), Some("{\"rows\":2}"));
    }

    #[test]
    fn result_without_text_or_structure_renders_nothing() {
        // An image-only result carries no text a model can read, so the caller
        // must fall back to the raw envelope rather than invent content.
        assert_eq!(
            render_content(&json!({"content": [{"type": "image", "data": "…"}]})),
            None
        );
        assert_eq!(render_content(&json!({"structuredContent": null})), None);
        assert_eq!(render_content(&json!({})), None);
    }

    #[test]
    fn tool_result_text_uses_the_shared_render() {
        let result = ToolResult {
            raw: json!({"content": [{"type": "text", "text": "hi"}]}),
        };
        assert_eq!(result.text().as_deref(), Some("hi"));
    }

    #[test]
    fn elicitation_is_declared_only_when_a_handler_can_answer_it() {
        // A capability is a promise: with nobody to ask, the bridge says
        // nothing rather than inviting a request it must refuse.
        assert!(client_config(false).capabilities.elicitation.is_none());
        assert!(client_config(true).capabilities.elicitation.is_some());
    }

    #[test]
    fn client_config_names_the_bridge_not_the_sdk() {
        // `default()` names `rmcp` via `Implementation::from_build_env`; every
        // server must instead log this bridge, with no capabilities declared.
        let config = client_config(false);
        assert_eq!(config.client_info.name, env!("CARGO_PKG_NAME"));
        assert_ne!(config.client_info.name, "rmcp");
        assert_eq!(
            config.client_info.title.as_deref(),
            Some("morphium MCP bridge")
        );
        assert_eq!(
            serde_json::to_value(&config.capabilities).unwrap(),
            json!({})
        );
    }
}
