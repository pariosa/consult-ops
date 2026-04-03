use sqlx::Executor;
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;

#[derive(Clone)]
pub struct Db {
    pub pool: Arc<SqlitePool>,
}

impl Db {
    /// Create a new Db instance and run migrations
    pub async fn new(database_url: &str) -> sqlx::Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        Self::migrate(&pool).await?;
        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    /// Run the table creation migrations
    async fn migrate(pool: &SqlitePool) -> sqlx::Result<()> {
        let sql = r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS clients (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            tax_id TEXT,
            phone TEXT,
            company_name TEXT,
            address TEXT,
            city TEXT,
            state TEXT,
            zip TEXT,
            country TEXT,
            created_at TEXT,
            updated_at TEXT
        );

        CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            client_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            start_date TEXT,
            end_date TEXT,
            description TEXT,
            created_at TEXT,
            updated_at TEXT,
            FOREIGN KEY(client_id) REFERENCES clients(id)
        );

        CREATE TABLE IF NOT EXISTS contracts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            status TEXT NOT NULL,
            signed_at TEXT,
            start_date TEXT,
            end_date TEXT,
            value REAL,
            currency TEXT,
            terms TEXT,
            notes TEXT,
            external_id TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT,
            FOREIGN KEY(project_id) REFERENCES projects(id)
        );

        CREATE TABLE IF NOT EXISTS invoices (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            contract_id INTEGER NOT NULL,
            invoice_number TEXT NOT NULL,
            status TEXT NOT NULL,
            issued_at TEXT,
            due_date TEXT,
            subtotal REAL,
            tax REAL,
            total REAL,
            currency TEXT,
            notes TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT,
            FOREIGN KEY(contract_id) REFERENCES contracts(id)
        );

        CREATE TABLE IF NOT EXISTS payments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            invoice_id INTEGER NOT NULL,
            paid_at TEXT,
            amount REAL NOT NULL,
            currency TEXT,
            method TEXT,
            reference TEXT,
            notes TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT,
            FOREIGN KEY(invoice_id) REFERENCES invoices(id)
        );
        "#;

        pool.execute(sql).await?;
        Ok(())
    }
}
