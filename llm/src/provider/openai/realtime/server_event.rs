//! Server→client WebSocket event types for the Realtime API.
//!
//! Every event emitted by the Realtime WebSocket server to the client. The
//! event `type` string is the discriminator of [`RealtimeServerEvent`]; the
//! payload structs carry only the event-specific fields (the tag is injected
//! by serde during serialization).
//!
//! See: <https://developers.openai.com/api/reference/resources/realtime/server-events>

use derive_more::{Display, IsVariant};
use serde::{Deserialize, Serialize};

use super::message::{AssistantContentPart, ConversationItem, LogProbProperties};
use super::response::{RealtimeError, RealtimeResponse};
use super::rest::{RealtimeSessionCreateResponse, RealtimeTranscriptionSessionCreateResponse};

/// An event emitted by the Realtime WebSocket server to the client.
///
/// The `type` field selects the variant.
#[derive(Debug, Clone, Serialize, Deserialize, Display, IsVariant)]
#[serde(tag = "type")]
#[display("{self:?}")]
pub enum RealtimeServerEvent {
    /// Returned when an error occurs, which could be a client problem or a server problem.
    #[serde(rename = "error")]
    Error(RealtimeServerEventError),

    /// Returned when a Session is created. Emitted automatically when a new
    /// connection is established as the first server event.
    #[serde(rename = "session.created")]
    SessionCreated(RealtimeServerEventSessionCreated),

    /// Returned when a Session is updated with a `session.update` event.
    #[serde(rename = "session.updated")]
    SessionUpdated(RealtimeServerEventSessionUpdated),

    /// Returned when a conversation item is added.
    #[serde(rename = "conversation.item.added")]
    ConversationItemAdded(RealtimeServerEventConversationItemAdded),

    /// Returned when a conversation item is done.
    #[serde(rename = "conversation.item.done")]
    ConversationItemDone(RealtimeServerEventConversationItemDone),

    /// Returned when a conversation item is retrieved.
    #[serde(rename = "conversation.item.retrieved")]
    ConversationItemRetrieved(RealtimeServerEventConversationItemRetrieved),

    /// Returned when input audio transcription is completed.
    #[serde(rename = "conversation.item.input_audio_transcription.completed")]
    ConversationItemInputAudioTranscriptionCompleted(
        RealtimeServerEventConversationItemInputAudioTranscriptionCompleted,
    ),

    /// Returned when input audio transcription is streamed as a delta.
    #[serde(rename = "conversation.item.input_audio_transcription.delta")]
    ConversationItemInputAudioTranscriptionDelta(
        RealtimeServerEventConversationItemInputAudioTranscriptionDelta,
    ),

    /// Returned when a transcription segment for an input audio item is completed.
    #[serde(rename = "conversation.item.input_audio_transcription.segment")]
    ConversationItemInputAudioTranscriptionSegment(
        RealtimeServerEventConversationItemInputAudioTranscriptionSegment,
    ),

    /// Returned when input audio transcription fails.
    #[serde(rename = "conversation.item.input_audio_transcription.failed")]
    ConversationItemInputAudioTranscriptionFailed(
        RealtimeServerEventConversationItemInputAudioTranscriptionFailed,
    ),

    /// Returned when an earlier assistant audio message item is truncated by the client.
    #[serde(rename = "conversation.item.truncated")]
    ConversationItemTruncated(RealtimeServerEventConversationItemTruncated),

    /// Returned when an item in the conversation is deleted by the client.
    #[serde(rename = "conversation.item.deleted")]
    ConversationItemDeleted(RealtimeServerEventConversationItemDeleted),

    /// Returned when an input audio buffer is committed, either by the client
    /// or automatically in server VAD mode.
    #[serde(rename = "input_audio_buffer.committed")]
    InputAudioBufferCommitted(RealtimeServerEventInputAudioBufferCommitted),

    /// Returned when a DTMF event is received on the input audio buffer.
    #[serde(rename = "input_audio_buffer.dtmf_event_received")]
    InputAudioBufferDtmfEventReceived(RealtimeServerEventInputAudioBufferDtmfEventReceived),

