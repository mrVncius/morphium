//! Realtime translation wire types.
//!
//! Translation sessions continuously translate source audio into target-language
//! audio and transcript deltas. A translation session is created through the
//! client-secret REST endpoint and then driven over the WebSocket with
//! [`RealtimeTranslationClientEvent`] and [`RealtimeTranslationServerEvent`].
//!
//! The event payload structs declared here omit their `type` field: serde
//! supplies it from the `#[serde(tag = "type")]` discriminator of the enclosing
//! union.
//!
//! See: <https://developers.openai.com/api/reference/resources/realtime/domain-types>

use derive_more::{Display, IsVariant};
use serde::{Deserialize, Serialize};

use super::response::RealtimeErrorEvent;
use super::session::NoiseReductionType;

/// Configuration for translation input and output audio.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranslationAudioConfig {
    /// Configuration for the translation input audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<RealtimeTranslationAudioConfigInput>,

    /// Configuration for the translated output audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<RealtimeTranslationAudioConfigOutput>,
}

/// Configuration for the translation input audio.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranslationAudioConfigInput {
    /// Optional input noise reduction. Set to `null` to disable it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noise_reduction: Option<RealtimeTranslationNoiseReduction>,

    /// Optional source-language transcription. When configured, the server emits
    /// `session.input_transcript.delta` events. Translation itself still runs
    /// from the input audio stream.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transcription: Option<RealtimeTranslationTranscription>,
}

/// Input noise reduction of a translation session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranslationNoiseReduction {
    /// Type of noise reduction. `near_field` is for close-talking microphones
    /// such as headphones, `far_field` is for far-field microphones such as
    /// laptop or conference room microphones.
    #[serde(rename = "type")]
    pub type_: NoiseReductionType,
}

/// Source-language transcription of a translation session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranslationTranscription {
    /// The transcription model used for source transcript deltas.
    pub model: String,
}

/// Configuration for the translated output audio.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranslationAudioConfigOutput {
    /// Target language for translated output audio and transcript deltas.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// Realtime translation session configuration.
///
/// Translation sessions stream source audio in and translated audio plus
/// transcript deltas out continuously.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranslationSessionCreateRequest {
    /// The Realtime translation model used for this session.
    pub model: String,

    /// Configuration for translation input and output audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<RealtimeTranslationAudioConfig>,
}

/// Translation session fields to update.
///
/// The session `type` and `model` are set at creation and cannot be changed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranslationSessionUpdateRequest {
    /// Configuration for translation input and output audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<RealtimeTranslationAudioConfig>,
}

/// A Realtime translation session.
///
/// Translation sessions continuously translate input audio into a target
/// language, emitting translated audio and transcript deltas.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranslationSession {
    /// Unique identifier for the session that looks like `sess_1234567890abcdef`.
    pub id: String,

    /// Configuration for translation input and output audio.
    pub audio: RealtimeTranslationAudioConfig,

    /// Expiration timestamp for the session, in seconds since epoch.
    pub expires_at: u64,

    /// The Realtime translation model used for this session. This field is set
    /// at session creation and cannot be changed with `session.update`.
    pub model: String,

    /// The session type. Always `translation` for Realtime translation sessions.
    #[serde(rename = "type")]
    pub type_: String,
}

/// Configuration for the translation client secret expiration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranslationClientSecretCreateRequestExpiresAfter {
    /// The anchor point for the client secret expiration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<RealtimeTranslationExpiresAfterAnchor>,

    /// The number of seconds from the anchor point to the expiration. Select a
    /// value between `10` and `7200` (2 hours). This defaults to 600 seconds
    /// (10 minutes) if not specified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seconds: Option<u64>,
}

/// The anchor point for a client secret expiration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub enum RealtimeTranslationExpiresAfterAnchor {
    /// `seconds` are added to the `created_at` time of the client secret to
    /// produce an expiration timestamp. This is the only supported anchor.
    #[serde(rename = "created_at")]
    CreatedAt,
}

/// Create a translation session and client secret for the Realtime API.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranslationClientSecretCreateRequest {
    /// Realtime translation session configuration.
    pub session: RealtimeTranslationSessionCreateRequest,

    /// Configuration for the client secret expiration. Expiration refers to the
    /// time after which a client secret will no longer be valid for creating
    /// sessions. The session itself may continue after that time once started.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<RealtimeTranslationClientSecretCreateRequestExpiresAfter>,
}

/// Response from creating a translation session and client secret.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranslationClientSecretCreateResponse {
    /// Expiration timestamp for the client secret, in seconds since epoch.
    pub expires_at: u64,

    /// The translation session the client secret was created for.
    pub session: RealtimeTranslationSession,

    /// The client secret value.
    pub value: String,
}

/// Payload for the `session.update` translation client event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranslationSessionUpdateEvent {
    /// Translation session fields to update. The session `type` and `model` are
    /// set at creation and cannot be changed.
    pub session: RealtimeTranslationSessionUpdateRequest,

    /// Optional client-generated ID used to identify this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

/// Payload for the `session.input_audio_buffer.append` translation client event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranslationInputAudioBufferAppendEvent {
    /// Base64-encoded audio bytes. Translation consumes 200 ms engine frames.
    pub audio: String,

    /// Optional client-generated ID used to identify this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

/// Payload for the `session.close` translation client event.
///
/// The server flushes pending input audio and emits any remaining translated
/// output before closing the session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranslationSessionCloseEvent {
    /// Optional client-generated ID used to identify this event.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

