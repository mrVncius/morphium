//! REST wire types for the OpenAI Realtime API.
//!
//! Covers the session, transcription-session, and client-secret endpoints:
//! `POST /realtime/sessions`, `POST /realtime/transcription_sessions`, and
//! `POST /realtime/client_secrets`.
//!
//! See: <https://developers.openai.com/api/reference/resources/realtime>

use derive_more::Display;
use serde::{Deserialize, Serialize};

use super::prompt::ResponsePrompt;
use super::response::ResponseMaxOutputTokens;
use super::session::{
    AudioTranscription, Modality, NoiseReduction, RealtimeAudioConfig, RealtimeAudioConfigOutput,
    RealtimeAudioFormats, RealtimeReasoning, RealtimeSessionCreateRequest, RealtimeToolsConfig,
    RealtimeTracingConfig, RealtimeTranscriptionSessionCreateRequest,
    RealtimeTranscriptionSessionTurnDetection, RealtimeTruncation, SessionInclude, Voice,
};
use super::tool::{RealtimeFunctionTool, RealtimeToolChoiceConfig};

/// The default value of [`RealtimeSessionCreateResponse::type_`].
fn realtime_type() -> String {
    "realtime".to_owned()
}

/// The default value of [`RealtimeSessionCreateResponse::object`].
fn realtime_session_object() -> String {
    "realtime.session".to_owned()
}

/// The default value of [`RealtimeTranscriptionSessionCreateResponse::type_`].
fn transcription_type() -> String {
    "transcription".to_owned()
}

/// The default value of
/// [`RealtimeTranscriptionSessionCreateResponse::object`].
fn realtime_transcription_session_object() -> String {
    "realtime.transcription_session".to_owned()
}

/// An ephemeral client secret returned by the API.
///
/// A short-lived token that can be passed to a client application, such as a
/// web frontend or mobile client, to grant access to the Realtime API without
/// leaking the main API key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ClientSecret {
    /// Timestamp for when the token expires, in seconds since the Unix epoch.
    pub expires_at: u64,

    /// Ephemeral key usable in client environments to authenticate
    /// connections to the Realtime API.
    pub value: String,
}

/// Body of the create-session request (`POST /realtime/sessions`).
///
/// Creates an ephemeral API token that can be used to authenticate browser
/// clients for the Realtime API, configured with the same session parameters
/// as the WebSocket `session.update` client event.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct SessionCreateRequest {
    /// Ephemeral key returned by the API.
    pub client_secret: ClientSecret,

    /// The format of input audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_audio_format: Option<InputAudioFormat>,

    /// Configuration for input audio transcription, defaults to off.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_audio_transcription: Option<AudioTranscription>,

    /// The default system instructions prepended to model calls.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// Maximum number of output tokens for a single assistant response,
    /// inclusive of tool calls, or `inf` for the maximum available tokens for
    /// the given model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_response_output_tokens: Option<ResponseMaxOutputTokens>,

    /// The set of modalities the model can respond with.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modalities: Vec<Modality>,

    /// The format of output audio.
    ///
    /// Carries the same value set as [`InputAudioFormat`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_audio_format: Option<InputAudioFormat>,

    /// Reference to a prompt template and its variables.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<ResponsePrompt>,

    /// The speed of the generated audio, between `0.25` and `1.5`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>,

    /// Sampling temperature for the model, between `0.6` and `1.2`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// How the model chooses tools: `auto`, `none`, or `required`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<String>,

    /// The tools available to the model.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<RealtimeFunctionTool>,

    /// Configuration for tracing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracing: Option<RealtimeTracingConfig>,

    /// Controls how the realtime conversation is truncated prior to model
    /// inference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub truncation: Option<RealtimeTruncation>,

    /// Configuration for turn detection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<RealtimeTranscriptionSessionTurnDetection>,

    /// The voice the model uses to respond.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,
}

/// Audio configuration of a [`SessionCreateResponse`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct SessionCreateResponseAudio {
    /// Configuration for input audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<SessionCreateResponseAudioInput>,

    /// Configuration for output audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<RealtimeAudioConfigOutput>,
}

/// Input audio configuration of a [`SessionCreateResponse`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct SessionCreateResponseAudioInput {
    /// The format of the input audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<RealtimeAudioFormats>,

    /// Configuration for input audio noise reduction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noise_reduction: Option<NoiseReduction>,

    /// Configuration for input audio transcription.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transcription: Option<AudioTranscription>,

    /// Configuration for turn detection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<RealtimeTranscriptionSessionTurnDetection>,
}

