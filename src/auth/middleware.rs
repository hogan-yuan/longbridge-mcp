use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::auth::token;
use crate::registry::UserRegistry;
use crate::tools::UserIdentity;

/// Auth middleware for MCP endpoints.
///
/// Validates Bearer token from Authorization header and injects `UserIdentity`
/// into request extensions so rmcp's `StreamableHttpService` can forward it
/// to tool handlers via `RequestContext`.
pub async fn mcp_auth_layer(
    mut req: Request,
    next: Next,
    jwt_secret: &[u8],
    registry: &UserRegistry,
) -> Response {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    let bearer_token = match auth_header {
        Some(h) if h.starts_with("Bearer ") => &h[7..],
        _ => {
            return (
                StatusCode::UNAUTHORIZED,
                [("WWW-Authenticate", "Bearer")],
                "missing or invalid Authorization header",
            )
                .into_response();
        }
    };

    let claims = match token::validate_token(jwt_secret, bearer_token, "access") {
        Ok(c) => c,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                [("WWW-Authenticate", "Bearer")],
                "invalid or expired token",
            )
                .into_response();
        }
    };

    if !registry.user_exists(&claims.sub).await {
        return (
            StatusCode::UNAUTHORIZED,
            [("WWW-Authenticate", "Bearer")],
            "user not found",
        )
            .into_response();
    }

    req.extensions_mut().insert(UserIdentity {
        user_id: claims.sub,
    });

    next.run(req).await
}
