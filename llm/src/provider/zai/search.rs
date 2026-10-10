//! Web Search wire types (`POST /paas/v4/web_search`).
//!
//! A standalone search endpoint, separate from chat completions: a query in,
//! ranked result objects out. See
//! <https://docs.z.ai/api-reference/tools/web-search>

use serde::{Deserialize, Serialize};

/// Search request body.
#[derive(Debug, Clone, Serialize)]
pub struct WebSearchRequest {
    /// Search engine code.
    pub search_engine: SearchEngine,

    /// The query text.
    pub search_query: String,

    /// Number of results to return (`1 … 50`, default 10).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u8>,

    /// Restrict results to a single whitelisted domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_domain_filter: Option<String>,

    /// Restrict results to a time window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_recency_filter: Option<SearchRecency>,
}

/// Search engine code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchEngine {
    /// Z.AI's premium search engine.
    #[serde(rename = "search-prime")]
    SearchPrime,
}

/// Recency window for a search.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SearchRecency {
    /// Within one day.
    #[serde(rename = "oneDay")]
    OneDay,
    /// Within one week.
    #[serde(rename = "oneWeek")]
    OneWeek,
    /// Within one month.
    #[serde(rename = "oneMonth")]
    OneMonth,
    /// Within one year.
    #[serde(rename = "oneYear")]
    OneYear,
    /// No limit (the server default).
    #[serde(rename = "noLimit")]
    NoLimit,
}

/// Search response body.
#[derive(Debug, Clone, Deserialize)]
pub struct WebSearchResponse {
    /// Task ID.
    #[serde(default)]
    pub id: String,

    /// Unix timestamp of creation.
    #[serde(default)]
    pub created: u64,

    /// Ranked results.
    #[serde(default)]
    pub search_result: Vec<WebSearchResult>,
}

/// One search result object.
#[derive(Debug, Clone, Deserialize)]
pub struct WebSearchResult {
    /// Result title.
    #[serde(default)]
    pub title: String,

    /// Content summary.
    #[serde(default)]
    pub content: String,

    /// Result URL.
    #[serde(default)]
    pub link: String,

    /// Website name.
    #[serde(default)]
    pub media: Option<String>,

    /// Website publication date.
    #[serde(default)]
    pub publish_date: Option<String>,
}
