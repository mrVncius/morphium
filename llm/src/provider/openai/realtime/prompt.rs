//! Prompt templates, cache breakpoints, and prompt variable content parts.
//!
//! See: <https://developers.openai.com/api/reference/resources/realtime>

use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Arbitrary key/value metadata attached to a prompt or response.
pub type Metadata = HashMap<String, serde_json::Value>;

/// A reference to a prompt template, optionally filled with variables.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ResponsePrompt {
    /// The unique ID of the prompt template.
    pub id: String,

    /// Variables to fill in the prompt template. Keys are variable names,
    /// values are either a plain string or a content part object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, PromptVariable>>,

    /// The version of the prompt template to use, or `null` for the default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// A value substituted into a prompt template variable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(untagged)]
#[display("{self:?}")]
pub enum PromptVariable {
    /// A plain text string value.
    Text(String),

    /// A text content part.
    InputText(ResponseInputText),

    /// An image content part.
    InputImage(ResponseInputImage),

    /// A file content part.
    InputFile(ResponseInputFile),
}

/// Marks the location at which the prompt cache may be broken.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct PromptCacheBreakpoint {
    /// The cache breakpoint mode. Always `explicit`.
    pub mode: PromptCacheMode,
}

/// The cache breakpoint mode. Always `explicit`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum PromptCacheMode {
    /// An explicit cache breakpoint.
    #[display("explicit")]
    Explicit,
}

/// A text content part of a prompt variable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ResponseInputText {
    /// The text content.
    pub text: String,

    /// The type of the content part. Always `input_text`.
    #[serde(rename = "type", default = "input_text_type")]
    pub type_: String,

    /// The cache breakpoint for this content part, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_cache_breakpoint: Option<PromptCacheBreakpoint>,
}

/// An image content part of a prompt variable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ResponseInputImage {
    /// The detail level of the image to be sent to the model.
    pub detail: ImageDetail,

    /// The type of the content part. Always `input_image`.
    #[serde(rename = "type", default = "input_image_type")]
    pub type_: String,

    /// The ID of the uploaded image file, if the image is a file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,

    /// The URL of the image, if the image is referenced by URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,

    /// The cache breakpoint for this content part, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_cache_breakpoint: Option<PromptCacheBreakpoint>,
}

/// A file content part of a prompt variable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ResponseInputFile {
    /// The type of the content part. Always `input_file`.
    #[serde(rename = "type", default = "input_file_type")]
    pub type_: String,

    /// The detail level of the file to be sent to the model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<FileDetail>,

    /// The base64-encoded contents of the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_data: Option<String>,

    /// The ID of the uploaded file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,

    /// The URL of the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,

    /// The name of the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// The cache breakpoint for this content part, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_cache_breakpoint: Option<PromptCacheBreakpoint>,
}

/// The detail level of an image to be sent to the model.
///
/// One of `low`, `high`, `auto`, or `original`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum ImageDetail {
    /// Low detail level.
    #[display("low")]
    Low,
    /// High detail level.
    #[display("high")]
    High,
    /// Auto detail level (default).
    #[display("auto")]
    Auto,
    /// Original detail level.
    #[display("original")]
    Original,
}

/// The detail level of a file to be sent to the model.
///
/// One of `auto`, `low`, or `high`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum FileDetail {
    /// Auto detail level (default).
    #[display("auto")]
    Auto,
    /// Low detail level.
    #[display("low")]
    Low,
    /// High detail level.
    #[display("high")]
    High,
}

/// The default value of [`ResponseInputText::type_`].
fn input_text_type() -> String {
    "input_text".to_owned()
}

/// The default value of [`ResponseInputImage::type_`].
fn input_image_type() -> String {
    "input_image".to_owned()
}

/// The default value of [`ResponseInputFile::type_`].
fn input_file_type() -> String {
    "input_file".to_owned()
}
