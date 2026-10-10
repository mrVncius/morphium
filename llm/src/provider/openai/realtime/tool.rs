//! Function and MCP tools, tool choice, and MCP result types.
//!
//! Covers the `tools` / `tool_choice` configuration of the Realtime API plus
//! the MCP conversation items (`mcp_list_tools`, `mcp_call`,
//! `mcp_approval_request`, `mcp_approval_response`), the MCP error union, and
//! the `mcp_list_tools.*` event payloads.

use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A function tool the model may call.
///
/// Maps to the `"function"` member of [`RealtimeToolsConfigUnion`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeFunctionTool {
    /// The description of the function, including guidance on when and how to
    /// call it, and guidance about what to tell the user when calling
    /// (if anything).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The name of the function.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Parameters of the function in JSON Schema.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    /// The type of the tool, i.e. `function`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// A remote Model Context Protocol (MCP) server exposed to the model.
///
/// Maps to the `"mcp"` member of [`RealtimeToolsConfigUnion`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct McpTool {
    /// A label for this MCP server, used to identify it in tool calls.
    pub server_label: String,
    /// The type of the MCP tool. Always `mcp`.
    #[serde(rename = "type")]
    pub type_: String,
    /// The tool invocation context(s).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_callers: Option<Vec<AllowedCaller>>,
    /// List of allowed tool names or a filter object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_tools: Option<McpAllowedTools>,
    /// An OAuth access token that can be used with a remote MCP server, either
    /// with a custom MCP server URL or a service connector.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authorization: Option<String>,
    /// Identifier for a service connector, like those available in ChatGPT.
    ///
    /// One of `server_url`, `connector_id`, or `tunnel_id` must be provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<ConnectorId>,
    /// Whether this MCP tool is deferred and discovered via tool search.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defer_loading: Option<bool>,
    /// Optional HTTP headers to send to the MCP server. Use for authentication
    /// or other purposes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    /// Specify which of the MCP server's tools require approval.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_approval: Option<McpApproval>,
    /// A description of the MCP server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_description: Option<String>,
    /// The URL of the MCP server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_url: Option<String>,
    /// The identifier of a Secure MCP Tunnel to reach the server through.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tunnel_id: Option<String>,
}

/// The tool invocation context of an MCP tool.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum AllowedCaller {
    /// The tool may be invoked directly by the model.
    #[serde(rename = "direct")]
    Direct,
    /// The tool may be invoked programmatically.
    #[serde(rename = "programmatic")]
    Programmatic,
}

/// Identifier for a ChatGPT service connector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum ConnectorId {
    /// Dropbox.
    #[serde(rename = "connector_dropbox")]
    Dropbox,
    /// Gmail.
    #[serde(rename = "connector_gmail")]
    Gmail,
    /// Google Calendar.
    #[serde(rename = "connector_googlecalendar")]
    GoogleCalendar,
    /// Google Drive.
    #[serde(rename = "connector_googledrive")]
    GoogleDrive,
    /// Microsoft Teams.
    #[serde(rename = "connector_microsoftteams")]
    MicrosoftTeams,
    /// Outlook Calendar.
    #[serde(rename = "connector_outlookcalendar")]
    OutlookCalendar,
    /// Outlook Email.
    #[serde(rename = "connector_outlookemail")]
    OutlookEmail,
    /// SharePoint.
    #[serde(rename = "connector_sharepoint")]
    SharePoint,
}

/// The set of MCP tools the model is allowed to call.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(untagged)]
#[display("{self:?}")]
pub enum McpAllowedTools {
    /// A string array of allowed tool names.
    Names(Vec<String>),
    /// A filter object to specify which tools are allowed.
    Filter(McpToolFilter),
}

/// A filter object to specify which MCP tools are allowed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct McpToolFilter {
    /// Indicates whether or not a tool modifies data or is read-only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// List of allowed tool names.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_names: Option<Vec<String>>,
}

/// Specify which of the MCP server's tools require approval.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct McpToolApprovalFilter {
    /// Tools that always require approval.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub always: Option<McpToolFilter>,
    /// Tools that never require approval.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub never: Option<McpToolFilter>,
}

/// A blanket MCP tool approval setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum McpToolApprovalSetting {
    /// Every tool on the server requires approval.
    #[serde(rename = "always")]
    Always,
    /// No tool on the server requires approval.
    #[serde(rename = "never")]
    Never,
}

/// Approval policy for the tools of an MCP server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(untagged)]
#[display("{self:?}")]
pub enum McpApproval {
    /// Filter-based approval, by read-only flag and/or tool names.
    Filter(McpToolApprovalFilter),
    /// A blanket `always` / `never` approval setting.
    Setting(McpToolApprovalSetting),
}

/// A single entry of the `tools` array.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(tag = "type")]
#[display("{self:?}")]
pub enum RealtimeToolsConfigUnion {
    /// A function tool.
    #[serde(rename = "function")]
    Function(Box<RealtimeFunctionTool>),
    /// A remote MCP server tool.
    #[serde(rename = "mcp")]
    Mcp(Box<McpTool>),
}

