use serde::{Deserialize, Serialize};

use super::request::AudioResponseFormat;

/// A transcription response in one of several formats.
#[derive(Debug, Clone)]
pub enum TranscriptionResponse {
    /// Plain text (`text`, `srt`, or `vtt` format).
    Text(String),
    /// JSON transcription object (`json` format).
    Json(Transcription),
    /// Verbose JSON transcription (`verbose_json` format).
    VerboseJson(TranscriptionVerbose),
    /// Diarized JSON transcription (`diarized_json` format).
    DiarizedJson(TranscriptionDiarized),
}

impl TranscriptionResponse {
    /// Determine which response format to use based on the request's format.
    pub fn format_for_request(format: Option<AudioResponseFormat>) -> AudioResponseFormat {
        format.unwrap_or(AudioResponseFormat::Json)
    }
}

/// Plain JSON transcription response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transcription {
    /// The transcribed text.
    pub text: String,
    /// Log probabilities of the tokens (when `logprobs` is included).
    #[serde(default)]
    pub logprobs: Option<Vec<TranscriptionLogprob>>,
    /// Usage statistics.
    #[serde(default)]
    pub usage: Option<TranscriptionUsage>,
}

/// Verbose JSON transcription response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionVerbose {
    /// The task type. Always `transcribe`.
    pub task: String,
    /// The language of the input audio.
    pub language: String,
    /// The duration of the input audio in seconds.
    pub duration: f64,
    /// The transcribed text.
    pub text: String,
    /// Segments of the transcribed text (when requested).
    #[serde(default)]
    pub segments: Option<Vec<TranscriptionSegment>>,
    /// Word-level timestamps (when requested).
    #[serde(default)]
    pub words: Option<Vec<TranscriptionWord>>,
    /// Usage statistics.
    #[serde(default)]
    pub usage: Option<DurationUsage>,
}

/// Diarized JSON transcription response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionDiarized {
    /// The task type. Always `transcribe`.
    pub task: String,
    /// The duration of the input audio in seconds.
    pub duration: f64,
    /// The concatenated transcript text with speaker labels.
    pub text: String,
    /// Segments annotated with timestamps and speaker labels.
    pub segments: Vec<TranscriptionDiarizedSegment>,
    /// Usage statistics.
    #[serde(default)]
    pub usage: Option<TranscriptionUsage>,
}

/// A segment of a verbose transcription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionSegment {
    /// Unique identifier of the segment.
    pub id: u32,
    /// Seek offset of the segment.
    pub seek: u32,
    /// Start time of the segment in seconds.
    pub start: f64,
    /// End time of the segment in seconds.
    pub end: f64,
    /// Text content of the segment.
    pub text: String,
    /// Array of token IDs for the text content.
    #[serde(default)]
    pub tokens: Vec<u32>,
    /// Temperature parameter used for generating the segment.
    pub temperature: f64,
    /// Average logprob of the segment.
    pub avg_logprob: f64,
    /// Compression ratio of the segment.
    pub compression_ratio: f64,
    /// Probability of no speech in the segment.
    pub no_speech_prob: f64,
}

/// A word with its timestamp from a verbose transcription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionWord {
    /// The text content of the word.
    pub word: String,
    /// Start time of the word in seconds.
    pub start: f64,
    /// End time of the word in seconds.
    pub end: f64,
}

/// A diarized segment annotated with speaker label.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionDiarizedSegment {
    /// The type of segment. Always `transcript.text.segment`.
    #[serde(rename = "type")]
    pub kind: String,
    /// Unique identifier for the segment.
    pub id: String,
    /// Start timestamp of the segment in seconds.
    pub start: f64,
    /// End timestamp of the segment in seconds.
    pub end: f64,
    /// Transcript text for this segment.
    pub text: String,
    /// Speaker label for this segment.
    pub speaker: String,
}

/// Log probability information for a token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionLogprob {
    /// The token in the transcription.
    #[serde(default)]
    pub token: Option<String>,
    /// The bytes of the token.
    #[serde(default)]
    pub bytes: Option<Vec<i64>>,
    /// The log probability of the token.
    #[serde(default)]
    pub logprob: Option<f64>,
}

/// Usage statistics for a transcription response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TranscriptionUsage {
    /// Token-based usage (models billed by token).
    Tokens(TokenUsage),
    /// Duration-based usage (models billed by audio duration).
    Duration(DurationUsage),
}

/// Token usage statistics for models billed by token usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    /// The type of usage object. Always `tokens`.
    #[serde(rename = "type")]
    pub kind: String,
    /// Number of input tokens billed.
    pub input_tokens: u32,
    /// Number of output tokens generated.
    pub output_tokens: u32,
    /// Total number of tokens used.
    pub total_tokens: u32,
    /// Details about the input tokens.
    #[serde(default)]
    pub input_token_details: Option<InputTokenDetails>,
}

/// Details about the input tokens billed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputTokenDetails {
    /// Number of audio tokens billed.
    #[serde(default)]
    pub audio_tokens: Option<u32>,
    /// Number of text tokens billed.
    #[serde(default)]
    pub text_tokens: Option<u32>,
}

/// Duration usage statistics for models billed by audio input duration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurationUsage {
    /// The type of usage object. Always `duration`.
    #[serde(rename = "type")]
    pub kind: String,
    /// Duration of the input audio in seconds.
    pub seconds: f64,
}
