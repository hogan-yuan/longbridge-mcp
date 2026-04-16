use longbridge::httpclient::{HttpClient, Json};
use reqwest::Method;
use rmcp::model::{CallToolResult, Content, ErrorData as McpError};

use crate::error::Error;
use crate::serialize::to_tool_json;

fn result_from_json(value: &serde_json::Value) -> Result<CallToolResult, McpError> {
    let json = to_tool_json(value).map_err(Error::Serialize)?;
    Ok(CallToolResult::success(vec![Content::text(json)]))
}

pub async fn http_get_tool(
    client: &HttpClient,
    path: &str,
    params: &[(&str, &str)],
) -> Result<CallToolResult, McpError> {
    let params: Vec<(&str, &str)> = params.to_vec();
    let resp = client
        .request(Method::GET, path)
        .query_params(params)
        .response::<Json<serde_json::Value>>()
        .send()
        .await
        .map_err(|e| Error::Other(e.to_string()))?;
    result_from_json(&resp.0)
}

pub async fn http_post_tool(
    client: &HttpClient,
    path: &str,
    body: serde_json::Value,
) -> Result<CallToolResult, McpError> {
    let resp = client
        .request(Method::POST, path)
        .body(Json(body))
        .response::<Json<serde_json::Value>>()
        .send()
        .await
        .map_err(|e| Error::Other(e.to_string()))?;
    result_from_json(&resp.0)
}

pub async fn http_delete_tool(
    client: &HttpClient,
    path: &str,
    body: serde_json::Value,
) -> Result<CallToolResult, McpError> {
    let resp = client
        .request(Method::DELETE, path)
        .body(Json(body))
        .response::<Json<serde_json::Value>>()
        .send()
        .await
        .map_err(|e| Error::Other(e.to_string()))?;
    result_from_json(&resp.0)
}
