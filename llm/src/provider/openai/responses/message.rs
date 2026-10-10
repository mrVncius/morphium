use derive_more::{Display, IsVariant};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The `input` field of a request.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(untagged)]
#[display("{self:?}")]
pub enum Input {
    /// A plain text string (becomes a user message).
    Text(String),
    /// An array of input items.
    Items(Vec<InputItem>),
}

/// Untagged wrapper for the `message` input item, handling both input and output messages.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(untagged)]
#[display("{self:?}")]
pub enum InputItemMessage {
    /// An input message (user, system, developer, or assistant role).
    Input(EasyInputMessage),
    /// An output message from a previous response (for multi-turn).
    Output(OutputMessage),
}

/// An item in the `input` array.
#[derive(Debug, Clone, Serialize, Deserialize, Display, IsVariant)]
#[serde(tag = "type")]
#[display("{self:?}")]
pub enum InputItem {
    /// A user/developer/system/assistant message.
    #[serde(rename = "message")]
    Message(InputItemMessage),

    /// File search call.
    #[serde(rename = "file_search_call")]
    FileSearchCall(FileSearchCallItem),

    /// Web search call.
    #[serde(rename = "web_search_call")]
    WebSearchCall(WebSearchCallItem),

    /// Function call.
    #[serde(rename = "function_call")]
    FunctionCall(FunctionCallItem),

    /// Function call output.
    #[serde(rename = "function_call_output")]
    FunctionCallOutput(FunctionCallOutputItem),

    /// Computer call.
    #[serde(rename = "computer_call")]
    ComputerCall(ComputerCallItem),

    /// Computer call output.
    #[serde(rename = "computer_call_output")]
    ComputerCallOutput(ComputerCallOutputItem),

    /// Reasoning content.
    #[serde(rename = "reasoning")]
    Reasoning(ReasoningItem),

    /// Compaction.
    #[serde(rename = "compaction")]
    Compaction(CompactionItem),

    /// Image generation call.
    #[serde(rename = "image_generation_call")]
    ImageGenerationCall(ImageGenerationCallItem),

    /// Code interpreter call.
    #[serde(rename = "code_interpreter_call")]
    CodeInterpreterCall(CodeInterpreterCallItem),

    /// Local shell call.
    #[serde(rename = "local_shell_call")]
    LocalShellCall(LocalShellCallItem),

    /// Local shell call output.
    #[serde(rename = "local_shell_call_output")]
    LocalShellCallOutput(LocalShellCallOutputItem),

    /// Shell call.
    #[serde(rename = "shell_call")]
    ShellCall(ShellCallItem),

    /// Shell call output.
    #[serde(rename = "shell_call_output")]
    ShellCallOutput(ShellCallOutputItem),

    /// Apply patch call.
    #[serde(rename = "apply_patch_call")]
    ApplyPatchCall(ApplyPatchCallItem),

    /// Apply patch call output.
    #[serde(rename = "apply_patch_call_output")]
    ApplyPatchCallOutput(ApplyPatchCallOutputItem),

    /// MCP list tools.
    #[serde(rename = "mcp_list_tools")]
    McpListTools(McpListToolsItem),

    /// MCP approval request.
    #[serde(rename = "mcp_approval_request")]
    McpApprovalRequest(McpApprovalRequestItem),

    /// MCP approval response.
    #[serde(rename = "mcp_approval_response")]
    McpApprovalResponse(McpApprovalResponseItem),

    /// MCP call.
    #[serde(rename = "mcp_call")]
    McpCall(McpCallItem),

    /// Custom tool call.
    #[serde(rename = "custom_tool_call")]
    CustomToolCall(CustomToolCallItem),

    /// Custom tool call output.
    #[serde(rename = "custom_tool_call_output")]
    CustomToolCallOutput(CustomToolCallOutputItem),

    /// Tool search call.
    #[serde(rename = "tool_search_call")]
    ToolSearchCall(ToolSearchCallItem),

    /// Tool search output.
    #[serde(rename = "tool_search_output")]
    ToolSearchOutput(ToolSearchOutputItem),

