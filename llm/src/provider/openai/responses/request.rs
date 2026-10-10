//! Request type for the Responses API.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::params::{MaxOutputTokens, Temperature, TopLogProbs, TopP};
use super::{Input, Tool, ToolChoice};

/// Request to create a model response.
///
/// Every field maps directly to a JSON key in the API spec.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsesRequest {
    /// Model ID (e.g. "gpt-5.4", "o3-mini").
    pub model: String,

    /// Text, image, or file inputs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<Input>,

    /// System/developer instructions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// Prompt template reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<ResponsePrompt>,

    /// Tools the model may call.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tool>,

    /// How the model selects tools.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,

    /// Allow parallel tool calls.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    /// Maximum tool calls.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_tool_calls: Option<u32>,

    /// What sampling temperature to use, between 0 and 2.
    /// Higher values like 0.8 make output more random, lower values like 0.2 make it more deterministic.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Temperature>,

    /// Nucleus sampling: consider tokens with top_p probability mass [0, 1].
    /// E.g. 0.1 means only the top 10% probability mass is considered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_p: Option<TopP>,

    /// An integer between 0 and 20 specifying the max number of most-likely tokens
    /// to return at each token position, each with an associated log probability.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<TopLogProbs>,

    /// An upper bound for the number of tokens that can be generated for a response,
    /// including visible output tokens and reasoning tokens (minimum 16).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<MaxOutputTokens>,

    /// Plain text or structured JSON.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<ResponseTextConfig>,

    /// Reasoning effort and summary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningConfig>,

    /// Previous response ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,

    /// Conversation this response belongs to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation: Option<ConversationParam>,

    /// Stream the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,

    /// Streaming options.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptions>,

    /// Whether to store the response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,

    /// Additional data to include.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<ResponseIncludable>,

    /// Run in background.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,

    /// Context management (compaction).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_management: Option<ContextManagement>,

    /// Truncation strategy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub truncation: Option<String>,

    /// Safety identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub safety_identifier: Option<String>,

    /// End-user identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// Service tier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,

    /// Custom metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,

    /// Prompt cache key.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,

    /// Prompt cache retention policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_cache_retention: Option<String>,
}

/// Prompt template reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsePrompt {
    /// The ID of the prompt template.
    pub id: String,
    /// Variables to fill in the prompt template.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, serde_json::Value>>,
    /// The version of the prompt template.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Conversation parameter: a string ID or `{ id }` object.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ConversationParam {
    /// Conversation ID as a string.
    Id(String),
    /// Conversation object with an `id` field.
    Object {
        /// The conversation ID.
        id: String,
    },
}

/// Streaming options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamOptions {
    /// Whether to include obfuscated content in the stream.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_obfuscation: Option<bool>,
}

/// Additional data to include in the response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseIncludable {
    /// Include file search call results.
    #[serde(rename = "file_search_call.results")]
    FileSearchCallResults,
    /// Include web search call results.
    #[serde(rename = "web_search_call.results")]
    WebSearchCallResults,
    /// Include web search call action sources.
    #[serde(rename = "web_search_call.action.sources")]
    WebSearchCallActionSources,
    /// Include message input image URLs.
    #[serde(rename = "message.input_image.image_url")]
    MessageInputImageImageUrl,
    /// Include computer call output image URLs.
    #[serde(rename = "computer_call_output.output.image_url")]
    ComputerCallOutputOutputImageUrl,
    /// Include code interpreter call outputs.
    #[serde(rename = "code_interpreter_call.outputs")]
    CodeInterpreterCallOutputs,
    /// Include reasoning encrypted content.
    #[serde(rename = "reasoning.encrypted_content")]
    ReasoningEncryptedContent,
    /// Include message output text logprobs.
    #[serde(rename = "message.output_text.logprobs")]
    MessageOutputTextLogprobs,
}

/// Context management (compaction) configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextManagement {
    /// The context management entry type. Currently only 'compaction' is supported.
    #[serde(rename = "type")]
    pub kind: String,
    /// Token threshold at which compaction should be triggered.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compact_threshold: Option<u32>,
}

/// Text output format configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTextConfig {
    /// The format of the output (text, JSON schema, or JSON object).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<ResponseFormat>,
    /// The verbosity level of the output.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<String>,
}

/// Response format: text, json_schema, or json_object.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResponseFormat {
    /// Plain text output.
    #[serde(rename = "text")]
    Text,
    /// JSON Schema structured output with a named schema.
    #[serde(rename = "json_schema")]
    JsonSchema {
        /// The name of the JSON schema.
        name: String,
        /// The JSON schema definition.
        #[serde(default)]
        schema: serde_json::Value,
        /// A description of the JSON schema.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Whether to enforce strict schema adherence.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        strict: Option<bool>,
    },
    /// JSON object output (free-form JSON).
    #[serde(rename = "json_object")]
    JsonObject,
}

/// Reasoning configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningConfig {
    /// Reasoning effort level (none, minimal, low, medium, high, xhigh).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effort: Option<ReasoningEffort>,
    /// Deprecated — use `summary` instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generate_summary: Option<String>,
    /// A summary of the reasoning process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

/// Reasoning effort levels for models that support it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasoningEffort {
    /// No reasoning effort.
    None,
    /// Minimal reasoning effort.
    Minimal,
    /// Low reasoning effort.
    Low,
    /// Medium reasoning effort.
    Medium,
    /// High reasoning effort.
    High,
    /// Extra-high reasoning effort.
    #[serde(rename = "xhigh")]
    XHigh,
}
