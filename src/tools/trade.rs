use rmcp::ErrorData as McpError;
use rmcp::model::CallToolResult;

use crate::error::Error;
use crate::registry::UserRegistry;
use crate::tools::parse;
use crate::tools::{
    CashFlowParam, EstimateMaxQtyParam, HistoryOrdersParam, OrderIdParam, ReplaceOrderParam,
    SubmitOrderParam, SymbolParam, tool_json, tool_result,
};

pub async fn account_balance(
    registry: &UserRegistry,
    user_id: &str,
) -> Result<CallToolResult, McpError> {
    let trade_ctx = registry.get_trade_context(user_id).await?;
    let result = trade_ctx
        .account_balance(None)
        .await
        .map_err(Error::longbridge)?;
    tool_json(&result)
}

pub async fn stock_positions(
    registry: &UserRegistry,
    user_id: &str,
) -> Result<CallToolResult, McpError> {
    let trade_ctx = registry.get_trade_context(user_id).await?;
    let result = trade_ctx
        .stock_positions(None)
        .await
        .map_err(Error::longbridge)?;
    tool_json(&result)
}

pub async fn fund_positions(
    registry: &UserRegistry,
    user_id: &str,
) -> Result<CallToolResult, McpError> {
    let trade_ctx = registry.get_trade_context(user_id).await?;
    let result = trade_ctx
        .fund_positions(None)
        .await
        .map_err(Error::longbridge)?;
    tool_json(&result)
}

pub async fn margin_ratio(
    registry: &UserRegistry,
    user_id: &str,
    p: SymbolParam,
) -> Result<CallToolResult, McpError> {
    let trade_ctx = registry.get_trade_context(user_id).await?;
    let result = trade_ctx
        .margin_ratio(p.symbol)
        .await
        .map_err(Error::longbridge)?;
    tool_json(&result)
}

pub async fn today_orders(
    registry: &UserRegistry,
    user_id: &str,
) -> Result<CallToolResult, McpError> {
    let trade_ctx = registry.get_trade_context(user_id).await?;
    let result = trade_ctx
        .today_orders(None)
        .await
        .map_err(Error::longbridge)?;
    tool_json(&result)
}

pub async fn order_detail(
    registry: &UserRegistry,
    user_id: &str,
    p: OrderIdParam,
) -> Result<CallToolResult, McpError> {
    let trade_ctx = registry.get_trade_context(user_id).await?;
    let result = trade_ctx
        .order_detail(p.order_id)
        .await
        .map_err(Error::longbridge)?;
    tool_json(&result)
}

pub async fn cancel_order(
    registry: &UserRegistry,
    user_id: &str,
    p: OrderIdParam,
) -> Result<CallToolResult, McpError> {
    let trade_ctx = registry.get_trade_context(user_id).await?;
    trade_ctx
        .cancel_order(p.order_id)
        .await
        .map_err(Error::longbridge)?;
    Ok(tool_result("order cancelled".to_string()))
}

pub async fn today_executions(
    registry: &UserRegistry,
    user_id: &str,
) -> Result<CallToolResult, McpError> {
    let trade_ctx = registry.get_trade_context(user_id).await?;
    let result = trade_ctx
        .today_executions(None)
        .await
        .map_err(Error::longbridge)?;
    tool_json(&result)
}

pub async fn history_orders(
    registry: &UserRegistry,
    user_id: &str,
    p: HistoryOrdersParam,
) -> Result<CallToolResult, McpError> {
    let start = parse::parse_rfc3339(&p.start_at)?;
    let end = parse::parse_rfc3339(&p.end_at)?;
    let mut opts = longbridge::trade::GetHistoryOrdersOptions::new()
        .start_at(start)
        .end_at(end);
    if let Some(symbol) = p.symbol {
        opts = opts.symbol(symbol);
    }
    let trade_ctx = registry.get_trade_context(user_id).await?;
    let result = trade_ctx
        .history_orders(opts)
        .await
        .map_err(Error::longbridge)?;
    tool_json(&result)
}

pub async fn history_executions(
    registry: &UserRegistry,
    user_id: &str,
    p: HistoryOrdersParam,
) -> Result<CallToolResult, McpError> {
    let start = parse::parse_rfc3339(&p.start_at)?;
    let end = parse::parse_rfc3339(&p.end_at)?;
    let mut opts = longbridge::trade::GetHistoryExecutionsOptions::new()
        .start_at(start)
        .end_at(end);
    if let Some(symbol) = p.symbol {
        opts = opts.symbol(symbol);
    }
    let trade_ctx = registry.get_trade_context(user_id).await?;
    let result = trade_ctx
        .history_executions(opts)
        .await
        .map_err(Error::longbridge)?;
    tool_json(&result)
}

pub async fn cash_flow(
    registry: &UserRegistry,
    user_id: &str,
    p: CashFlowParam,
) -> Result<CallToolResult, McpError> {
    let start = parse::parse_rfc3339(&p.start_at)?;
    let end = parse::parse_rfc3339(&p.end_at)?;
    let opts = longbridge::trade::GetCashFlowOptions::new(start, end);
    let trade_ctx = registry.get_trade_context(user_id).await?;
    let result = trade_ctx.cash_flow(opts).await.map_err(Error::longbridge)?;
    tool_json(&result)
}

