use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;
use rmcp::schemars::JsonSchema;
use rmcp::serde::Deserialize;

use crate::counter::symbol_to_counter_id;
use crate::registry::UserRegistry;
use crate::tools::http_client::http_get_tool;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SymbolParam {
    /// Security symbol, e.g. "700.HK"
    pub symbol: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct FinancialReportParam {
    /// Security symbol, e.g. "AAPL.US"
    pub symbol: String,
    /// Report type: "annual" or "quarterly"
    pub report_type: Option<String>,
}

pub async fn financial_report(
    registry: &UserRegistry,
    user_id: &str,
    p: FinancialReportParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = symbol_to_counter_id(&p.symbol);
    let mut params: Vec<(&str, &str)> = vec![("counter_id", cid.as_str())];
    let report_type = p.report_type.unwrap_or_default();
    if !report_type.is_empty() {
        params.push(("report", report_type.as_str()));
    }
    http_get_tool(&client, "/v1/quote/financial-reports", &params).await
}

pub async fn institution_rating(
    registry: &UserRegistry,
    user_id: &str,
    p: SymbolParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = symbol_to_counter_id(&p.symbol);
    let params = [("counter_id", cid.as_str())];
    let ratings = http_get_tool(&client, "/v1/quote/institution-rating-latest", &params).await;
    let instratings = http_get_tool(&client, "/v1/quote/institution-ratings", &params).await;
    match (ratings, instratings) {
        (Ok(r), Ok(i)) => {
            let r_text = r
                .content
                .first()
                .and_then(|c| c.as_text())
                .map(|t| t.text.as_str())
                .unwrap_or("null");
            let i_text = i
                .content
                .first()
                .and_then(|c| c.as_text())
                .map(|t| t.text.as_str())
                .unwrap_or("null");
            let combined = format!(r#"{{"analyst":{r_text},"instratings":{i_text}}}"#);
            Ok(CallToolResult::success(vec![rmcp::model::Content::text(
                combined,
            )]))
        }
        (Err(e), _) | (_, Err(e)) => Err(e),
    }
}

pub async fn institution_rating_detail(
    registry: &UserRegistry,
    user_id: &str,
    p: SymbolParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = symbol_to_counter_id(&p.symbol);
    http_get_tool(
        &client,
        "/v1/quote/institution-ratings/detail",
        &[("counter_id", cid.as_str())],
    )
    .await
}

pub async fn dividend(
    registry: &UserRegistry,
    user_id: &str,
    p: SymbolParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = symbol_to_counter_id(&p.symbol);
    http_get_tool(
        &client,
        "/v1/quote/dividends",
        &[("counter_id", cid.as_str())],
    )
    .await
}

pub async fn dividend_detail(
    registry: &UserRegistry,
    user_id: &str,
    p: SymbolParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = symbol_to_counter_id(&p.symbol);
    http_get_tool(
        &client,
        "/v1/quote/dividends/details",
        &[("counter_id", cid.as_str())],
    )
    .await
}

pub async fn forecast_eps(
    registry: &UserRegistry,
    user_id: &str,
    p: SymbolParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = symbol_to_counter_id(&p.symbol);
    http_get_tool(
        &client,
        "/v1/quote/forecast-eps",
        &[("counter_id", cid.as_str())],
    )
    .await
}

pub async fn consensus(
    registry: &UserRegistry,
    user_id: &str,
    p: SymbolParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = symbol_to_counter_id(&p.symbol);
    http_get_tool(
        &client,
        "/v1/quote/financial-consensus-detail",
        &[("counter_id", cid.as_str())],
    )
    .await
}

pub async fn valuation(
    registry: &UserRegistry,
    user_id: &str,
    p: SymbolParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = symbol_to_counter_id(&p.symbol);
    http_get_tool(
        &client,
        "/v1/quote/valuation",
        &[("counter_id", cid.as_str())],
    )
    .await
}

pub async fn valuation_history(
    registry: &UserRegistry,
    user_id: &str,
    p: SymbolParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = symbol_to_counter_id(&p.symbol);
    http_get_tool(
        &client,
        "/v1/quote/valuation/detail",
        &[("counter_id", cid.as_str())],
    )
    .await
}

pub async fn industry_valuation(
    registry: &UserRegistry,
    user_id: &str,
    p: SymbolParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = symbol_to_counter_id(&p.symbol);
    http_get_tool(
        &client,
        "/v1/quote/industry-valuation-comparison",
        &[("counter_id", cid.as_str())],
    )
    .await
}

pub async fn industry_valuation_dist(
    registry: &UserRegistry,
    user_id: &str,
    p: SymbolParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = symbol_to_counter_id(&p.symbol);
    http_get_tool(
        &client,
        "/v1/quote/industry-valuation-distribution",
        &[("counter_id", cid.as_str())],
    )
    .await
}

pub async fn company(
    registry: &UserRegistry,
    user_id: &str,
    p: SymbolParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = symbol_to_counter_id(&p.symbol);
    http_get_tool(
        &client,
        "/v1/quote/comp-overview",
        &[("counter_id", cid.as_str())],
    )
    .await
}

pub async fn executive(
    registry: &UserRegistry,
    user_id: &str,
    p: SymbolParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = symbol_to_counter_id(&p.symbol);
    http_get_tool(
        &client,
        "/v1/quote/company-professionals",
        &[("counter_ids", cid.as_str())],
    )
    .await
}

pub async fn shareholder(
    registry: &UserRegistry,
    user_id: &str,
    p: SymbolParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = symbol_to_counter_id(&p.symbol);
    http_get_tool(
        &client,
        "/v1/quote/shareholders",
        &[("counter_id", cid.as_str())],
    )
    .await
}

pub async fn fund_holder(
    registry: &UserRegistry,
    user_id: &str,
    p: SymbolParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = symbol_to_counter_id(&p.symbol);
    http_get_tool(
        &client,
        "/v1/quote/fund-holders",
        &[("counter_id", cid.as_str())],
    )
    .await
}

pub async fn corp_action(
    registry: &UserRegistry,
    user_id: &str,
    p: SymbolParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = symbol_to_counter_id(&p.symbol);
    http_get_tool(
        &client,
        "/v1/quote/company-act",
        &[
            ("counter_id", cid.as_str()),
            ("req_type", "1"),
            ("version", "3"),
        ],
    )
    .await
}

pub async fn invest_relation(
    registry: &UserRegistry,
    user_id: &str,
    p: SymbolParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = symbol_to_counter_id(&p.symbol);
    http_get_tool(
        &client,
        "/v1/quote/invest-relations",
        &[("counter_id", cid.as_str()), ("count", "0")],
    )
    .await
}

pub async fn operating(
    registry: &UserRegistry,
    user_id: &str,
    p: SymbolParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = symbol_to_counter_id(&p.symbol);
    http_get_tool(
        &client,
        "/v1/quote/operatings",
        &[("counter_id", cid.as_str())],
    )
    .await
}