    /// Returned when the input audio buffer is cleared by the client.
    #[serde(rename = "input_audio_buffer.cleared")]
    InputAudioBufferCleared(RealtimeServerEventInputAudioBufferCleared),

    /// Returned when the server VAD detects the start of speech in the input audio buffer.
    #[serde(rename = "input_audio_buffer.speech_started")]
    InputAudioBufferSpeechStarted(RealtimeServerEventInputAudioBufferSpeechStarted),

    /// Returned when the server VAD detects the end of speech in the input audio buffer.
    #[serde(rename = "input_audio_buffer.speech_stopped")]
    InputAudioBufferSpeechStopped(RealtimeServerEventInputAudioBufferSpeechStopped),

    /// Returned when the configured idle timeout elapses without user speech.
    #[serde(rename = "input_audio_buffer.timeout_triggered")]
    InputAudioBufferTimeoutTriggered(RealtimeServerEventInputAudioBufferTimeoutTriggered),

    /// Returned when the server begins streaming audio to the client.
    #[serde(rename = "output_audio_buffer.started")]
    OutputAudioBufferStarted(RealtimeServerEventOutputAudioBufferStarted),

    /// Returned when the output audio buffer has been completely drained on the server.
    #[serde(rename = "output_audio_buffer.stopped")]
    OutputAudioBufferStopped(RealtimeServerEventOutputAudioBufferStopped),

    /// Returned when the output audio buffer is cleared.
    #[serde(rename = "output_audio_buffer.cleared")]
    OutputAudioBufferCleared(RealtimeServerEventOutputAudioBufferCleared),

    /// Returned when a new Response is created. The first event of response creation.
    #[serde(rename = "response.created")]
    ResponseCreated(RealtimeServerEventResponseCreated),

    /// Returned when a Response is done streaming.
    #[serde(rename = "response.done")]
    ResponseDone(RealtimeServerEventResponseDone),

    /// Returned when a new Item is created during Response generation.
    #[serde(rename = "response.output_item.added")]
    ResponseOutputItemAdded(RealtimeServerEventResponseOutputItemAdded),

    /// Returned when an Item is done streaming during Response generation.
    #[serde(rename = "response.output_item.done")]
    ResponseOutputItemDone(RealtimeServerEventResponseOutputItemDone),

    /// Returned when a new content part is added to an assistant message item.
    #[serde(rename = "response.content_part.added")]
    ResponseContentPartAdded(RealtimeServerEventResponseContentPartAdded),

    /// Returned when a content part is done streaming in an assistant message item.
    #[serde(rename = "response.content_part.done")]
    ResponseContentPartDone(RealtimeServerEventResponseContentPartDone),

    /// Returned when there is a delta update to the output text.
    #[serde(rename = "response.output_text.delta")]
    ResponseTextDelta(RealtimeServerEventResponseTextDelta),

    /// Returned when the output text is done streaming.
    #[serde(rename = "response.output_text.done")]
    ResponseTextDone(RealtimeServerEventResponseTextDone),

    /// Returned when there is a delta update to the output audio transcript.
    #[serde(rename = "response.output_audio_transcript.delta")]
    ResponseAudioTranscriptDelta(RealtimeServerEventResponseAudioTranscriptDelta),

    /// Returned when the output audio transcript is done streaming.
    #[serde(rename = "response.output_audio_transcript.done")]
    ResponseAudioTranscriptDone(RealtimeServerEventResponseAudioTranscriptDone),

    /// Returned when there is a delta update to the output audio.
    #[serde(rename = "response.output_audio.delta")]
    ResponseAudioDelta(RealtimeServerEventResponseAudioDelta),

    /// Returned when the output audio is done streaming.
    #[serde(rename = "response.output_audio.done")]
    ResponseAudioDone(RealtimeServerEventResponseAudioDone),

    /// Returned when there is a delta update to the function call arguments.
    #[serde(rename = "response.function_call_arguments.delta")]
    ResponseFunctionCallArgumentsDelta(RealtimeServerEventResponseFunctionCallArgumentsDelta),

