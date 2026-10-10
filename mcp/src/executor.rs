//! McpExecutor — [`ToolExecutor`] impl for one or more MCP servers.
//!
//! Servers are reached over streamable HTTP or stdio and connected **lazily** —
//! the first [`refresh`](McpExecutor::refresh) or `run()` performs the
//! handshake. The kernel's `tools()` is synchronous, so discovery is explicit:
//! `refresh()` connects, lists, and replaces the manifest snapshot the host
//! reads; a refresh that fails leaves the previous snapshot untouched.
//!
//! A name is callable **only if the owning server advertises it**: `run()`
//! resolves the agent-visible name against each server's `tools/list` reply
//! (fetched on first use, refreshed by `refresh()`, prefix and allowlist
//! applied) and answers [`ToolError::UnknownTool`] otherwise. An unknown name
//! is never forwarded to a server, so `UnknownTool` means the same thing here
//! as it does for a local executor.
//!
//! Which error a miss gets says what the executor knows: a name outside every
//! server's configured namespace is [`ToolError::UnknownTool`] without dialing
//! anything, while a name that a server *could* own but that only unreachable
//! servers were asked about is [`ToolError::Unavailable`].
//!
//! A server's own answer is classified the same way (see `classify_error`):
//! a `-32602` (unknown tool) reply means our cached manifest is stale and is
//! reported as [`ToolError::UnknownTool`], `-32601`/`-32022` (no `tools/call`;
//! version rejected) as [`ToolError::Unavailable`], and `-32021` (a client
//! capability we do not declare) as [`ToolError::Transport`] — a fault no
//! amount of routing repairs.
//!
//! # Exposure policy
//!
//! Every advertised name is `mcp__<namespace>__<tool>`, sanitized to
//! `[A-Za-z0-9_]` and capped at 64 characters (SEP-986). A collision or an
//! over-long name gets a stable hash suffix, so two servers — or two tools —
//! can never silently shadow one another.
//!
//! A foreign server's annotations are hints, never proof, so a tool defaults
//! to [`ReplaySafety::Unsafe`]. A host that knows better opts in per server
//! with [`McpServerConfig::replay_safety`], or per tool with
//! [`McpServerConfig::replay_safety_by_tool`]; the executor reports the highest
//! [`replay_limit`](McpServerConfig::replay_limit) across its servers. A
//! per-tool timeout is applied inside `run()` (configured by
//! `tool_timeout_ms`, default 60s), because the kernel's executor contract has
//! no descriptor field to carry it.
//!
//! Connecting a server and listing its tools are bounded by
//! `startup_timeout_ms` (default 10s) and, over HTTP, retried on a transient
//! transport failure. A tool call is **never** retried — it may already have
//! run. The server's own `instructions` are surfaced through
//! [`McpExecutor::instructions`] for the host to place.
//!
//! # Deliberate non-goals
//!
//! The `io.modelcontextprotocol/tasks` extension (SEP-2663) is not driven: a
//! server that answers a call with a task handle the client never asked for
//! is reported as a [`ToolError::Transport`] naming it, rather than a poll loop.
//! A client that declares the extension would call `call_tool_once` and poll
//! `tasks/get` instead.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, OnceLock, RwLock};
use std::time::Duration;

use morphium::effect::{EffectId, EffectInvocation, Outcome, ReplaySafety};
use morphium::primitive::Id;
use morphium::tool::{Tool, ToolError, ToolExecutor, ToolName, ToolOutput, ToolReply, ToolRequest};
use rmcp::model::ErrorCode;
use rmcp::service::{ClientInitializeError, ServiceError};
use serde_json::Value;
use tokio::sync::{Mutex, MutexGuard};

use super::client::{Client, ClientOptions, InputHandler, RawTool, render_content};
use crate::error::Error;

/// The shape every exposed name is built from: `mcp__<namespace>__<tool>`.
///
/// The `mcp__` prefix is the de-facto market convention — both Codex and pi
/// use it — and it keeps a bridge tool from colliding with a local tool that
/// happens to be named `<server>__<tool>`.
const MCP_PREFIX: &str = "mcp__";

/// SEP-986 caps tool names at 64 characters (inclusive).
const MAX_TOOL_NAME_LEN: usize = 64;

/// Room reserved for the collision/truncation suffix: `_` + 8 hex digits.
const HASH_SUFFIX_LEN: usize = 9;

/// The longest a sanitized namespace may be, so that
/// `MCP_PREFIX + namespace + "__" + <tool> + <suffix>` can still fit in
/// [`MAX_TOOL_NAME_LEN`] with at least one character of tool.
const MAX_NAMESPACE_LEN: usize = MAX_TOOL_NAME_LEN - MCP_PREFIX.len() - 2 - HASH_SUFFIX_LEN;

/// Fallback per-tool timeout when a server config declares none. The tool
/// timeout is carried on the descriptor, where the engine enforces it.
const DEFAULT_TOOL_TIMEOUT_MS: u64 = 60_000;

/// Fallback bound on connecting a server and fetching its first `tools/list`.
const DEFAULT_STARTUP_TIMEOUT_MS: u64 = 10_000;

/// Backoff between retries of an idempotent HTTP request; its length is the
/// maximum number of retries. Tool calls are never retried — only connect and
/// `tools/list` are, and only over HTTP.
const RETRY_DELAYS_MS: [u64; 2] = [250, 1_000];

/// Replace every character SEP-986 does not allow with `_`.
fn sanitize(component: &str) -> String {
    component
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// A deterministic 8-hex FNV-1a digest.
///
/// Name suffixes must be stable across runs and Rust releases; this is not a
/// cryptographic use, only a stable identifier.
fn hash8(input: &str) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{:08x}", (hash & 0xffff_ffff) as u32)
}

/// How to reach a single MCP server.
#[derive(Debug, Clone)]
pub enum McpTransport {
    /// A remote server speaking MCP over streamable HTTP.
    Http {
        /// Server URL (e.g. `https://api.example.com/mcp`).
        url: String,
        /// HTTP headers to include in every request.
        ///
        /// Most servers expect at minimum:
        /// ```text
        /// Authorization: $token_type <token>
        /// ```
        headers: HashMap<String, String>,
    },
    /// A local server spawned as a child process speaking MCP over stdio.
    ///
    /// The child is spawned with piped stdin/stdout; its stderr is
    /// inherited, and dropping the executor kills the process.
    Stdio {
        /// Program to spawn, resolved on `PATH` like any shell command.
        command: String,
        /// Extra arguments passed to the program.
        args: Vec<String>,
        /// Environment variables set for the child, in addition to the
        /// inherited environment.
        ///
        /// This is how a token-based server is launched (e.g. a Slack server
        /// reading `SLACK_MCP_XOXP_TOKEN`): the value never has to live in the
        /// parent process's environment.
        env: HashMap<String, String>,
        /// Working directory for the child, when it needs one other than the
        /// bridge's own.
        cwd: Option<String>,
    },
}

impl McpTransport {
    /// Whether reaching this server involves the network.
    ///
    /// Only HTTP requests are retried on a transient failure: a stdio child is
    /// spawned locally, so a spawn failure is deterministic and retrying it
    /// only delays the error.
    fn is_http(&self) -> bool {
        matches!(self, Self::Http { .. })
    }
}

