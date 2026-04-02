-- clients table
CREATE TABLE clients (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    phone TEXT
);

-- projects table
CREATE TABLE projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    client_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    start_date TEXT,
    FOREIGN KEY(client_id) REFERENCES clients(id)
);

-- contracts table
CREATE TABLE contracts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    signed_at TEXT,
    details TEXT,
    FOREIGN KEY(project_id) REFERENCES projects(id)
);

-- invoices table
CREATE TABLE invoices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    contract_id INTEGER NOT NULL,
    due_date TEXT,
    amount REAL,
    FOREIGN KEY(contract_id) REFERENCES contracts(id)
);

-- payments table
CREATE TABLE payments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    invoice_id INTEGER NOT NULL,
    paid_at TEXT,
    amount REAL,
    FOREIGN KEY(invoice_id) REFERENCES invoices(id)
);

-- users table (for auth)
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL
);