/// A Realtime session configuration object.
///
/// Returned by `POST /realtime/sessions`, alongside the ephemeral
/// [`ClientSecret`] used to authenticate client connections.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct SessionCreateResponse {
    /// Unique identifier for the session that looks like
    /// `sess_1234567890abcdef`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Configuration for input and output audio for the session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<SessionCreateResponseAudio>,

    /// Timestamp for when the session expires, in seconds since the Unix
    /// epoch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u64>,

    /// Additional fields to include in server outputs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<SessionInclude>,

    /// The default system instructions prepended to model calls.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// Maximum number of output tokens for a single assistant response,
    /// inclusive of tool calls, or `inf` for the maximum available tokens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<ResponseMaxOutputTokens>,

    /// The model used for this session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// The object type. Always `realtime.session`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,

    /// The set of modalities the model can respond with.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub output_modalities: Vec<Modality>,

    /// How the model chooses tools.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<String>,

    /// The tools available to the model.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<RealtimeFunctionTool>,

    /// Configuration for tracing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracing: Option<RealtimeTracingConfig>,

    /// Configuration for turn detection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<RealtimeTranscriptionSessionTurnDetection>,
}

/// A Realtime session configuration object.
///
/// This is the session shape delivered in the WebSocket `session.created`
/// event, and attached to a created client secret.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeSessionCreateResponse {
    /// Unique identifier for the session that looks like
    /// `sess_1234567890abcdef`.
    pub id: String,

    /// The object type. Always `realtime.session`.
    #[serde(default = "realtime_session_object")]
    pub object: String,

    /// The type of session. Always `realtime` for the Realtime API.
    #[serde(rename = "type", default = "realtime_type")]
    pub type_: String,

    /// Configuration for input and output audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<RealtimeAudioConfig>,

    /// Timestamp for when the session expires, in seconds since the Unix
    /// epoch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u64>,

    /// Additional fields to include in server outputs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<SessionInclude>>,

    /// The default system instructions prepended to model calls.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// Maximum number of output tokens for a single assistant response,
    /// inclusive of tool calls, or `inf` for the maximum available tokens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<ResponseMaxOutputTokens>,

    /// The model used for this session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// The set of modalities the model can respond with.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub output_modalities: Vec<Modality>,

    /// Reference to a prompt template and its variables.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<ResponsePrompt>,

    /// Configuration for model reasoning.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<RealtimeReasoning>,

    /// How the model chooses tools.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<RealtimeToolChoiceConfig>,

    /// The tools available to the model.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: RealtimeToolsConfig,

    /// Configuration for tracing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracing: Option<RealtimeTracingConfig>,

    /// Controls how the realtime conversation is truncated prior to model
    /// inference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub truncation: Option<RealtimeTruncation>,
}

/// Audio configuration of a
/// [`RealtimeTranscriptionSessionCreateResponse`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranscriptionSessionCreateResponseAudio {
    /// Configuration for input audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<RealtimeTranscriptionSessionCreateResponseAudioInput>,
}

/// Input audio configuration of a
/// [`RealtimeTranscriptionSessionCreateResponse`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranscriptionSessionCreateResponseAudioInput {
    /// The format of the input audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<RealtimeAudioFormats>,

    /// Configuration for input audio noise reduction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noise_reduction: Option<NoiseReduction>,

    /// Configuration for input audio transcription.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transcription: Option<AudioTranscription>,

    /// Configuration for turn detection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<RealtimeTranscriptionSessionTurnDetection>,
}

/// A Realtime transcription session configuration object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranscriptionSessionCreateResponse {
    /// Unique identifier for the session that looks like
    /// `sess_1234567890abcdef`.
    pub id: String,

    /// The object type. Always `realtime.transcription_session`.
    #[serde(default = "realtime_transcription_session_object")]
    pub object: String,

    /// The type of session. Always `transcription` for transcription
    /// sessions.
    #[serde(rename = "type", default = "transcription_type")]
    pub type_: String,

    /// Configuration for input audio for the session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<RealtimeTranscriptionSessionCreateResponseAudio>,

    /// Timestamp for when the session expires, in seconds since the Unix
    /// epoch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u64>,

    /// Additional fields to include in server outputs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<SessionInclude>>,
}

