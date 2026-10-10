//! Response status, output, and token accounting.

use serde::{Deserialize, Serialize};

use super::{
    Content, ContentPart, InputItem, MaxOutputTokens, ReasoningConfig, ResponseTextConfig,
    Temperature, Tool, ToolChoice, TopLogProbs, TopP,
};

/// A model response from the stateless Responses endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponsesResponse {
    /// Unique response identifier.
    pub id: String,
    /// Always response.
    #[serde(rename = "object")]
    pub object_type: String,
    /// Creation time as Unix seconds.
    pub created_at: u64,
    /// Generation status.
    pub status: ResponseStatus,
    /// Model used to generate the response.
    pub model: String,
    /// Ordered reasoning, message, and tool-call output; replay in the next input.
    pub output: Vec<InputItem>,
    /// Error when generation failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ResponseError>,
    /// Truncation details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incomplete_details: Option<IncompleteDetails>,
    /// Token accounting, available in the final streaming response.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
    /// Echoed instructions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Echoed reasoning configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningConfig>,
    /// Echoed text configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<ResponseTextConfig>,
    /// Available tools.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tool>,
    /// Tool selection policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoice>,
    /// Always true when returned: parallel tool calling is enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    /// Always false when returned: responses are not stored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    /// Always null: no server-side conversation chaining.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    /// Echoed maximum output tokens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<MaxOutputTokens>,
    /// Echoed temperature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Temperature>,
    /// Echoed nucleus sampling parameter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_p: Option<TopP>,
    /// Echoed number of token log probabilities.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<TopLogProbs>,
    /// Echoed end-user identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

/// Response lifecycle status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseStatus {
    /// Still generating.
    InProgress,
    /// Finished successfully.
    Completed,
    /// Truncated or filtered.
    Incomplete,
    /// Generation failed.
    Failed,
}

/// Generation failure details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseError {
    /// Error code.
    pub code: String,
    /// Human-readable message.
    pub message: String,
}

/// Incomplete generation details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncompleteDetails {
    /// Why generation stopped.
    pub reason: IncompleteReason,
}

/// Reasons for incomplete output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncompleteReason {
    /// Output token limit reached.
    MaxOutputTokens,
    /// Output filtered.
    ContentFilter,
}

/// Token usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    /// Input token count.
    pub input_tokens: u64,
    /// Input token breakdown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_tokens_details: Option<InputTokensDetails>,
    /// Output token count, including reasoning.
    pub output_tokens: u64,
    /// Output token breakdown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_tokens_details: Option<OutputTokensDetails>,
    /// Total input and output tokens.
    pub total_tokens: u64,
}

/// Context-cache accounting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputTokensDetails {
    /// Input tokens served from the context cache.
    pub cached_tokens: u64,
}

/// Thinking-mode accounting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputTokensDetails {
    /// Generated reasoning tokens.
    pub reasoning_tokens: u64,
}

impl ResponsesResponse {
    /// Concatenate only visible output text, excluding reasoning and tool calls.
    pub fn text(&self) -> String {
        let mut result = String::new();
        for item in &self.output {
            if let InputItem::Message {
                content: Content::Parts(parts),
                ..
            } = item
            {
                for part in parts {
                    if let ContentPart::OutputText { text, .. } = part {
                        result.push_str(text);
                    }
                }
            }
        }
        result
    }
}
