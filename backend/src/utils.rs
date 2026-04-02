use chrono::NaiveDate;

pub fn parse_date(s: &str) -> NaiveDate {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
}

pub fn format_currency(amount: f64) -> String {
    format!("${:.2}", amount)
}