    /// Returned when the function call arguments are done streaming.
    #[serde(rename = "response.function_call_arguments.done")]
    ResponseFunctionCallArgumentsDone(RealtimeServerEventResponseFunctionCallArgumentsDone),

    /// Returned when there is a delta update to the MCP tool call arguments.
    #[serde(rename = "response.mcp_call_arguments.delta")]
    ResponseMCPCallArgumentsDelta(RealtimeServerEventResponseMCPCallArgumentsDelta),

    /// Returned when the MCP tool call arguments are finalized.
    #[serde(rename = "response.mcp_call_arguments.done")]
    ResponseMCPCallArgumentsDone(RealtimeServerEventResponseMCPCallArgumentsDone),

    /// Returned when an MCP tool call is in progress.
    #[serde(rename = "response.mcp_call.in_progress")]
    ResponseMCPCallInProgress(RealtimeServerEventResponseMCPCallInProgress),

    /// Returned when an MCP tool call completes.
    #[serde(rename = "response.mcp_call.completed")]
    ResponseMCPCallCompleted(RealtimeServerEventResponseMCPCallCompleted),

    /// Returned when an MCP tool call fails.
    #[serde(rename = "response.mcp_call.failed")]
    ResponseMCPCallFailed(RealtimeServerEventResponseMCPCallFailed),

    /// Returned when an MCP list tools call is in progress.
    #[serde(rename = "mcp_list_tools.in_progress")]
    MCPListToolsInProgress(RealtimeServerEventMCPListToolsInProgress),

    /// Returned when an MCP list tools call completes.
    #[serde(rename = "mcp_list_tools.completed")]
    MCPListToolsCompleted(RealtimeServerEventMCPListToolsCompleted),

    /// Returned when an MCP list tools call fails.
    #[serde(rename = "mcp_list_tools.failed")]
    MCPListToolsFailed(RealtimeServerEventMCPListToolsFailed),

    /// Emitted at the beginning of a Response to indicate the updated rate limits.
    #[serde(rename = "rate_limits.updated")]
    RateLimitsUpdated(RealtimeServerEventRateLimitsUpdated),

    /// Returned when a conversation is created. Emitted right after session creation.
    #[serde(rename = "conversation.created")]
    ConversationCreated(RealtimeServerEventConversationCreated),

    /// Returned when a conversation item is created.
    #[serde(rename = "conversation.item.created")]
    ConversationItemCreated(RealtimeServerEventConversationItemCreated),
}

/// Payload of the `error` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventError {
    /// Details of the error.
    pub error: RealtimeError,
    /// The unique ID of the server event.
    pub event_id: String,
}

/// The session configuration carried by `session.created` and `session.updated`.
///
/// Either a realtime session or a transcription-only session.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(untagged)]
#[display("{self:?}")]
pub enum RealtimeServerEventSession {
    /// A realtime session configuration.
    Realtime(Box<RealtimeSessionCreateResponse>),
    /// A transcription-only session configuration.
    Transcription(Box<RealtimeTranscriptionSessionCreateResponse>),
}

/// Payload of the `session.created` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventSessionCreated {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The session configuration.
    pub session: RealtimeServerEventSession,
}

/// Payload of the `session.updated` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventSessionUpdated {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The session configuration.
    pub session: RealtimeServerEventSession,
}

/// Payload of the `conversation.item.added` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventConversationItemAdded {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The item that was added.
    pub item: ConversationItem,
    /// The ID of the preceding item after which the new item was inserted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_item_id: Option<String>,
}

/// Payload of the `conversation.item.done` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventConversationItemDone {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The item that is done.
    pub item: ConversationItem,
    /// The ID of the preceding item after which the item was inserted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_item_id: Option<String>,
}

/// Payload of the `conversation.item.retrieved` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventConversationItemRetrieved {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The retrieved item.
    pub item: ConversationItem,
}

/// Payload of the `conversation.item.input_audio_transcription.completed` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionCompleted {
    /// The index of the content part containing the audio.
    pub content_index: u64,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item containing the audio that was transcribed.
    pub item_id: String,
    /// The transcribed text.
    pub transcript: String,
    /// Usage statistics for the transcription, billed according to the ASR
    /// model's pricing rather than the realtime model's pricing.
    pub usage: RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsage,
    /// The languages detected in the audio.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub languages: Vec<TranscriptionLanguage>,
    /// The log probabilities of the transcription.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<Vec<LogProbProperties>>,
}

