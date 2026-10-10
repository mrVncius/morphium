//! Tool definitions and tool choice for the Responses API.

use derive_more::{Display, IsVariant};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A tool the model may call.
///
/// Maps to the `tools[]` request field.
#[derive(Debug, Clone, Serialize, Deserialize, Display, IsVariant)]
#[serde(tag = "type")]
#[display("{self:?}")]
pub enum Tool {
    /// Function calling
    #[serde(rename = "function")]
    Function {
        /// The name of the function to call.
        name: String,
        /// A description of what the function does, used by the model to choose when and how to call the function.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// The parameters the function accepts, described as a JSON Schema object.
        #[serde(default)]
        parameters: serde_json::Value,
        /// Whether to enable strict schema adherence for the function.
        #[serde(default)]
        strict: bool,
        /// Whether to defer loading the function definition.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        defer_loading: Option<bool>,
    },

    /// File search
    #[serde(rename = "file_search")]
    FileSearch {
        /// The IDs of the vector stores to search.
        vector_store_ids: Vec<String>,
        /// The maximum number of results to return.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        max_num_results: Option<u32>,
        /// Ranking options for search results.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        ranking_options: Option<FileSearchRankingOptions>,
        /// Filters to apply to the search results.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        filters: Option<FileSearchFilter>,
    },

    /// Computer use (legacy)
    #[serde(rename = "computer")]
    Computer {},

    /// Computer use preview
    #[serde(rename = "computer_use_preview")]
    ComputerUsePreview {
        /// The display height in pixels.
        display_height: u32,
        /// The display width in pixels.
        display_width: u32,
        /// The computer environment (Windows, Mac, Linux, etc.).
        environment: ComputerEnvironment,
    },

    /// Web search (new variant)
    #[serde(rename = "web_search")]
    WebSearch {
        /// Filters for the web search (allowed domains).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        filters: Option<WebSearchFilters>,
        /// The context size for the search.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        search_context_size: Option<SearchContextSize>,
        /// The user's location for geographical search context.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        user_location: Option<UserLocation>,
    },

    /// Web search preview (legacy variant)
    #[serde(rename = "web_search_preview")]
    WebSearchPreview {
        /// The domains to search within.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        domains: Vec<String>,
        /// The context size for the search.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        search_context_size: Option<SearchContextSize>,
        /// The user's location for geographical search context.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        user_location: Option<UserLocation>,
        /// The content types to search (text, image).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        search_content_types: Option<Vec<SearchContentType>>,
    },

    /// MCP (Model Context Protocol)
    #[serde(rename = "mcp")]
    Mcp {
        /// The label identifying the MCP server.
        server_label: String,
        /// The URL of the MCP server.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        server_url: Option<String>,
        /// The connector ID for the MCP connection.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        connector_id: Option<String>,
        /// The tools allowed to be called on the MCP server.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        allowed_tools: Option<McpAllowedTools>,
        /// The authorization header for the MCP connection.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        authorization: Option<String>,
        /// Additional headers for the MCP connection.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        headers: Option<HashMap<String, String>>,
        /// Requires explicit approval for tool calls.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        require_approval: Option<McpApprovalPolicy>,
        /// Whether to defer loading the MCP tool configuration.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        defer_loading: Option<bool>,
        /// A description of the MCP server.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        server_description: Option<String>,
    },

    /// Code interpreter
    #[serde(rename = "code_interpreter")]
    CodeInterpreter {
        /// The container configuration for code execution.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        container: Option<CodeInterpreterContainer>,
    },

    /// Image generation
    #[serde(rename = "image_generation")]
    ImageGeneration {
        /// The image generation action (generate, edit, auto).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        action: Option<ImageGenAction>,
        /// Whether to return a transparent or opaque background.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        background: Option<ImageGenBackground>,
        /// The input fidelity (high or low).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        input_fidelity: Option<ImageGenFidelity>,
        /// The mask to use for inpainting.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        input_image_mask: Option<ImageGenMask>,
        /// The model to use for image generation.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        model: Option<String>,
        /// Moderation level (auto or low).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        moderation: Option<ImageGenModeration>,
        /// Compression level for the output.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        output_compression: Option<u32>,
        /// The output format (png, webp, jpeg).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        output_format: Option<ImageGenFormat>,
        /// Whether to stream partial images.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        partial_images: Option<u32>,
        /// The quality of the generated image.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        quality: Option<ImageGenQuality>,
        /// The size of the generated image.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        size: Option<String>,
    },

