//! Live MCP stdio evidence — discover → `tools/list` → `tools/call` → render,
//! the server's `instructions` surviving the handshake, and the two behaviours
//! a live server is the only way to prove: a mid-call input round (SEP-2322
//! MRTR) answered by the host, and an abandoned call telling the server to
//! stop.
//!
//! Every other test in this crate proves what the bridge does with hand-built
//! values or a dead transport; none of them has ever spoken MCP.
//!
//! `harness = false` gives this binary two roles. Spawned by `cargo test` with
//! no arguments it is the **client**; spawned by itself with an argv marker it
//! serves MCP on stdin/stdout. The role rides argv rather than an environment
//! variable because `std::env::set_var` is `unsafe` under the workspace's
//! `unsafe_code = "forbid"`. (Per-server *child* env is supported and is what
//! `env_reaches_the_child` exercises — it just cannot select this role, because
//! the parent process decides that before any child exists.)

use std::collections::HashMap;
use std::future::Future;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use mcp::client::{Client, ClientOptions, InputHandler};
use mcp::{McpExecutor, McpServerConfig, McpTransport};
use morphium::tool::{Arguments, CallId, ToolExecutor, ToolName, ToolReply, ToolRequest};
use rmcp::model::{
    CallToolRequestParams, CallToolResponse, CallToolResult, ContentBlock, ErrorCode,
    InputRequests, InputRequiredResult, JsonObject, ListToolsResult, PaginatedRequestParams,
    ProtocolVersion, ServerCapabilities, ServerConfig, Tool,
};
use rmcp::service::RequestContext;
use rmcp::{ErrorData as McpError, RoleServer, ServerHandler, ServiceExt};
use serde_json::{Value, json};

/// The argv marker that turns this binary into the server.
const SERVER_FLAG: &str = "--mcp-test-server";

/// A minimal MCP server: `echo`, an `ask` that needs input mid-call, and a
/// `wait_for_cancel` that only ever ends by cancellation.
#[derive(Clone)]
struct TestServer {
    /// Where `wait_for_cancel` records that it was cancelled; `None` when the
    /// client passed no path.
    marker: Option<PathBuf>,
}

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
        let schema = || -> JsonObject {
            serde_json::from_value(json!({
                "type": "object",
                "properties": { "message": { "type": "string" } },
                "required": ["message"]
            }))
            .expect("valid schema")
        };
        Ok(ListToolsResult::with_all_items(vec![
            Tool::new("echo", "Echo the supplied message back", schema()),
            Tool::new("ask", "Greet whoever the host names", schema()),
            Tool::new(
                "wait_for_cancel",
                "Never returns; ends only when cancelled",
                schema(),
            ),
            Tool::new("env", "Report the injected MORPHIUM_TEST_TOKEN", schema()),
            Tool::new("cwd", "Report the process working directory", schema()),
        ]))
    }

    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, McpError> {
        match request.name.as_ref() {
            "echo" => {
                let message = message(&request);
                let mut result = CallToolResult::success(vec![ContentBlock::text(message.clone())]);
                result.structured_content = Some(json!({ "echo": message }));
                Ok(result.into())
            }
            // SEP-2322: the first round asks; the retry carries the answers.
            "ask" => match answered_name(&request) {
                Some(name) => {
                    let mut result =
                        CallToolResult::success(vec![ContentBlock::text(format!("hello {name}"))]);
                    result.structured_content = Some(json!({ "name": name }));
                    Ok(result.into())
                }
                None => Ok(CallToolResponse::InputRequired(
                    InputRequiredResult::from_input_requests(ask_for_name()),
                )),
            },
            // Proves the per-server environment was injected, not inherited
            // from the parent's own environment.
            "env" => {
                let token = std::env::var("MORPHIUM_TEST_TOKEN").unwrap_or_default();
                let mut result = CallToolResult::success(vec![ContentBlock::text(token.clone())]);
                result.structured_content = Some(json!({ "token": token }));
                Ok(result.into())
            }
            // Proves the configured working directory reached the child.
            "cwd" => {
                let dir = std::env::current_dir()
                    .map(|p| p.to_string_lossy().into_owned())
                    .unwrap_or_default();
                let mut result = CallToolResult::success(vec![ContentBlock::text(dir.clone())]);
                result.structured_content = Some(json!({ "cwd": dir }));
                Ok(result.into())
            }
            // Holds the round open until the peer cancels it. Returning on its
            // own would mean the cancellation never arrived, so the marker
            // file is written only on the cancelled path.
            "wait_for_cancel" => {
                context.ct.cancelled().await;
                if let Some(path) = &self.marker {
                    let _ = std::fs::write(path, "cancelled");
                }
                Err(McpError::new(ErrorCode::INTERNAL_ERROR, "cancelled", None))
            }
            other => Err(McpError::new(
                ErrorCode::METHOD_NOT_FOUND,
                format!("no tool named `{other}`"),
                None,
            )),
        }
    }
}

