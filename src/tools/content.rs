use longbridge::ContentContext;
use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;
use rmcp::schemars::JsonSchema;
use rmcp::serde::Deserialize;

use crate::error::Error;
use crate::tools::{create_config, create_http_client, tool_json};

fn content_base_url(language: &Option<String>) -> &'static str {
    match language {
        Some(lang) if lang.contains("zh") => "https://longbridge.com/zh-CN",
        _ => "https://longbridge.com",
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SymbolParam {
    /// Security symbol, e.g. "700.HK"
    pub symbol: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct NewsDetailParam {
    /// News ID
    pub news_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TopicIdParam {
    /// Topic ID
    pub topic_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TopicCreateParam {
    /// Topic title
    pub title: String,
    /// Topic body content
    pub body: String,
    /// Related security symbols
    pub symbols: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct TopicCreateReplyParam {
    /// Topic ID to reply to
    pub topic_id: String,
    /// Reply body content
    pub body: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct FilingDetailParam {
    /// Filing ID
    pub filing_id: String,
}

pub async fn news(token: &str, p: SymbolParam) -> Result<CallToolResult, McpError> {
    let ctx = ContentContext::new(create_config(token));
    let result = ctx.news(p.symbol).await.map_err(Error::longbridge)?;
    tool_json(&result)
}

pub async fn news_detail(
    p: NewsDetailParam,
    language: Option<String>,
) -> Result<CallToolResult, McpError> {
    let url = format!("{}/news/{}", content_base_url(&language), p.news_id);
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| McpError::internal_error(e.to_string(), None))?;
    let body = resp
        .text()
        .await
        .map_err(|e| McpError::internal_error(e.to_string(), None))?;
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(
        body,
    )]))
}

pub async fn topic(token: &str, p: SymbolParam) -> Result<CallToolResult, McpError> {
    let ctx = ContentContext::new(create_config(token));
    let result = ctx.topics(p.symbol).await.map_err(Error::longbridge)?;
    tool_json(&result)
}

pub async fn topic_detail(
    p: TopicIdParam,
    language: Option<String>,
) -> Result<CallToolResult, McpError> {
    let url = format!("{}/topics/{}", content_base_url(&language), p.topic_id);
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| McpError::internal_error(e.to_string(), None))?;
    let body = resp
        .text()
        .await
        .map_err(|e| McpError::internal_error(e.to_string(), None))?;
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(
        body,
    )]))
}

pub async fn topic_replies(
    p: TopicIdParam,
    language: Option<String>,
) -> Result<CallToolResult, McpError> {
    let url = format!(
        "{}/topics/{}/replies",
        content_base_url(&language),
        p.topic_id
    );
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| McpError::internal_error(e.to_string(), None))?;
    let body = resp
        .text()
        .await
        .map_err(|e| McpError::internal_error(e.to_string(), None))?;
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(
        body,
    )]))
}

pub async fn topic_create(token: &str, p: TopicCreateParam) -> Result<CallToolResult, McpError> {
    let client = create_http_client(token);
    let mut body = serde_json::json!({
        "title": p.title,
        "body": p.body,
    });
    if let Some(symbols) = p.symbols {
        body["symbols"] = serde_json::json!(symbols);
    }
    crate::tools::http_client::http_post_tool(&client, "/v1/social/topic/create", body).await
}

pub async fn topic_create_reply(
    token: &str,
    p: TopicCreateReplyParam,
) -> Result<CallToolResult, McpError> {
    let client = create_http_client(token);
    let body = serde_json::json!({
        "topic_id": p.topic_id,
        "body": p.body,
    });
    crate::tools::http_client::http_post_tool(&client, "/v1/social/topic/reply", body).await
}

pub async fn filing_detail(
    p: FilingDetailParam,
    language: Option<String>,
) -> Result<CallToolResult, McpError> {
    let url = format!("{}/filings/{}", content_base_url(&language), p.filing_id);
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| McpError::internal_error(e.to_string(), None))?;
    let body = resp
        .text()
        .await
        .map_err(|e| McpError::internal_error(e.to_string(), None))?;
    Ok(CallToolResult::success(vec![rmcp::model::Content::text(
        body,
    )]))
}