/// Usage statistics of a completed input audio transcription.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(tag = "type")]
#[display("{self:?}")]
pub enum RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsage {
    /// Usage statistics for models billed by token usage.
    #[serde(rename = "tokens")]
    Tokens(RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsageTokens),
    /// Usage statistics for models billed by audio input duration.
    #[serde(rename = "duration")]
    Duration(RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsageDuration),
}

/// Token-based usage statistics of a completed input audio transcription.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsageTokens {
    /// Number of input tokens billed for this request.
    pub input_tokens: u64,
    /// Number of output tokens generated.
    pub output_tokens: u64,
    /// Total number of tokens used (input + output).
    pub total_tokens: u64,
    /// Details about the input tokens billed for this request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_token_details: Option<
        RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsageTokensInputTokenDetails,
    >,
}

/// Details about the input tokens billed for a transcription request.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsageTokensInputTokenDetails
{
    /// Number of audio tokens billed for this request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio_tokens: Option<u64>,
    /// Number of text tokens billed for this request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_tokens: Option<u64>,
}

/// Duration-based usage statistics of a completed input audio transcription.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionCompletedUsageDuration {
    /// Duration of the input audio in seconds.
    pub seconds: f64,
}

/// A language detected in transcribed audio.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct TranscriptionLanguage {
    /// The code of a language detected in the audio.
    pub code: String,
}

/// Payload of the `conversation.item.input_audio_transcription.delta` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionDelta {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item containing the audio that is being transcribed.
    pub item_id: String,
    /// The index of the content part in the item's content array.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_index: Option<u64>,
    /// The text delta.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delta: Option<String>,
    /// The log probabilities of the transcription.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<Vec<LogProbProperties>>,
}

/// Payload of the `conversation.item.input_audio_transcription.segment` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionSegment {
    /// The segment identifier.
    pub id: String,
    /// The index of the input audio content part within the item.
    pub content_index: u64,
    /// End time of the segment in seconds.
    pub end: f64,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item containing the input audio content.
    pub item_id: String,
    /// The detected speaker label for this segment.
    pub speaker: String,
    /// Start time of the segment in seconds.
    pub start: f64,
    /// The text for this segment.
    pub text: String,
}

/// Payload of the `conversation.item.input_audio_transcription.failed` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionFailed {
    /// The index of the content part containing the audio.
    pub content_index: u64,
    /// Details of the transcription error.
    pub error: RealtimeServerEventConversationItemInputAudioTranscriptionFailedError,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the user message item.
    pub item_id: String,
}

/// Details of a failed input audio transcription.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionFailedError {
    /// Error code, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// A human-readable error message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Parameter related to the error, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
    /// The type of error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}

/// Payload of the `conversation.item.truncated` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventConversationItemTruncated {
    /// The duration up to which the audio was truncated, in milliseconds.
    pub audio_end_ms: u64,
    /// The index of the content part that was truncated.
    pub content_index: u64,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the assistant message item that was truncated.
    pub item_id: String,
}

/// Payload of the `conversation.item.deleted` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventConversationItemDeleted {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item that was deleted.
    pub item_id: String,
}

/// Payload of the `input_audio_buffer.committed` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventInputAudioBufferCommitted {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the user message item that will be created.
    pub item_id: String,
    /// The ID of the preceding item after which the new item will be inserted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_item_id: Option<String>,
}

/// Payload of the `input_audio_buffer.dtmf_event_received` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventInputAudioBufferDtmfEventReceived {
    /// The telephone keypad that was pressed by the user.
    pub event: String,
    /// UTC Unix timestamp when the DTMF event was received by the server.
    pub received_at: u64,
}

/// Payload of the `input_audio_buffer.cleared` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventInputAudioBufferCleared {
    /// The unique ID of the server event.
    pub event_id: String,
}