/// The `message` argument, or the empty string.
fn message(request: &CallToolRequestParams) -> String {
    request
        .arguments
        .as_ref()
        .and_then(|args| args.get("message"))
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned()
}

/// The name the host answered with, when this round carries one.
fn answered_name(request: &CallToolRequestParams) -> Option<String> {
    request
        .input_responses
        .as_ref()?
        .get("answer")?
        .get("content")?
        .get("name")?
        .as_str()
        .map(str::to_owned)
}

/// One elicitation request, built from JSON so the test does not depend on the
/// SDK's Rust shapes.
fn ask_for_name() -> InputRequests {
    serde_json::from_value(json!({
        "answer": {
            "method": "elicitation/create",
            "params": {
                "message": "What is your name?",
                "requestedSchema": {
                    "type": "object",
                    "properties": { "name": { "type": "string" } }
                }
            }
        }
    }))
    .expect("a valid elicitation request")
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    if std::env::args().any(|arg| arg == SERVER_FLAG) {
        let server = TestServer {
            marker: std::env::args().nth(2).map(PathBuf::from),
        }
        .serve(rmcp::transport::stdio())
        .await
        .expect("serve MCP over stdio");
        server.waiting().await.expect("run until stdin closes");
        return;
    }
    round_trip().await.expect("live stdio round trip");
    executor_round_trip()
        .await
        .expect("live stdio through the kernel seam");
    env_reaches_the_child()
        .await
        .expect("per-server env injection");
    cwd_is_applied().await.expect("per-server cwd");
    answered_input_round().await.expect("mid-call input round");
    unanswered_input_is_reported()
        .await
        .expect("unanswered input");
    abandoned_call_is_cancelled().await.expect("cancellation");
    println!("live stdio evidence: ok");
}

/// The transport that spawns this binary as the server, inheriting the
/// parent's environment and working directory.
fn server_transport(extra: &[String]) -> std::io::Result<McpTransport> {
    server_transport_with(extra, HashMap::new(), None)
}

/// The transport that spawns this binary as the server with a per-server
/// environment and working directory.
fn server_transport_with(
    extra: &[String],
    env: HashMap<String, String>,
    cwd: Option<String>,
) -> std::io::Result<McpTransport> {
    let mut args = vec![SERVER_FLAG.to_owned()];
    args.extend_from_slice(extra);
    Ok(McpTransport::Stdio {
        command: std::env::current_exe()?.to_string_lossy().into_owned(),
        args,
        env,
        cwd,
    })
}

/// The host side of the input seam: accepts and names Ferris.
fn namer() -> InputHandler {
    Arc::new(
        |asked: Value| -> Pin<Box<dyn Future<Output = Option<Value>> + Send>> {
            Box::pin(async move {
                // The bridge hands the server's request over verbatim.
                assert_eq!(asked["answer"]["method"], json!("elicitation/create"));
                Some(json!({
                    "answer": { "action": "accept", "content": { "name": "Ferris" } }
                }))
            })
        },
    )
}

/// discover → `tools/list` → `tools/call` → render, over real pipes.
async fn round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::connect(&server_transport(&[])?).await?;

    // The stateless 2026-07-28 revision was negotiated, so `Auto` did not fall
    // back to the legacy `initialize` handshake.
    assert_eq!(
        client.protocol_version().as_ref(),
        Some(&ProtocolVersion::V_2026_07_28),
        "Auto must select the modern revision when the server supports it"
    );

    // Server-wide guidance survives the handshake and is surfaced verbatim for
    // the host to place in the model's context.
    assert_eq!(
        client.instructions().as_deref(),
        Some("Call `echo` to echo a message.")
    );

    let tools = client.list_tools().await?;
    let echo = tools
        .iter()
        .find(|tool| tool.name.as_str() == "echo")
        .expect("echo is advertised");
    assert_eq!(echo.input_schema["required"], json!(["message"]));

    let result = client.call("echo", json!({ "message": "hello" })).await?;
    // The two output channels the bridge keeps distinct: text for the model,
    // structured content for the host.
    assert_eq!(result.text().as_deref(), Some("hello"));
    assert!(!result.is_error());
    assert_eq!(result.raw["structuredContent"], json!({ "echo": "hello" }));
    assert_eq!(result.raw["content"][0]["type"], json!("text"));
    // A 2026-07-28 server MUST send `resultType`; a client treats absence as
    // "complete", so seeing it on the wire is the evidence it was present.
    assert_eq!(result.raw["resultType"], json!("complete"));
    Ok(())
}

