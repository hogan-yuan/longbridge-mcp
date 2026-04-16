use axum::http::StatusCode;
use axum::response::IntoResponse;
use prometheus::{
    Encoder, HistogramVec, IntCounter, IntCounterVec, IntGauge, Opts, Registry, TextEncoder,
};

use std::sync::LazyLock;

static REGISTRY: LazyLock<Registry> = LazyLock::new(Registry::new);

static ACTIVE_SESSIONS: LazyLock<IntGauge> = LazyLock::new(|| {
    let gauge = IntGauge::new("mcp_active_sessions", "Active user sessions in memory").unwrap();
    REGISTRY.register(Box::new(gauge.clone())).unwrap();
    gauge
});

static ACTIVE_QUOTE_CONTEXTS: LazyLock<IntGauge> = LazyLock::new(|| {
    let gauge =
        IntGauge::new("mcp_active_quote_contexts", "Active QuoteContext instances").unwrap();
    REGISTRY.register(Box::new(gauge.clone())).unwrap();
    gauge
});

static ACTIVE_TRADE_CONTEXTS: LazyLock<IntGauge> = LazyLock::new(|| {
    let gauge =
        IntGauge::new("mcp_active_trade_contexts", "Active TradeContext instances").unwrap();
    REGISTRY.register(Box::new(gauge.clone())).unwrap();
    gauge
});

static REGISTERED_USERS_TOTAL: LazyLock<IntGauge> = LazyLock::new(|| {
    let gauge =
        IntGauge::new("mcp_registered_users_total", "Total registered users in DB").unwrap();
    REGISTRY.register(Box::new(gauge.clone())).unwrap();
    gauge
});

static OAUTH_AUTHORIZATIONS_TOTAL: LazyLock<IntCounter> = LazyLock::new(|| {
    let counter = IntCounter::new(
        "mcp_oauth_authorizations_total",
        "Total OAuth authorizations completed",
    )
    .unwrap();
    REGISTRY.register(Box::new(counter.clone())).unwrap();
    counter
});

static TOOL_CALLS_TOTAL: LazyLock<IntCounterVec> = LazyLock::new(|| {
    let counter = IntCounterVec::new(
        Opts::new("mcp_tool_calls_total", "Total tool calls"),
        &["tool_name"],
    )
    .unwrap();
    REGISTRY.register(Box::new(counter.clone())).unwrap();
    counter
});

static TOOL_CALL_ERRORS_TOTAL: LazyLock<IntCounterVec> = LazyLock::new(|| {
    let counter = IntCounterVec::new(
        Opts::new("mcp_tool_call_errors_total", "Total tool call errors"),
        &["tool_name"],
    )
    .unwrap();
    REGISTRY.register(Box::new(counter.clone())).unwrap();
    counter
});

static TOOL_CALL_DURATION: LazyLock<HistogramVec> = LazyLock::new(|| {
    let histogram = HistogramVec::new(
        prometheus::HistogramOpts::new(
            "mcp_tool_call_duration_seconds",
            "Tool call duration in seconds",
        ),
        &["tool_name"],
    )
    .unwrap();
    REGISTRY.register(Box::new(histogram.clone())).unwrap();
    histogram
});

pub fn record_tool_call(tool_name: &str, duration_secs: f64, is_error: bool) {
    TOOL_CALLS_TOTAL.with_label_values(&[tool_name]).inc();
    TOOL_CALL_DURATION
        .with_label_values(&[tool_name])
        .observe(duration_secs);
    if is_error {
        TOOL_CALL_ERRORS_TOTAL.with_label_values(&[tool_name]).inc();
    }
}

pub fn set_active_sessions(count: i64) {
    ACTIVE_SESSIONS.set(count);
}

pub fn set_active_quote_contexts(count: i64) {
    ACTIVE_QUOTE_CONTEXTS.set(count);
}

pub fn set_active_trade_contexts(count: i64) {
    ACTIVE_TRADE_CONTEXTS.set(count);
}

pub fn set_registered_users_total(count: i64) {
    REGISTERED_USERS_TOTAL.set(count);
}

pub fn inc_oauth_authorizations() {
    OAUTH_AUTHORIZATIONS_TOTAL.inc();
}

pub async fn metrics_handler() -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();
    let mut buffer = Vec::new();
    match encoder.encode(&metric_families, &mut buffer) {
        Ok(()) => (
            StatusCode::OK,
            [("content-type", "text/plain; version=0.0.4")],
            buffer,
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            [("content-type", "text/plain; version=0.0.4")],
            format!("encode error: {e}").into_bytes(),
        ),
    }
}
