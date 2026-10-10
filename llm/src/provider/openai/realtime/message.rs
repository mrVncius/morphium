//! Conversation items and their content parts.
//!
//! The conversation history shared by the Realtime WebSocket session and the
//! REST conversation resource: message items, function call items, MCP items,
//! transcription segments, and log probability properties.
//!
//! See: <https://developers.openai.com/api/reference/resources/realtime>

use derive_more::Display;
use serde::{Deserialize, Serialize};

use super::prompt::ImageDetail;
use super::tool::{
    RealtimeMcpApprovalRequest, RealtimeMcpApprovalResponse, RealtimeMcpListTools,
    RealtimeMcpToolCall,
};

/// The status of a conversation item.
///
/// One of `completed`, `incomplete`, or `in_progress`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum ItemStatus {
    /// The item completed successfully.
    #[serde(rename = "completed")]
    #[display("completed")]
    Completed,
    /// The item was not completed.
    #[serde(rename = "incomplete")]
    #[display("incomplete")]
    Incomplete,
    /// The item is still being processed.
    #[serde(rename = "in_progress")]
    #[display("in_progress")]
    InProgress,
}

/// A content part within a system message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct SystemContentPart {
    /// The text content of the part.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// The type of the content part. Always `input_text`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// A content part within a user message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct UserContentPart {
    /// Base64-encoded audio bytes, if this part is audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<String>,
    /// The detail level of the image, if this part is an image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<ImageDetail>,
    /// The URL of the image, if this part is an image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// The text content of the part, if this part is text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// The transcript of the audio, if this part is audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transcript: Option<String>,
    /// The type of the content part: `input_text`, `input_audio`, or `input_image`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// A content part within an assistant message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct AssistantContentPart {
    /// Base64-encoded audio bytes, if this part is audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<String>,
    /// The text content of the part, if this part is text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// The transcript of the audio, if this part is audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transcript: Option<String>,
    /// The type of the content part: `output_text` or `output_audio`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// A system message item in the conversation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeConversationItemSystemMessage {
    /// The content of the message.
    #[serde(default)]
    pub content: Vec<SystemContentPart>,
    /// The role of the message. Always `system`.
    pub role: String,
    /// The type of the item. Always `message`.
    #[serde(rename = "type")]
    pub type_: String,
    /// The unique ID of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The object type. Always `realtime.item`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    /// The status of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// A user message item in the conversation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeConversationItemUserMessage {
    /// The content of the message.
    #[serde(default)]
    pub content: Vec<UserContentPart>,
    /// The role of the message. Always `user`.
    pub role: String,
    /// The type of the item. Always `message`.
    #[serde(rename = "type")]
    pub type_: String,
    /// The unique ID of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The object type. Always `realtime.item`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    /// The status of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// An assistant message item in the conversation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeConversationItemAssistantMessage {
    /// The content of the message.
    #[serde(default)]
    pub content: Vec<AssistantContentPart>,
    /// The role of the message. Always `assistant`.
    pub role: String,
    /// The type of the item. Always `message`.
    #[serde(rename = "type")]
    pub type_: String,
    /// The unique ID of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The object type. Always `realtime.item`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    /// The status of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// A function call item in the conversation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeConversationItemFunctionCall {
    /// The arguments of the function call, as a JSON string.
    pub arguments: String,
    /// The name of the function being called.
    pub name: String,
    /// The type of the item. Always `function_call`.
    #[serde(rename = "type")]
    pub type_: String,
    /// The unique ID of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The ID of the function call, used when submitting the call output.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
    /// The object type. Always `realtime.item`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    /// The status of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// A function call output item in the conversation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeConversationItemFunctionCallOutput {
    /// The ID of the function call this output belongs to.
    pub call_id: String,
    /// The output of the function call.
    pub output: String,
    /// The type of the item. Always `function_call_output`.
    #[serde(rename = "type")]
    pub type_: String,
    /// The unique ID of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The object type. Always `realtime.item`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    /// The status of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// An item in the conversation.
///
/// Note: the variant order matters for untagged deserialization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(untagged)]
#[display("{self:?}")]
pub enum ConversationItem {
    /// A system message.
    System(RealtimeConversationItemSystemMessage),
    /// A user message.
    User(RealtimeConversationItemUserMessage),
    /// An assistant message.
    Assistant(RealtimeConversationItemAssistantMessage),
    /// A function call.
    FunctionCall(RealtimeConversationItemFunctionCall),
    /// A function call output.
    FunctionCallOutput(RealtimeConversationItemFunctionCallOutput),
    /// An MCP approval response.
    McpApprovalResponse(RealtimeMcpApprovalResponse),
    /// An MCP list tools result.
    McpListTools(RealtimeMcpListTools),
    /// An MCP tool call.
    McpCall(RealtimeMcpToolCall),
    /// An MCP approval request.
    McpApprovalRequest(RealtimeMcpApprovalRequest),
}

/// A content part of a [`ConversationItemWithReference`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ConversationItemWithReferenceContent {
    /// The ID of the content part.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Base64-encoded audio bytes, if this part is audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<String>,
    /// The text content of the part, if this part is text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// The transcript of the audio, if this part is audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transcript: Option<String>,
    /// The type of the content part: `input_audio`, `input_text`, `item_reference`, or `text`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// A conversation item that may reference an existing item instead of resending it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ConversationItemWithReference {
    /// The unique ID of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The arguments of the function call, as a JSON string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
    /// The ID of the function call, used when submitting the call output.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
    /// The content of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<ConversationItemWithReferenceContent>>,
    /// The name of the function being called.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The object type. Always `realtime.item`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    /// The output of the function call.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// The role of the message: `user`, `assistant`, or `system`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// The status of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
    /// The type of the item: `message`, `function_call`, or `function_call_output`.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// A segment of an input audio transcription.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ConversationItemInputAudioTranscriptionSegment {
    /// The unique ID of the segment.
    pub id: String,
    /// The index of the content part within the item.
    pub content_index: u64,
    /// The end time of the segment, in milliseconds.
    pub end: f64,
    /// The unique ID of the event that produced the segment.
    pub event_id: String,
    /// The ID of the item the segment belongs to.
    pub item_id: String,
    /// The speaker of the segment, if known.
    pub speaker: String,
    /// The start time of the segment, in milliseconds.
    pub start: f64,
    /// The text of the segment.
    pub text: String,
    /// The type of the event. Always `conversation.item.input_audio_transcription.segment`.
    #[serde(rename = "type")]
    pub type_: String,
}

/// The log probability information for a transcription token.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct LogProbProperties {
    /// The token text.
    pub token: String,
    /// The bytes of the token.
    #[serde(default)]
    pub bytes: Vec<u8>,
    /// The log probability of the token.
    pub logprob: f64,
}
