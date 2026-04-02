// db.rs
use sqlx::sqlite::SqlitePool;
use sqlx::Executor;
use std::sync::Arc;

#[derive(Clone)]
pub struct Db {
    pub pool: Arc<SqlitePool>,
}

impl Db {
    /// Create a new Db instance and run migrations
    pub async fn new(database_url: &str) -> sqlx::Result<Self> {
        // Connect to SQLite database
        let pool = SqlitePool::connect(database_url).await?;

        // Run migrations
        Self::migrate(&pool).await?;

        Ok(Self { pool: Arc::new(pool) })
    }

    /// Run the table creation migrations
    async fn migrate(pool: &SqlitePool) -> sqlx::Result<()> {
        // Execute multiple CREATE TABLE statements
        let sql = r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                email TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS clients (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                email TEXT NOT NULL,
                phone TEXT
            );

            CREATE TABLE IF NOT EXISTS projects (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                client_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                start_date TEXT NOT NULL,
                FOREIGN KEY(client_id) REFERENCES clients(id)
            );

            CREATE TABLE IF NOT EXISTS contracts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id INTEGER NOT NULL,
                signed_at TEXT,
                details TEXT,
                FOREIGN KEY(project_id) REFERENCES projects(id)
            );

            CREATE TABLE IF NOT EXISTS invoices (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                contract_id INTEGER NOT NULL,
                due_date TEXT NOT NULL,
                amount REAL NOT NULL,
                FOREIGN KEY(contract_id) REFERENCES contracts(id)
            );

            CREATE TABLE IF NOT EXISTS payments (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                invoice_id INTEGER NOT NULL,
                paid_at TEXT NOT NULL,
                amount REAL NOT NULL,
                FOREIGN KEY(invoice_id) REFERENCES invoices(id)
            );
        "#;

        // Run the SQL
        pool.execute(sql).await?;

        Ok(())
    }
}