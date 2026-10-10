//! DeepSeek's stateless Responses API wire.
//!
//! Send complete history in [`ResponsesRequest::input`] on every turn. Output
//! [`InputItem`] values can be replayed directly, including reasoning and calls.
//! [`StreamEvent`] carries sequence numbers and all semantic events; completed,
//! incomplete, and failed events terminate the stream without a `[DONE]` marker.
//!
//! See <https://api-docs.deepseek.com/api/create-response> and
//! <https://api-docs.deepseek.com/guides/responses_api>.

mod message;
mod params;
mod request;
mod response;
mod stream;
mod tool;

pub use message::*;
pub use params::*;
pub use request::*;
pub use response::*;
pub use stream::*;
pub use tool::*;
