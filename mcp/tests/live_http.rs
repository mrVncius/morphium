//! Live MCP streamable-HTTP evidence — the counterpart to `live_stdio.rs`.
//!
//! The retry classifier, the startup bound, custom headers, and the stateless
//! `2026-07-28` flow were previously unit-tested in isolation but had never
//! crossed a real HTTP connection. This target serves the real protocol on
//! loopback with rmcp's server side and exercises the bridge as a client:
//!
//! 1. a `tools/list` that answers `500` once is retried and the call lands;
//! 2. `server/discover` wins over the legacy `initialize` handshake, and
//!    `instructions` survives the round trip;
//! 3. a server that accepts the connection but never answers trips
//!    `startup_timeout_ms` and surfaces `StartupTimeout`;
//! 4. the configured static header rides every request, including the call.
//!
//! The server runs in-process; no child process or argv role selector is
//! needed. The failure is injected by an axum middleware in front of rmcp's
//! service, because `ServerHandler` can only answer in-protocol (a JSON-RPC
//! error) — the bridge retries transport failures, not server errors.

use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use axum::Router;
use axum::body::{Body, to_bytes};
use axum::extract::{Request, State};
use axum::http::{StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::Response;
use mcp::client::Client;
use mcp::error::Error;
use mcp::{McpExecutor, McpServerConfig, McpTransport};
use morphium::effect::{EffectInvocation, ReplaySafety};
use morphium::tool::{
    Arguments, CallId, ToolExecutor, ToolName, ToolOutput, ToolReply, ToolRequest,
};
use rmcp::model::{
    CallToolRequestParams, CallToolResponse, CallToolResult, ContentBlock, JsonObject,
    ListToolsResult, PaginatedRequestParams, ProtocolVersion, ServerCapabilities, ServerConfig,
    Tool,
};
use rmcp::service::RequestContext;
use rmcp::transport::streamable_http_server::{
    StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
};
use rmcp::{ErrorData as McpError, RoleServer, ServerHandler};
use serde_json::{Value, json};

/// What the middleware saw, plus how many `tools/list` requests to fail.
#[derive(Clone, Default)]
struct Probe {
    failures_left: Arc<AtomicUsize>,
    methods: Arc<Mutex<Vec<String>>>,
    header_values: Arc<Mutex<Vec<String>>>,
}

/// The one tool the HTTP server advertises.
#[derive(Clone)]
struct TestServer;

impl ServerHandler for TestServer {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_instructions("Call `echo` to echo a message.")
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, McpError> {
        let schema: JsonObject = serde_json::from_value(json!({
            "type": "object",
            "properties": { "message": { "type": "string" } },
            "required": ["message"]
        }))
        .expect("valid schema");
        Ok(ListToolsResult::with_all_items(vec![Tool::new(
            "echo",
            "Echo the supplied message back",
            schema,
        )]))
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, McpError> {
        let message = request
            .arguments
            .as_ref()
            .and_then(|args| args.get("message"))
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned();
        let mut result = CallToolResult::success(vec![ContentBlock::text(message.clone())]);
        result.structured_content = Some(json!({ "echo": message }));
        Ok(result.into())
    }
}

/// Record the request, fail the first `tools/list` with a transport-class
/// response, and otherwise forward the untouched body to rmcp.
async fn intercept(State(probe): State<Probe>, request: Request, next: Next) -> Response {
    if let Some(value) = request
        .headers()
        .get("x-morphium-test")
        .and_then(|value| value.to_str().ok())
    {
        probe
            .header_values
            .lock()
            .expect("probe mutex")
            .push(value.to_owned());
    }

    let (parts, body) = request.into_parts();
    let bytes = to_bytes(body, 1 << 20)
        .await
        .expect("the client sends a bounded request body");
    let method = serde_json::from_slice::<Value>(&bytes)
        .ok()
        .and_then(|body| body.get("method")?.as_str().map(str::to_owned));
    if let Some(method) = &method {
        probe
            .methods
            .lock()
            .expect("probe mutex")
            .push(method.clone());
    }

    let fails_now = method.as_deref() == Some("tools/list")
        && probe
            .failures_left
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |left| {
                left.checked_sub(1)
            })
            .is_ok();
    if fails_now {
        return Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(header::CONTENT_TYPE, "text/plain")
            .body(Body::from("transient HTTP failure"))
            .expect("build the injected failure");
    }

    next.run(Request::from_parts(parts, Body::from(bytes)))
        .await
}

/// Serve the real MCP protocol on a loopback port and return its URL.
async fn spawn(probe: Probe) -> String {
    let config = StreamableHttpServerConfig::default()
        .with_legacy_session_mode(false)
        .with_json_response(true)
        .with_sse_keep_alive(None);
    let service: StreamableHttpService<TestServer, LocalSessionManager> =
        StreamableHttpService::new(|| Ok(TestServer), Default::default(), config);
    let router = Router::new()
        .nest_service("/mcp", service)
        .layer(middleware::from_fn_with_state(probe, intercept));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind a loopback listener");
    let address = listener.local_addr().expect("listener address");
    tokio::spawn(async move {
        let _ = axum::serve(listener, router).await;
    });
    format!("http://{address}/mcp")
}

fn config(
    url: String,
    headers: HashMap<String, String>,
    startup_timeout_ms: Option<u64>,
) -> McpServerConfig {
    McpServerConfig {
        name: "remote".into(),
        transport: McpTransport::Http { url, headers },
        allowed_tools: None,
        disabled_tools: None,
        replay_safety: ReplaySafety::Unsafe,
        replay_safety_by_tool: None,
        replay_limit: 1,
        prefix: None,
        tool_timeout_ms: None,
        startup_timeout_ms,
    }
}

/// One caller-side invocation of the exposed name.
fn invoke(executor: &McpExecutor, name: &str, arguments: Value) -> EffectInvocation {
    let request = ToolRequest::new(
        CallId::parse(format!("c-{name}")).expect("literal call id"),
        ToolName::parse(name).expect("literal tool name"),
        Arguments::parse(arguments.to_string()).expect("literal arguments"),
    );
    EffectInvocation::first(executor.effect_id(&request), request)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn transient_http_failure_is_retried_and_headers_ride_every_request() {
    let probe = Probe::default();
    probe.failures_left.store(1, Ordering::SeqCst);
    let url = spawn(probe.clone()).await;

    let mut headers = HashMap::new();
    headers.insert("x-morphium-test".into(), "expected-value".into());
    let executor = McpExecutor::connect(vec![config(url, headers, None)])
        .await
        .expect("the injected 500 is retried and the second list succeeds");

    let names: Vec<String> = executor
        .tools()
        .into_iter()
        .map(|tool| tool.name.into_string())
        .collect();
    assert_eq!(names, vec!["mcp__remote__echo"]);

    // A call is a separate HTTP request, and must never be retried; reaching
    // the tool at all proves the session outlived the failed list.
    let reply = executor
        .run(invoke(
            &executor,
            "mcp__remote__echo",
            json!({ "message": "pong" }),
        ))
        .await
        .expect("the tool call succeeds");
    let ToolReply::Output { output, is_error } = reply else {
        panic!("a tool call must produce an output, not a graph");
    };
    assert_eq!(output, ToolOutput::Inline("pong".into()));
    assert!(!is_error);

    let methods = probe.methods.lock().expect("probe mutex").clone();
    assert!(
        methods.iter().any(|method| method == "server/discover"),
        "the stateless revision must be discovered, not initialized: {methods:?}"
    );
    assert!(
        !methods.iter().any(|method| method == "initialize"),
        "Auto must not fall back to the legacy handshake: {methods:?}"
    );
    assert_eq!(
        methods
            .iter()
            .filter(|method| method.as_str() == "tools/list")
            .count(),
        2,
        "exactly one failed list and one retry: {methods:?}"
    );

    let header_values = probe.header_values.lock().expect("probe mutex").clone();
    assert!(!header_values.is_empty(), "no request reached the server");
    assert!(
        header_values.iter().all(|value| value == "expected-value"),
        "every request must carry the configured static header: {header_values:?}"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn modern_http_discovery_surfaces_protocol_and_instructions() {
    let url = spawn(Probe::default()).await;
    let client = Client::connect(&McpTransport::Http {
        url,
        headers: HashMap::new(),
    })
    .await
    .expect("connect over streamable HTTP");

    assert_eq!(
        client.protocol_version().as_ref(),
        Some(&ProtocolVersion::V_2026_07_28),
        "Auto must select the modern revision over HTTP too"
    );
    assert_eq!(
        client.instructions().as_deref(),
        Some("Call `echo` to echo a message.")
    );
    let tools = client.list_tools().await.expect("list tools");
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].name, "echo");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_silent_server_trips_the_startup_timeout() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind a loopback listener");
    let address = listener.local_addr().expect("listener address");
    tokio::spawn(async move {
        // Accept and hold: the connection succeeds, the response never does.
        let mut held = Vec::new();
        while let Ok((socket, _)) = listener.accept().await {
            held.push(socket);
        }
    });

    let started = Instant::now();
    let error = match McpExecutor::connect(vec![config(
        format!("http://{address}/mcp"),
        HashMap::new(),
        Some(150),
    )])
    .await
    {
        Ok(_) => panic!("a server that never answers must not connect"),
        Err(error) => error,
    };
    assert!(
        matches!(error, Error::StartupTimeout { .. }),
        "expected StartupTimeout, got {error:?}"
    );
    assert!(
        started.elapsed() >= Duration::from_millis(140),
        "the bound must be the configured 150ms, not an immediate refusal"
    );
}