    /// Compaction trigger.
    #[serde(rename = "compaction_trigger")]
    CompactionTrigger(CompactionTriggerItem),

    /// Item reference.
    #[serde(rename = "item_reference")]
    ItemReference(ItemReferenceItem),
}

/// An item in the `output` array of a response.
#[derive(Debug, Clone, Serialize, Deserialize, Display, IsVariant)]
#[serde(tag = "type")]
#[display("{self:?}")]
pub enum OutputItem {
    /// Assistant message.
    #[serde(rename = "message")]
    Message(OutputMessage),

    /// File search call.
    #[serde(rename = "file_search_call")]
    FileSearchCall(FileSearchCallItem),

    /// Web search call.
    #[serde(rename = "web_search_call")]
    WebSearchCall(WebSearchCallItem),

    /// Function call.
    #[serde(rename = "function_call")]
    FunctionCall(FunctionCallItem),

    /// Function call output.
    #[serde(rename = "function_call_output")]
    FunctionCallOutput(FunctionCallOutputItem),

    /// Computer call.
    #[serde(rename = "computer_call")]
    ComputerCall(ComputerCallItem),

    /// Computer call output.
    #[serde(rename = "computer_call_output")]
    ComputerCallOutput(ComputerCallOutputItem),

    /// Reasoning content.
    #[serde(rename = "reasoning")]
    Reasoning(ReasoningItem),

    /// Tool search call.
    #[serde(rename = "tool_search_call")]
    ToolSearchCall(ToolSearchCallItem),

    /// Tool search output.
    #[serde(rename = "tool_search_output")]
    ToolSearchOutput(ToolSearchOutputItem),

    /// Compaction.
    #[serde(rename = "compaction")]
    Compaction(CompactionItem),

    /// Image generation call.
    #[serde(rename = "image_generation_call")]
    ImageGenerationCall(ImageGenerationCallItem),

    /// Code interpreter call.
    #[serde(rename = "code_interpreter_call")]
    CodeInterpreterCall(CodeInterpreterCallItem),

    /// Local shell call.
    #[serde(rename = "local_shell_call")]
    LocalShellCall(LocalShellCallItem),

    /// Local shell call output.
    #[serde(rename = "local_shell_call_output")]
    LocalShellCallOutput(LocalShellCallOutputItem),

    /// Shell call.
    #[serde(rename = "shell_call")]
    ShellCall(ShellCallItem),

    /// Shell call output.
    #[serde(rename = "shell_call_output")]
    ShellCallOutput(ShellCallOutputItem),

    /// Apply patch call.
    #[serde(rename = "apply_patch_call")]
    ApplyPatchCall(ApplyPatchCallItem),

    /// Apply patch call output.
    #[serde(rename = "apply_patch_call_output")]
    ApplyPatchCallOutput(ApplyPatchCallOutputItem),

    /// MCP list tools.
    #[serde(rename = "mcp_list_tools")]
    McpListTools(McpListToolsItem),

    /// MCP approval request.
    #[serde(rename = "mcp_approval_request")]
    McpApprovalRequest(McpApprovalRequestItem),

    /// MCP approval response.
    #[serde(rename = "mcp_approval_response")]
    McpApprovalResponse(McpApprovalResponseItem),

    /// MCP call.
    #[serde(rename = "mcp_call")]
    McpCall(McpCallItem),

    /// Custom tool call.
    #[serde(rename = "custom_tool_call")]
    CustomToolCall(CustomToolCallItem),

    /// Custom tool call output.
    #[serde(rename = "custom_tool_call_output")]
    CustomToolCallOutput(CustomToolCallOutputItem),

    /// Catch-all for unknown output types.
    #[serde(other)]
    Unknown,
}

/// Content of a message – either a plain string or a list of parts.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(untagged)]
#[display("{self:?}")]
pub enum MessageContent {
    /// A plain text content.
    Text(String),
    /// A list of structured content parts.
    Parts(Vec<ContentPart>),
}

