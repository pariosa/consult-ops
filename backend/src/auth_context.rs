// backend/src/auth_context.rs

use crate::db::Db;
use actix_web::web;
use actix_web::{Error, FromRequest, HttpRequest, dev::Payload, error::ErrorUnauthorized};
use chrono::Utc;
use futures_util::future::{LocalBoxFuture, Ready, ready};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub user_type: String,
    pub jti: String,
    pub exp: usize,
}

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: i64,
    pub email: String,
    pub user_type: String,
    pub jti: String,
}
impl FromRequest for AuthUser {
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization").cloned();
        let db = req.app_data::<web::Data<Db>>().cloned();

        Box::pin(async move {
            let db = db.ok_or_else(|| ErrorUnauthorized("Database unavailable"))?;

            let auth_header =
                auth_header.ok_or_else(|| ErrorUnauthorized("Missing Authorization header"))?;

            let auth_value = auth_header
                .to_str()
                .map_err(|_| ErrorUnauthorized("Invalid Authorization header"))?;

            let token = auth_value
                .strip_prefix("Bearer ")
                .ok_or_else(|| ErrorUnauthorized("Missing bearer token"))?;

            let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

            let decoded = decode::<Claims>(
                token,
                &DecodingKey::from_secret(jwt_secret.as_bytes()),
                &Validation::default(),
            )
            .map_err(|_| ErrorUnauthorized("Invalid or expired token"))?;

            let claims = decoded.claims;

            let user_id = claims
                .sub
                .parse::<i64>()
                .map_err(|_| ErrorUnauthorized("Invalid token subject"))?;

            let now = Utc::now().to_rfc3339();

            let session = sqlx::query_scalar::<_, i64>(
                r#"
                SELECT id
                FROM auth_sessions
                WHERE user_id = $1
                  AND token_jti = $2
                  AND revoked_at IS NULL
                  AND expires_at > $3
                "#,
            )
            .bind(user_id)
            .bind(&claims.jti)
            .bind(&now)
            .fetch_optional(&*db.pool)
            .await
            .map_err(|_| ErrorUnauthorized("Invalid session"))?;

            if session.is_none() {
                return Err(ErrorUnauthorized("Session expired or revoked"));
            }

            let _ = sqlx::query(
                r#"
                UPDATE auth_sessions
                SET last_seen_at = $1
                WHERE token_jti = $2
                "#,
            )
            .bind(&now)
            .bind(&claims.jti)
            .execute(&*db.pool)
            .await;

            Ok(AuthUser {
                id: user_id,
                email: claims.email,
                user_type: claims.user_type,
                jti: claims.jti,
            })
        })
    }
}
