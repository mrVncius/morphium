# `mcp` — MCP client bridge

`McpExecutor` implements [`morphium::tool::ToolExecutor`] for one or more MCP
servers reached over **stdio** or **streamable HTTP**. It is an adapter: it
implements the kernel's seam and never adds a core type or re-implements a core
rule. Discovery is explicit because the kernel's `tools()` is synchronous — call
`refresh()` (or `McpExecutor::connect`) before a turn, then read the snapshot.

## Beta behavior

| Area | Behavior |
|---|---|
| Protocol | Prefers stateless `2026-07-28` via `server/discover`; falls back to the legacy `initialize` handshake. |
| Names | `mcp__<server>__<tool>`; sanitized to `[A-Za-z0-9_]`, capped at 64 chars; collisions/truncation get a stable 8-hex FNV suffix. |
| Approval | Untrusted by default: `Ask` unless the server proves read-only. Annotations can only raise caution, never lower it. |
| Timeouts | Per-server startup bound (default 10s) and per-tool bound (default 60s). |
| Retries | Connect/`tools/list` only, HTTP only, two retries at 250ms and 1s. `tools/call` is **never** retried — it may already have run. |
| Instructions | `Client::instructions` / `McpExecutor::instructions`; placement is the host's call. |
| Schemas | `type: "object"` and `properties: {}` are injected only when missing; complete schemas pass through untouched. |
| Results | Text blocks joined for the model; otherwise structured JSON; otherwise `None`. |
| Errors | `-32602` → unknown tool, `-32601`/`-32022` → unavailable, `-32021` → transport failure (does not fall through). |
| Runtime | MRTR input rounds, cancellation, per-server `env`/`cwd`, allowlists and deny-lists. |

### Retry limitation (decided: ship transport-only)

A plain-text HTTP `5xx` — the common gateway/proxy failure — surfaces as a
transport failure and **is** retried end-to-end (`tests/live_http.rs`). A
`429`/`5xx` whose body is a JSON-RPC error is surfaced as an MCP error and is
**not** retried, because rmcp does not expose the HTTP status on its error types.

The beta decision is to accept this and document it rather than wrap the HTTP
transport or duplicate rmcp. Upstream follow-up: propose a status or
`is_retryable()` accessor in rmcp, then revisit after beta.

## Deliberate non-goals / deferred

- **OAuth** — post-beta; loopback browser callback with PKCE, using rmcp's
  `auth` feature (CIMD preferred over DCR). User credentials for stdio servers
  belong in the child's `env`.
- **Resources as core primitives** — permanent non-goal. If needed, expose
  `list_mcp_resources` / `read_mcp_resource` as bridge-level tools over the
  existing seam.
- **Caching hints** (`ttlMs`/`cacheScope`) and **`subscriptions/listen`** —
  post-beta.
- **Progress forwarding**, **prompts/completion**, **roots/sampling/logging** —
  not surfaced.
- **Tasks extension (SEP-2663)** — not driven; an unsolicited task handle fails
  loudly with a named cause.
- **Hosted-server smoke test** — not in the default suite. The real protocol is
  exercised deterministically on loopback; add an env-gated live target only if
  a specific hosted server becomes part of launch acceptance.

## Verification

```sh
cargo test -p mcp          # unit tests + live stdio + live HTTP
cargo clippy -p mcp --all-targets
cargo doc -p mcp --no-deps
```

`tests/live_stdio.rs` proves discover → list → call → render, the `instructions`
round trip, an MRTR round, cancellation, and child `env`/`cwd` over real pipes.
`tests/live_http.rs` proves the stateless flow, custom headers, retry of a
plain-text `500`, and the startup timeout over real loopback HTTP.

## Host integration

No crate in this workspace consumes `McpExecutor` yet; host work is a separate
workstream, not part of the bridge beta. The first consumer must:

1. Load user configuration into `McpServerConfig` (including `env`, `cwd`,
   `tool_timeout_ms`, `startup_timeout_ms`, allow/deny lists).
2. Compose the executor with local tools (`Add` / `Namespace`) and decide
   routing precedence.
3. Render `Ask` decisions — most MCP tools require confirmation, so a host that
   cannot render them will park every call.
4. Place `McpExecutor::instructions()` in the model context (namespace
   description or system prompt).
