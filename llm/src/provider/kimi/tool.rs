//! Tool (function) definition types for the chat completion API.
//!
//! Only `function` tools are expressible here; the built-in tools
//! (`$web_search`, `$date`, …) are not part of this surface's vocabulary.

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

    /// Whether to strictly constrain arguments to the schema.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

/// Controls which tool to call.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolChoice {
    /// A string option (`auto`, `none`, `required`).
    String(ToolChoiceString),
    /// Force a specific function.
    Specific {
        /// The type of the tool (always `function`).
        #[serde(rename = "type")]
        r#type: String,
        /// The function to call.
        function: FunctionName,
    },
}

/// Tool choice string options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolChoiceString {
    /// The model will not call any tool.
    #[serde(rename = "none")]
    None,
    /// The model decides whether to call a tool.
    #[serde(rename = "auto")]
    Auto,
    /// The model must call a tool (K3 only; K2.x reject it).
    #[serde(rename = "required")]
    Required,
}

/// Function name specification for `tool_choice`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionName {
    /// The name of the function to call.
    pub name: String,
}
