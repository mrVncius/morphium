//! Tool (function) definition types for the chat completion API.
//!
//! Z.AI's `parameters` is an arbitrary JSON Schema object (`additionalProperties:
//! true`), so it is carried as a `serde_json::Value` rather than a typed shape.
//! Only `function` tools are expressible here; the retrieval and web-search
//! tools are not part of this surface's tool vocabulary.

use serde::{Deserialize, Serialize};

/// A tool definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// Type of tool (always `function`).
    #[serde(rename = "type")]
    pub tool_type: ToolType,

    /// Function definition.
    pub function: FunctionDefinition,
}

/// Type of tool.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolType {
    /// A user-defined function.
    #[serde(rename = "function")]
    Function,
}

/// Function tool definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    /// Function name.
    pub name: String,

    /// Description of what the function does.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// JSON Schema for the function parameters.
    pub parameters: serde_json::Value,
}
