//! Realtime session configuration types.
//!
//! Covers the session object and the session-update request: audio
//! configuration, audio formats, turn detection, noise reduction, input audio
//! transcription, truncation, tracing, and reasoning — plus the transcription
//! session variant used by transcription-only sessions.
//!
//! See: <https://developers.openai.com/api/reference/resources/realtime>

use derive_more::Display;
use serde::{Deserialize, Serialize};

use super::prompt::ResponsePrompt;
use super::response::ResponseMaxOutputTokens;
use super::rest::InputAudioFormat;
use super::tool::{RealtimeFunctionTool, RealtimeToolChoiceConfig, RealtimeToolsConfigUnion};

/// The `tools` configuration of a session: the list of function and MCP tool
/// definitions the model may call.
pub type RealtimeToolsConfig = Vec<RealtimeToolsConfigUnion>;

/// The session created by the server, using the older flat session fields.
///
/// This is the shape carried by the `session.created` server event of the beta
/// interface, where audio formats, truncation, and tool choice are top-level
/// fields rather than nested under `audio`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeSession {
    /// Unique identifier for the session that looks like `sess_1234567890abcdef`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Expiration timestamp for the session, in seconds since epoch.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u64>,

    /// Additional fields to include in the session's server events, such as
    /// `item.input_audio_transcription.logprobs`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<SessionInclude>>,

    /// The format of the input audio: `pcm16`, `g711_ulaw`, or `g711_alaw`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_audio_format: Option<InputAudioFormat>,

    /// Configuration for input audio noise reduction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_audio_noise_reduction: Option<NoiseReduction>,

    /// Configuration for input audio transcription.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_audio_transcription: Option<AudioTranscription>,

    /// The default system instructions prepended to model calls.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// Maximum number of output tokens for a single assistant response, or the
    /// string `"inf"` for no limit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_response_output_tokens: Option<ResponseMaxOutputTokens>,

    /// The set of modalities the model can respond with: `text` and/or `audio`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<Modality>>,

    /// The Realtime model used for this session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// The object type. Always `realtime.session`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,

    /// The format of the output audio: `pcm16`, `g711_ulaw`, or `g711_alaw`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_audio_format: Option<InputAudioFormat>,

    /// The prompt template used to generate the session's instructions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<ResponsePrompt>,

    /// The speed of the model's spoken response, as a multiple of the original
    /// speed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>,

    /// Sampling temperature for the model, between 0.6 and 1.2.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// How the model chooses tools.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<String>,

    /// Tools available to the model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<RealtimeFunctionTool>>,

    /// Configuration for tracing of the session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracing: Option<RealtimeTracingConfig>,

    /// Configuration for turn detection, or `null` to turn it off.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<RealtimeAudioInputTurnDetection>,

    /// The voice the model uses to respond.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,
}

/// Realtime session configuration sent by the client when creating or updating
/// a session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeSessionCreateRequest {
    /// The type of session to create. Always `realtime` for the Realtime API.
    #[serde(rename = "type")]
    pub type_: String,

    /// Configuration for input and output audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<RealtimeAudioConfig>,

    /// Additional fields to include in the session's server events, such as
    /// `item.input_audio_transcription.logprobs`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<SessionInclude>>,

    /// The default system instructions prepended to model calls.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// Maximum number of output tokens for a single assistant response, or the
    /// string `"inf"` for no limit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<ResponseMaxOutputTokens>,

    /// The Realtime model used for this session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// The set of modalities the model can respond with: `text` and/or `audio`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_modalities: Option<Vec<Modality>>,

    /// Whether the model may call multiple tools in parallel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,

    /// The prompt template used to generate the session's instructions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<ResponsePrompt>,

    /// Configuration for reasoning-capable Realtime models.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<RealtimeReasoning>,

    /// How the model chooses tools.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<RealtimeToolChoiceConfig>,

    /// Tools available to the model.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tools: Option<RealtimeToolsConfig>,

    /// Configuration for tracing of the session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracing: Option<RealtimeTracingConfig>,

    /// How the conversation is truncated when it exceeds the model's input
    /// token limit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub truncation: Option<RealtimeTruncation>,
}

/// Configuration for input and output audio of a session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeAudioConfig {
    /// Configuration for the input audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<RealtimeAudioConfigInput>,

    /// Configuration for the output audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<RealtimeAudioConfigOutput>,
}

/// Configuration for the input audio of a session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeAudioConfigInput {
    /// The format of the input audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<RealtimeAudioFormats>,

    /// Configuration for input audio noise reduction. Set to `null` to turn off.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noise_reduction: Option<NoiseReduction>,

    /// Configuration for input audio transcription. Defaults to off, and can be
    /// set to `null` to turn it off once on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transcription: Option<AudioTranscription>,

    /// Configuration for turn detection, or `null` to turn it off.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<RealtimeAudioInputTurnDetection>,
}

