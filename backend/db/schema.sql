-- clients table
CREATE TABLE clients (
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

-- projects table
CREATE TABLE projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    client_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    start_date TEXT,
    description TEXT,
    end_date TEXT,
    created_at TEXT,
    updated_at TEXT,
    FOREIGN KEY(client_id) REFERENCES clients(id)
);

-- contracts table
CREATE TABLE contracts ( 
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
    FOREIGN KEY(project_id) REFERENCES projects(id)
);

-- invoices table
CREATE TABLE invoices (
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
    FOREIGN KEY(contract_id) REFERENCES contracts(id)
);

-- payments table
CREATE TABLE payments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    invoice_id INTEGER NOT NULL,
    paid_at TEXT,
    amount REAL NOT NULL,
    currency TEXT,
    method TEXT,
    reference TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY(invoice_id) REFERENCES invoices(id)
);

-- users table (for auth)
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL
);