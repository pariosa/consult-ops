// src/auth.rs
use crate::db::Db;
use actix_web::{HttpResponse, Responder, web};
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use rand::{RngCore, rngs::OsRng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct PasswordResetToken {
    pub id: i64,
    pub user_id: i64,
    pub expires_at: String,
    pub used_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthInfo {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterInfo {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
    pub user_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub password: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password_hash: String,
    pub name: Option<String>,
    pub user_type: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub message: String,
    pub user_id: i64,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub user_type: String,
    pub exp: usize,
}

#[derive(Debug, Serialize)]
pub struct LoginUser {
    pub id: i64,
    pub email: String,
    pub name: Option<String>,
    pub user_type: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: LoginUser,
}

pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| e.to_string())
}

fn verify_password(password: &str, password_hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(password_hash) {
        Ok(hash) => hash,
        Err(_) => return false,
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

fn generate_reset_token() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    hex::encode(bytes)
}

fn hash_token(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}

pub async fn register(db: web::Data<Db>, info: web::Json<RegisterInfo>) -> impl Responder {
    let hash = match hash_password(&info.password) {
        Ok(hash) => hash,
        Err(e) => return HttpResponse::InternalServerError().body(e),
    };

    let result = sqlx::query(
        r#"
        INSERT INTO users (email, password_hash, created_at, updated_at)
        VALUES (?, ?, ?, ?)
        "#,
    )
    .bind(&info.email)
    .bind(&hash)
    .bind(Utc::now().to_rfc3339())
    .bind(Utc::now().to_rfc3339())
    .execute(&*db.pool)
    .await;

    match result {
        Ok(rec) => HttpResponse::Ok().json(AuthResponse {
            message: "User registered".to_string(),
            user_id: rec.last_insert_rowid(),
            email: info.email.clone(),
        }),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

pub async fn login(db: web::Data<Db>, info: web::Json<AuthInfo>) -> impl Responder {
    let user = sqlx::query_as::<_, User>(
        r#"
    SELECT id, email, password_hash, name, user_type
    FROM users
    WHERE email = ?
    "#,
    )
    .bind(&info.email)
    .fetch_one(&*db.pool)
    .await;

    let user = match user {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid email or password"),
    };

    if verify_password(&info.password, &user.password_hash) {
        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "consult-ops-local-dev-secret".to_string());

        let exp = (Utc::now() + Duration::hours(24)).timestamp() as usize;

        let claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            user_type: user.user_type.clone(),
            exp,
        };

        let token = match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_bytes()),
        ) {
            Ok(token) => token,
            Err(err) => {
                return HttpResponse::InternalServerError().body(err.to_string());
            }
        };

        HttpResponse::Ok().json(LoginResponse {
            token,
            user: LoginUser {
                id: user.id,
                email: user.email,
                name: user.name,
                user_type: user.user_type,
            },
        })
    } else {
        HttpResponse::Unauthorized().body("Invalid email or password")
    }
}

pub async fn forgot_password(
    db: web::Data<Db>,
    info: web::Json<ForgotPasswordRequest>,
) -> impl Responder {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, password_hash, name
        FROM users
        WHERE email = ?
        "#,
    )
    .bind(&info.email)
    .fetch_one(&*db.pool)
    .await;

    let user = match user {
        Ok(user) => user,
        Err(_) => {
            return HttpResponse::Ok().body(
                "If an account exists for this email, a password reset link has been generated.",
            );
        }
    };

    let raw_token = generate_reset_token();
    let token_hash = hash_token(&raw_token);
    let expires_at = (Utc::now() + Duration::minutes(30)).to_rfc3339();

    let result = sqlx::query(
        r#"
    INSERT INTO password_reset_tokens
    (user_id, token_hash, expires_at, created_at)
    VALUES (?, ?, ?, ?)
    "#,
    )
    .bind(user.id)
    .bind(&token_hash)
    .bind(&expires_at)
    .bind(Utc::now().to_rfc3339())
    .execute(&*db.pool)
    .await;

    match result {
        Ok(_) => {
            // Dev-only response. Later, email this token instead.
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Password reset token generated",
                "reset_token": raw_token
            }))
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn reset_password(
    db: web::Data<Db>,
    info: web::Json<ResetPasswordRequest>,
) -> impl Responder {
    let token_hash = hash_token(&info.token);

    let reset = sqlx::query_as::<_, PasswordResetToken>(
        r#"
    SELECT id, user_id, expires_at, used_at
    FROM password_reset_tokens
    WHERE token_hash = ?
    "#,
    )
    .bind(&token_hash)
    .fetch_one(&*db.pool)
    .await;

    let reset = match reset {
        Ok(reset) => reset,
        Err(_) => return HttpResponse::Unauthorized().body("Invalid or expired reset token"),
    };

    if reset.used_at.is_some() {
        return HttpResponse::Unauthorized().body("Reset token already used");
    }

    let expires_at = match chrono::DateTime::parse_from_rfc3339(&reset.expires_at) {
        Ok(date) => date.with_timezone(&Utc),
        Err(_) => return HttpResponse::Unauthorized().body("Invalid reset token"),
    };

    if expires_at < Utc::now() {
        return HttpResponse::Unauthorized().body("Reset token expired");
    }

    let new_hash = match hash_password(&info.password) {
        Ok(hash) => hash,
        Err(e) => return HttpResponse::InternalServerError().body(e),
    };

    let result = sqlx::query(
        r#"
    UPDATE users
    SET password_hash = ?, updated_at = ?
    WHERE id = ?
    "#,
    )
    .bind(&new_hash)
    .bind(Utc::now().to_rfc3339())
    .bind(reset.user_id)
    .execute(&*db.pool)
    .await;

    if let Err(e) = result {
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    let _ = sqlx::query(
        r#"
    UPDATE password_reset_tokens
    SET used_at = ?
    WHERE id = ?
    "#,
    )
    .bind(Utc::now().to_rfc3339())
    .bind(reset.id)
    .execute(&*db.pool)
    .await;

    HttpResponse::Ok().body("Password reset successful")
}