    /// Local shell
    #[serde(rename = "local_shell")]
    LocalShell {},

    /// Shell
    #[serde(rename = "shell")]
    Shell {
        /// The shell environment configuration.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        environment: Option<ShellEnvironment>,
    },

    /// Custom tool
    #[serde(rename = "custom")]
    Custom {
        /// The name of the custom tool.
        name: String,
        /// A description of what the custom tool does.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Whether to defer loading the tool definition.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        defer_loading: Option<bool>,
        /// The format of the custom tool (text or grammar).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        format: Option<CustomToolFormat>,
    },

    /// Namespace (grouped function/custom tools)
    #[serde(rename = "namespace")]
    Namespace {
        /// A description of the namespace.
        description: String,
        /// The name of the namespace.
        name: String,
        /// The tools in the namespace.
        tools: Vec<NsTool>,
    },

    /// Tool search (deferred tool discovery)
    #[serde(rename = "tool_search")]
    ToolSearch {
        /// A description of the tool search.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Whether tools are executed on the server or client.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        execution: Option<ToolSearchExecution>,
        /// The schema for tool search parameters.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        parameters: Option<serde_json::Value>,
    },

    /// Apply patch
    #[serde(rename = "apply_patch")]
    ApplyPatch {},
}

/// Controls which tool the model calls.
///
/// Maps to the `tool_choice` request field.
#[derive(Debug, Clone, Serialize, Deserialize, Display, IsVariant)]
#[serde(untagged)]
#[display("{self:?}")]
pub enum ToolChoice {
    /// `"none"`, `"auto"`, or `"required"`
    String(ToolChoiceString),

    /// Constrain to a predefined set of tools
    AllowedTools {
        /// The mode for tool selection (auto or required).
        mode: AllowedToolsMode,
        /// The list of allowed tool configurations.
        tools: Vec<serde_json::Value>,
        /// Always `allowed_tools`.
        #[serde(rename = "type")]
        kind: AllowedToolsType,
    },

    /// Force a built-in tool type
    BuiltInType {
        /// The built-in tool type to force.
        #[serde(rename = "type")]
        tool_type: BuiltInToolType,
    },

    /// Force a specific function
    Function {
        /// The name of the function to force.
        name: String,
        /// Always `function`.
        #[serde(rename = "type")]
        kind: FunctionToolType,
    },

    /// Force a specific MCP tool
    Mcp {
        /// The label of the MCP server.
        server_label: String,
        /// Always `mcp`.
        #[serde(rename = "type")]
        kind: McpToolType,
        /// The name of the MCP tool to force.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        name: Option<String>,
    },

    /// Force a specific custom tool
    Custom {
        /// The name of the custom tool.
        name: String,
        /// Always `custom`.
        #[serde(rename = "type")]
        kind: CustomToolType,
    },

    /// Force apply_patch
    ApplyPatch {
        /// Always `apply_patch`.
        #[serde(rename = "type")]
        kind: ApplyPatchToolType,
    },

    /// Force shell
    Shell {
        /// Always `shell`.
        #[serde(rename = "type")]
        kind: ShellToolType,
    },
}

/// Tool choice string value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum ToolChoiceString {
    /// The model will not call any tool.
    #[display("None")]
    None,
    /// The model can decide whether to call a tool.
    #[display("Auto")]
    Auto,
    /// The model must call a tool.
    #[display("Required")]
    Required,
}

/// Mode for allowed tools selection.
/// Always `allowed_tools`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum AllowedToolsMode {
    /// The model can decide whether to use allowed tools.
    #[display("auto")]
    Auto,
    /// The model must use allowed tools.
    #[display("required")]
    Required,
}

/// Always `allowed_tools`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum AllowedToolsType {
    #[serde(rename = "allowed_tools")]
    /// Always `allowed_tools`.
    #[display("allowed_tools")]
    AllowedTools,
}

/// Always `function`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum FunctionToolType {
    #[serde(rename = "function")]
    /// Always `function`.
    #[display("function")]
    Function,
}

/// Always `mcp`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum McpToolType {
    #[serde(rename = "mcp")]
    /// Always `mcp`.
    #[display("mcp")]
    Mcp,
}

/// Always `custom`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum CustomToolType {
    #[serde(rename = "custom")]
    /// Always `custom`.
    #[display("custom")]
    Custom,
}

/// Always `apply_patch`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum ApplyPatchToolType {
    #[serde(rename = "apply_patch")]
    /// Always `apply_patch`.
    #[display("apply_patch")]
    ApplyPatch,
}