/// Configuration for a single MCP server that the agent can call.
#[derive(Debug, Clone)]
pub struct McpServerConfig {
    /// Human-readable name — for logging and error messages, and for the
    /// default tool namespace when [`prefix`](Self::prefix) is `None`.
    pub name: String,
    /// How to reach the server.
    pub transport: McpTransport,
    /// Optional per-server tool allowlist.
    /// [`ToolError::UnknownTool`].
    ///
    /// `None` exposes every tool the server advertises. `Some(list)` exposes
    /// only the listed tools, whose names may be given either as the agent
    /// sees them (`"mcp__docker__read"`) or as the server knows them
    /// (`"read"`). An empty list exposes nothing. A tool the allowlist
    /// excludes is not exposed and not callable — `run()` answers
    /// [`ToolError::UnknownTool`].
    pub allowed_tools: Option<Vec<String>>,
    /// Optional per-server tool deny-list, applied **after**
    /// [`allowed_tools`](Self::allowed_tools).
    ///
    /// Names may be given as the agent sees them (`"mcp__docker__read"`) or as
    /// the server knows them (`"read"`). A denied tool is not exposed and not
    /// callable — `run()` answers [`ToolError::UnknownTool`] — even when the
    /// allow-list would otherwise admit it.
    pub disabled_tools: Option<Vec<String>>,
    /// Default replay posture for this server's tools.
    ///
    /// MCP tool annotations are untrusted hints, so this defaults to
    /// [`ReplaySafety::Unsafe`]. A host that knows a server's tools are safe to
    /// repeat opts in explicitly, per server or per tool via
    /// [`replay_safety_by_tool`](Self::replay_safety_by_tool).
    pub replay_safety: ReplaySafety,
    /// Per-tool replay-safety overrides, keyed by the agent-visible name or the
    /// server-side name.
    ///
    /// A matching override wins over [`replay_safety`](Self::replay_safety);
    /// the exposed name is checked first, then the raw name.
    pub replay_safety_by_tool: Option<HashMap<String, ReplaySafety>>,
    /// The highest automatic-replay attempt this server permits.
    ///
    /// The kernel's `replay_limit` is executor-wide, so an executor with
    /// several servers reports the highest configured bound.
    pub replay_limit: u32,
    /// The namespace every tool name from this server is exposed under, as
    /// `mcp__{prefix}__{tool}`.
    ///
    /// `None` uses [`name`](Self::name), so servers with distinct names can
    /// never expose the same agent-visible name — what the spec asks an
    /// aggregating client to guarantee, honored by construction rather than by
    /// a tiebreak. Set an explicit prefix only when the name is not a
    /// tool-name-friendly string; either way the namespace is sanitized to
    /// `[A-Za-z0-9_]` before use.
    ///
    /// Two servers that share a `name` and declare no prefix share a
    /// namespace; there the manifest merge's first-wins rule (`Earlier`) still
    /// decides, and a call routes to the first server in the configured list.
    pub prefix: Option<String>,
    /// Per-tool execution timeout in milliseconds, carried on every descriptor
    /// this server contributes and enforced by the engine.
    ///
    /// `None` uses the bridge default (currently 60s).
    pub tool_timeout_ms: Option<u64>,
    /// Bound on connecting this server and fetching its first `tools/list`.
    ///
    /// `None` uses the bridge default (currently 10s). A server that cannot answer
    /// in time surfaces on `refresh()` and is [`ToolError::Unavailable`]
    /// on `run()`, so a composite can route the call on.
    pub startup_timeout_ms: Option<u64>,
}

/// The tools one server advertises, resolved against its configuration.
///
/// Pure: built from a `tools/list` reply, no I/O. Keeps the agent-visible name
/// next to the server-side name so `execute()` can decide ownership without
/// another round trip.
struct ServerRoute {
    advertised: Vec<AdvertisedTool>,
    /// The per-tool timeout this server's descriptors carry, resolved once at
    /// build time.
    tool_timeout_ms: u64,
    /// The server's default replay posture and per-tool overrides, resolved
    /// once at build time.
    replay_safety: ReplaySafety,
    replay_safety_by_tool: Option<HashMap<String, ReplaySafety>>,
}

struct AdvertisedTool {
    /// The name the agent sees and calls, admitted once at discovery.
    exposed: ToolName,
    /// The tool as the server describes it, under its own name.
    raw: RawTool,
}

impl ServerRoute {
    /// Filter, rename, and validate the server's tools for its configuration.
    ///
    /// The agent-visible name is admitted into a [`ToolName`] here, once.
    /// [`expose`] always yields a non-empty name — every name starts with
    /// [`MCP_PREFIX`] — so a rejection means the exposure invariant itself
    /// broke. That is surfaced as an error instead of dropping the descriptor
    /// or panicking on server-supplied data.
    fn build(
        config: &McpServerConfig,
        tools: Vec<RawTool>,
    ) -> Result<Self, morphium::tool::ToolValueError> {
        let mut seen: HashSet<String> = HashSet::new();
        let mut advertised = Vec::with_capacity(tools.len());
        for raw in tools {
            let mut exposed = expose(config, &raw.name);
            if !seen.insert(exposed.clone()) {
                // Two of this server's tool names sanitized to the same
                // exposed name: make each unique with a deterministic suffix
                // so neither silently shadows the other.
                let prefix = namespace_prefix(config);
                let budget = MAX_TOOL_NAME_LEN - prefix.len() - HASH_SUFFIX_LEN;
                let sanitized = sanitize(&raw.name);
                let keep = sanitized.len().min(budget);
                exposed = format!("{prefix}{}_{}", &sanitized[..keep], hash8(&raw.name));
                seen.insert(exposed.clone());
            }
            if !allowed(config, &exposed, &raw.name) || denied(config, &exposed, &raw.name) {
                continue;
            }
            let exposed = ToolName::parse(exposed)?;
            advertised.push(AdvertisedTool { exposed, raw });
        }
        Ok(Self {
            advertised,
            tool_timeout_ms: config.tool_timeout_ms.unwrap_or(DEFAULT_TOOL_TIMEOUT_MS),
            replay_safety: config.replay_safety,
            replay_safety_by_tool: config.replay_safety_by_tool.clone(),
        })
    }

    /// The manifest entries this server contributes, in the server's order.
    ///
    /// Names were admitted into [`ToolName`]s when the route was built, so the
    /// manifest is assembled from validated names without re-parsing. The
    /// server's input schema is passed through unchanged.
    fn tools(&self) -> Vec<Tool> {
        self.advertised
            .iter()
            .map(|t| Tool {
                name: t.exposed.clone(),
                // The kernel's `Tool` has no display-name field, so a title
                // only backfills an absent description — it never replaces one.
                description: t
                    .raw
                    .description
                    .clone()
                    .or_else(|| t.raw.title.clone())
                    .unwrap_or_default(),
                parameters: normalize_input_schema(&t.raw.input_schema),
            })
            .collect()
    }

    /// The server-side name for an agent-visible name, or `None` when this
    /// server does not advertise it.
    fn raw_name(&self, exposed: &str) -> Option<&str> {
        self.advertised
            .iter()
            .find(|t| t.exposed.as_str() == exposed)
            .map(|t| t.raw.name.as_str())
    }