/// A content part inside a message.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(tag = "type")]
#[display("{self:?}")]
pub enum ContentPart {
    /// Text input content for the model.
    #[serde(rename = "input_text")]
    InputText {
        /// The text input to the model.
        text: String,
    },

    /// Image input content for the model.
    #[serde(rename = "input_image")]
    InputImage {
        /// The detail level of the image.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        detail: Option<ImageDetail>,
        /// The ID of the file to be sent to the model.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        file_id: Option<String>,
        /// The URL of the image to be sent to the model.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        image_url: Option<String>,
    },

    /// File input content for the model.
    #[serde(rename = "input_file")]
    InputFile {
        /// The detail level of the file.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        detail: Option<FileDetail>,
        /// The content of the file to be sent to the model.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        file_data: Option<String>,
        /// The ID of the file to be sent to the model.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        file_id: Option<String>,
        /// The URL of the file to be sent to the model.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        file_url: Option<String>,
        /// The name of the file to be sent to the model.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        filename: Option<String>,
    },
}

/// Output content part inside an assistant message.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(tag = "type")]
#[display("{self:?}")]
pub enum OutputContent {
    /// Text output from the model.
    #[serde(rename = "output_text")]
    OutputText {
        /// The text output from the model.
        text: String,
        /// The annotations of the text output.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        annotations: Vec<Annotation>,
        /// The log probabilities of the text output.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        logprobs: Vec<Logprob>,
    },
    /// A refusal explanation from the model.
    #[serde(rename = "refusal")]
    Refusal {
        /// The refusal explanation from the model.
        refusal: String,
    },
}

/// The role of the message input. One of `user`, `assistant`, `system`, or `developer`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    /// The `user` role.
    #[display("user")]
    User,
    /// The `assistant` role.
    #[display("assistant")]
    Assistant,
    /// The `system` role.
    #[display("system")]
    System,
    /// The `developer` role.
    #[display("developer")]
    Developer,
}

/// The status of an item. One of `in_progress`, `completed`, or `incomplete`.
///
/// Populated when items are returned via API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum ItemStatus {
    /// The item is in progress.
    #[display("in_progress")]
    InProgress,
    /// The item is completed.
    #[display("completed")]
    Completed,
    /// The item is incomplete.
    #[display("incomplete")]
    Incomplete,
}

/// Labels an `assistant` message as intermediate commentary (`commentary`) or
/// the final answer (`final_answer`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum Phase {
    /// Intermediate commentary from the assistant.
    #[display("commentary")]
    Commentary,
    /// The final answer from the assistant.
    #[display("final_answer")]
    FinalAnswer,
}

/// The detail level of the image to be sent to the model.
///
/// One of `high`, `low`, `auto`, or `original`. Defaults to `auto`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum ImageDetail {
    /// Low detail level.
    #[display("low")]
    Low,
    /// High detail level.
    #[display("high")]
    High,
    /// Auto detail level (default).
    #[display("auto")]
    Auto,
    /// Original detail level.
    #[display("original")]
    Original,
}

/// The detail level of the file to be sent to the model.
///
/// Use `low` for the default rendering behavior, or `high` to render the file
/// at higher quality. Defaults to `low`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum FileDetail {
    /// Low detail level (default).
    #[display("low")]
    Low,
    /// High detail level.
    #[display("high")]
    High,
}

/// An annotation within a text output.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(tag = "type")]
#[display("{self:?}")]
pub enum Annotation {
    /// A citation to a file.
    #[serde(rename = "file_citation")]
    FileCitation {
        /// The ID of the file.
        file_id: String,
        /// The filename of the file cited.
        filename: String,
        /// The index of the file citation in the message.
        index: u32,
    },
    /// A citation to a URL.
    #[serde(rename = "url_citation")]
    UrlCitation {
        /// The index of the first character of the URL citation in the message.
        start_index: u32,
        /// The index of the last character of the URL citation in the message.
        end_index: u32,
        /// The URL of the web resource.
        url: String,
        /// The title of the web resource.
        title: String,
    },
    /// A file path annotation.
    #[serde(rename = "file_path")]
    FilePath {
        /// The ID of the file.
        file_id: String,
        /// The index of the file path in the message.
        index: u32,
    },
    /// A citation to a container file.
    #[serde(rename = "container_file_citation")]
    ContainerFileCitation {
        /// The ID of the container file.
        container_id: String,
        /// The ID of the file.
        file_id: String,
        /// The filename of the container file cited.
        filename: String,
        /// The index of the first character of the container file citation in the message.
        start_index: u32,
        /// The index of the last character of the container file citation in the message.
        end_index: u32,
    },
}

