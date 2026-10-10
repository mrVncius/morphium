//! Tool definition types for the chat completion API.
//!
//! Types that describe the tools (functions) the model may call,
//! including JSON Schema parameter definitions and tool-choice
//! configuration.

use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A tool definition
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct Tool {
    /// Type of tool (always "function" for user-defined tools)
    #[serde(rename = "type")]
    pub tool_type: ToolType,

    /// Function definition
    pub function: FunctionDefinition,
}

/// Type of tool
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum ToolType {
    /// The type of the tool. Currently, only `function` is supported.
    #[serde(rename = "function")]
    #[display("function")]
    Function,
}

/// Function tool definition
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct FunctionDefinition {
    /// Function name
    pub name: String,

    /// Description of what the function does
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// JSON Schema for function parameters
    pub parameters: FunctionParameters,

    /// Strict mode: enforce schema compliance (requires /beta endpoint).
    /// When enabled, all object properties must be required and additionalProperties: false.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

/// Function parameters JSON Schema
///
/// In strict mode, the following rules apply:
/// - All object properties must be listed in `required`
/// - `additionalProperties: false` is required for all objects
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct FunctionParameters {
    /// Schema type - must be "object" for root
    #[serde(rename = "type")]
    pub schema_type: ParameterType,

    /// Description of the parameters object
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Properties of the object
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, ParameterSchema>,

    /// Required property names (all properties must be required in strict mode)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,

    /// Whether additional properties are allowed (must be false in strict mode)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<bool>,

    /// Definitions for $ref references
    #[serde(rename = "$def", default, skip_serializing_if = "HashMap::is_empty")]
    pub definitions: HashMap<String, ParameterSchema>,
}

/// Parameter type for JSON Schema
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "lowercase")]
pub enum ParameterType {
    /// JSON Schema type `object`.
    #[display("object")]
    Object,
    /// JSON Schema type `string`.
    #[display("string")]
    String,
    /// JSON Schema type `number`.
    #[display("number")]
    Number,
    /// JSON Schema type `integer`.
    #[display("integer")]
    Integer,
    /// JSON Schema type `boolean`.
    #[display("boolean")]
    Boolean,
    /// JSON Schema type `array`.
    #[display("array")]
    Array,
    /// JSON Schema type `null`.
    #[display("null")]
    Null,
}

/// Schema for a single parameter property
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ParameterSchema {
    /// Parameter type
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param_type: Option<ParameterType>,

    /// Description of the parameter
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Enum values (for string enums)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<serde_json::Value>>,

    /// Constant value
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constant: Option<serde_json::Value>,

    /// Default value
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<serde_json::Value>,

    /// String format (email, hostname, ipv4, ipv6, uuid)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// Regex pattern for string validation
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,

    /// Minimum value for numbers
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,

    /// Maximum value for numbers
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,

    /// Exclusive minimum (value must be greater than this)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<f64>,

    /// Exclusive maximum (value must be less than this)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<f64>,

    /// Multiple of constraint for numbers
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<f64>,

    /// For array types - schema of array items
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<ParameterSchema>>,

    /// Nested object properties
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, ParameterSchema>,

    /// Required nested properties
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,

    /// Whether additional properties are allowed
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<bool>,

    /// anyOf schemas (for union types)
    #[serde(rename = "anyOf", default, skip_serializing_if = "Vec::is_empty")]
    pub any_of: Vec<ParameterSchema>,

    /// Reference to a definition
    #[serde(rename = "$ref", default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

/// Controls which tool to call
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
#[serde(untagged)]
pub enum ToolChoice {
    /// Simple string option
    String(ToolChoiceString),
    /// Force a specific tool
    Specific {
        /// The type of the tool. Currently, only `function` is supported.
        r#type: String,
        /// The function to call.
        function: FunctionName,
    },
}

/// Tool choice string options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum ToolChoiceString {
    /// Model will not call any tool
    #[serde(rename = "none")]
    #[display("none")]
    None,
    /// Model can choose to call a tool or generate a message (default when tools present)
    #[serde(rename = "auto")]
    #[display("auto")]
    Auto,
    /// Model must call one or more tools
    #[serde(rename = "required")]
    #[display("required")]
    Required,
}

/// Function name specification for tool_choice
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct FunctionName {
    /// The name of the function to call.
    pub name: String,
}
