//! OpenAI Transcriptions API client and types.
//!
//! See: <https://developers.openai.com/api/reference/responses/overview>

mod request;
mod response;
mod stream;

pub use request::*;
pub use response::*;
pub use stream::*;