pub async fn submit_order(
    registry: &UserRegistry,
    user_id: &str,
    p: SubmitOrderParam,
) -> Result<CallToolResult, McpError> {
    use longbridge::Decimal;
    use longbridge::trade::{
        OrderSide, OrderType, OutsideRTH, SubmitOrderOptions, TimeInForceType,
    };
    use std::str::FromStr;

    let order_type = p
        .order_type
        .parse::<OrderType>()
        .map_err(|e| McpError::invalid_params(format!("invalid order_type: {e}"), None))?;
    let side = p
        .side
        .parse::<OrderSide>()
        .map_err(|e| McpError::invalid_params(format!("invalid side: {e}"), None))?;
    let quantity = Decimal::from_str(&p.submitted_quantity)
        .map_err(|e| McpError::invalid_params(format!("invalid quantity: {e}"), None))?;
    let tif = p
        .time_in_force
        .parse::<TimeInForceType>()
        .map_err(|e| McpError::invalid_params(format!("invalid time_in_force: {e}"), None))?;

    let mut opts = SubmitOrderOptions::new(p.symbol, order_type, side, quantity, tif);

    if let Some(ref price) = p.submitted_price {
        opts = opts.submitted_price(Decimal::from_str(price).map_err(|e| {
            McpError::invalid_params(format!("invalid submitted_price: {e}"), None)
        })?);
    }
    if let Some(ref price) = p.trigger_price {
        opts =
            opts.trigger_price(Decimal::from_str(price).map_err(|e| {
                McpError::invalid_params(format!("invalid trigger_price: {e}"), None)
            })?);
    }
    if let Some(ref v) = p.limit_offset {
        opts =
            opts.limit_offset(Decimal::from_str(v).map_err(|e| {
                McpError::invalid_params(format!("invalid limit_offset: {e}"), None)
            })?);
    }
    if let Some(ref v) = p.trailing_amount {
        opts = opts.trailing_amount(Decimal::from_str(v).map_err(|e| {
            McpError::invalid_params(format!("invalid trailing_amount: {e}"), None)
        })?);
    }
    if let Some(ref v) = p.trailing_percent {
        opts = opts.trailing_percent(Decimal::from_str(v).map_err(|e| {
            McpError::invalid_params(format!("invalid trailing_percent: {e}"), None)
        })?);
    }
    if let Some(ref date) = p.expire_date {
        opts = opts.expire_date(parse::parse_date(date)?);
    }
    if let Some(ref rth) = p.outside_rth {
        opts = opts
            .outside_rth(rth.parse::<OutsideRTH>().map_err(|e| {
                McpError::invalid_params(format!("invalid outside_rth: {e}"), None)
            })?);
    }

    let trade_ctx = registry.get_trade_context(user_id).await?;
    let result = trade_ctx
        .submit_order(opts)
        .await
        .map_err(Error::longbridge)?;
    tool_json(&result)
}

pub async fn replace_order(
    registry: &UserRegistry,
    user_id: &str,
    p: ReplaceOrderParam,
) -> Result<CallToolResult, McpError> {
    use longbridge::Decimal;
    use longbridge::trade::ReplaceOrderOptions;
    use std::str::FromStr;

    let quantity = Decimal::from_str(&p.quantity)
        .map_err(|e| McpError::invalid_params(format!("invalid quantity: {e}"), None))?;
    let mut opts = ReplaceOrderOptions::new(p.order_id, quantity);
    if let Some(ref v) = p.price {
        opts = opts.price(
            Decimal::from_str(v)
                .map_err(|e| McpError::invalid_params(format!("invalid price: {e}"), None))?,
        );
    }
    if let Some(ref v) = p.trigger_price {
        opts =
            opts.trigger_price(Decimal::from_str(v).map_err(|e| {
                McpError::invalid_params(format!("invalid trigger_price: {e}"), None)
            })?);
    }
    if let Some(ref v) = p.limit_offset {
        opts =
            opts.limit_offset(Decimal::from_str(v).map_err(|e| {
                McpError::invalid_params(format!("invalid limit_offset: {e}"), None)
            })?);
    }
    if let Some(ref v) = p.trailing_amount {
        opts = opts.trailing_amount(Decimal::from_str(v).map_err(|e| {
            McpError::invalid_params(format!("invalid trailing_amount: {e}"), None)
        })?);
    }
    if let Some(ref v) = p.trailing_percent {
        opts = opts.trailing_percent(Decimal::from_str(v).map_err(|e| {
            McpError::invalid_params(format!("invalid trailing_percent: {e}"), None)
        })?);
    }
    let trade_ctx = registry.get_trade_context(user_id).await?;
    trade_ctx
        .replace_order(opts)
        .await
        .map_err(Error::longbridge)?;
    Ok(tool_result("order replaced".to_string()))
}

pub async fn estimate_max_purchase_quantity(
    registry: &UserRegistry,
    user_id: &str,
    p: EstimateMaxQtyParam,
) -> Result<CallToolResult, McpError> {
    use longbridge::Decimal;
    use longbridge::trade::{EstimateMaxPurchaseQuantityOptions, OrderSide, OrderType};
    use std::str::FromStr;

    let side = p
        .side
        .parse::<OrderSide>()
        .map_err(|e| McpError::invalid_params(format!("invalid side: {e}"), None))?;
    let order_type = p
        .order_type
        .parse::<OrderType>()
        .map_err(|e| McpError::invalid_params(format!("invalid order_type: {e}"), None))?;
    let mut opts = EstimateMaxPurchaseQuantityOptions::new(p.symbol, order_type, side);
    if let Some(ref v) = p.price {
        opts = opts.price(
            Decimal::from_str(v)
                .map_err(|e| McpError::invalid_params(format!("invalid price: {e}"), None))?,
        );
    }
    let trade_ctx = registry.get_trade_context(user_id).await?;
    let result = trade_ctx
        .estimate_max_purchase_quantity(opts)
        .await
        .map_err(Error::longbridge)?;
    tool_json(&result)
}