/// The log probability information for a token.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct Logprob {
    /// The token text.
    pub token: String,
    /// The bytes of the token.
    #[serde(default)]
    pub bytes: Vec<i64>,
    /// The log probability of the token.
    pub logprob: f64,
    /// The top log probabilities of the token.
    #[serde(default)]
    pub top_logprobs: Vec<TopLogprob>,
}

/// A top alternative token and its log probability.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct TopLogprob {
    /// The token text.
    pub token: String,
    /// The bytes of the token.
    #[serde(default)]
    pub bytes: Vec<i64>,
    /// The log probability of the token.
    pub logprob: f64,
}

/// A flexible input message with role + content.
///
/// Handles both simple string content and structured content parts,
/// and can carry optional `status` (for resuming conversations) or `phase`
/// (for labeling assistant messages as commentary vs final answer).
///
/// Note: We omit `type` here because it conflicts with serde's `#[serde(tag = "type")]`
/// on the parent `InputItem` enum in serde >=1.0.228.
/// Serde automatically outputs the `type` field from the tag.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct EasyInputMessage {
    /// Text, image, or audio input to the model.
    pub content: MessageContent,
    /// The role of the message input.
    pub role: Role,
    /// The status of the message input.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
    /// Labels an assistant message as intermediate commentary or the final answer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<Phase>,
}

/// Full-fidelity input message (deprecated, use `EasyInputMessage` instead).
pub type InputMessage = EasyInputMessage;

/// Assistant output message (used in both input & output).
///
/// Note: We omit `type` here because it conflicts with serde's `#[serde(tag = "type")]`
/// on the parent `OutputItem` enum in serde >=1.0.228.
/// Use the `OutputItem` enum variant to determine the item type instead.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct OutputMessage {
    /// The unique ID of the output message.
    pub id: String,
    /// The content of the output message.
    pub content: Vec<OutputContent>,
    /// The role of the output message. Always `assistant`.
    pub role: Role,
    /// The status of the message input.
    pub status: ItemStatus,
    /// Labels an assistant message as intermediate commentary or the final answer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<Phase>,
}

/// A file search tool call item.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct FileSearchCallItem {
    /// The unique ID of the file search tool call.
    pub id: String,
    /// The queries used to search for files.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queries: Option<Vec<String>>,
    /// The status of the file search tool call.
    pub status: ItemStatus,
    /// The results of the file search tool call.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<FileSearchResult>>,
}

/// A result from a file search.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct FileSearchResult {
    /// The unique ID of the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// The name of the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    /// The text that was retrieved from the file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// The relevance score of the file - a value between 0 and 1.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    /// Set of key-value pairs that can be attached to an object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<HashMap<String, serde_json::Value>>,
}

/// A web search tool call item.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct WebSearchCallItem {
    /// The unique ID of the web search tool call.
    pub id: String,
    /// The action taken in this web search call.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<WebSearchAction>,
    /// The status of the web search tool call.
    pub status: WebSearchStatus,
}

/// An action within a web search call.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(tag = "type")]
#[display("{self:?}")]
pub enum WebSearchAction {
    /// A search action.
    #[serde(rename = "search")]
    Search {
        /// `[DEPRECATED]` The search query.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        query: Option<String>,
        /// The search queries.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        queries: Option<Vec<String>>,
        /// The sources used in the search.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        sources: Option<Vec<WebSearchSource>>,
    },
    /// An action to open a page.
    #[serde(rename = "open_page")]
    OpenPage {
        /// The URL opened by the model.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        url: Option<String>,
    },
    /// An action to find text within a page.
    #[serde(rename = "find_in_page")]
    FindInPage {
        /// The pattern or text to search for within the page.
        pattern: String,
        /// The URL of the page searched for the pattern.
        url: String,
    },
}

