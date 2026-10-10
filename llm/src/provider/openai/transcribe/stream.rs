use super::response::{TranscriptionLogprob, TranscriptionUsage};

/// A streaming event from the transcription API.
#[derive(Debug, Clone)]
pub enum TranscriptionStreamEvent {
    /// A delta update to the transcript text.
    TranscriptTextDelta {
        /// The incremental text chunk.
        delta: String,
        /// Log probabilities for this delta (when requested).
        logprobs: Option<Vec<TranscriptionLogprob>>,
    },
    /// The final transcript text.
    TranscriptTextDone {
        /// The complete transcribed text.
        text: String,
        /// Log probabilities (when requested).
        logprobs: Option<Vec<TranscriptionLogprob>>,
        /// Usage statistics.
        usage: Option<TranscriptionUsage>,
    },
}
