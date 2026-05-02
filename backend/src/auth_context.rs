// backend/src/auth_context.rs

use actix_web::{Error, FromRequest, HttpRequest, dev::Payload, error::ErrorUnauthorized};
use futures_util::future::{Ready, ready};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub user_type: String,
    pub exp: usize,
}

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: i64,
    pub email: String,
    pub user_type: String,
}

impl FromRequest for AuthUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let Some(auth_header) = req.headers().get("Authorization") else {
            return ready(Err(ErrorUnauthorized("Missing Authorization header")));
        };

        let Ok(auth_value) = auth_header.to_str() else {
            return ready(Err(ErrorUnauthorized("Invalid Authorization header")));
        };

        let Some(token) = auth_value.strip_prefix("Bearer ") else {
            return ready(Err(ErrorUnauthorized("Missing bearer token")));
        };

        let jwt_secret =
            std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret-change-me".to_string());

        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        );

        match decoded {
            Ok(token_data) => {
                let claims = token_data.claims;

                let Ok(id) = claims.sub.parse::<i64>() else {
                    return ready(Err(ErrorUnauthorized("Invalid token subject")));
                };

                ready(Ok(AuthUser {
                    id,
                    email: claims.email,
                    user_type: claims.user_type,
                }))
            }
            Err(_) => ready(Err(ErrorUnauthorized("Invalid or expired token"))),
        }
    }
}