    /// The replay posture for one advertised tool.
    fn replay_safety(&self, exposed: &str) -> ReplaySafety {
        let raw = self.raw_name(exposed);
        self.replay_safety_by_tool
            .as_ref()
            .and_then(|overrides| {
                overrides
                    .get(exposed)
                    .or_else(|| raw.and_then(|raw| overrides.get(raw)))
            })
            .copied()
            .unwrap_or(self.replay_safety)
    }
}

/// Give an MCP tool an object schema some model providers will accept.
///
/// Several providers reject an object schema without `properties`, and a
/// server that sends `{}` means "no arguments". Add a missing `type` and, for
/// an object schema, a missing `properties`; never rewrite anything else — the
/// server's schema is otherwise passed through untouched.
fn normalize_input_schema(schema: &Value) -> Value {
    let Value::Object(map) = schema else {
        return schema.clone();
    };
    let mut map = map.clone();
    match map.get("type") {
        None => {
            map.insert("type".into(), Value::String("object".into()));
            map.entry("properties")
                .or_insert_with(|| Value::Object(Default::default()));
        }
        Some(Value::String(kind)) if kind == "object" => {
            map.entry("properties")
                .or_insert_with(|| Value::Object(Default::default()));
        }
        _ => {}
    }
    Value::Object(map)
}

/// The namespace a server's tools live under.
///
/// An explicit [`prefix`](McpServerConfig::prefix) wins; otherwise the
/// server's [`name`](McpServerConfig::name) is the default, so two servers
/// with distinct names cannot collide on an agent-visible name. The default
/// belongs here, not in host composition: a host-side namespace wrapper
/// whole executor with one string and cannot vary per server.
fn namespace(config: &McpServerConfig) -> &str {
    config.prefix.as_deref().unwrap_or(&config.name)
}

/// The `mcp__<namespace>__` prefix shared by every name a server exposes.
///
/// The namespace is sanitized to `[A-Za-z0-9_]` and length-capped so a long or
/// punctuation-heavy server name cannot produce an invalid tool name.
fn namespace_prefix(config: &McpServerConfig) -> String {
    let mut namespace = sanitize(namespace(config));
    if namespace.len() > MAX_NAMESPACE_LEN {
        namespace.truncate(MAX_NAMESPACE_LEN);
    }
    format!("{MCP_PREFIX}{namespace}__")
}

/// The agent-visible name of one of a server's tools: `mcp__<ns>__<tool>`.
///
/// The name is sanitized and capped at [`MAX_TOOL_NAME_LEN`]; a tool segment
/// that would overflow the cap is truncated and suffixed with a stable hash so
/// the name stays unique.
fn expose(config: &McpServerConfig, tool: &str) -> String {
    let prefix = namespace_prefix(config);
    let sanitized = sanitize(tool);
    let budget = MAX_TOOL_NAME_LEN - prefix.len() - HASH_SUFFIX_LEN;
    if sanitized.len() <= budget {
        format!("{prefix}{sanitized}")
    } else {
        format!("{prefix}{}_{}", &sanitized[..budget], hash8(tool))
    }
}

/// Whether the server's allowlist admits a tool, under either name.
fn allowed(config: &McpServerConfig, exposed: &str, raw: &str) -> bool {
    match &config.allowed_tools {
        Some(allowed) => allowed.iter().any(|a| a == exposed || a == raw),
        None => true,
    }
}

/// Whether the server's deny-list drops a tool, under either name.
///
/// Applied after [`allowed`], so a name that is both allow-listed and
/// deny-listed is dropped — the deny-list states the more specific intent.
fn denied(config: &McpServerConfig, exposed: &str, raw: &str) -> bool {
    match &config.disabled_tools {
        Some(disabled) => disabled.iter().any(|d| d == exposed || d == raw),
        None => false,
    }
}

/// Whether this server *could* expose `exposed`, from its namespace alone.
///
/// An I/O-free gate: every name the server exposes carries its namespace, so a
/// name outside it never belongs to this server — and an unreachable server is
/// not consulted (nor blamed) for it.
fn could_own(config: &McpServerConfig, exposed: &str) -> bool {
    exposed.starts_with(&namespace_prefix(config))
}

/// A [`ToolExecutor`] that connects to one or more MCP servers.
///
/// Servers are connected lazily on first use. If a server is unreachable the
/// error surfaces on the first call that needs it.
///
/// The manifest holds one entry per exposed tool name, each built as
/// `mcp__<namespace>__<tool>` from the server's
/// [`prefix`](McpServerConfig::prefix) or else its
/// [`name`](McpServerConfig::name), so distinct servers cannot collide; only
/// two servers that sanitize to the same namespace can, and there the first in
/// the list owns the name — which is also the server a call for it is routed
/// to.
pub struct McpExecutor {
    servers: Arc<Vec<ConnectedServer>>,
    /// The synchronous manifest the kernel reads, rebuilt by [`refresh`](Self::refresh).
    manifest: RwLock<Vec<Tool>>,
    /// The replay posture of each advertised tool, rebuilt with the manifest.
    replay: RwLock<HashMap<String, ReplaySafety>>,
}

struct ConnectedServer {
    config: McpServerConfig,
    /// The handler for mid-call input requests, shared by every server and
    /// read once, at connect time — installing one after a server has
    /// connected does not reach the session already established.
    input: Arc<OnceLock<InputHandler>>,
    /// The live session and its resolved tool set, once a call has needed it.
    state: Mutex<Option<Connected>>,
}

struct Connected {
    client: Client,
    route: ServerRoute,
}

impl ConnectedServer {
    /// The bound on connecting this server and fetching its tool list.
    fn startup_timeout(&self) -> Duration {
        Duration::from_millis(
            self.config
                .startup_timeout_ms
                .unwrap_or(DEFAULT_STARTUP_TIMEOUT_MS),
        )
    }

    /// Connect and fetch the first tool list, bounded by the startup timeout
    /// and — over HTTP only — retried on a transient transport error.
    ///
    /// `tools/list` is idempotent, so a retry cannot double an effect; a tool
    /// call is never retried, because it may already have run.
    async fn connect_and_list(&self) -> Result<Connected, Error> {
        let startup = self.startup_timeout();
        let http = self.config.transport.is_http();
        let mut attempt = 0;
        loop {
            let outcome = tokio::time::timeout(startup, async {
                let client = Client::connect_with(
                    &self.config.transport,
                    ClientOptions {
                        input: self.input.get().cloned(),
                    },
                )
                .await?;
                let tools = client.list_tools().await?;
                Ok::<_, Error>(Connected {
                    client,
                    route: ServerRoute::build(&self.config, tools)?,
                })
            })
            .await;
            let result = match outcome {
                Ok(result) => result,
                Err(_) => Err(Error::StartupTimeout {
                    server: self.config.name.clone(),
                    ms: startup.as_millis() as u64,
                }),
            };
            match result {
                Ok(connected) => return Ok(connected),
                Err(error) if http && attempt < RETRY_DELAYS_MS.len() && is_retryable(&error) => {
                    tokio::time::sleep(Duration::from_millis(RETRY_DELAYS_MS[attempt])).await;
                    attempt += 1;
                }
                Err(error) => return Err(error),
            }
        }
    }