/// Payload of the `input_audio_buffer.speech_started` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventInputAudioBufferSpeechStarted {
    /// Millisecond offset of audio written to the input audio buffer at the time
    /// the speech started.
    pub audio_start_ms: u64,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item associated with this segment.
    pub item_id: String,
}

/// Payload of the `input_audio_buffer.speech_stopped` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventInputAudioBufferSpeechStopped {
    /// Millisecond offset of audio written to the input audio buffer at the time
    /// the speech stopped.
    pub audio_end_ms: u64,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item associated with this segment.
    pub item_id: String,
}

/// Payload of the `input_audio_buffer.timeout_triggered` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventInputAudioBufferTimeoutTriggered {
    /// Millisecond offset of audio written to the input audio buffer at the time
    /// the timeout was triggered.
    pub audio_end_ms: u64,
    /// Millisecond offset of audio written to the input audio buffer that was
    /// after the playback time of the last model response.
    pub audio_start_ms: u64,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item associated with this segment.
    pub item_id: String,
}

/// Payload of the `output_audio_buffer.started` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventOutputAudioBufferStarted {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The unique ID of the response that produced the audio.
    pub response_id: String,
}

/// Payload of the `output_audio_buffer.stopped` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventOutputAudioBufferStopped {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The unique ID of the response that produced the audio.
    pub response_id: String,
}

/// Payload of the `output_audio_buffer.cleared` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventOutputAudioBufferCleared {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The unique ID of the response that produced the audio.
    pub response_id: String,
}

/// Payload of the `response.created` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventResponseCreated {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The response resource.
    pub response: RealtimeResponse,
}

/// Payload of the `response.done` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventResponseDone {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The response resource.
    pub response: RealtimeResponse,
}

/// Payload of the `response.output_item.added` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventResponseOutputItemAdded {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The item that was added.
    pub item: ConversationItem,
    /// The index of the output item in the response.
    pub output_index: u64,
    /// The ID of the response.
    pub response_id: String,
}

/// Payload of the `response.output_item.done` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventResponseOutputItemDone {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The item that is done.
    pub item: ConversationItem,
    /// The index of the output item in the response.
    pub output_index: u64,
    /// The ID of the response.
    pub response_id: String,
}

/// Payload of the `response.content_part.added` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventResponseContentPartAdded {
    /// The index of the content part in the item's content array.
    pub content_index: u64,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item to which the content part was added.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u64,
    /// The content part that was added.
    pub part: AssistantContentPart,
    /// The ID of the response.
    pub response_id: String,
}

/// Payload of the `response.content_part.done` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventResponseContentPartDone {
    /// The index of the content part in the item's content array.
    pub content_index: u64,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item to which the content part belongs.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u64,
    /// The content part that is done.
    pub part: AssistantContentPart,
    /// The ID of the response.
    pub response_id: String,
}

/// Payload of the `response.output_text.delta` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventResponseTextDelta {
    /// The index of the content part in the item's content array.
    pub content_index: u64,
    /// The text delta.
    pub delta: String,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u64,
    /// The ID of the response.
    pub response_id: String,
}

/// Payload of the `response.output_text.done` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventResponseTextDone {
    /// The index of the content part in the item's content array.
    pub content_index: u64,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u64,
    /// The ID of the response.
    pub response_id: String,
    /// The final text content.
    pub text: String,
}

/// Payload of the `response.output_audio_transcript.delta` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventResponseAudioTranscriptDelta {
    /// The index of the content part in the item's content array.
    pub content_index: u64,
    /// The transcript delta.
    pub delta: String,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u64,
    /// The ID of the response.
    pub response_id: String,
}

/// Payload of the `response.output_audio_transcript.done` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventResponseAudioTranscriptDone {
    /// The index of the content part in the item's content array.
    pub content_index: u64,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u64,
    /// The ID of the response.
    pub response_id: String,
    /// The final transcript of the audio.
    pub transcript: String,
}

/// Payload of the `response.output_audio.delta` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventResponseAudioDelta {
    /// The index of the content part in the item's content array.
    pub content_index: u64,
    /// Base64-encoded audio delta.
    pub delta: String,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u64,
    /// The ID of the response.
    pub response_id: String,
}