/// Always `shell`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum ShellToolType {
    #[serde(rename = "shell")]
    /// Always `shell`.
    #[display("shell")]
    Shell,
}

/// Built-in tool type identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum BuiltInToolType {
    /// File search tool.
    #[display("FileSearch")]
    FileSearch,
    /// Web search preview tool.
    #[display("WebSearchPreview")]
    WebSearchPreview,
    /// Legacy computer use tool.
    #[display("Computer")]
    Computer,
    /// Computer use preview tool.
    #[display("ComputerUsePreview")]
    ComputerUsePreview,
    /// Computer use tool.
    #[display("ComputerUse")]
    ComputerUse,
    /// Web search preview (2025-03-11) tool.
    #[display("WebSearchPreview20250311")]
    WebSearchPreview20250311,
    /// Image generation tool.
    #[display("ImageGeneration")]
    ImageGeneration,
    /// Code interpreter tool.
    #[display("CodeInterpreter")]
    CodeInterpreter,
}

/// Computer environment type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum ComputerEnvironment {
    /// Windows operating system.
    #[display("Windows")]
    Windows,
    /// macOS operating system.
    #[display("Mac")]
    Mac,
    /// Linux operating system.
    #[display("Linux")]
    Linux,
    /// Ubuntu operating system.
    #[display("Ubuntu")]
    Ubuntu,
    /// Browser environment.
    #[display("Browser")]
    Browser,
}

/// Search context size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum SearchContextSize {
    /// Low search context size.
    #[display("low")]
    Low,
    /// Medium search context size.
    #[display("medium")]
    Medium,
    /// High search context size.
    #[display("high")]
    High,
}

/// Search content type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum SearchContentType {
    /// Search text content.
    #[display("text")]
    Text,
    /// Search image content.
    #[display("image")]
    Image,
}

/// Image generation action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum ImageGenAction {
    /// Generate a new image from a prompt.
    #[display("generate")]
    Generate,
    /// Edit an existing image.
    #[display("edit")]
    Edit,
    /// Automatically determine the action.
    #[display("auto")]
    Auto,
}

/// Image generation background type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum ImageGenBackground {
    /// Transparent background.
    #[display("transparent")]
    Transparent,
    /// Opaque background.
    #[display("opaque")]
    Opaque,
    /// Automatically determine the background.
    #[display("auto")]
    Auto,
}

/// Input fidelity for image generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum ImageGenFidelity {
    /// High fidelity.
    #[display("high")]
    High,
    /// Low fidelity.
    #[display("low")]
    Low,
}

/// Moderation level for image generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum ImageGenModeration {
    /// Automatic moderation.
    #[display("auto")]
    Auto,
    /// Low moderation.
    #[display("low")]
    Low,
}

/// Output format for image generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum ImageGenFormat {
    /// PNG format.
    #[display("png")]
    Png,
    /// WebP format.
    #[display("webp")]
    Webp,
    /// JPEG format.
    #[display("jpeg")]
    Jpeg,
}

/// Quality level for image generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum ImageGenQuality {
    /// Low quality.
    #[display("low")]
    Low,
    /// Medium quality.
    #[display("medium")]
    Medium,
    /// High quality.
    #[display("high")]
    High,
    /// Automatically determine quality.
    #[display("auto")]
    Auto,
}

/// Execution mode for tool search.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum ToolSearchExecution {
    /// Tools are executed on the server.
    #[display("Server")]
    Server,
    /// Tools are executed on the client.
    #[display("Client")]
    Client,
}

/// Ranking options for file search results.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct FileSearchRankingOptions {
    /// The ranker to use.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ranker: Option<String>,
    /// The minimum score threshold for results.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<f32>,
    /// Hybrid search configuration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hybrid_search: Option<HybridSearchWeights>,
}

/// Weights for hybrid search (embedding + text similarity).
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct HybridSearchWeights {
    /// The weight for embedding similarity.
    pub embedding_weight: f64,
    /// The weight for text similarity.
    pub text_weight: f64,
}

/// Filter for file search results.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(untagged)]
#[display("{self:?}")]
pub enum FileSearchFilter {
    /// A comparison-based filter.
    Comparison(ComparisonFilter),
    /// A compound (AND/OR) filter.
    Compound(CompoundFilter),
}

/// A comparison filter for file search.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ComparisonFilter {
    /// The key to filter on.
    pub key: String,
    /// The comparison operator.
    #[serde(rename = "type")]
    pub operator: ComparisonOp,
    /// The value to compare against.
    pub value: serde_json::Value,
}