/// A source used in a web search.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct WebSearchSource {
    /// The type of source. Always `url`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The URL of the source.
    pub url: String,
}

/// The status of a web search tool call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum WebSearchStatus {
    /// The web search is in progress.
    #[display("in_progress")]
    InProgress,
    /// The web search is searching.
    #[display("searching")]
    Searching,
    /// The web search completed successfully.
    #[display("completed")]
    Completed,
    /// The web search failed.
    #[display("failed")]
    Failed,
}

/// A function tool call item.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct FunctionCallItem {
    /// The unique ID of the function tool call generated by the model.
    pub call_id: String,
    /// The name of the function to run.
    pub name: String,
    /// A JSON string of the arguments to pass to the function.
    pub arguments: String,
    /// The unique ID of the function tool call.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The namespace of the function to run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// The status of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// A function call output item.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct FunctionCallOutputItem {
    /// The ID of the function tool call that produced the output.
    pub call_id: String,
    /// The output of the function call.
    #[serde(default)]
    pub output: serde_json::Value,
    /// The unique ID of the function call output item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The status of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// A computer tool call item.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ComputerCallItem {
    /// The unique ID of the computer call.
    pub id: String,
    /// An identifier used when responding to the tool call with output.
    pub call_id: String,
    /// The pending safety checks for the computer call.
    #[serde(default)]
    pub pending_safety_checks: Vec<SafetyCheck>,
    /// The status of the item.
    pub status: ItemStatus,
    /// The action of the computer call.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<ComputerAction>,
    /// Flattened batched actions for computer_use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<ComputerAction>>,
}

/// A safety check associated with a computer call.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct SafetyCheck {
    /// The ID of the pending safety check.
    pub id: String,
    /// The type of the pending safety check.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Details about the pending safety check.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// A computer tool call output item.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ComputerCallOutputItem {
    /// The ID of the computer tool call that produced the output.
    pub call_id: String,
    /// The output of the computer tool call.
    pub output: ComputerScreenshot,
    /// The ID of the computer tool call output.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The safety checks reported by the API that have been acknowledged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acknowledged_safety_checks: Option<Vec<SafetyCheck>>,
    /// The status of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// A screenshot output from a computer call.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ComputerScreenshot {
    /// The type of the output. Always `computer_screenshot`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The identifier of an uploaded file that contains the screenshot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// The URL of the screenshot image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

/// An action for a computer call.
#[derive(Debug, Clone, Serialize, Deserialize, Display, IsVariant)]
#[serde(tag = "type")]
#[display("{self:?}")]
pub enum ComputerAction {
    /// A click action.
    #[serde(rename = "click")]
    Click {
        /// Indicates which mouse button was pressed.
        button: MouseButton,
        /// The x-coordinate.
        x: i64,
        /// The y-coordinate.
        y: i64,
        /// The keys being held.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        keys: Option<Vec<String>>,
    },
    /// A double click action.
    #[serde(rename = "double_click")]
    DoubleClick {
        /// The x-coordinate.
        x: i64,
        /// The y-coordinate.
        y: i64,
        /// The keys being held.
        keys: Vec<String>,
    },
    /// A drag action.
    #[serde(rename = "drag")]
    Drag {
        /// An array of coordinates representing the path.
        path: Vec<Coord>,
        /// The keys being held.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        keys: Option<Vec<String>>,
    },
    /// A collection of keypresses.
    #[serde(rename = "keypress")]
    Keypress {
        /// The keys being pressed.
        keys: Vec<String>,
    },
    /// A mouse move action.
    #[serde(rename = "move")]
    Move {
        /// The x-coordinate.
        x: i64,
        /// The y-coordinate.
        y: i64,
        /// The keys being held.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        keys: Option<Vec<String>>,
    },
    /// A screenshot action.
    #[serde(rename = "screenshot")]
    Screenshot,
    /// A scroll action.
    #[serde(rename = "scroll")]
    Scroll {
        /// The horizontal scroll distance.
        scroll_x: i64,
        /// The vertical scroll distance.
        scroll_y: i64,
        /// The x-coordinate.
        x: i64,
        /// The y-coordinate.
        y: i64,
        /// The keys being held.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        keys: Option<Vec<String>>,
    },
    /// An action to type in text.
    #[serde(rename = "type")]
    Type {
        /// The text to type.
        text: String,
    },
    /// A wait action.
    #[serde(rename = "wait")]
    Wait,
}

