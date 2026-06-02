// backend/src/utils.rs
use chrono::NaiveDate;

pub fn parse_date(s: &str) -> NaiveDate {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
}

pub fn format_currency(amount: f64) -> String {
    format!("${:.2}", amount)
}

pub fn jwt_secret() -> String {
    match std::env::var("JWT_SECRET") {
        Ok(secret) if !secret.trim().is_empty() => secret,

        _ if cfg!(test) => "test-secret-that-is-long-enough".to_string(),

        _ if std::env::var("APP_ENV").unwrap_or_default() == "development" => {
            "dev-secret-that-is-long-enough".to_string()
        }

        _ => panic!("JWT_SECRET must be set"),
    }
}