/// The same live server, reached through the kernel's `ToolExecutor` seam:
/// discovery is explicit, `tools()` reads the snapshot, and `run()` maps the
/// MCP envelope into a bounded kernel result.
async fn executor_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let executor = McpExecutor::connect(vec![McpServerConfig {
        name: "test".into(),
        transport: server_transport(&[])?,
        allowed_tools: None,
        disabled_tools: None,
        replay_safety: morphium::effect::ReplaySafety::Unsafe,
        replay_safety_by_tool: None,
        replay_limit: 1,
        prefix: None,
        tool_timeout_ms: None,
        startup_timeout_ms: None,
    }])
    .await?;

    let tools = executor.tools();
    assert!(
        tools
            .iter()
            .any(|tool| tool.name.as_str() == "mcp__test__echo"),
        "refresh must populate the synchronous manifest: {tools:?}"
    );

    let request = ToolRequest::new(
        CallId::parse("call-1").expect("literal call id"),
        ToolName::parse("mcp__test__echo").expect("literal tool name"),
        Arguments::parse(json!({ "message": "hello" }).to_string()).expect("literal arguments"),
    );
    let invocation =
        morphium::effect::EffectInvocation::first(executor.effect_id(&request), request);
    let reply = executor.run(invocation).await?;
    let ToolReply::Output { output, is_error } = reply else {
        panic!("MCP never declares a graph");
    };
    assert!(!is_error);
    assert_eq!(output, morphium::tool::ToolOutput::Inline("hello".into()));
    Ok(())
}

/// A per-server environment variable is injected into the child, so a
/// token-based server can be launched without touching the parent's own env.
async fn env_reaches_the_child() -> Result<(), Box<dyn std::error::Error>> {
    let mut env = HashMap::new();
    env.insert("MORPHIUM_TEST_TOKEN".to_owned(), "token-123".to_owned());
    let client = Client::connect(&server_transport_with(&[], env, None)?).await?;
    let result = client.call("env", json!({})).await?;
    assert_eq!(result.text().as_deref(), Some("token-123"));
    Ok(())
}

/// The configured working directory is applied to the child.
async fn cwd_is_applied() -> Result<(), Box<dyn std::error::Error>> {
    let here = std::env::current_dir()?;
    let client = Client::connect(&server_transport_with(
        &[],
        HashMap::new(),
        Some(here.to_string_lossy().into_owned()),
    )?)
    .await?;
    let reported = client.call("cwd", json!({})).await?;
    let reported = std::fs::canonicalize(reported.text().unwrap_or_default())?;
    assert_eq!(reported, std::fs::canonicalize(&here)?);
    Ok(())
}

/// A server that needs input mid-call is answered by the installed handler,
/// and the call completes — no new parking state in the engine.
async fn answered_input_round() -> Result<(), Box<dyn std::error::Error>> {
    let options = ClientOptions {
        input: Some(namer()),
    };
    let client = Client::connect_with(&server_transport(&[])?, options).await?;

    // The handler ran: the server saw the answer and finished the call.
    let result = client.call("ask", json!({ "message": "unused" })).await?;
    assert_eq!(result.text().as_deref(), Some("hello Ferris"));
    assert_eq!(result.raw["structuredContent"], json!({ "name": "Ferris" }));
    Ok(())
}

/// With no handler installed the capability is not declared, and a server that
/// asks anyway gets an error naming the gap — never a silent decline.
async fn unanswered_input_is_reported() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::connect(&server_transport(&[])?).await?;
    let err = client
        .call("ask", json!({ "message": "unused" }))
        .await
        .expect_err("a request with nobody to answer it must fail");
    assert!(
        err.to_string().contains("input handler"),
        "the error must name the missing handler: {err}"
    );
    Ok(())
}

/// Abandoning the call future tells the server to stop: the server only ever
/// writes its marker after the cancellation arrives.
async fn abandoned_call_is_cancelled() -> Result<(), Box<dyn std::error::Error>> {
    let marker = std::env::temp_dir().join(format!("mcp-cancel-{}.txt", std::process::id()));
    let _ = std::fs::remove_file(&marker);
    let transport = server_transport(&[marker.to_string_lossy().into_owned()])?;
    let client = Client::connect(&transport).await?;

    let abandoned = tokio::time::timeout(
        Duration::from_millis(300),
        client.call("wait_for_cancel", json!({ "message": "go" })),
    )
    .await;
    assert!(abandoned.is_err(), "the round must not answer on its own");

    let mut observed = None;
    for _ in 0..40 {
        if let Ok(text) = std::fs::read_to_string(&marker) {
            observed = Some(text);
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    let _ = std::fs::remove_file(&marker);
    assert_eq!(
        observed.as_deref(),
        Some("cancelled"),
        "the server must observe the cancellation when the caller abandons"
    );
    Ok(())
}
