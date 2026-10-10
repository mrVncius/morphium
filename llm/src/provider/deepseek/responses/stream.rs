//! Semantic SSE events; terminal events replace the usual `[DONE]` marker.

use serde::{Deserialize, Serialize};

use super::{ContentPart, InputItem, ResponsesResponse};

/// A semantic event with its monotonically increasing sequence number.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamEvent {
    /// Sequence number assigned by the server.
    pub sequence_number: u64,
    /// Typed event payload; serialized alongside sequence_number.
    #[serde(flatten)]
    pub event: StreamEventKind,
}

/// Supported Responses streaming payloads.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum StreamEventKind {
    /// Response created.
    #[serde(rename = "response.created")]
    ResponseCreated {
        /// Initial response with in_progress status.
        response: ResponsesResponse,
    },
    /// Generation started.
    #[serde(rename = "response.in_progress")]
    ResponseInProgress {
        /// Current response.
        response: ResponsesResponse,
    },
    /// Output item started.
    #[serde(rename = "response.output_item.added")]
    OutputItemAdded {
        /// Output item index.
        output_index: u32,
        /// Initial output item.
        item: InputItem,
    },
    /// Output item finished.
    #[serde(rename = "response.output_item.done")]
    OutputItemDone {
        /// Output item index.
        output_index: u32,
        /// Final output item.
        item: InputItem,
    },
    /// Content part started.
    #[serde(rename = "response.content_part.added")]
    ContentPartAdded {
        /// Parent item ID.
        item_id: String,
        /// Output item index.
        output_index: u32,
        /// Content part index.
        content_index: u32,
        /// Initial content part.
        part: ContentPart,
    },
    /// Content part finished.
    #[serde(rename = "response.content_part.done")]
    ContentPartDone {
        /// Parent item ID.
        item_id: String,
        /// Output item index.
        output_index: u32,
        /// Content part index.
        content_index: u32,
        /// Final content part.
        part: ContentPart,
    },
    /// Incremental chain of thought.
    #[serde(rename = "response.reasoning_text.delta")]
    ReasoningTextDelta {
        /// Parent item ID.
        item_id: String,
        /// Output item index.
        output_index: u32,
        /// Content part index.
        content_index: u32,
        /// Incremental reasoning.
        delta: String,
    },
    /// Complete chain of thought.
    #[serde(rename = "response.reasoning_text.done")]
    ReasoningTextDone {
        /// Parent item ID.
        item_id: String,
        /// Output item index.
        output_index: u32,
        /// Content part index.
        content_index: u32,
        /// Complete reasoning.
        text: String,
    },
    /// Incremental visible text.
    #[serde(rename = "response.output_text.delta")]
    OutputTextDelta {
        /// Parent item ID.
        item_id: String,
        /// Output item index.
        output_index: u32,
        /// Content part index.
        content_index: u32,
        /// Incremental text.
        delta: String,
        /// Token log probabilities, when requested.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        logprobs: Option<Vec<serde_json::Value>>,
    },
    /// Complete visible text.
    #[serde(rename = "response.output_text.done")]
    OutputTextDone {
        /// Parent item ID.
        item_id: String,
        /// Output item index.
        output_index: u32,
        /// Content part index.
        content_index: u32,
        /// Complete text.
        text: String,
        /// Token log probabilities, when requested.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        logprobs: Option<Vec<serde_json::Value>>,
    },
    /// Incremental function arguments.
    #[serde(rename = "response.function_call_arguments.delta")]
    FunctionCallArgumentsDelta {
        /// Function call item ID.
        item_id: String,
        /// Output item index.
        output_index: u32,
        /// Argument fragment.
        delta: String,
    },
    /// Complete function arguments.
    #[serde(rename = "response.function_call_arguments.done")]
    FunctionCallArgumentsDone {
        /// Function call item ID.
        item_id: String,
        /// Output item index.
        output_index: u32,
        /// Complete JSON-encoded arguments.
        arguments: String,
    },
    /// Incremental apply_patch input.
    #[serde(rename = "response.custom_tool_call_input.delta")]
    CustomToolCallInputDelta {
        /// Custom tool item ID.
        item_id: String,
        /// Output item index.
        output_index: u32,
        /// Input fragment.
        delta: String,
    },
    /// Complete apply_patch input.
    #[serde(rename = "response.custom_tool_call_input.done")]
    CustomToolCallInputDone {
        /// Custom tool item ID.
        item_id: String,
        /// Output item index.
        output_index: u32,
        /// Complete tool input.
        input: String,
    },
    /// Successful terminal event.
    #[serde(rename = "response.completed")]
    ResponseCompleted {
        /// Complete response including usage.
        response: ResponsesResponse,
    },
    /// Truncated terminal event.
    #[serde(rename = "response.incomplete")]
    ResponseIncomplete {
        /// Incomplete response with details.
        response: ResponsesResponse,
    },
    /// Failed terminal event.
    #[serde(rename = "response.failed")]
    ResponseFailed {
        /// Failed response with error details.
        response: ResponsesResponse,
    },
}

impl StreamEvent {
    /// Whether this event ends the response, regardless of HTTP stream lifetime.
    pub fn is_terminal(&self) -> bool {
        matches!(
            self.event,
            StreamEventKind::ResponseCompleted { .. }
                | StreamEventKind::ResponseIncomplete { .. }
                | StreamEventKind::ResponseFailed { .. }
        )
    }
}