    /// Re-list an already-connected server under the same startup bound and
    /// HTTP retry policy as the first list.
    async fn relist(&self, conn: &mut Connected) -> Result<(), Error> {
        let startup = self.startup_timeout();
        let http = self.config.transport.is_http();
        let mut attempt = 0;
        loop {
            match tokio::time::timeout(startup, conn.client.list_tools()).await {
                Ok(Ok(tools)) => {
                    conn.route = ServerRoute::build(&self.config, tools)?;
                    return Ok(());
                }
                Ok(Err(error))
                    if http && attempt < RETRY_DELAYS_MS.len() && is_retryable(&error) =>
                {
                    tokio::time::sleep(Duration::from_millis(RETRY_DELAYS_MS[attempt])).await;
                    attempt += 1;
                }
                Ok(Err(error)) => return Err(error),
                Err(_) => {
                    return Err(Error::StartupTimeout {
                        server: self.config.name.clone(),
                        ms: startup.as_millis() as u64,
                    });
                }
            }
        }
    }

    /// Lock this server, connecting it if needed and — on first use, or when
    /// `refresh` — asking it for its tool list.
    ///
    /// On `Ok`, the guarded state is always `Some`.
    async fn connected(&self, refresh: bool) -> Result<MutexGuard<'_, Option<Connected>>, Error> {
        let mut guard = self.state.lock().await;
        match guard.as_mut() {
            Some(_) if !refresh => {}
            Some(conn) => self.relist(conn).await?,
            None => *guard = Some(self.connect_and_list().await?),
        }
        Ok(guard)
    }
}

/// Union tool lists in deterministic order; the first server to advertise a
/// name owns it.
fn merge_tools(lists: impl IntoIterator<Item = Vec<Tool>>) -> Vec<Tool> {
    let mut merged = Vec::new();
    let mut seen = HashSet::new();
    for list in lists {
        for tool in list {
            // Names are checked at descriptor construction, so this set only
            // decides ownership between servers advertising the same name.
            if seen.insert(tool.name.as_str().to_owned()) {
                merged.push(tool);
            }
        }
    }
    merged
}

/// Whether a failure is worth retrying for an idempotent HTTP request.
///
/// Only transport-class failures qualify. A plain-text HTTP 5xx (the common
/// gateway/proxy failure) surfaces as one and is retried end-to-end — see
/// `tests/live_http.rs`. rmcp does not surface the HTTP status on a request
/// error, so a 5xx that arrives as a JSON-RPC error is *not* retried. That
/// residual gap is documented in the crate README and deferred to an upstream
/// rmcp accessor rather than a transport wrapper.
fn is_retryable(error: &Error) -> bool {
    matches!(
        error,
        Error::Service(ServiceError::TransportClosed)
            | Error::Service(ServiceError::TransportSend(_))
            | Error::Initialize(ClientInitializeError::ConnectionClosed(_))
            | Error::Initialize(ClientInitializeError::TransportError { .. })
    )
}

impl McpExecutor {
    /// Create a new executor without connecting to any server.
    ///
    /// Connection is deferred until the first call to [`refresh`](Self::refresh)
    /// or [`run`](ToolExecutor::run).
    pub fn new(configs: Vec<McpServerConfig>) -> Self {
        let input: Arc<OnceLock<InputHandler>> = Arc::new(OnceLock::new());
        let servers = configs
            .into_iter()
            .map(|config| ConnectedServer {
                config,
                input: Arc::clone(&input),
                state: Mutex::new(None),
            })
            .collect();
        Self {
            servers: Arc::new(servers),
            manifest: RwLock::new(Vec::new()),
            replay: RwLock::new(HashMap::new()),
        }
    }

    /// Create an executor and make its manifest ready for `tools()`.
    ///
    /// This is the constructor a host should use at startup: it connects every
    /// server and performs the first `tools/list` before the executor is
    /// handed to the runtime.
    pub async fn connect(configs: Vec<McpServerConfig>) -> Result<Self, Error> {
        let executor = Self::new(configs);
        executor.refresh().await?;
        Ok(executor)
    }

    /// Connect every server and rebuild the synchronous manifest snapshot.
    ///
    /// The kernel's [`tools`](ToolExecutor::tools) is synchronous and has no
    /// error channel, so discovery is explicit: the host calls this before a
    /// turn and handles the failure. A failed refresh leaves the previous
    /// snapshot untouched. That includes a server advertising a name the
    /// bridge cannot expose: the error is reported rather than silently
    /// publishing a partial manifest.
    pub async fn refresh(&self) -> Result<(), Error> {
        let mut lists = Vec::with_capacity(self.servers.len());
        let mut replay = HashMap::new();
        for server in self.servers.iter() {
            let guard = server.connected(true).await?;
            // `connected` returned `Ok`, so the session exists.
            let Some(conn) = guard.as_ref() else { continue };
            let tools = conn.route.tools();
            for tool in &tools {
                // First server to advertise a name owns it, exactly as the
                // manifest merge decides; the replay map must agree.
                replay
                    .entry(tool.name.as_str().to_owned())
                    .or_insert_with(|| conn.route.replay_safety(tool.name.as_str()));
            }
            lists.push(tools);
        }

        // One entry per name, owned by the first server that advertises it —
        // the server `run` routes to. A name two servers expose without a
        // prefix is shadowed, never listed twice.
        let merged = merge_tools(lists);

        *self.manifest.write().unwrap_or_else(|e| e.into_inner()) = merged;
        *self.replay.write().unwrap_or_else(|e| e.into_inner()) = replay;
        Ok(())
    }

    /// The human-readable `instructions` each reachable server advertised.
    ///
    /// This is the server-wide guidance an agent author wants in the model's
    /// context (both Codex and pi place it there) — the bridge only surfaces
    /// it; where it goes is the host's call. Servers that sent none, or that
    /// are unreachable, are omitted: this is advisory metadata and never fails
    /// a caller.
    pub async fn instructions(&self) -> Vec<(String, String)> {
        let mut collected = Vec::new();
        for server in self.servers.iter() {
            if let Ok(guard) = server.connected(false).await
                && let Some(conn) = guard.as_ref()
                && let Some(text) = conn.client.instructions()
            {
                collected.push((server.config.name.clone(), text));
            }
        }
        collected
    }

    /// Install the handler that answers a server's mid-call input request
    /// (SEP-2322 MRTR).
    ///
    /// Without one the bridge declares no elicitation capability, so a
    /// conformant server will not ask; a server that asks anyway fails the
    /// call with an error naming the gap instead of a silent decline.
    /// Install it before the first call — a server that has already
    /// connected keeps the handshake it made.
    pub fn with_input(self, handler: InputHandler) -> Self {
        // Every server shares one slot, so setting it on any of them sets it
        // for all; with no servers configured there is nobody to answer.
        if let Some(server) = self.servers.first() {
            let _ = server.input.set(handler);
        }
        self
    }
}

impl ToolExecutor for McpExecutor {
    fn tools(&self) -> Vec<Tool> {
        self.manifest
            .read()
            .map(|manifest| manifest.clone())
            .unwrap_or_default()
    }