/// A mouse button type for computer actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum MouseButton {
    /// Left mouse button.
    #[display("left")]
    Left,
    /// Right mouse button.
    #[display("right")]
    Right,
    /// Wheel mouse button.
    #[display("wheel")]
    Wheel,
    /// Back mouse button.
    #[display("back")]
    Back,
    /// Forward mouse button.
    #[display("forward")]
    Forward,
}

/// A coordinate with x and y values.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct Coord {
    /// The x-coordinate.
    pub x: i64,
    /// The y-coordinate.
    pub y: i64,
}

/// A reasoning item produced by the model.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ReasoningItem {
    /// The unique ID of the reasoning item.
    pub id: String,
    /// The reasoning summaries.
    #[serde(default)]
    pub summary: Vec<SummaryText>,
    /// The reasoning content.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<ReasoningText>>,
    /// An encrypted version of reasoning tokens.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encrypted_content: Option<String>,
    /// The status of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// A summary text entry within a reasoning item.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct SummaryText {
    /// The summary text.
    pub text: String,
    /// The type of summary.
    #[serde(rename = "type")]
    pub kind: String,
}

/// A reasoning content entry within a reasoning item.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ReasoningText {
    /// The reasoning content text.
    pub text: String,
    /// The type of reasoning content.
    #[serde(rename = "type")]
    pub kind: String,
}

/// A compaction item for compressing conversation history.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct CompactionItem {
    /// The encrypted content of the compaction.
    pub encrypted_content: String,
    /// The unique ID of the compaction item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// A compaction trigger item.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct CompactionTriggerItem {}

/// A reference to another item in the input or output.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ItemReferenceItem {
    /// The ID of the referenced item.
    pub id: String,
    /// The type of the referenced item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub item_type: Option<String>,
}

/// An image generation tool call item.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ImageGenerationCallItem {
    /// The unique ID of the image generation call.
    pub id: String,
    /// The result of the image generation call.
    pub result: String,
    /// The status of the image generation call.
    pub status: ImageGenCallStatus,
}

/// The status of an image generation call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum ImageGenCallStatus {
    /// The image generation call is in progress.
    #[display("in_progress")]
    InProgress,
    /// The image generation call completed successfully.
    #[display("completed")]
    Completed,
    /// The model is generating the image.
    #[display("generating")]
    Generating,
    /// The image generation call failed.
    #[display("failed")]
    Failed,
}

/// A code interpreter tool call item.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct CodeInterpreterCallItem {
    /// The unique ID of the code interpreter call.
    pub id: String,
    /// The code that was interpreted.
    #[serde(default)]
    pub code: Option<String>,
    /// The ID of the container used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    /// The outputs of the code interpreter call.
    #[serde(default)]
    pub outputs: Vec<CodeInterpreterOutput>,
    /// The status of the code interpreter call.
    pub status: CodeInterpreterStatus,
}

/// An output from a code interpreter call.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(tag = "type")]
#[display("{self:?}")]
pub enum CodeInterpreterOutput {
    /// Text logs output from code execution.
    #[serde(rename = "logs")]
    Logs {
        /// The text logs.
        logs: String,
    },
    /// An image output from code execution.
    #[serde(rename = "image")]
    Image {
        /// The image URL.
        url: String,
    },
}

