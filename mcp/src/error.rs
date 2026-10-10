use thiserror::Error;

/// Errors that can occur during MCP client operations.
#[allow(clippy::large_enum_variant)] // Error path: one value per failed connection.
#[derive(Debug, Error)]
pub enum Error {
    /// The client failed to initialize (transport handshake failed).
    #[error("client initialization failed: {0}")]
    Initialize(#[from] rmcp::service::ClientInitializeError),

    /// The MCP service returned an error.
    #[error("service error: {0}")]
    Service(#[from] rmcp::service::ServiceError),

    /// JSON serialization or deserialization failed.
    #[error("JSON error: {0}")]
    Serde(#[from] serde_json::Error),

    /// Spawning or driving a stdio child process failed.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// A server advertised a tool the bridge cannot expose as a valid name.
    ///
    /// Raised at route discovery, where the agent-visible name is admitted
    /// into a [`ToolName`](morphium::tool::ToolName); the descriptor is
    /// reported rather than silently dropped.
    #[error("invalid tool name advertised by an MCP server: {0}")]
    InvalidToolName(#[from] morphium::tool::ToolValueError),

    /// Arguments must be a JSON object.
    #[error("arguments must be a JSON object")]
    InvalidArguments,

    /// The server answered `tools/call` with a variant this bridge does not
    /// drive — the SEP-2663 task lifecycle is a documented non-goal.
    #[error("unsupported tools/call response variant (SEP-2663 tasks are not supported)")]
    UnsupportedResponse,

    /// The server kept asking for input past the bridge's round cap.
    #[error("server did not finish the call within {max_rounds} input rounds")]
    InputRoundsExceeded {
        /// The cap that was exceeded.
        max_rounds: usize,
    },

    /// The server requested input but no handler is installed to answer it.
    #[error("server requested input but this bridge has no input handler installed")]
    InputHandlerMissing,

    /// The installed handler declined to answer the server's request.
    #[error("the input handler declined to answer the server's request")]
    InputDeclined,

    /// Connecting to a server, or listing its tools, exceeded the startup
    /// bound. The peer may still be starting; a composite routes the call on.
    #[error("MCP server `{server}` did not respond within {ms}ms")]
    StartupTimeout {
        /// The server that did not answer in time.
        server: String,
        /// The bound that was exceeded, in milliseconds.
        ms: u64,
    },
}
