//! Realtime client→server WebSocket events.
//!
//! These are the events the OpenAI Realtime WebSocket server accepts from the
//! client. Each variant of [`RealtimeClientEvent`] carries the payload struct
//! named by the schema for that event.
//!
//! See: <https://developers.openai.com/api/reference/resources/realtime/client-events>

use derive_more::{Display, IsVariant};
use serde::{Deserialize, Serialize};

use super::message::ConversationItem;
use super::response::RealtimeResponseCreateParams;
use super::session::{RealtimeSessionCreateRequest, RealtimeTranscriptionSessionCreateRequest};

/// An event sent from the client to the Realtime WebSocket server.
///
/// Discriminated on the `type` JSON key, whose values contain dots.
#[derive(Debug, Clone, Serialize, Deserialize, Display, IsVariant)]
#[serde(tag = "type")]
#[display("{self:?}")]
pub enum RealtimeClientEvent {
    /// Update the session's configuration (`session.update`).
    #[serde(rename = "session.update")]
    SessionUpdate(Box<RealtimeClientEventSessionUpdate>),

    /// Append audio bytes to the input audio buffer (`input_audio_buffer.append`).
    #[serde(rename = "input_audio_buffer.append")]
    InputAudioBufferAppend(Box<RealtimeClientEventInputAudioBufferAppend>),

    /// Commit the input audio buffer (`input_audio_buffer.commit`).
    #[serde(rename = "input_audio_buffer.commit")]
    InputAudioBufferCommit(Box<RealtimeClientEventInputAudioBufferCommit>),

    /// Clear the input audio buffer (`input_audio_buffer.clear`).
    #[serde(rename = "input_audio_buffer.clear")]
    InputAudioBufferClear(Box<RealtimeClientEventInputAudioBufferClear>),

    /// Add a new item to the conversation (`conversation.item.create`).
    #[serde(rename = "conversation.item.create")]
    ConversationItemCreate(Box<RealtimeClientEventConversationItemCreate>),

    /// Retrieve the server's representation of an item (`conversation.item.retrieve`).
    #[serde(rename = "conversation.item.retrieve")]
    ConversationItemRetrieve(Box<RealtimeClientEventConversationItemRetrieve>),

    /// Truncate a previous assistant message's audio (`conversation.item.truncate`).
    #[serde(rename = "conversation.item.truncate")]
    ConversationItemTruncate(Box<RealtimeClientEventConversationItemTruncate>),

    /// Remove an item from the conversation history (`conversation.item.delete`).
    #[serde(rename = "conversation.item.delete")]
    ConversationItemDelete(Box<RealtimeClientEventConversationItemDelete>),

    /// Instruct the server to create a response (`response.create`).
    #[serde(rename = "response.create")]
    ResponseCreate(Box<RealtimeClientEventResponseCreate>),

    /// Cancel an in-progress response (`response.cancel`).
    #[serde(rename = "response.cancel")]
    ResponseCancel(Box<RealtimeClientEventResponseCancel>),

    /// Cut off the current audio response (`output_audio_buffer.clear`).
    #[serde(rename = "output_audio_buffer.clear")]
    OutputAudioBufferClear(Box<RealtimeClientEventOutputAudioBufferClear>),
}

/// Either a Realtime session or a transcription session configuration, distinguished
/// structurally rather than by a tag.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(untagged)]
#[display("{self:?}")]
pub enum SessionUpdateTarget {
    /// A Realtime session configuration.
    Realtime(Box<RealtimeSessionCreateRequest>),

    /// A transcription session configuration.
    Transcription(Box<RealtimeTranscriptionSessionCreateRequest>),
}

/// Payload for the `session.update` client event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeClientEventSessionUpdate {
    /// Update the Realtime session. Choose either a realtime session or a
    /// transcription session.
    pub session: SessionUpdateTarget,

    /// Optional client-generated ID used to identify this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

/// Payload for the `input_audio_buffer.append` client event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeClientEventInputAudioBufferAppend {
    /// Base64-encoded audio bytes. This must be in the format specified by the
    /// `input_audio_format` field in the session configuration.
    pub audio: String,

    /// Optional client-generated ID used to identify this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

/// Payload for the `input_audio_buffer.commit` client event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeClientEventInputAudioBufferCommit {
    /// Optional client-generated ID used to identify this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

/// Payload for the `input_audio_buffer.clear` client event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeClientEventInputAudioBufferClear {
    /// Optional client-generated ID used to identify this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

/// Payload for the `conversation.item.create` client event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeClientEventConversationItemCreate {
    /// The item to add to the conversation.
    pub item: ConversationItem,

    /// Optional client-generated ID used to identify this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,

    /// The ID of the preceding item after which the new item will be inserted. If
    /// not set, the new item is appended to the end of the conversation. If set to
    /// `root`, the new item is added to the beginning of the conversation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_item_id: Option<String>,
}

/// Payload for the `conversation.item.retrieve` client event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeClientEventConversationItemRetrieve {
    /// The ID of the item to retrieve.
    pub item_id: String,

    /// Optional client-generated ID used to identify this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

/// Payload for the `conversation.item.truncate` client event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeClientEventConversationItemTruncate {
    /// Inclusive duration up to which audio is truncated, in milliseconds. If the
    /// value is greater than the actual audio duration, the server responds with
    /// an error.
    pub audio_end_ms: u64,

    /// The index of the content part to truncate. Set this to `0`.
    pub content_index: u64,

    /// The ID of the assistant message item to truncate. Only assistant message
    /// items can be truncated.
    pub item_id: String,

    /// Optional client-generated ID used to identify this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

/// Payload for the `conversation.item.delete` client event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeClientEventConversationItemDelete {
    /// The ID of the item to delete.
    pub item_id: String,

    /// Optional client-generated ID used to identify this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

/// Payload for the `response.create` client event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeClientEventResponseCreate {
    /// Create a new Realtime response with these parameters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<RealtimeResponseCreateParams>,

    /// Optional client-generated ID used to identify this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

/// Payload for the `response.cancel` client event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeClientEventResponseCancel {
    /// A specific response ID to cancel — if not provided, cancels an in-progress
    /// response in the default conversation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_id: Option<String>,

    /// Optional client-generated ID used to identify this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

/// Payload for the `output_audio_buffer.clear` client event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeClientEventOutputAudioBufferClear {
    /// The unique ID of the client event used for error handling.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}
