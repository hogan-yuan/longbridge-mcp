use std::path::Path;

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::error::Error;

const JWT_SECRET_FILE: &str = "jwt-secret";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// User ID (unique identifier)
    pub sub: String,
    /// Expiration time (UTC timestamp)
    pub exp: u64,
    /// Issued at (UTC timestamp)
    pub iat: u64,
    /// Token type: "access" or "refresh"
    pub typ: String,
}

pub fn load_or_create_jwt_secret(mcp_dir: &Path) -> Result<Vec<u8>, Error> {
    let path = mcp_dir.join(JWT_SECRET_FILE);
    if path.exists() {
        Ok(std::fs::read(&path)?)
    } else {
        let secret: [u8; 32] = rand::random();
        std::fs::write(&path, secret)?;
        Ok(secret.to_vec())
    }
}

pub fn issue_access_token(secret: &[u8], user_id: &str) -> Result<String, Error> {
    let now = jsonwebtoken::get_current_timestamp();
    let claims = Claims {
        sub: user_id.to_string(),
        exp: now + 3600, // 1 hour
        iat: now,
        typ: "access".to_string(),
    };
    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )?;
    Ok(token)
}

pub fn issue_refresh_token(secret: &[u8], user_id: &str) -> Result<String, Error> {
    let now = jsonwebtoken::get_current_timestamp();
    let claims = Claims {
        sub: user_id.to_string(),
        exp: now + 7 * 24 * 3600, // 7 days
        iat: now,
        typ: "refresh".to_string(),
    };
    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )?;
    Ok(token)
}

pub fn validate_token(secret: &[u8], token: &str, expected_type: &str) -> Result<Claims, Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_required_spec_claims(&["sub", "exp", "iat"]);
    let data =
        jsonwebtoken::decode::<Claims>(token, &DecodingKey::from_secret(secret), &validation)?;
    if data.claims.typ != expected_type {
        return Err(Error::OAuth(format!(
            "expected token type '{}', got '{}'",
            expected_type, data.claims.typ
        )));
    }
    Ok(data.claims)
}
