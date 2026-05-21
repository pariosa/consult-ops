use actix_web::{App, web};
use backend::{db::Db, routes};
use sqlx::SqlitePool;
use std::fs;
use std::sync::Arc;
pub async fn setup_test_db() -> Db {
    let pool = SqlitePool::connect(":memory:")
        .await
        .expect("failed to create in-memory sqlite db");

    let schema = fs::read_to_string("db/schema.sql").expect("failed to read schema.sql");

    for statement in schema.split(';') {
        let statement = statement.trim();

        if statement.is_empty() {
            continue;
        }

        sqlx::query(statement)
            .execute(&pool)
            .await
            .unwrap_or_else(|err| {
                panic!(
                    "failed to execute schema statement:\n{}\n\nerror: {}",
                    statement, err
                );
            });
    }

    Db {
        pool: Arc::new(pool),
    }
}

pub fn test_app(
    db: Db,
) -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new()
        .app_data(web::Data::new(db))
        .configure(routes::config)
}