/// Configuration for the output audio of a session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeAudioConfigOutput {
    /// The format of the output audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<RealtimeAudioFormats>,

    /// The speed of the model's spoken response, as a multiple of the original
    /// speed. `1.0` is the default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>,

    /// The voice the model uses to respond.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,
}

/// The audio format of a session, discriminated on the `type` JSON key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(tag = "type")]
#[display("{self:?}")]
pub enum RealtimeAudioFormats {
    /// 24 kHz PCM audio, `audio/pcm`.
    #[serde(rename = "audio/pcm")]
    Pcm(PcmAudio),

    /// G.711 μ-law audio, `audio/pcmu`.
    #[serde(rename = "audio/pcmu")]
    Pcmu(PcmuAudio),

    /// G.711 A-law audio, `audio/pcma`.
    #[serde(rename = "audio/pcma")]
    Pcma(PcmaAudio),
}

/// The PCM audio format. Only a 24 kHz sample rate is supported.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct PcmAudio {
    /// The sample rate of the audio. Always `24000`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<u32>,

    /// The audio format. Always `audio/pcm`.
    ///
    /// Carried by the `type` tag of [`RealtimeAudioFormats::Pcm`], so it is not
    /// emitted again here.
    #[serde(rename = "type", default = "pcm_audio_type", skip_serializing)]
    pub type_: String,
}

/// The G.711 μ-law format.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct PcmuAudio {
    /// The audio format. Always `audio/pcmu`.
    ///
    /// Carried by the `type` tag of [`RealtimeAudioFormats::Pcmu`], so it is not
    /// emitted again here.
    #[serde(rename = "type", default = "pcmu_audio_type", skip_serializing)]
    pub type_: String,
}

/// The G.711 A-law format.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct PcmaAudio {
    /// The audio format. Always `audio/pcma`.
    ///
    /// Carried by the `type` tag of [`RealtimeAudioFormats::Pcma`], so it is not
    /// emitted again here.
    #[serde(rename = "type", default = "pcma_audio_type", skip_serializing)]
    pub type_: String,
}

/// Turn detection configuration, either Server VAD or Semantic VAD.
///
/// Set to `null` to disable turn detection, in which case the client must
/// trigger model responses manually.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[serde(tag = "type")]
#[display("{self:?}")]
pub enum RealtimeAudioInputTurnDetection {
    /// Server-side voice activity detection based on audio volume.
    #[serde(rename = "server_vad")]
    ServerVad(ServerVad),

    /// Server-side semantic turn detection, which uses a model to determine
    /// when the user has finished speaking.
    #[serde(rename = "semantic_vad")]
    SemanticVad(SemanticVad),
}

/// Server-side voice activity detection (VAD), which flips on when user speech
/// is detected and off after a period of silence.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ServerVad {
    /// Whether to automatically generate a response when a VAD stop event
    /// occurs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_response: Option<bool>,

    /// Optional timeout after which a model response is triggered
    /// automatically, in milliseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idle_timeout_ms: Option<u64>,

    /// Whether to automatically interrupt any ongoing response when a VAD start
    /// event occurs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interrupt_response: Option<bool>,

    /// Amount of audio to include before the VAD-detected speech, in
    /// milliseconds. Defaults to 300 ms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix_padding_ms: Option<u64>,

    /// Duration of silence that ends speech detection, in milliseconds.
    /// Defaults to 500 ms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub silence_duration_ms: Option<u64>,

    /// Activation threshold for VAD (0.0 to 1.0). Defaults to 0.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f32>,
}

/// Server-side semantic turn detection, which uses a model to determine when
/// the user has finished speaking.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct SemanticVad {
    /// Whether to automatically generate a response when a VAD stop event
    /// occurs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_response: Option<bool>,

    /// How eagerly the model responds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eagerness: Option<SemanticEagerness>,

    /// Whether to automatically interrupt any ongoing response when a VAD start
    /// event occurs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interrupt_response: Option<bool>,
}

/// The eagerness of the model to respond in `semantic_vad` mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum SemanticEagerness {
    /// Wait longer for the user to continue speaking.
    #[serde(rename = "low")]
    Low,

    /// The default eagerness.
    #[serde(rename = "medium")]
    Medium,

    /// Respond more quickly.
    #[serde(rename = "high")]
    High,

    /// Equivalent to `medium`.
    #[serde(rename = "auto")]
    Auto,
}

/// The type of input audio noise reduction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum NoiseReductionType {
    /// For close-talking microphones such as headphones.
    #[serde(rename = "near_field")]
    NearField,

    /// For far-field microphones such as laptop or conference room microphones.
    #[serde(rename = "far_field")]
    FarField,
}

