//! Function and apply_patch tools in the flat Responses wire format.

use serde::{Deserialize, Serialize};

/// Supported tools. Other built-in tool types are ignored by DeepSeek.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Tool {
    /// Function tool; names must be unique, nonempty, at most 128 characters,
    /// and match [a-zA-Z0-9_-]+.
    Function {
        /// Function name.
        name: String,
        /// Description used by the model to select the tool.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// JSON schema for arguments; omitted means an empty parameter list.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        parameters: Option<serde_json::Value>,
        /// Strict argument schema adherence.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        strict: Option<bool>,
    },
    /// Custom tool; only the name apply_patch is supported.
    Custom {
        /// Must be apply_patch.
        name: String,
        /// Tool description.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Custom tool format definition.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        format: Option<serde_json::Value>,
    },
}

/// Tool selection as a string policy or a specific function.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolChoice {
    /// General selection policy.
    Mode(ToolChoiceMode),
    /// Force a specific function.
    Function(NamedFunction),
}

/// General tool selection policies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolChoiceMode {
    /// Do not call tools.
    None,
    /// Model chooses between a message and tool calls.
    Auto,
    /// Require one or more tool calls.
    Required,
}

/// A specific function selected via `{"type":"function","name":...}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename = "function")]
pub struct NamedFunction {
    /// Function name.
    pub name: String,
}
