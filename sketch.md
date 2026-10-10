# Morphium

Morphium is an agentic harness built on a functional, typed state machine.
The state machine owns the agent loop — reasoning, tool use, approval, and
resume; the harness supplies the model, tools, policy, and persistence.

It is not a provider SDK, not a database, and not an application. Hosts supply
the model, the tools, and the transport. Morphium owns the execution rules.

## The shape

```text
morphium/
├── Cargo.toml                  # feature: testing
├── src/
│   ├── lib.rs                  # public API: Morphium, start, resume, events
│   │
│   ├── kernel/                 # the loop and its contracts; may not `use crate::harness`
│   │   ├── mod.rs
│   │   ├── ports.rs            # ModelClient, ContextBuilder, ToolRunner, Policy,
│   │   │                       #   EventSink, Journal
│   │   ├── types.rs            # Message, Reply, Usage, ToolCall, ToolSpec,
│   │   │                       #   ToolResult, Submitted, Failure
│   │   ├── state.rs            # State: serde, schema-versioned
│   │   │                       #   durable history + pending work
│   │   │                       #   derived status/pending rebuilt on load
│   │   │                       #   reserved: ext, active_agent, branch_id
│   │   ├── signal.rs           # ToolResult | Approval | UserMessage | Cancel | Timeout
│   │   ├── event.rs            # public taxonomy; serde; durable vs transient
│   │   └── advance.rs          # advance(state, &ports) -> Outcome
│   │                           # resume(state, signal, &ports) -> Outcome
│   │
│   ├── harness/                # concrete implementations of the ports
│   │   ├── mod.rs
│   │   ├── context.rs          # default ContextBuilder: system prompt + windowed history
│   │   ├── model/
│   │   │   ├── mod.rs
│   │   │   └── retry.rs        # model-only retry policy and backoff
│   │   ├── policy.rs           # default Policy: rules; Allow / Deny / Ask
│   │   ├── limits.rs           # iteration/call/token/time budgets; Bounding
│   │   ├── events.rs           # EventSink impls: in-memory, callback, jsonl
│   │   ├── session/
│   │   │   ├── mod.rs
│   │   │   ├── store.rs        # Journal implementation helpers / session record
│   │   │   ├── memory.rs       # in-memory append-only backend
│   │   │   └── manager.rs      # load -> advance -> append-before-effect
│   │   └── tools/
│   │       ├── mod.rs
│   │       ├── registry.rs     # specs, lookup, per-session tool set
│   │       ├── envelope.rs     # output bounding + dispatch wire envelope
│   │       ├── local.rs        # caller-supplied tool -> Submitted::Done
│   │       └── dispatched.rs   # emits request over a transport -> Submitted::Pending
│   │
│   ├── run.rs                  # composition root: the only file naming concrete harness types
│   └── testing.rs              # #[cfg(feature = "testing")]: ScriptedModel, FakeTools,
│                               #   AllowAll, EventCollector, MemoryStore, contract suites
│
├── tests/
│   ├── loop_with_fakes.rs      # no network, no container
│   ├── state_roundtrip.rs      # from_slice(to_vec(state)) == state
│   ├── resume.rs               # approval / tool result / cancel / timeout
│   └── contracts.rs            # same suite run against every adapter
│
└── examples/
    └── minimal.rs
```

This crate contains no model-provider adapter and no database backend. No
`anthropic.rs`, no `sqlite.rs`. Hosts bring those behind the ports. The `llm/`
and `mcp/` directories are not part of this crate and are not workspace members.

## Vocabulary

- **State** — the session's current position: durable history plus derived
  status and pending work.
- **Event** — one durable fact about what happened.
- **Signal** — something from outside that resumes a parked session: a tool
  result, an approval decision, a user message, cancel, or timeout.
- **Reply** — one complete assistant response. A reply either finishes the turn
  or carries tool calls.
- **ToolCall** — the model's request to use a tool.
- **ToolSpec** — the tool manifest shown to the model.
- **ToolResult** — the settled outcome of a tool call.
- **Submitted** — the immediate result of handing a call to a tool runner:
  either `Done` or `Pending`.
- **Outcome** — what `advance` or `resume` produced: continue, parked, finished,
  stopped, or failed.

Do not reintroduce the old names: tape, derived, effect invocation, assembly,
environment, driver.

## The loop

`advance` performs one step. It is not an unbounded loop; the session manager
repeats it while the outcome is `Continue`. This keeps limits, cancellation,
and persistence at a clear boundary.

A step does one of two things:

1. Ask the model for a reply.
2. Execute the batch of tool calls that reply requested.

The loop stops when it produces a final reply, parks for approval, parks for a
dispatched tool result, is stopped, or fails.

`resume` applies one signal and then advances. A parked session has no work
running; it is waiting for exactly one of the signals above.

## State and events

The event history is the truth. `State` is a snapshot plus caches:

- durable: schema version, session identity, instructions, event history,
  reserved extension fields;
- derived and rebuilt on load: status, pending calls, pending approvals,
  pending submissions, resolved calls, succeeded tools, usage, iteration count.

Never serialize a derived value as if it were authoritative. A loaded state
must be recomputable from its history, and a state round-trip must preserve the
rebuilt result.

Every event that matters for resume is durable and serializable. Streaming
deltas, UI frames, and debug traces are transient and are not resume facts.

The public event taxonomy should be small and explicit:

