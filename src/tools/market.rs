use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;
use rmcp::schemars::JsonSchema;
use rmcp::serde::Deserialize;

use crate::counter::{index_symbol_to_counter_id, symbol_to_counter_id};
use crate::registry::UserRegistry;
use crate::tools::http_client::http_get_tool;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SymbolParam {
    /// Security symbol, e.g. "700.HK"
    pub symbol: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MarketParam {
    /// Market code: HK, US, CN, SG
    pub market: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct BrokerHoldingDailyParam {
    /// Security symbol, e.g. "700.HK"
    pub symbol: String,
    /// Broker participant number
    pub broker_id: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct IndexSymbolParam {
    /// Index symbol, e.g. "HSI.HK"
    pub symbol: String,
}

pub async fn market_status(
    registry: &UserRegistry,
    user_id: &str,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    http_get_tool(&client, "/v1/quote/market-status", &[]).await
}

pub async fn broker_holding(
    registry: &UserRegistry,
    user_id: &str,
    p: SymbolParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = symbol_to_counter_id(&p.symbol);
    http_get_tool(
        &client,
        "/v1/quote/broker-holding",
        &[("counter_id", cid.as_str())],
    )
    .await
}

pub async fn broker_holding_detail(
    registry: &UserRegistry,
    user_id: &str,
    p: SymbolParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = symbol_to_counter_id(&p.symbol);
    http_get_tool(
        &client,
        "/v1/quote/broker-holding/detail",
        &[("counter_id", cid.as_str())],
    )
    .await
}

pub async fn broker_holding_daily(
    registry: &UserRegistry,
    user_id: &str,
    p: BrokerHoldingDailyParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = symbol_to_counter_id(&p.symbol);
    http_get_tool(
        &client,
        "/v1/quote/broker-holding/daily",
        &[
            ("counter_id", cid.as_str()),
            ("parti_number", p.broker_id.as_str()),
        ],
    )
    .await
}

pub async fn ah_premium(
    registry: &UserRegistry,
    user_id: &str,
    p: SymbolParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = symbol_to_counter_id(&p.symbol);
    http_get_tool(
        &client,
        "/v1/quote/ahpremium/klines",
        &[("counter_id", cid.as_str())],
    )
    .await
}

pub async fn ah_premium_intraday(
    registry: &UserRegistry,
    user_id: &str,
    p: SymbolParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = symbol_to_counter_id(&p.symbol);
    http_get_tool(
        &client,
        "/v1/quote/ahpremium/timeshares",
        &[("counter_id", cid.as_str()), ("days", "1")],
    )
    .await
}

pub async fn trade_stats(
    registry: &UserRegistry,
    user_id: &str,
    p: SymbolParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = symbol_to_counter_id(&p.symbol);
    http_get_tool(
        &client,
        "/v1/quote/trades-statistics",
        &[("counter_id", cid.as_str())],
    )
    .await
}

pub async fn anomaly(
    registry: &UserRegistry,
    user_id: &str,
    p: MarketParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let market_upper = p.market.to_uppercase();
    http_get_tool(
        &client,
        "/v1/quote/changes",
        &[("market", market_upper.as_str()), ("category", "0")],
    )
    .await
}

pub async fn constituent(
    registry: &UserRegistry,
    user_id: &str,
    p: IndexSymbolParam,
) -> Result<CallToolResult, McpError> {
    let client = registry.get_http_client(user_id).await?;
    let cid = index_symbol_to_counter_id(&p.symbol);
    http_get_tool(
        &client,
        "/v1/quote/index-constituents",
        &[("counter_id", cid.as_str())],
    )
    .await
}
