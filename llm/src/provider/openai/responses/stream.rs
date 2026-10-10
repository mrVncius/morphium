//! Streaming event types for the Responses API.

use derive_more::{Display, IsVariant};
use serde::{Deserialize, Serialize};

use super::{OutputContent, OutputItem, ResponsesResponse};

/// A streaming event from the Responses API.
///
/// Each event has a `type` field that determines which variant.
#[derive(Debug, Clone, Serialize, Deserialize, Display, IsVariant)]
#[serde(tag = "type")]
#[display("{self:?}")]
pub enum StreamEvent {
    /// Emitted when the response is created.
    #[serde(rename = "response.created")]
    ResponseCreated {
        /// The response object.
        response: ResponsesResponse,
    },
    /// Emitted when the response is in progress.
    #[serde(rename = "response.in_progress")]
    ResponseInProgress {
        /// The response object.
        response: ResponsesResponse,
    },
    /// Emitted when a new output item is added.
    #[serde(rename = "response.output_item.added")]
    OutputItemAdded {
        /// The index of the output item.
        output_index: u32,
        /// The output item that was added.
        item: OutputItem,
    },
    /// Emitted when a new content part is added.
    #[serde(rename = "response.content_part.added")]
    ContentPartAdded {
        /// The ID of the item.
        item_id: String,
        /// The index of the output item.
        output_index: u32,
        /// The index of the content part.
        content_index: u32,
        /// The content part that was added.
        part: OutputContent,
    },
    /// Emitted when there is a delta update to output text.
    #[serde(rename = "response.output_text.delta")]
    OutputTextDelta {
        /// The ID of the item.
        item_id: String,
        /// The index of the output item.
        output_index: u32,
        /// The index of the content part.
        content_index: u32,
        /// The delta text content.
        delta: String,
    },
    /// Emitted when an output text part is complete.
    #[serde(rename = "response.output_text.done")]
    OutputTextDone {
        /// The ID of the item.
        item_id: String,
        /// The index of the output item.
        output_index: u32,
        /// The index of the content part.
        content_index: u32,
        /// The complete text content.
        text: String,
    },
    /// Emitted when a content part is complete.
    #[serde(rename = "response.content_part.done")]
    ContentPartDone {
        /// The ID of the item.
        item_id: String,
        /// The index of the output item.
        output_index: u32,
        /// The index of the content part.
        content_index: u32,
        /// The completed content part.
        part: OutputContent,
    },
    /// Emitted when an output item is complete.
    #[serde(rename = "response.output_item.done")]
    OutputItemDone {
        /// The index of the output item.
        output_index: u32,
        /// The completed output item.
        item: OutputItem,
    },
    /// Emitted when the response is completed.
    #[serde(rename = "response.completed")]
    ResponseCompleted {
        /// The completed response object.
        response: ResponsesResponse,
    },
    /// Emitted when the response fails.
    #[serde(rename = "response.failed")]
    ResponseFailed {
        /// The failed response object.
        response: ResponsesResponse,
    },
}