/// Configuration for input audio noise reduction.
///
/// Noise reduction filters audio added to the input audio buffer before it is
/// sent to VAD and the model, which can improve VAD accuracy and model
/// perception of the input audio. Set to `null` to turn it off.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct NoiseReduction {
    /// The type of noise reduction.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<NoiseReductionType>,
}

/// Configuration for input audio transcription.
///
/// Transcription runs asynchronously through the `/audio/transcriptions`
/// endpoint and should be treated as guidance of the input audio content rather
/// than as precisely what the model heard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct AudioTranscription {
    /// How long the model waits before emitting transcription text.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<TranscriptionDelay>,

    /// Keywords to bias the transcription towards.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,

    /// The language of the input audio, in ISO-639-1 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// The possible input audio languages configured for transcription, in
    /// ISO-639-1 format.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,

    /// The model used for transcription.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,

    /// An optional text prompt to guide the transcription.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
}

/// How long the model waits before emitting transcription text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum TranscriptionDelay {
    /// Emit transcription text as soon as possible.
    #[serde(rename = "minimal")]
    Minimal,

    /// Low transcription delay.
    #[serde(rename = "low")]
    Low,

    /// Medium transcription delay.
    #[serde(rename = "medium")]
    Medium,

    /// High transcription delay.
    #[serde(rename = "high")]
    High,

    /// Highest transcription delay.
    #[serde(rename = "xhigh")]
    Xhigh,
}

/// How the conversation is truncated when it exceeds the model's input token
/// limit.
#[derive(Debug, Clone, PartialEq, Display)]
#[display("{self:?}")]
pub enum RealtimeTruncation {
    /// Truncate at the model's default token limits (`"auto"`). This is the
    /// default strategy.
    Auto,

    /// Never truncate; the server errors when the conversation exceeds the
    /// model's input token limit (`"disabled"`).
    Disabled,

    /// Retain a fraction of the conversation tokens when the conversation
    /// exceeds the input token limit.
    RetentionRatio(RetentionRatioTruncation),
}

impl Serialize for RealtimeTruncation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Auto => serializer.serialize_str("auto"),
            Self::Disabled => serializer.serialize_str("disabled"),
            Self::RetentionRatio(truncation) => truncation.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for RealtimeTruncation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        /// The wire representation: a strategy string or a retention-ratio
        /// object.
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Repr {
            /// `"auto"` or `"disabled"`.
            Strategy(String),
            /// A retention-ratio truncation object.
            RetentionRatio(RetentionRatioTruncation),
        }

        match Repr::deserialize(deserializer)? {
            Repr::Strategy(strategy) => match strategy.as_str() {
                "auto" => Ok(Self::Auto),
                "disabled" => Ok(Self::Disabled),
                other => Err(serde::de::Error::unknown_variant(
                    other,
                    &["auto", "disabled", "retention_ratio"],
                )),
            },
            Repr::RetentionRatio(truncation) => Ok(Self::RetentionRatio(truncation)),
        }
    }
}

/// Retain a fraction of the conversation tokens when the conversation exceeds
/// the model's input token limit.
///
/// This amortizes truncations across multiple turns, which improves cached
/// token usage.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RetentionRatioTruncation {
    /// Fraction of post-instruction conversation tokens to retain (`0.0` to
    /// `1.0`) when the conversation exceeds the input token limit.
    pub retention_ratio: f32,

    /// The truncation strategy. Always `retention_ratio`.
    #[serde(rename = "type", default = "retention_ratio_type")]
    pub type_: String,

    /// Optional custom token limits for this truncation strategy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_limits: Option<TruncationTokenLimits>,
}

/// Custom token limits for a truncation strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct TruncationTokenLimits {
    /// Maximum tokens allowed in the conversation after instructions, which
    /// include tool definitions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_instructions: Option<u64>,
}

/// Configuration for writing session traces to the Traces Dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(untagged)]
#[display("{self:?}")]
pub enum RealtimeTracingConfig {
    /// Enable tracing with default values for the workflow name, group id, and
    /// metadata. Always `auto`.
    Auto(String),

    /// Granular configuration for tracing.
    Configuration(TracingConfiguration),
}

/// Granular configuration for tracing of a session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct TracingConfiguration {
    /// The group id to attach to this trace, for filtering and grouping in the
    /// Traces Dashboard.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,

    /// Arbitrary metadata to attach to this trace, for filtering in the Traces
    /// Dashboard.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,

    /// The name of the workflow to attach to this trace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow_name: Option<String>,
}

