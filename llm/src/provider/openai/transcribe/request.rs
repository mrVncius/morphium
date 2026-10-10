use serde::Serialize;

/// ID of the model to use for transcription.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AudioModel {
    /// OpenAI's open source Whisper V2 model.
    #[serde(rename = "whisper-1")]
    Whisper1,
    /// GPT-4o transcription model.
    #[serde(rename = "gpt-4o-transcribe")]
    Gpt4oTranscribe,
    /// GPT-4o mini transcription model.
    #[serde(rename = "gpt-4o-mini-transcribe")]
    Gpt4oMiniTranscribe,
    /// GPT-4o mini transcription model (2025-12-15 snapshot).
    #[serde(rename = "gpt-4o-mini-transcribe-2025-12-15")]
    Gpt4oMiniTranscribe20251215,
    /// GPT-4o transcription model with speaker diarization.
    #[serde(rename = "gpt-4o-transcribe-diarize")]
    Gpt4oTranscribeDiarize,
}

impl std::fmt::Display for AudioModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Whisper1 => write!(f, "whisper-1"),
            Self::Gpt4oTranscribe => write!(f, "gpt-4o-transcribe"),
            Self::Gpt4oMiniTranscribe => write!(f, "gpt-4o-mini-transcribe"),
            Self::Gpt4oMiniTranscribe20251215 => {
                write!(f, "gpt-4o-mini-transcribe-2025-12-15")
            }
            Self::Gpt4oTranscribeDiarize => write!(f, "gpt-4o-transcribe-diarize"),
        }
    }
}

/// The format of the transcription output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AudioResponseFormat {
    /// Plain JSON response with transcribed text.
    Json,
    /// Plain text response.
    Text,
    /// SRT subtitle format.
    Srt,
    /// Verbose JSON response with timestamps and segments.
    #[serde(rename = "verbose_json")]
    VerboseJson,
    /// VTT subtitle format.
    Vtt,
    /// JSON response with speaker diarization.
    #[serde(rename = "diarized_json")]
    DiarizedJson,
}

impl std::fmt::Display for AudioResponseFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json => write!(f, "json"),
            Self::Text => write!(f, "text"),
            Self::Srt => write!(f, "srt"),
            Self::VerboseJson => write!(f, "verbose_json"),
            Self::Vtt => write!(f, "vtt"),
            Self::DiarizedJson => write!(f, "diarized_json"),
        }
    }
}

/// Controls how the audio is cut into chunks.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChunkingStrategy {
    /// Automatically set chunking parameters.
    Auto,
    /// Manual VAD (Voice Activity Detection) configuration.
    ServerVad(VadConfig),
}

/// Voice Activity Detection configuration. Must have `"type": "server_vad"`.
#[derive(Debug, Clone, Serialize)]
pub struct VadConfig {
    /// Must be `server_vad`.
    #[serde(rename = "type")]
    pub kind: VadConfigType,
    /// Amount of audio to include before the VAD detected speech (ms).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix_padding_ms: Option<u32>,
    /// Duration of silence to detect speech stop (ms).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub silence_duration_ms: Option<u32>,
    /// Sensitivity threshold (0.0 to 1.0) for voice activity detection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<f64>,
}

/// The type of VAD config. Always `server_vad`.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum VadConfigType {
    /// Server-side voice activity detection.
    #[serde(rename = "server_vad")]
    ServerVad,
}

/// Additional information to include in the transcription response.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TranscriptionInclude {
    /// Return log probabilities of the tokens.
    Logprobs,
}

impl std::fmt::Display for TranscriptionInclude {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Logprobs => write!(f, "logprobs"),
        }
    }
}

/// Timestamp granularity for `verbose_json` responses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimestampGranularity {
    /// Word-level timestamps.
    Word,
    /// Segment-level timestamps.
    Segment,
}

impl std::fmt::Display for TimestampGranularity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Word => write!(f, "word"),
            Self::Segment => write!(f, "segment"),
        }
    }
}

/// Request to create a transcription.
#[derive(Debug, Clone)]
pub struct TranscriptionRequest {
    /// The raw audio file bytes to transcribe.
    pub file: Vec<u8>,
    /// The filename (with extension) for the audio file.
    pub filename: String,
    /// ID of the model to use.
    pub model: AudioModel,
    /// Controls how the audio is cut into chunks.
    pub chunking_strategy: Option<ChunkingStrategy>,
    /// Additional information to include in the response.
    pub include: Vec<TranscriptionInclude>,
    /// Optional list of speaker names for diarization.
    pub known_speaker_names: Vec<String>,
    /// Optional list of audio data URLs as speaker references.
    pub known_speaker_references: Vec<String>,
    /// The language of the input audio (ISO-639-1, e.g. "en").
    pub language: Option<String>,
    /// Optional text to guide the model's style or continue a previous segment.
    pub prompt: Option<String>,
    /// The format of the output.
    pub response_format: Option<AudioResponseFormat>,
    /// Whether to stream the transcription.
    pub stream: Option<bool>,
    /// Sampling temperature (0 to 1).
    pub temperature: Option<f64>,
    /// Timestamp granularities for verbose_json.
    pub timestamp_granularities: Vec<TimestampGranularity>,
}