/// Comparison operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonOp {
    /// Equal to.
    #[display("eq")]
    Eq,
    /// Not equal to.
    #[display("ne")]
    Ne,
    /// Greater than.
    #[display("gt")]
    Gt,
    /// Greater than or equal to.
    #[display("gte")]
    Gte,
    /// Less than.
    #[display("lt")]
    Lt,
    /// Less than or equal to.
    #[display("lte")]
    Lte,
    /// In the set.
    #[serde(rename = "in")]
    #[display("in")]
    In,
    /// Not in the set.
    #[serde(rename = "nin")]
    #[display("nin")]
    Nin,
}

/// A compound filter (AND/OR) for file search.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct CompoundFilter {
    /// The list of filters to combine.
    pub filters: Vec<FileSearchFilter>,
    /// The compound operator (AND or OR).
    #[serde(rename = "type")]
    pub op: CompoundOp,
}

/// Compound operator for combining filters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum CompoundOp {
    /// All filters must match.
    #[display("and")]
    And,
    /// Any filter must match.
    #[display("or")]
    Or,
}

/// Filters for web search results.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct WebSearchFilters {
    /// The domains allowed in the search results.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_domains: Option<Vec<String>>,
}

/// User location information for geographical search context.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct UserLocation {
    /// The type of location (e.g., `approximate`).
    #[serde(rename = "type")]
    pub location_type: String,
    /// The city.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The country.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The region.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// The timezone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

/// Allowed tools configuration for MCP.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(untagged)]
#[display("{self:?}")]
pub enum McpAllowedTools {
    /// List of allowed tool names.
    Names(Vec<String>),
    /// Filter configuration for allowed tools.
    Filter(McpToolFilter),
}

/// Filter for MCP tools.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct McpToolFilter {
    /// Whether to allow only read-only tools.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// The list of allowed tool names.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_names: Option<Vec<String>>,
}

/// Approval policy for MCP tool calls.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(untagged)]
#[display("{self:?}")]
pub enum McpApprovalPolicy {
    /// Filter-based approval policy.
    Filter(McpApprovalFilter),
    /// Global approval setting.
    Setting(McpApprovalSetting),
}

/// Filter-based approval configuration for MCP.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct McpApprovalFilter {
    /// Tools that always require approval.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub always: Option<McpToolFilter>,
    /// Tools that never require approval.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub never: Option<McpToolFilter>,
}

/// Global approval setting for MCP tool calls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
pub enum McpApprovalSetting {
    /// Always require approval for all tools.
    #[display("always")]
    Always,
    /// Never require approval for any tool.
    #[display("never")]
    Never,
}

/// Container configuration for code interpreter.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(untagged)]
#[display("{self:?}")]
pub enum CodeInterpreterContainer {
    /// A reference to an existing container by ID.
    Id(String),
    /// A new container configuration.
    Config(CodeInterpreterContainerConfig),
}

/// Configuration for a code interpreter container.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct CodeInterpreterContainerConfig {
    /// The type of container. Always `auto`.
    #[serde(rename = "type")]
    pub kind: CodeInterpreterContainerType,
    /// File IDs to make available in the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    /// Memory limit for the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_limit: Option<String>,
    /// Network access policy for the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_policy: Option<ContainerNetworkPolicy>,
}

/// Container type for code interpreter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display)]
pub enum CodeInterpreterContainerType {
    /// Automatically provision a container.
    #[serde(rename = "auto")]
    #[display("auto")]
    Auto,
}

/// Network access policy for a container.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(untagged)]
#[display("{self:?}")]
pub enum ContainerNetworkPolicy {
    /// Network access is disabled.
    Disabled(ContainerNetworkDisabled),
    /// Only specified domains are accessible.
    Allowlist(ContainerNetworkAllowlist),
}

/// Network disabled configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ContainerNetworkDisabled {
    /// The type of network policy. Always `disabled`.
    #[serde(rename = "type")]
    pub kind: String,
}

/// Network allowlist configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ContainerNetworkAllowlist {
    /// The type of network policy. Always `allowlist`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The list of allowed domains.
    pub allowed_domains: Vec<String>,
    /// Secret values for specific domains.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain_secrets: Option<Vec<DomainSecret>>,
}

/// A domain secret for container network access.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct DomainSecret {
    /// The domain the secret applies to.
    pub domain: String,
    /// The name of the secret.
    pub name: String,
    /// The value of the secret.
    pub value: String,
}