/// The status of a code interpreter call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum CodeInterpreterStatus {
    /// The code interpretation is in progress.
    #[display("in_progress")]
    InProgress,
    /// The code interpretation completed successfully.
    #[display("completed")]
    Completed,
    /// The code interpretation is incomplete.
    #[display("incomplete")]
    Incomplete,
    /// The code interpretation is currently interpreting.
    #[display("interpreting")]
    Interpreting,
    /// The code interpretation failed.
    #[display("failed")]
    Failed,
}

/// A local shell tool call item.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct LocalShellCallItem {
    /// The unique ID of the local shell call.
    pub id: String,
    /// The action to perform.
    pub action: LocalShellAction,
    /// An identifier used when responding with output.
    pub call_id: String,
    /// The status of the item.
    pub status: ItemStatus,
}

/// An action to execute in a local shell.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct LocalShellAction {
    /// The type of action.
    #[serde(rename = "type")]
    pub kind: String,
    /// The command to execute.
    pub command: Vec<String>,
    /// Environment variables for the command.
    #[serde(default)]
    pub env: HashMap<String, String>,
    /// Timeout in milliseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
    /// The user to run the command as.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// The working directory for the command.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

/// A local shell call output item.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct LocalShellCallOutputItem {
    /// The unique ID of the local shell call output.
    pub id: String,
    /// The output from the local shell command.
    pub output: String,
    /// The status of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// A shell tool call item.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ShellCallItem {
    /// The action to perform.
    pub action: ShellAction,
    /// An identifier used when responding with output.
    pub call_id: String,
    /// The unique ID of the shell call.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The shell environment configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment: Option<serde_json::Value>,
    /// The status of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// An action to execute in a shell.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ShellAction {
    /// The commands to execute.
    pub commands: Vec<String>,
    /// Maximum length of the output.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_output_length: Option<u64>,
    /// Timeout in milliseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
}

/// A shell call output item.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ShellCallOutputItem {
    /// The ID of the shell call that produced the output.
    pub call_id: String,
    /// The output chunks.
    #[serde(default)]
    pub output: Vec<ShellOutputChunk>,
    /// The unique ID of the shell call output.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Maximum output length.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_output_length: Option<u64>,
    /// The status of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// A chunk of output from a shell command.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ShellOutputChunk {
    /// The outcome of the shell command.
    pub outcome: ShellOutcome,
    /// The standard error output.
    pub stderr: String,
    /// The standard output.
    pub stdout: String,
}

/// The outcome of a shell command.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(tag = "type")]
#[display("{self:?}")]
pub enum ShellOutcome {
    /// The command timed out.
    #[serde(rename = "timeout")]
    Timeout,
    /// The command exited with a status code.
    #[serde(rename = "exit")]
    Exit {
        /// The exit code.
        exit_code: i64,
    },
}

/// An apply patch tool call item.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ApplyPatchCallItem {
    /// The ID of the apply_patch call.
    pub call_id: String,
    /// The patch operation to perform.
    pub operation: PatchOperation,
    /// The status of the apply patch call.
    pub status: ApplyPatchStatus,
    /// The unique ID of the apply patch call item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// A patch operation to apply to a file.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(tag = "type")]
#[display("{self:?}")]
pub enum PatchOperation {
    /// Create a new file with the given diff and path.
    #[serde(rename = "create_file")]
    CreateFile {
        /// The diff to apply.
        diff: String,
        /// The file path.
        path: String,
    },
    /// Delete a file at the given path.
    #[serde(rename = "delete_file")]
    DeleteFile {
        /// The file path.
        path: String,
    },
    /// Update an existing file with the given diff and path.
    #[serde(rename = "update_file")]
    UpdateFile {
        /// The diff to apply.
        diff: String,
        /// The file path.
        path: String,
    },
}

/// The status of an apply patch call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum ApplyPatchStatus {
    /// The apply patch operation is in progress.
    #[display("in_progress")]
    InProgress,
    /// The apply patch operation completed successfully.
    #[display("completed")]
    Completed,
}