/// Body of the create-transcription-session request
/// (`POST /realtime/transcription_sessions`).
///
/// Creates a transcription session that streams transcriptions of the input
/// audio without generating model responses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct TranscriptionSessionCreateRequest {
    /// Additional fields to include in server outputs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<SessionInclude>,

    /// The format of input audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_audio_format: Option<InputAudioFormat>,

    /// Configuration for input audio noise reduction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_audio_noise_reduction: Option<NoiseReduction>,

    /// Configuration for input audio transcription.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_audio_transcription: Option<AudioTranscription>,

    /// Configuration for turn detection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<RealtimeTranscriptionSessionTurnDetection>,
}

/// A new Realtime transcription session configuration.
///
/// When a session is created on the server via the REST API, the session
/// object also contains an ephemeral key. This property is not present when a
/// session is updated via the WebSocket API.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct TranscriptionSessionCreateResponse {
    /// Ephemeral key returned by the API. Only present when the session is
    /// created on the server via the REST API.
    pub client_secret: ClientSecret,

    /// The format of input audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_audio_format: Option<InputAudioFormat>,

    /// Configuration for input audio transcription.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_audio_transcription: Option<AudioTranscription>,

    /// The set of modalities the model can respond with.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modalities: Vec<Modality>,

    /// Configuration for turn detection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<RealtimeTranscriptionSessionTurnDetection>,
}

/// Configuration for a client secret's expiration.
///
/// Expiration refers to the time after which a client secret will no longer be
/// valid for creating sessions; the session itself may continue after that
/// time once started.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ClientSecretExpiresAfter {
    /// The anchor point for the client secret expiration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<ClientSecretExpiresAfterAnchor>,

    /// The number of seconds from the anchor point to the expiration, between
    /// `10` and `7200`. Defaults to 600 seconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seconds: Option<u64>,
}

/// The anchor point for a client secret expiration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub enum ClientSecretExpiresAfterAnchor {
    /// Seconds are added to the `created_at` time of the client secret to
    /// produce an expiration timestamp.
    #[serde(rename = "created_at")]
    CreatedAt,
}

/// The session configuration attached to a client secret.
///
/// Choose either a realtime session or a transcription session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[serde(untagged)]
#[display("{self:?}")]
pub enum ClientSecretCreateRequestSession {
    /// Configuration for a realtime session.
    Realtime(Box<RealtimeSessionCreateRequest>),

    /// Configuration for a transcription session.
    Transcription(Box<RealtimeTranscriptionSessionCreateRequest>),
}

/// Body of the create-client-secret request
/// (`POST /realtime/client_secrets`).
///
/// Creates a short-lived token that grants access to the Realtime API without
/// leaking the main API key, with a session configuration that is applied to
/// any sessions created using that token.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ClientSecretCreateRequest {
    /// Configuration for the client secret expiration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_after: Option<ClientSecretExpiresAfter>,

    /// Session configuration to use for the client secret.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<ClientSecretCreateRequestSession>,
}

/// The session returned with a created client secret.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[serde(untagged)]
#[display("{self:?}")]
pub enum ClientSecretSession {
    /// A realtime session.
    Realtime(Box<RealtimeSessionCreateResponse>),

    /// A transcription session.
    Transcription(Box<RealtimeTranscriptionSessionCreateResponse>),
}

/// A created client secret and its effective session object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ClientSecretCreateResponse {
    /// Timestamp for when the client secret expires, in seconds since the Unix
    /// epoch.
    pub expires_at: u64,

    /// The session configuration attached to the client secret.
    pub session: ClientSecretSession,

    /// The client secret value, a string that looks like `ek_1234`.
    pub value: String,
}

/// The format of a raw audio stream.
///
/// Used by the legacy flat session shapes, which carry the audio format as a
/// plain string rather than a [`RealtimeAudioFormats`] object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub enum InputAudioFormat {
    /// 16-bit signed little-endian PCM audio.
    #[serde(rename = "pcm16")]
    Pcm16,

    /// G.711 μ-law audio.
    #[serde(rename = "g711_ulaw")]
    G711Ulaw,

    /// G.711 A-law audio.
    #[serde(rename = "g711_alaw")]
    G711Alaw,
}