    fn effect_id(&self, request: &ToolRequest) -> EffectId {
        effect_id_for(request)
    }

    fn replay_safety(&self, tool: &str) -> ReplaySafety {
        self.replay
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .get(tool)
            .copied()
            .unwrap_or(ReplaySafety::Unsafe)
    }

    fn replay_limit(&self) -> u32 {
        self.servers
            .iter()
            .map(|server| server.config.replay_limit)
            .max()
            .unwrap_or(1)
    }

    async fn reconcile(
        &self,
        _invocation: &EffectInvocation,
    ) -> Result<Option<Outcome>, ToolError> {
        Ok(None)
    }

    async fn run(&self, invocation: EffectInvocation) -> Result<ToolReply, ToolError> {
        let request = invocation.request;
        let tool_name = request.name.as_str().to_owned();
        // `Arguments` guarantees the text is a complete JSON value; MCP still
        // requires the object shape the server's schema describes, so that
        // stays a runtime check here.
        let arguments: Value =
            serde_json::from_str(request.arguments.as_str()).map_err(|error| {
                ToolError::InvalidArguments {
                    tool: tool_name.clone(),
                    message: format!("arguments must be a JSON object: {error}"),
                }
            })?;
        if !arguments.is_object() {
            return Err(ToolError::InvalidArguments {
                tool: tool_name.clone(),
                message: "arguments must be a JSON object".into(),
            });
        }

        // A server that cannot be reached does not own the name: remember it
        // and keep routing, the way a host-side composite does.
        let mut unavailable: Option<ToolError> = None;

        for server in self.servers.iter() {
            // A name outside this server's namespace is nobody's here: no dial,
            // no blame on an unreachable server.
            if !could_own(&server.config, &tool_name) {
                continue;
            }
            let guard = match server.connected(false).await {
                Ok(guard) => guard,
                Err(error) => {
                    unavailable.get_or_insert_with(|| ToolError::Unavailable {
                        tool: tool_name.clone(),
                        message: format!(
                            "MCP server `{}` unavailable: {error}",
                            server.config.name
                        ),
                    });
                    continue;
                }
            };
            // Unreachable: `connected` returned `Ok`, so the session exists.
            let Some(conn) = guard.as_ref() else { continue };
            // A name this server does not advertise is not this server's to
            // answer for — an unknown name never reaches the wire.
            let Some(raw) = conn.route.raw_name(&tool_name).map(str::to_owned) else {
                continue;
            };

            let timeout = Duration::from_millis(conn.route.tool_timeout_ms);
            let result = match tokio::time::timeout(
                timeout,
                conn.client.call(&raw, arguments.clone()),
            )
            .await
            {
                Ok(Ok(result)) => result,
                Ok(Err(error)) => return Err(classify_error(&tool_name, error)),
                Err(_) => return Err(ToolError::Timeout { tool: tool_name }),
            };

            // A server-reported tool failure ran and failed: the kernel models
            // that as a successful reply carrying `is_error`, not a seam error.
            let text = if result.is_error() {
                result.text().unwrap_or_else(|| result.raw.to_string())
            } else {
                render_content(&result.raw).unwrap_or_else(|| result.raw.to_string())
            };

            // The kernel pairs the reply with the invocation's own call id;
            // the bridge never echoes one back.
            return Ok(ToolReply::Output {
                output: ToolOutput::Inline(text),
                is_error: result.is_error(),
            });
        }

        Err(unavailable.unwrap_or(ToolError::UnknownTool { tool: tool_name }))
    }
}

/// A stable identity for one call, supplied to the kernel by this executor.
///
/// The kernel mints no effect identity; the same request must always yield the
/// same id so a replay reuses it. FNV-1a over the call id, name and arguments
/// is deterministic across runs and builds.
fn effect_id_for(request: &ToolRequest) -> EffectId {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in request
        .id
        .as_str()
        .bytes()
        .chain(request.name.as_str().bytes())
        .chain(request.arguments.as_str().bytes())
    {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&hash.to_le_bytes());
    EffectId(Id::from_bytes(bytes))
}

/// Map an MCP failure onto the kernel's error classes.
///
/// A server's error code says what the *bridge* knows, so the class it earns
/// decides whether a host-side composite may fall through:
///
/// * `-32602` `INVALID_PARAMS` is how a `2026-07-28` server answers a call for
///   a tool it does not recognise (SEP-2164 upgrades unknown-tool to it), so it
///   is [`ToolError::UnknownTool`] — exactly the stale-manifest miss a set must
///   route past.
/// * `-32601` `METHOD_NOT_FOUND` means the peer does not implement
///   `tools/call` at all: that server cannot answer, so
///   [`ToolError::Unavailable`].
/// * `-32022` `UNSUPPORTED_PROTOCOL_VERSION` is likewise per-peer — another
///   server may speak the version we negotiated — so also `Unavailable`, with
///   the code named in the message.
/// * `-32021` `MISSING_REQUIRED_CLIENT_CAPABILITY` is *our* configuration: the
///   server wants a capability this tools-only bridge deliberately does not
///   declare (see [`client::client_config`](crate::client)). No routing fixes
///   that, so it is a [`ToolError::Transport`] failure rather than being
///   silently absorbed by the next executor in a set.
fn classify_error(tool: &str, error: Error) -> ToolError {
    if let Error::Service(ServiceError::McpError(data)) = &error {
        return classify_code(tool, data.code, &data.message);
    }
    match error {
        Error::InvalidArguments => ToolError::InvalidArguments {
            tool: tool.to_owned(),
            message: "arguments must be a JSON object".into(),
        },
        // SEP-2663: a server may answer a call with a task handle the client
        // never asked for, and the SDK's `call_tool` helper refuses to drive
        // that lifecycle. Tasks are a deliberate non-goal here (see the module
        // docs), but the failure should say so rather than be opaque.
        Error::Service(ServiceError::UnexpectedResponse) => ToolError::Transport(
            "server returned an unsolicited SEP-2663 task handle, which this bridge does not \
             support (a declared tasks client would poll tasks/get)"
                .into(),
        ),
        other => ToolError::Transport(format!("MCP execute: {other}")),
    }
}