/// Payload for the `session.created` translation server event.
///
/// Emitted automatically when a new connection is established as the first
/// server event, and contains the default translation session configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranslationSessionCreatedEvent {
    /// The unique ID of the server event.
    pub event_id: String,

    /// The translation session configuration.
    pub session: RealtimeTranslationSession,
}

/// Payload for the `session.updated` translation server event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranslationSessionUpdatedEvent {
    /// The unique ID of the server event.
    pub event_id: String,

    /// The translation session configuration.
    pub session: RealtimeTranslationSession,
}

/// Payload for the `session.closed` translation server event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranslationSessionClosedEvent {
    /// The unique ID of the server event.
    pub event_id: String,
}

/// Payload for the `session.input_transcript.delta` translation server event.
///
/// Emitted only when `audio.input.transcription` is configured. Transcript
/// deltas are append-only text fragments; clients should not insert
/// unconditional spaces between deltas.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranslationInputTranscriptDeltaEvent {
    /// Append-only source-language transcript text.
    pub delta: String,

    /// The unique ID of the server event.
    pub event_id: String,

    /// Timing metadata for stream alignment, derived from the translation frame
    /// when available. It advances in 200 ms increments, but multiple transcript
    /// deltas may share the same `elapsed_ms`. Treat it as alignment metadata,
    /// not a unique transcript-delta identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elapsed_ms: Option<f64>,
}

/// Payload for the `session.output_transcript.delta` translation server event.
///
/// Transcript deltas are append-only text fragments; clients should not insert
/// unconditional spaces between deltas.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranslationOutputTranscriptDeltaEvent {
    /// Append-only transcript text for the translated output audio.
    pub delta: String,

    /// The unique ID of the server event.
    pub event_id: String,

    /// Timing metadata for stream alignment, derived from the translation frame
    /// when available. It advances in 200 ms increments, but multiple transcript
    /// deltas may share the same `elapsed_ms`. Treat it as alignment metadata,
    /// not a unique transcript-delta identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elapsed_ms: Option<f64>,
}

/// Audio encoding of a translation output audio delta.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub enum RealtimeTranslationOutputAudioDeltaFormat {
    /// 16-bit PCM audio.
    #[serde(rename = "pcm16")]
    Pcm16,
}

/// Payload for the `session.output_audio.delta` translation server event.
///
/// The `delta` contains a PCM16 audio chunk whose length can vary. Clients
/// should decode and queue the complete delta instead of assuming a fixed byte
/// or sample count.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranslationOutputAudioDeltaEvent {
    /// Base64-encoded translated audio data.
    pub delta: String,

    /// The unique ID of the server event.
    pub event_id: String,

    /// Number of audio channels.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channels: Option<u32>,

    /// Timing metadata for stream alignment, derived from the translation frame
    /// when available. Treat `elapsed_ms` as alignment metadata, not a unique
    /// event identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elapsed_ms: Option<f64>,

    /// Audio encoding for `delta`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<RealtimeTranslationOutputAudioDeltaFormat>,

    /// Sample rate of the audio delta.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<u32>,
}

/// An event sent from the client to the Realtime translation WebSocket server.
///
/// Discriminated on the `type` JSON key, whose values contain dots.
#[derive(Debug, Clone, Serialize, Deserialize, Display, IsVariant)]
#[serde(tag = "type")]
#[display("{self:?}")]
pub enum RealtimeTranslationClientEvent {
    /// Update the translation session's configuration (`session.update`).
    #[serde(rename = "session.update")]
    SessionUpdate(RealtimeTranslationSessionUpdateEvent),

    /// Append audio bytes to the input audio buffer
    /// (`session.input_audio_buffer.append`).
    #[serde(rename = "session.input_audio_buffer.append")]
    InputAudioBufferAppend(RealtimeTranslationInputAudioBufferAppendEvent),

    /// Gracefully close the translation session (`session.close`).
    #[serde(rename = "session.close")]
    SessionClose(RealtimeTranslationSessionCloseEvent),
}

/// An event sent from the Realtime translation WebSocket server to the client.
///
/// Discriminated on the `type` JSON key, whose values contain dots.
#[derive(Debug, Clone, Serialize, Deserialize, Display, IsVariant)]
#[serde(tag = "type")]
#[display("{self:?}")]
pub enum RealtimeTranslationServerEvent {
    /// Returned when an error occurs, which could be a client problem or a
    /// server problem (`error`).
    #[serde(rename = "error")]
    Error(RealtimeErrorEvent),

    /// Returned when a translation session is created (`session.created`).
    #[serde(rename = "session.created")]
    SessionCreated(RealtimeTranslationSessionCreatedEvent),

    /// Returned when a translation session is updated with a `session.update`
    /// event, unless there is an error (`session.updated`).
    #[serde(rename = "session.updated")]
    SessionUpdated(RealtimeTranslationSessionUpdatedEvent),

    /// Returned when a realtime translation session is closed (`session.closed`).
    #[serde(rename = "session.closed")]
    SessionClosed(RealtimeTranslationSessionClosedEvent),

    /// Returned when optional source-language transcript text is available
    /// (`session.input_transcript.delta`).
    #[serde(rename = "session.input_transcript.delta")]
    InputTranscriptDelta(RealtimeTranslationInputTranscriptDeltaEvent),

    /// Returned when translated transcript text is available
    /// (`session.output_transcript.delta`).
    #[serde(rename = "session.output_transcript.delta")]
    OutputTranscriptDelta(RealtimeTranslationOutputTranscriptDeltaEvent),

    /// Returned when translated output audio is available
    /// (`session.output_audio.delta`).
    #[serde(rename = "session.output_audio.delta")]
    OutputAudioDelta(RealtimeTranslationOutputAudioDeltaEvent),
}