/// Controls which tool the model calls.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(untagged)]
#[display("{self:?}")]
pub enum RealtimeToolChoiceConfig {
    /// `"none"`, `"auto"`, or `"required"`.
    Options(ToolChoiceOptions),
    /// Force the model to call a specific function.
    Function(ToolChoiceFunction),
    /// Force the model to call a specific MCP tool.
    Mcp(ToolChoiceMcp),
}

/// A tool choice string value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum ToolChoiceOptions {
    /// The model must not call any tool.
    #[serde(rename = "none")]
    None,
    /// The model decides whether to call a tool.
    #[serde(rename = "auto")]
    Auto,
    /// The model must call one or more tools.
    #[serde(rename = "required")]
    Required,
}

/// Forces the model to call a specific function.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ToolChoiceFunction {
    /// The name of the function to call.
    pub name: String,
    /// For function calling, the type is always `function`.
    #[serde(rename = "type")]
    pub type_: String,
}

/// Forces the model to call a specific tool on a remote MCP server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ToolChoiceMcp {
    /// The label of the MCP server to use.
    pub server_label: String,
    /// For MCP tools, the type is always `mcp`.
    #[serde(rename = "type")]
    pub type_: String,
    /// The name of the tool to call on the server.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// A Realtime item requesting human approval of a tool invocation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeMcpApprovalRequest {
    /// The unique ID of the approval request.
    pub id: String,
    /// A JSON string of arguments for the tool.
    pub arguments: String,
    /// The name of the tool to run.
    pub name: String,
    /// The label of the MCP server making the request.
    pub server_label: String,
    /// The type of the item. Always `mcp_approval_request`.
    #[serde(rename = "type")]
    pub type_: String,
}

/// A Realtime item responding to an MCP approval request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeMcpApprovalResponse {
    /// The unique ID of the approval response.
    pub id: String,
    /// The ID of the approval request being answered.
    pub approval_request_id: String,
    /// Whether the request was approved.
    pub approve: bool,
    /// The type of the item. Always `mcp_approval_response`.
    #[serde(rename = "type")]
    pub type_: String,
    /// Optional reason for the decision.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// A Realtime item listing tools available on an MCP server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeMcpListTools {
    /// The label of the MCP server.
    pub server_label: String,
    /// The tools available on the server.
    pub tools: Vec<McpListTool>,
    /// The type of the item. Always `mcp_list_tools`.
    #[serde(rename = "type")]
    pub type_: String,
    /// The unique ID of the list.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// A tool available on an MCP server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct McpListTool {
    /// The JSON schema describing the tool's input.
    pub input_schema: serde_json::Value,
    /// The name of the tool.
    pub name: String,
    /// Additional annotations about the tool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<serde_json::Value>,
    /// The description of the tool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// A Realtime item representing an invocation of a tool on an MCP server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeMcpToolCall {
    /// The unique ID of the tool call.
    pub id: String,
    /// A JSON string of the arguments passed to the tool.
    pub arguments: String,
    /// The name of the tool that was run.
    pub name: String,
    /// The label of the MCP server running the tool.
    pub server_label: String,
    /// The type of the item. Always `mcp_call`.
    #[serde(rename = "type")]
    pub type_: String,
    /// The ID of an associated approval request, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_request_id: Option<String>,
    /// The error from the tool call, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<RealtimeMcpError>,
    /// The output from the tool call.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

/// The error from an MCP tool call.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(untagged)]
#[display("{self:?}")]
pub enum RealtimeMcpError {
    /// An MCP protocol error.
    Protocol(RealtimeMcpProtocolError),
    /// A tool execution error.
    ToolExecution(RealtimeMcpToolExecutionError),
    /// An HTTP error.
    Http(RealtimeMcphttpError),
}

/// An MCP protocol error.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeMcpProtocolError {
    /// The error code.
    pub code: i64,
    /// The error message.
    pub message: String,
    /// The error type. Always `protocol_error`.
    #[serde(rename = "type")]
    pub type_: String,
}

/// An MCP tool execution error.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeMcpToolExecutionError {
    /// The error message.
    pub message: String,
    /// The error type. Always `tool_execution_error`.
    #[serde(rename = "type")]
    pub type_: String,
}

/// An HTTP error returned by an MCP server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeMcphttpError {
    /// The HTTP status code.
    pub code: i64,
    /// The error message.
    pub message: String,
    /// The error type. Always `http_error`.
    #[serde(rename = "type")]
    pub type_: String,
}

/// `mcp_list_tools.in_progress` — the server started listing an MCP server's tools.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct McpListToolsInProgress {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the MCP list-tools item.
    pub item_id: String,
    /// The event type, always `mcp_list_tools.in_progress`.
    #[serde(rename = "type")]
    pub type_: String,
}

/// `mcp_list_tools.failed` — listing an MCP server's tools failed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct McpListToolsFailed {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the MCP list-tools item.
    pub item_id: String,
    /// The event type, always `mcp_list_tools.failed`.
    #[serde(rename = "type")]
    pub type_: String,
}

/// `mcp_list_tools.completed` — listing an MCP server's tools completed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct McpListToolsCompleted {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the MCP list-tools item.
    pub item_id: String,
    /// The event type, always `mcp_list_tools.completed`.
    #[serde(rename = "type")]
    pub type_: String,
}