/// The [`ToolError`] one MCP error code earns; see [`classify_error`].
fn classify_code(tool: &str, code: ErrorCode, message: &str) -> ToolError {
    match code {
        ErrorCode::INVALID_PARAMS => ToolError::UnknownTool {
            tool: tool.to_owned(),
        },
        ErrorCode::METHOD_NOT_FOUND => ToolError::Unavailable {
            tool: tool.to_owned(),
            message: format!("server does not implement tools/call: {message}"),
        },
        ErrorCode::UNSUPPORTED_PROTOCOL_VERSION => ToolError::Unavailable {
            tool: tool.to_owned(),
            message: format!("server rejected the negotiated protocol version: {message}"),
        },
        ErrorCode::MISSING_REQUIRED_CLIENT_CAPABILITY => ToolError::Transport(format!(
            "server requires a client capability this bridge does not declare: {message}"
        )),
        other => ToolError::Transport(format!("MCP error {}: {message}", other.0)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    use morphium::tool::{Arguments, CallId};
    use rmcp::model::ErrorData;
    use serde_json::Value;

    /// A one-attempt invocation for a caller-side call: the executor is the
    /// only thing under test, so the identity is the caller's to mint.
    fn invoke(executor: &McpExecutor, name: &str, arguments: Value) -> EffectInvocation {
        let request = ToolRequest::new(
            CallId::parse(format!("c-{name}")).expect("literal call id"),
            ToolName::parse(name).expect("literal tool name"),
            Arguments::parse(arguments.to_string()).expect("literal arguments"),
        );
        EffectInvocation::first(executor.effect_id(&request), request)
    }

    fn tool(name: &str) -> RawTool {
        RawTool {
            name: name.into(),
            description: Some(format!("{name} tool")),
            title: None,
            input_schema: serde_json::json!({"type": "object", "properties": {}}),
            output_schema: None,
            annotations: None,
        }
    }

    /// A `ServiceError::McpError` with the given code, as a live call would
    /// surface it through [`Error::Service`].
    fn mcp_error(code: ErrorCode) -> Error {
        Error::Service(ServiceError::McpError(ErrorData::new(code, "boom", None)))
    }

    fn dead() -> McpTransport {
        McpTransport::Http {
            url: "http://127.0.0.1:1/mcp".into(),
            headers: HashMap::new(),
        }
    }

    fn config(allowed: Option<Vec<String>>, prefix: Option<String>) -> McpServerConfig {
        McpServerConfig {
            name: "srv".into(),
            transport: dead(),
            allowed_tools: allowed,
            disabled_tools: None,
            replay_safety: ReplaySafety::Unsafe,
            replay_safety_by_tool: None,
            replay_limit: 1,
            prefix,
            tool_timeout_ms: None,
            startup_timeout_ms: None,
        }
    }

    fn route(config: &McpServerConfig, names: &[&str]) -> ServerRoute {
        ServerRoute::build(config, names.iter().map(|n| tool(n)).collect())
            .expect("literal tool names are exposed non-empty")
    }

    #[test]
    fn no_allowlist_exposes_every_advertised_tool_under_the_default_namespace() {
        // Without an explicit prefix the server's name namespaces its tools,
        // under the `mcp__` marker both reference clients use.
        let cfg = config(None, None);
        let route = route(&cfg, &["read", "write"]);
        let names: Vec<_> = route
            .tools()
            .into_iter()
            .map(|d| d.name.into_string())
            .collect();
        assert_eq!(names, ["mcp__srv__read", "mcp__srv__write"]);
    }

    #[test]
    fn the_server_route_carries_the_configured_tool_timeout() {
        // The kernel has no descriptor timeout, so `run` enforces this one.
        let cfg = config(None, None);
        assert_eq!(
            route(&cfg, &["read"]).tool_timeout_ms,
            DEFAULT_TOOL_TIMEOUT_MS
        );

        let override_cfg = McpServerConfig {
            tool_timeout_ms: Some(1_234),
            ..config(None, None)
        };
        assert_eq!(route(&override_cfg, &["read"]).tool_timeout_ms, 1_234);
    }

    #[test]
    fn replay_safety_defaults_to_unsafe_and_uses_overrides() {
        let cfg = config(None, None);
        let default_route = route(&cfg, &["read", "write"]);
        assert_eq!(
            default_route.replay_safety("mcp__srv__read"),
            ReplaySafety::Unsafe
        );

        let mut overrides = HashMap::new();
        overrides.insert("read".into(), ReplaySafety::Idempotent);
        let cfg = McpServerConfig {
            replay_safety: ReplaySafety::Reconciling,
            replay_safety_by_tool: Some(overrides),
            ..config(None, None)
        };
        let overridden = route(&cfg, &["read", "write"]);
        assert_eq!(
            overridden.replay_safety("mcp__srv__read"),
            ReplaySafety::Idempotent,
            "a raw-name override wins"
        );
        assert_eq!(
            overridden.replay_safety("mcp__srv__write"),
            ReplaySafety::Reconciling
        );
    }

    #[test]
    fn replay_limit_is_the_highest_configured_bound() {
        let mut first = config(None, None);
        first.replay_limit = 2;
        let mut second = config(None, None);
        second.name = "other".into();
        second.replay_limit = 5;
        assert_eq!(McpExecutor::new(vec![first, second]).replay_limit(), 5);
    }

    #[test]
    fn replay_safety_answers_from_the_refreshed_map() {
        let executor = McpExecutor::new(Vec::new());
        executor
            .replay
            .write()
            .unwrap()
            .insert("mcp__srv__read".into(), ReplaySafety::Idempotent);
        assert_eq!(
            executor.replay_safety("mcp__srv__read"),
            ReplaySafety::Idempotent
        );
        assert_eq!(
            executor.replay_safety("mcp__srv__write"),
            ReplaySafety::Unsafe,
            "an unknown name is never assumed replayable"
        );
    }

    #[test]
    fn names_are_sanitized_into_a_valid_tool_name() {
        // A server name with a space and a dot and a tool name with a dash must
        // still produce a name SEP-986 accepts.
        let cfg = McpServerConfig {
            name: "my server.v2".into(),
            ..config(None, None)
        };
        let route = route(&cfg, &["read-file"]);
        let name = route.tools()[0].name.clone();
        assert_eq!(name.as_str(), "mcp__my_server_v2__read_file");
        assert!(
            name.as_str()
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_'),
            "every exposed char must be valid: {}",
            name.as_str()
        );
    }

    #[test]
    fn long_names_stay_within_the_cap_and_unique() {
        // A 40-char namespace and a 40-char tool overflow 64; the result must
        // still fit and stay distinct from a sibling tool.
        let cfg = McpServerConfig {
            name: "n".repeat(40),
            ..config(None, None)
        };
        let route = route(&cfg, &[&"t".repeat(40), &format!("{}x", "t".repeat(39))]);
        let names: Vec<_> = route
            .tools()
            .into_iter()
            .map(|d| d.name.into_string())
            .collect();
        assert!(
            names.iter().all(|n| n.len() <= MAX_TOOL_NAME_LEN),
            "names must fit the cap: {names:?}"
        );
        assert_ne!(names[0], names[1], "capped names must not collide");
    }

    #[test]
    fn two_unprefixed_servers_never_collide_on_a_bare_tool_name() {
        // Both servers advertise `read` and declare no prefix. Namespacing by
        // name means two entries survive the merge, each routing to its own
        // server — the spec's "aggregating clients SHOULD disambiguate".
        let alpha = |name: &str| McpServerConfig {
            name: name.into(),
            ..config(None, None)
        };
        let a = ServerRoute::build(&alpha("alpha"), vec![tool("read")])
            .expect("literal tool names are exposed non-empty");
        let b = ServerRoute::build(&alpha("beta"), vec![tool("read")])
            .expect("literal tool names are exposed non-empty");
        let names: Vec<_> = merge_tools(vec![a.tools(), b.tools()])
            .into_iter()
            .map(|tool| tool.name.into_string())
            .collect();
        assert_eq!(names, ["mcp__alpha__read", "mcp__beta__read"]);

        // Each entry resolves against its own server and no other, so a call
        // reaches the server that advertised it.
        assert_eq!(a.raw_name("mcp__alpha__read"), Some("read"));
        assert_eq!(a.raw_name("mcp__beta__read"), None);
        assert_eq!(b.raw_name("mcp__beta__read"), Some("read"));
        assert!(could_own(&alpha("alpha"), "mcp__alpha__read"));
        assert!(!could_own(&alpha("alpha"), "mcp__beta__read"));
    }

    #[test]
    fn colliding_sanitized_tool_names_get_distinct_suffixes() {
        // `a-b` and `a.b` both sanitize to `a_b`; neither may silently shadow
        // the other in the manifest.
        let cfg = config(None, None);
        let route = route(&cfg, &["a-b", "a.b"]);
        let names: Vec<_> = route
            .tools()
            .into_iter()
            .map(|d| d.name.into_string())
            .collect();
        assert_eq!(names.len(), 2, "{names:?}");
        assert_ne!(names[0], names[1], "{names:?}");
        assert!(names.contains(&"mcp__srv__a_b".to_string()), "{names:?}");
        // The suffixed name still routes to the server's own tool name.
        assert_eq!(route.raw_name(&names[1]), Some("a.b"));
    }

    #[test]
    fn prefix_is_applied_to_exposed_names_only() {
        let cfg = config(None, Some("docker".into()));
        let route = route(&cfg, &["read"]);
        assert_eq!(route.tools()[0].name.as_str(), "mcp__docker__read");
        // The name that reaches the server is still the server's own.
        assert_eq!(route.raw_name("mcp__docker__read"), Some("read"));
        assert_eq!(route.raw_name("read"), None);
    }

    #[test]
    fn allowlist_matches_exposed_or_raw_name() {
        let prefixed = route(
            &config(
                Some(vec!["mcp__docker__read".into()]),
                Some("docker".into()),
            ),
            &["read", "write"],
        );
        assert_eq!(prefixed.tools().len(), 1);
        assert_eq!(prefixed.raw_name("mcp__docker__read"), Some("read"));
        assert_eq!(prefixed.raw_name("mcp__docker__write"), None);

        // Under the default namespace the agent-visible name is
        // `mcp__srv__read`, but an allowlist written in the server's own
        // vocabulary still admits it, and the bare name is not routable.
        let raw = route(&config(Some(vec!["read".into()]), None), &["read", "write"]);
        assert_eq!(raw.tools().len(), 1);
        assert_eq!(raw.raw_name("mcp__srv__read"), Some("read"));
        assert_eq!(raw.raw_name("read"), None);
        assert_eq!(raw.raw_name("mcp__srv__write"), None);
    }

    #[test]
    fn deny_list_wins_over_allow_list() {
        // A tool that is both allow-listed and deny-listed is dropped: the
        // deny-list states the more specific intent.
        let allowed_only = route(&config(Some(vec!["read".into()]), None), &["read", "write"]);
        assert_eq!(allowed_only.tools().len(), 1);

        let disabled = McpServerConfig {
            disabled_tools: Some(vec!["read".into()]),
            ..config(Some(vec!["read".into()]), None)
        };
        let route = route(&disabled, &["read", "write"]);
        assert!(route.tools().is_empty());
        assert_eq!(route.raw_name("mcp__srv__read"), None);
    }

    #[test]
    fn deny_list_matches_exposed_or_raw_name() {
        let by_exposed = McpServerConfig {
            disabled_tools: Some(vec!["mcp__srv__read".into()]),
            ..config(None, None)
        };
        assert!(route(&by_exposed, &["read"]).tools().is_empty());

        let by_raw = McpServerConfig {
            disabled_tools: Some(vec!["read".into()]),
            ..config(None, None)
        };
        assert!(route(&by_raw, &["read"]).tools().is_empty());
    }

    #[test]
    fn empty_allowlist_exposes_nothing() {
        let cfg = config(Some(vec![]), None);
        let route = route(&cfg, &["read"]);
        assert!(route.tools().is_empty());
        assert_eq!(route.raw_name("read"), None);
    }

    #[test]
    fn unadvertised_name_has_no_route() {
        // A name sharing the server's prefix but absent from `tools/list`
        // belongs to nobody — it must resolve to `NotFound`, not be forwarded.
        let cfg = config(None, Some("docker".into()));
        let route = route(&cfg, &["read"]);
        assert_eq!(route.raw_name("mcp__docker__nope"), None);
    }

    #[test]
    fn prefix_gate_rejects_foreign_namespaces() {
        let cfg = config(None, Some("docker".into()));
        assert!(could_own(&cfg, "mcp__docker__read"));
        assert!(could_own(&cfg, "mcp__docker__"));
        assert!(!could_own(&cfg, "docker__read")); // no `mcp__` marker
        assert!(!could_own(&cfg, "mcp__other__read"));
        assert!(!could_own(&cfg, "read"));
        // Without an explicit prefix the server's own name is the namespace,
        // so a bare name still belongs to nobody.
        let default = config(None, None);
        assert!(could_own(&default, "mcp__srv__read"));
        assert!(!could_own(&default, "read"));
        assert!(!could_own(&default, "mcp__other__read"));
    }

    #[tokio::test]
    async fn unreachable_server_surfaces_on_first_refresh() {
        let executor = McpExecutor::new(vec![config(None, None)]);
        // Connection is lazy — the failure appears on first use, not at
        // construction.
        let err = tokio::time::timeout(Duration::from_secs(10), executor.refresh())
            .await
            .expect("connect to a dead port must fail fast")
            .expect_err("refresh must fail");
        assert!(
            matches!(err, Error::Initialize(_) | Error::Service(_)),
            "got {err:?}"
        );
    }

    #[tokio::test]
    async fn a_failed_refresh_preserves_the_previous_manifest() {
        // Discovery is fallible, but the kernel's `tools()` reads a snapshot:
        // a refresh that fails must leave the last good manifest in place.
        let cfg = config(None, None);
        let previous = route(&cfg, &["read"]).tools();
        let executor = McpExecutor::new(vec![cfg]);
        *executor.manifest.write().unwrap_or_else(|e| e.into_inner()) = previous.clone();

        tokio::time::timeout(Duration::from_secs(10), executor.refresh())
            .await
            .expect("connect to a dead port must fail fast")
            .expect_err("refresh must fail");

        assert_eq!(executor.tools(), previous);
    }

    #[tokio::test]
    async fn unreachable_server_is_unavailable_on_run() {
        let executor = McpExecutor::new(vec![config(None, None)]);
        // A name inside the server's namespace but behind a dead transport is
        // `Unavailable` (not `UnknownTool`) so a composite routes the call on.
        let err = tokio::time::timeout(
            Duration::from_secs(10),
            executor.run(invoke(&executor, "mcp__srv__read", serde_json::json!({}))),
        )
        .await
        .expect("connect to a dead port must fail fast")
        .expect_err("run must fail");
        assert!(matches!(err, ToolError::Unavailable { .. }), "got {err:?}");
    }

    #[tokio::test]
    async fn bare_name_outside_every_namespace_is_unknown_without_dialing() {
        // The default namespace is the server's name, so a bare `read` belongs
        // to nobody: it is `UnknownTool` immediately, never blamed on the dead
        // server behind it.
        let executor = McpExecutor::new(vec![config(None, None)]);
        let err = tokio::time::timeout(
            Duration::from_secs(10),
            executor.run(invoke(&executor, "read", serde_json::json!({}))),
        )
        .await
        .expect("no dial happens, so nothing times out")
        .expect_err("unknown name must fail");
        assert!(matches!(err, ToolError::UnknownTool { .. }), "got {err:?}");
    }

    #[tokio::test]
    async fn stdio_missing_binary_surfaces_on_first_refresh() {
        let executor = McpExecutor::new(vec![McpServerConfig {
            name: "srv".into(),
            transport: McpTransport::Stdio {
                command: "morphium-no-such-binary".into(),
                args: vec![],
                env: HashMap::new(),
                cwd: None,
            },
            allowed_tools: None,
            disabled_tools: None,
            replay_safety: ReplaySafety::Unsafe,
            replay_safety_by_tool: None,
            replay_limit: 1,
            prefix: None,
            tool_timeout_ms: None,
            startup_timeout_ms: None,
        }]);
        // Spawning a missing stdio binary must fail on first use, just
        // like an unreachable HTTP server.
        let err = tokio::time::timeout(Duration::from_secs(10), executor.refresh())
            .await
            .expect("spawn of a missing binary must fail fast")
            .expect_err("refresh must fail");
        assert!(
            matches!(err, Error::Io(_) | Error::Initialize(_) | Error::Service(_)),
            "got {err:?}"
        );
    }

    #[test]
    fn title_backfills_only_a_missing_description() {
        // The descriptor has no display-name field: a title fills the gap when
        // the server sent no description, and never overrides a real one.
        let cfg = config(None, None);
        let mut untitled = tool("a");
        untitled.description = None;
        untitled.title = Some("Do A".into());
        let mut described = tool("b");
        described.title = Some("A Label".into());
        let descriptors = ServerRoute::build(&cfg, vec![untitled, described])
            .expect("literal tool names are exposed non-empty")
            .tools();
        assert_eq!(descriptors[0].description, "Do A");
        assert_eq!(descriptors[1].description, "b tool");
    }

    #[test]
    fn schema_without_type_gains_object_and_properties() {
        // Some providers reject an object schema with no `properties`; `{}`
        // means "no arguments", so normalize it rather than pass it through.
        let cfg = config(None, None);
        let mut advertised = tool("q");
        advertised.input_schema = serde_json::json!({});
        let schema = ServerRoute::build(&cfg, vec![advertised])
            .expect("literal tool names are exposed non-empty")
            .tools()[0]
            .parameters
            .clone();
        assert_eq!(
            schema,
            serde_json::json!({"type": "object", "properties": {}})
        );
    }

    #[test]
    fn object_schema_missing_properties_gains_an_empty_map() {
        let cfg = config(None, None);
        let mut advertised = tool("q");
        advertised.input_schema = serde_json::json!({"type": "object"});
        let schema = ServerRoute::build(&cfg, vec![advertised])
            .expect("literal tool names are exposed non-empty")
            .tools()[0]
            .parameters
            .clone();
        assert_eq!(
            schema,
            serde_json::json!({"type": "object", "properties": {}})
        );
    }

    #[test]
    fn a_complete_schema_is_passed_through_untouched() {
        let cfg = config(None, None);
        let original = serde_json::json!({
            "type": "object",
            "properties": {"message": {"type": "string"}},
            "required": ["message"]
        });
        let mut advertised = tool("echo");
        advertised.input_schema = original.clone();
        let schema = ServerRoute::build(&cfg, vec![advertised])
            .expect("literal tool names are exposed non-empty")
            .tools()[0]
            .parameters
            .clone();
        assert_eq!(schema, original);
    }

    #[test]
    fn startup_timeout_defaults_and_is_overridable() {
        let server = ConnectedServer {
            config: config(None, None),
            input: Arc::new(OnceLock::new()),
            state: Mutex::new(None),
        };
        assert_eq!(
            server.startup_timeout(),
            Duration::from_millis(DEFAULT_STARTUP_TIMEOUT_MS)
        );

        let custom = ConnectedServer {
            config: McpServerConfig {
                startup_timeout_ms: Some(2_500),
                ..config(None, None)
            },
            input: Arc::new(OnceLock::new()),
            state: Mutex::new(None),
        };
        assert_eq!(custom.startup_timeout(), Duration::from_millis(2_500));
    }

    #[test]
    fn only_transport_failures_are_retryable() {
        // A retry is only safe for an idempotent request that dropped; a
        // JSON-RPC error is an answer, not a transient blip.
        assert!(is_retryable(&Error::Service(ServiceError::TransportClosed)));
        assert!(!is_retryable(&mcp_error(ErrorCode::INVALID_PARAMS)));
        assert!(!is_retryable(&Error::InvalidArguments));
    }

    #[tokio::test]
    async fn instructions_skips_unreachable_servers() {
        // Advisory metadata must never fail a caller; an unreachable server is
        // simply absent from the list.
        let executor = McpExecutor::new(vec![config(None, None)]);
        assert!(executor.instructions().await.is_empty());
    }

    // ── phase 1c: error classes survive the bridge ──────────────────────

    #[test]
    fn unknown_tool_code_is_unknown_so_sets_fall_through() {
        // `-32602` is how a 2026-07-28 server reports an unknown tool; a
        // composite must be able to route past a stale manifest.
        let err = classify_error("read", mcp_error(ErrorCode::INVALID_PARAMS));
        assert!(matches!(err, ToolError::UnknownTool { .. }), "got {err:?}");
    }

    #[test]
    fn missing_method_makes_the_server_unavailable() {
        let err = classify_error("read", mcp_error(ErrorCode::METHOD_NOT_FOUND));
        assert!(matches!(err, ToolError::Unavailable { .. }), "got {err:?}");
    }

    #[test]
    fn unsupported_version_makes_the_server_unavailable() {
        // Another server may speak the version we negotiated, so this is a
        // per-peer miss rather than a dead end.
        let err = classify_error("read", mcp_error(ErrorCode::UNSUPPORTED_PROTOCOL_VERSION));
        assert!(matches!(err, ToolError::Unavailable { .. }), "got {err:?}");
    }

    #[test]
    fn unsolicited_task_handle_is_a_transport_failure_naming_the_cause() {
        // SEP-2663: the SDK reports an undriven task as `UnexpectedResponse`;
        // the class is `Transport`, and the message must name the cause so the
        // failure is diagnosable rather than opaque.
        let err = classify_error("read", Error::Service(ServiceError::UnexpectedResponse));
        let ToolError::Transport(message) = &err else {
            panic!("expected Transport, got {err:?}");
        };
        assert!(
            message.contains("task"),
            "message must name the cause: {message}"
        );
    }

    #[test]
    fn missing_client_capability_is_our_fault_not_a_fall_through() {
        // No amount of routing supplies a capability this bridge refuses to
        // declare — the failure must surface, not be absorbed by the next
        // executor in a set.
        let err = classify_error(
            "read",
            mcp_error(ErrorCode::MISSING_REQUIRED_CLIENT_CAPABILITY),
        );
        assert!(matches!(err, ToolError::Transport(_)), "got {err:?}");
    }
}