/// Mask configuration for image inpainting.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ImageGenMask {
    /// The file ID of the mask image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// The URL of the mask image.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
}

/// Shell execution environment.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(untagged)]
#[display("{self:?}")]
pub enum ShellEnvironment {
    /// Automatically provision a container.
    ContainerAuto(ShellContainerAuto),
    /// Run locally.
    Local(ShellLocal),
    /// Reference an existing container.
    ContainerRef(ShellContainerRef),
}

/// Automatically provisioned container configuration for shell execution.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ShellContainerAuto {
    /// The type of environment. Always `container_auto`.
    #[serde(rename = "type")]
    pub kind: String,
    /// File IDs to make available in the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    /// Memory limit for the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_limit: Option<String>,
    /// Network access policy for the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_policy: Option<ContainerNetworkPolicy>,
    /// Skills available in the container.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skills: Option<Vec<SkillDef>>,
}

/// Local shell execution configuration.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ShellLocal {
    /// The type of environment. Always `local`.
    #[serde(rename = "type")]
    pub kind: String,
    /// Skills available in the local environment.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skills: Option<Vec<LocalSkill>>,
}

/// Reference to an existing container for shell execution.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct ShellContainerRef {
    /// The ID of the existing container.
    pub container_id: String,
    /// The type of environment. Always `container_ref`.
    #[serde(rename = "type")]
    pub kind: String,
}

/// A skill definition (reference or inline).
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(untagged)]
#[display("{self:?}")]
pub enum SkillDef {
    /// Reference an existing skill by ID.
    Reference(SkillReference),
    /// Define a skill inline.
    Inline(InlineSkill),
}

/// A reference to an existing skill.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct SkillReference {
    /// The ID of the skill.
    pub skill_id: String,
    /// The type of skill reference. Always `reference`.
    #[serde(rename = "type")]
    pub kind: String,
    /// The version of the skill.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// An inline skill definition.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct InlineSkill {
    /// A description of the skill.
    pub description: String,
    /// The name of the skill.
    pub name: String,
    /// The source of the inline skill.
    pub source: InlineSkillSource,
    /// The type of skill definition. Always `inline`.
    #[serde(rename = "type")]
    pub kind: String,
}

/// The source of an inline skill.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct InlineSkillSource {
    /// The data (code) of the skill.
    pub data: String,
    /// The media type of the skill source.
    pub media_type: String,
    /// The type of source. Always `inline`.
    #[serde(rename = "type")]
    pub kind: String,
}

/// A local skill definition.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct LocalSkill {
    /// A description of the skill.
    pub description: String,
    /// The name of the skill.
    pub name: String,
    /// The path to the skill.
    pub path: String,
}

/// Format for a custom tool.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(untagged)]
#[display("{self:?}")]
pub enum CustomToolFormat {
    /// Text format (free-form text).
    Text(CustomToolFormatText),
    /// Grammar-constrained format.
    Grammar(CustomToolFormatGrammar),
}

/// Text format for a custom tool.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct CustomToolFormatText {
    /// The type of format. Always `text`.
    #[serde(rename = "type")]
    pub kind: String,
}

/// Grammar-constrained format for a custom tool.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[display("{self:?}")]
pub struct CustomToolFormatGrammar {
    /// The grammar definition.
    pub definition: String,
    /// The syntax of the grammar.
    pub syntax: String,
    /// The type of format. Always `grammar`.
    #[serde(rename = "type")]
    pub kind: String,
}

/// A tool definition inside a namespace.
#[derive(Debug, Clone, Serialize, Deserialize, Display)]
#[serde(tag = "type")]
#[display("{self:?}")]
pub enum NsTool {
    /// Function calling
    #[serde(rename = "function")]
    Function {
        /// The name of the function to call.
        name: String,
        /// A description of what the function does, used by the model to choose when and how to call the function.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// The parameters the function accepts, described as a JSON Schema object.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        parameters: Option<serde_json::Value>,
        /// Whether to enable strict schema adherence for the function.
        #[serde(default)]
        strict: bool,
        /// Whether to defer loading the function definition.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        defer_loading: Option<bool>,
    },
    /// Custom tool
    #[serde(rename = "custom")]
    Custom {
        /// The name of the custom tool.
        name: String,
        /// A description of what the custom tool does.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        /// Whether to defer loading the tool definition.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        defer_loading: Option<bool>,
        /// The format of the custom tool (text or grammar).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        format: Option<CustomToolFormat>,
    },
}