/// Configuration for reasoning-capable Realtime models such as
/// `gpt-realtime-2`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeReasoning {
    /// Constrains effort on reasoning for reasoning-capable Realtime models.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effort: Option<RealtimeReasoningEffort>,
}

/// How much effort a reasoning-capable Realtime model spends on reasoning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum RealtimeReasoningEffort {
    /// Minimal reasoning effort.
    #[serde(rename = "minimal")]
    Minimal,

    /// Low reasoning effort.
    #[serde(rename = "low")]
    Low,

    /// Medium reasoning effort.
    #[serde(rename = "medium")]
    Medium,

    /// High reasoning effort.
    #[serde(rename = "high")]
    High,

    /// Highest reasoning effort.
    #[serde(rename = "xhigh")]
    Xhigh,
}

/// A modality the session or response may use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum Modality {
    /// Text output.
    #[serde(rename = "text")]
    Text,

    /// Audio output.
    #[serde(rename = "audio")]
    Audio,
}

/// The voice the model uses to respond: either a built-in or custom voice name,
/// or a reference to a custom voice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(untagged)]
#[display("{self:?}")]
pub enum Voice {
    /// A built-in voice name such as `alloy` or `marin`, or a custom voice
    /// name.
    Name(String),

    /// A reference to a custom voice.
    Id(VoiceId),
}

/// A reference to a custom voice.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct VoiceId {
    /// The custom voice ID, e.g. `voice_1234`.
    pub id: String,
}

/// Configuration for input and output audio of a transcription session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranscriptionSessionAudio {
    /// Configuration for the input audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input: Option<RealtimeTranscriptionSessionAudioInput>,
}

/// Configuration for the input audio of a transcription session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranscriptionSessionAudioInput {
    /// The format of the input audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<RealtimeAudioFormats>,

    /// Configuration for input audio noise reduction. Set to `null` to turn off.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub noise_reduction: Option<NoiseReduction>,

    /// Configuration for input audio transcription.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transcription: Option<AudioTranscription>,

    /// Configuration for turn detection, or `null` to turn it off. For
    /// `gpt-realtime-whisper` this must be `null`; VAD is not supported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turn_detection: Option<RealtimeTranscriptionSessionAudioInputTurnDetection>,
}

/// Turn detection configuration of a transcription session, either Server VAD
/// or Semantic VAD.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[serde(tag = "type")]
#[display("{self:?}")]
pub enum RealtimeTranscriptionSessionAudioInputTurnDetection {
    /// Server-side voice activity detection based on audio volume.
    #[serde(rename = "server_vad")]
    ServerVad(ServerVad),

    /// Server-side semantic turn detection, which uses a model to determine
    /// when the user has finished speaking.
    #[serde(rename = "semantic_vad")]
    SemanticVad(SemanticVad),
}

/// Configuration for turn detection of a transcription session.
///
/// Can be set to `null` to turn it off. For `gpt-realtime-whisper` this must be
/// `null`; VAD is not supported.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranscriptionSessionTurnDetection {
    /// Amount of audio to include before the VAD-detected speech, in
    /// milliseconds. Defaults to 300 ms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix_padding_ms: Option<u64>,

    /// Duration of silence that ends speech detection, in milliseconds.
    /// Defaults to 500 ms.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub silence_duration_ms: Option<u64>,

    /// Activation threshold for VAD (0.0 to 1.0). Defaults to 0.5.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f32>,

    /// The type of turn detection. Only `server_vad` is currently supported.
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// Realtime transcription session configuration sent by the client when
/// creating or updating a transcription-only session.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct RealtimeTranscriptionSessionCreateRequest {
    /// The type of session to create. Always `transcription` for transcription
    /// sessions.
    #[serde(rename = "type")]
    pub type_: String,

    /// Configuration for input and output audio.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio: Option<RealtimeTranscriptionSessionAudio>,

    /// Additional fields to include in the session's server events, such as
    /// `item.input_audio_transcription.logprobs`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<SessionInclude>>,
}

/// An additional field the server includes in the session's events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum SessionInclude {
    /// Include log probabilities of the input audio transcription.
    #[serde(rename = "item.input_audio_transcription.logprobs")]
    InputAudioTranscriptionLogprobs,
}

/// The default value of [`PcmAudio::type_`].
fn pcm_audio_type() -> String {
    "audio/pcm".to_owned()
}

/// The default value of [`PcmuAudio::type_`].
fn pcmu_audio_type() -> String {
    "audio/pcmu".to_owned()
}

/// The default value of [`PcmaAudio::type_`].
fn pcma_audio_type() -> String {
    "audio/pcma".to_owned()
}

/// The default value of [`RetentionRatioTruncation::type_`].
fn retention_ratio_type() -> String {
    "retention_ratio".to_owned()
}
