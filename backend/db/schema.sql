PRAGMA foreign_keys = ON;

-- users table
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    name TEXT,
    user_type TEXT NOT NULL DEFAULT 'consultant',
    created_at TEXT,
    updated_at TEXT
);

-- organizations table
CREATE TABLE IF NOT EXISTS organizations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    created_at TEXT,
    updated_at TEXT
);

-- organization members table
CREATE TABLE IF NOT EXISTS organization_members (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    organization_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    role TEXT NOT NULL DEFAULT 'viewer',
    created_at TEXT,
    updated_at TEXT,
    FOREIGN KEY(organization_id) REFERENCES organizations(id),
    FOREIGN KEY(user_id) REFERENCES users(id),
    UNIQUE(organization_id, user_id)
);

-- password reset tokens table
CREATE TABLE IF NOT EXISTS password_reset_tokens (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    token_hash TEXT NOT NULL,
    expires_at TEXT NOT NULL,
    used_at TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY(user_id) REFERENCES users(id)
);

-- clients table
CREATE TABLE IF NOT EXISTS clients (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    organization_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    tax_id TEXT,
    phone TEXT,
    company_name TEXT,
    address TEXT,
    city TEXT,
    state TEXT,
    zip TEXT,
    country TEXT,
    created_at TEXT,
    updated_at TEXT,
    FOREIGN KEY(organization_id) REFERENCES organizations(id)
);

-- projects table
CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    organization_id INTEGER NOT NULL,
    client_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    start_date TEXT,
    description TEXT,
    end_date TEXT,
    created_at TEXT,
    updated_at TEXT,
    FOREIGN KEY(organization_id) REFERENCES organizations(id),
    FOREIGN KEY(client_id) REFERENCES clients(id)
);

-- contracts table
CREATE TABLE IF NOT EXISTS contracts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    organization_id INTEGER NOT NULL,
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
    FOREIGN KEY(organization_id) REFERENCES organizations(id),
    FOREIGN KEY(project_id) REFERENCES projects(id)
);

-- invoices table
CREATE TABLE IF NOT EXISTS invoices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    organization_id INTEGER NOT NULL,
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
    FOREIGN KEY(organization_id) REFERENCES organizations(id),
    FOREIGN KEY(contract_id) REFERENCES contracts(id)
);

-- payments table
CREATE TABLE IF NOT EXISTS payments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    organization_id INTEGER NOT NULL,
    invoice_id INTEGER NOT NULL,
    paid_at TEXT,
    amount REAL NOT NULL,
    currency TEXT,
    method TEXT,
    reference TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT,
    FOREIGN KEY(organization_id) REFERENCES organizations(id),
    FOREIGN KEY(invoice_id) REFERENCES invoices(id)
);

CREATE TABLE IF NOT EXISTS engagements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    organization_id INTEGER NOT NULL,
    project_id INTEGER NOT NULL,
    contractor_name TEXT NOT NULL,
    contractor_email TEXT NOT NULL,
    role TEXT NOT NULL,
    title TEXT NOT NULL,
    scope_of_work TEXT NOT NULL,
    deliverables TEXT,
    repo_url TEXT,
    amount_cents INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'usd',
    due_date TEXT,
    status TEXT NOT NULL DEFAULT 'draft',
    platform_fee_status TEXT NOT NULL DEFAULT 'pending',
    contract_id INTEGER,
    invoice_id INTEGER,
    payment_id INTEGER,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS engagement_milestones (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    engagement_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    amount_cents INTEGER NOT NULL DEFAULT 0,
    due_date TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS engagement_billing (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    engagement_id INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,
    billing_type TEXT NOT NULL,
    amount_cents INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'usd',
    status TEXT NOT NULL DEFAULT 'pending',
    stripe_checkout_session_id TEXT,
    stripe_payment_intent_id TEXT,
    paid_at TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);