/// An apply patch call output item.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ApplyPatchCallOutputItem {
    /// The ID of the apply patch call.
    pub call_id: String,
    /// The status of the output.
    pub status: ApplyPatchOutputStatus,
    /// The unique ID of the apply patch call output item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The output details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
}

/// The status of an apply patch output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum ApplyPatchOutputStatus {
    /// The apply patch output was completed successfully.
    #[display("completed")]
    Completed,
    /// The apply patch output failed.
    #[display("failed")]
    Failed,
}

/// An MCP list tools item.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct McpListToolsItem {
    /// The unique ID of the MCP list tools item.
    pub id: String,
    /// The label of the MCP server.
    pub server_label: String,
    /// The tools available on the MCP server.
    #[serde(default)]
    pub tools: Vec<McpToolDef>,
    /// Error information if the MCP server could not be reached.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// A tool definition from an MCP server.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct McpToolDef {
    /// The name of the MCP tool.
    pub name: String,
    /// The JSON Schema input schema for the tool.
    pub input_schema: serde_json::Value,
    /// The description of the MCP tool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Annotations for the MCP tool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<serde_json::Value>,
}

/// An MCP approval request item.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct McpApprovalRequestItem {
    /// The unique ID of the approval request.
    pub id: String,
    /// The arguments for the tool call.
    pub arguments: String,
    /// The name of the tool to be called.
    pub name: String,
    /// The label of the MCP server.
    pub server_label: String,
}

/// An MCP approval response item.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct McpApprovalResponseItem {
    /// The ID of the approval request being responded to.
    pub approval_request_id: String,
    /// Whether the tool call is approved.
    pub approve: bool,
    /// The unique ID of the approval response item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The reason for the approval decision.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// An MCP tool call item.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct McpCallItem {
    /// The unique ID of the MCP call.
    pub id: String,
    /// The arguments passed to the MCP tool.
    pub arguments: String,
    /// The name of the MCP tool called.
    pub name: String,
    /// The label of the MCP server.
    pub server_label: String,
    /// The ID of the approval request, if required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_request_id: Option<String>,
    /// Error information if the call failed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// The output from the MCP tool call.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    /// The status of the MCP call.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<McpCallStatus>,
}

/// The status of an MCP call.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum McpCallStatus {
    /// The MCP call is in progress.
    #[display("in_progress")]
    InProgress,
    /// The MCP call completed successfully.
    #[display("completed")]
    Completed,
    /// The MCP call is incomplete.
    #[display("incomplete")]
    Incomplete,
    /// The MCP tool is being called.
    #[display("calling")]
    Calling,
    /// The MCP call failed.
    #[display("failed")]
    Failed,
}

/// A custom tool call item.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct CustomToolCallItem {
    /// The ID of the custom tool call.
    pub call_id: String,
    /// The input to the custom tool.
    pub input: String,
    /// The name of the custom tool.
    pub name: String,
    /// The unique ID of the custom tool call item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The namespace of the custom tool.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// A custom tool call output item.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct CustomToolCallOutputItem {
    /// The ID of the custom tool call that produced the output.
    pub call_id: String,
    /// The output from the custom tool.
    #[serde(default)]
    pub output: serde_json::Value,
    /// The unique ID of the custom tool call output item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The status of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// A tool search call item.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ToolSearchCallItem {
    /// The arguments for the tool search.
    #[serde(default)]
    pub arguments: serde_json::Value,
    /// The unique ID of the tool search call.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The ID of the tool search call.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
    /// Where the tools are executed (server or client).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution: Option<String>,
    /// The status of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}

/// A tool search output item.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ToolSearchOutputItem {
    /// The tools found by the search.
    #[serde(default)]
    pub tools: Vec<super::Tool>,
    /// The unique ID of the tool search output item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The ID of the tool search call.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
    /// Where the tools are executed (server or client).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution: Option<String>,
    /// The status of the item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ItemStatus>,
}
