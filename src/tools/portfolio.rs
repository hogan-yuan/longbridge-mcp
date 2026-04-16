use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;
use rmcp::schemars::JsonSchema;
use rmcp::serde::Deserialize;

use crate::counter::symbol_to_counter_id;
use crate::registry::UserRegistry;
use crate::tools::http_client::http_get_tool;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ProfitAnalysisDetailParam {
    /// Security symbol, e.g. "700.HK"
    pub symbol: String,
}

pub async fn exchange_rate(
    registry: &UserRegistry,
    user_id: &str,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    http_get_tool(&client, "/v1/asset/exchange_rates", &[]).await
}

pub async fn profit_analysis(
    registry: &UserRegistry,
    user_id: &str,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    http_get_tool(&client, "/v1/portfolio/profit-analysis-summary", &[]).await
}

pub async fn profit_analysis_detail(
    registry: &UserRegistry,
    user_id: &str,
    p: ProfitAnalysisDetailParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = symbol_to_counter_id(&p.symbol);
    http_get_tool(
        &client,
        "/v1/portfolio/profit-analysis/detail",
        &[("counter_id", cid.as_str())],
    )
    .await
}