- session started;
- user input;
- assistant reply;
- tool call started;
- tool result;
- tool pending;
- tool failed;
- tool rejected;
- approval requested;
- approval decided;
- stopped;
- failed.

An event is appended before the effect it authorizes. A tool never runs unless
the fact that it started is already durable.

## Statuses and outcomes

Status is one of:

- `Running`
- `PendingApproval`
- `PendingSubmission`
- `Finished`
- `Stopped`
- `Failed`

`Finished` means the last reply had no tool calls. `Stopped` is a clean pause;
a later user message may reopen it. `Failed` is terminal and absorbing.

`Outcome` mirrors those stops: `Continue`, `Parked`, `Finished`, `Stopped`,
`Failed`. Do not add a second status field that can disagree with the event
history.

## Ports

There are six ports. They are the whole boundary between the kernel and the
world.

- **ModelClient** — takes the context and tool specs, returns one complete
  `Reply`. Streaming belongs outside the kernel.
- **ContextBuilder** — turns state into `Context`: system prompt plus
  model-visible history.
- **ToolRunner** — advertises specs and runs one call, returning `Submitted`.
- **Policy** — classifies a batch of calls as allow, deny, or ask.
- **EventSink** — observes committed events. Observation is not durability.
- **Journal** — loads and appends the session's event history against an
  expected position.

The kernel may not import the harness. The harness implements the ports. The
composition root names the concrete implementations; nothing else does.

`EventSink` is deliberately separate from `Journal`. An event can be observed
without being durable; a journal append is fallible and must finish before the
next effect.

## Tools and approval

A tool call carries its id, name, arguments, and the assistant's call order.
The id is the correlation key. The call order is preserved even when a batch
finishes out of order.

A tool runner either completes immediately (`Done`) or accepts work for later
(`Pending`). A pending call parks the session; its result arrives as a signal
and is correlated by call id. A call is never dispatched twice under the same
identity.

Policy is batch-shaped:

- allow;
- deny with a reason;
- ask.

If any call in the batch asks, all non-denied calls park together. Denials are
recorded immediately as rejections; a rejection is not a failed execution and
must never run. An approved call executes without asking again. A denied call
is never executed later.

Tool errors are recorded as failures. They are not policy rejections. Tool
calls are never retried automatically; retrying a side effect is a host
decision.

Tool output is bounded at settlement. A result is either truncated or refused;
it is never silently written unbounded.

## Context

The default context is the system prompt plus a window of model-visible
history. The projection must preserve the pairing between an assistant tool
call and its result, and must preserve call order.

When a window trims history:

- drop messages from the front;
- drop leading orphaned tool results;
- never leave a tool result without its call, or a call without its result
  inside the window;
- keep the system prompt separate from the message history.

How much history to include is configuration. The kernel owns which facts are
model-visible, not how many.

## Limits and retry

The harness owns budgets and the clock:

- maximum iterations;
- maximum tool calls;
- input/output token budget;
- wall-clock deadline;
- per-tool timeout;
- maximum parallel tools;
- maximum output size.

A budget breach becomes a timeout signal or a terminal `Failed` fact; it must
not hang or silently stop.

Model retries are for transient model failures only, with bounded exponential
backoff. Tool calls are never retried by this layer. Retry policy is a pure
value so it can be tested without sleeping.

## Persistence and resume

The session store is an append-only event log with an expected head. An append
either lands whole at the expected position or returns a conflict; it never
silently overwrites or partially writes.

Idempotency is scoped, not global: a call id is unique within its session and
turn, or carries a stable identity minted at dispatch. A bare call id reused in
a later turn must not be mistaken for an already-settled call.

The manager loads events, rebuilds state, advances, and appends before effects.
A crash between an effect and its settlement leaves a recorded start with no
result; resume must treat that as pending or unknown, never as success.

This crate ships only the in-memory backend. Durable backends implement the
same journal contract elsewhere.

## Invariants

These are the rules that keep the loop honest:

- a reply is assistant-only and has unique call ids;
- a tool result requires a started call;
- a call settles at most once;
- a call cannot be rejected after it started;
- a new reply is not allowed while calls are owed or approval is pending;
- approval is only for calls that are actually pending;
- approval decisions name only pending calls, with no duplicates;
- a user message reopens a finished or stopped session, never a failed one;
- no event follows a terminal failure;
- a failed tool call and a policy rejection remain distinct;
- derived status never disagrees with the recorded history.

## Testing

No network and no container in the default suite.

- `loop_with_fakes`: scripted model plus fake tools; text replies, tool batches,
  approval, dispatched pending results, failure paths.
- `state_roundtrip`: serialize, deserialize, rebuild derived state, compare;
  unknown schema versions are refused.
- `resume`: each signal type resumes or refuses correctly.
- `contracts`: the same suite runs against every adapter implementation.

The `testing` feature provides `ScriptedModel`, `FakeTools`, `AllowAll`,
`EventCollector`, and an in-memory journal. Contract suites live beside the
fakes so adapters in other crates can reuse them.

## Non-goals

Do not add these to the first version:

- graph or scope execution;
- artifact storage and artifact-resolving ports;
- concrete model providers or database backends;
- streaming inside the kernel;
- automatic tool retries;
- a seven-service environment or a nine-generic drive function;
- a public tape validator as the main state type;
- tool executor combinators;
- full effect reconciliation machinery.

Those can return later, behind a real use case. The first version earns its
complexity by making one resumable loop correct.
