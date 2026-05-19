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
    status TEXT NOT NULL DEFAULT 'active',
    created_at TEXT,
    updated_at TEXT,
    FOREIGN KEY(organization_id) REFERENCES organizations(id),
    FOREIGN KEY(user_id) REFERENCES users(id),
    UNIQUE(organization_id, user_id)
);
 
CREATE TABLE IF NOT EXISTS organization_invitations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    organization_id INTEGER NOT NULL,
    email TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'member',
    token TEXT NOT NULL UNIQUE,
    status TEXT NOT NULL DEFAULT 'pending',
    invited_by_user_id INTEGER,
    accepted_by_user_id INTEGER,
    expires_at TEXT NOT NULL,
    accepted_at TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(organization_id, email, status)
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
-- engagements table
CREATE TABLE IF NOT EXISTS engagements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    organization_id INTEGER NOT NULL,
    project_id INTEGER NOT NULL,
    engagement_type TEXT NOT NULL DEFAULT 'software',
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
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
--engagement milestones table
CREATE TABLE IF NOT EXISTS engagement_milestones (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    engagement_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    amount_cents INTEGER NOT NULL DEFAULT 0,
    due_date TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);
-- engagement billing table
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

-- operational events table
CREATE TABLE IF NOT EXISTS operational_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    organization_id INTEGER NOT NULL,
    actor_user_id INTEGER NULL,
    entity_type TEXT NOT NULL,
    entity_id INTEGER NOT NULL,
    event_type TEXT NOT NULL,
    from_status TEXT NULL,
    to_status TEXT NULL,
    metadata TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS parties (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    organization_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    email TEXT,
    party_type TEXT NOT NULL,
    is_verified INTEGER NOT NULL DEFAULT 0,

    verification_status TEXT NOT NULL DEFAULT 'unverified',
    verified_at TEXT,
    verification_method TEXT,

    linked_user_id INTEGER,
    linked_client_id INTEGER,
    linked_organization_id INTEGER,

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS operational_agreements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    organization_id INTEGER NOT NULL,
    engagement_id INTEGER,
    title TEXT NOT NULL,
    agreement_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft',
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS agreement_payout_rules (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    agreement_id INTEGER NOT NULL,
    from_party_id INTEGER NOT NULL,
    to_party_id INTEGER NOT NULL,
    rule_type TEXT NOT NULL,
    percent INTEGER,
    amount_cents INTEGER,
    trigger_event TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS operational_transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    organization_id INTEGER NOT NULL,
    agreement_id INTEGER,
    engagement_id INTEGER,
    milestone_id INTEGER,
    from_party_id INTEGER NOT NULL,
    to_party_id INTEGER NOT NULL,
    transaction_type TEXT NOT NULL,
    amount_cents INTEGER NOT NULL,
    currency TEXT NOT NULL DEFAULT 'usd',
    status TEXT NOT NULL DEFAULT 'pending',
    trigger_event TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS notifications (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    organization_id INTEGER NOT NULL,
    user_id INTEGER,
    recipient_email TEXT,
    notification_type TEXT NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    entity_type TEXT,
    entity_id INTEGER,
    read_at TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS notification_jobs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    notification_id INTEGER,
    channel TEXT NOT NULL DEFAULT 'email',
    status TEXT NOT NULL DEFAULT 'pending',
    attempts INTEGER NOT NULL DEFAULT 0,
    last_error TEXT,
    run_after TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE party_payment_profiles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    party_id INTEGER NOT NULL,
    organization_id INTEGER NOT NULL,

    payment_role TEXT NOT NULL, -- payer, payee, both

    stripe_customer_id TEXT,
    stripe_payment_method_id TEXT,
    payer_authorization_status TEXT NOT NULL DEFAULT 'not_configured',
    payer_authorized_at TEXT,
    payer_authorization_scope TEXT, -- single_milestone, engagement, agreement

    stripe_connect_account_id TEXT,
    stripe_connect_onboarding_status TEXT NOT NULL DEFAULT 'not_started',
    payout_status TEXT NOT NULL DEFAULT 'not_ready',
    payout_verified_at TEXT,

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);