/// Payload of the `response.output_audio.done` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventResponseAudioDone {
    /// The index of the content part in the item's content array.
    pub content_index: u64,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u64,
    /// The ID of the response.
    pub response_id: String,
}

/// Payload of the `response.function_call_arguments.delta` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventResponseFunctionCallArgumentsDelta {
    /// The ID of the function call.
    pub call_id: String,
    /// The arguments delta as a JSON string.
    pub delta: String,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the function call item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u64,
    /// The ID of the response.
    pub response_id: String,
}

/// Payload of the `response.function_call_arguments.done` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventResponseFunctionCallArgumentsDone {
    /// The final arguments as a JSON string.
    pub arguments: String,
    /// The ID of the function call.
    pub call_id: String,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the function call item.
    pub item_id: String,
    /// The name of the function that was called.
    pub name: String,
    /// The index of the output item in the response.
    pub output_index: u64,
    /// The ID of the response.
    pub response_id: String,
}

/// Payload of the `response.mcp_call_arguments.delta` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventResponseMCPCallArgumentsDelta {
    /// The JSON-encoded arguments delta.
    pub delta: String,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the MCP tool call item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u64,
    /// The ID of the response.
    pub response_id: String,
    /// If present, indicates the delta text was obfuscated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub obfuscation: Option<String>,
}

/// Payload of the `response.mcp_call_arguments.done` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventResponseMCPCallArgumentsDone {
    /// The final JSON-encoded arguments string.
    pub arguments: String,
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the MCP tool call item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u64,
    /// The ID of the response.
    pub response_id: String,
}

/// Payload of the `response.mcp_call.in_progress` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventResponseMCPCallInProgress {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the MCP tool call item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u64,
}

/// Payload of the `response.mcp_call.completed` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventResponseMCPCallCompleted {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the MCP tool call item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u64,
}

/// Payload of the `response.mcp_call.failed` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventResponseMCPCallFailed {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the MCP tool call item.
    pub item_id: String,
    /// The index of the output item in the response.
    pub output_index: u64,
}

/// Payload of the `mcp_list_tools.in_progress` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventMCPListToolsInProgress {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the MCP list tools item.
    pub item_id: String,
}

/// Payload of the `mcp_list_tools.completed` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventMCPListToolsCompleted {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the MCP list tools item.
    pub item_id: String,
}

/// Payload of the `mcp_list_tools.failed` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventMCPListToolsFailed {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The ID of the MCP list tools item.
    pub item_id: String,
}

/// Payload of the `rate_limits.updated` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventRateLimitsUpdated {
    /// The unique ID of the server event.
    pub event_id: String,
    /// List of rate limit information.
    pub rate_limits: Vec<RateLimit>,
}

/// A single rate limit entry reported by the `rate_limits.updated` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RateLimit {
    /// The maximum allowed value for the rate limit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    /// The name of the rate limit (`requests`, `tokens`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<RateLimitName>,
    /// The remaining value before the limit is reached.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining: Option<u64>,
    /// Seconds until the rate limit resets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reset_seconds: Option<f64>,
}

/// The name of a rate limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub enum RateLimitName {
    /// Request-count rate limit.
    #[serde(rename = "requests")]
    Requests,
    /// Token-count rate limit.
    #[serde(rename = "tokens")]
    Tokens,
}

/// The conversation resource carried by the `conversation.created` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ConversationResource {
    /// The unique ID of the conversation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The object type, must be `realtime.conversation`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
}

/// Payload of the `conversation.created` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventConversationCreated {
    /// The conversation resource.
    pub conversation: ConversationResource,
    /// The unique ID of the server event.
    pub event_id: String,
}

/// Payload of the `conversation.item.created` event.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeServerEventConversationItemCreated {
    /// The unique ID of the server event.
    pub event_id: String,
    /// The item that was created.
    pub item: ConversationItem,
    /// The ID of the preceding item after which the new item was inserted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_item_id: Option<String>,
}
