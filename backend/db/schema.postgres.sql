-- USERS

CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    name TEXT,
    user_type TEXT NOT NULL DEFAULT 'consultant',

    created_at TEXT,
    updated_at TEXT,

    email_verified_at TEXT,
    disabled_at TEXT,
    last_login_at TEXT,

    mfa_enabled BOOLEAN NOT NULL DEFAULT false,
    mfa_secret_encrypted TEXT,
    password_changed_at TEXT,

    current_organization_id BIGINT
);

-- ORGANIZATIONS

CREATE TABLE IF NOT EXISTS organizations (
    id BIGSERIAL PRIMARY KEY,

    name TEXT NOT NULL,
    slug TEXT UNIQUE,

    created_by_user_id BIGINT REFERENCES users(id),

    created_at TEXT,
    updated_at TEXT
);

ALTER TABLE users
ADD CONSTRAINT fk_users_current_org
FOREIGN KEY(current_organization_id)
REFERENCES organizations(id);

-- EMAIL VERIFICATION TOKENS

CREATE TABLE IF NOT EXISTS email_verification_tokens (
    id BIGSERIAL PRIMARY KEY,

    user_id BIGINT NOT NULL,

    token_hash TEXT NOT NULL UNIQUE,

    expires_at TEXT NOT NULL,
    used_at TEXT,
    created_at TEXT NOT NULL,

    FOREIGN KEY(user_id) REFERENCES users(id)
);

-- AUTH SESSIONS

CREATE TABLE IF NOT EXISTS auth_sessions (
    id BIGSERIAL PRIMARY KEY,

    user_id BIGINT NOT NULL,

    token_jti TEXT NOT NULL UNIQUE,

    revoked_at TEXT,
    expires_at TEXT NOT NULL,
    created_at TEXT NOT NULL,
    last_seen_at TEXT,

    FOREIGN KEY(user_id) REFERENCES users(id)
);

-- AUTH ATTEMPTS

CREATE TABLE IF NOT EXISTS auth_attempts (
    id BIGSERIAL PRIMARY KEY,

    email TEXT,
    ip_address TEXT,

    action TEXT NOT NULL,
    success INTEGER NOT NULL,

    created_at TEXT NOT NULL
);

-- AUDIT EVENTS

CREATE TABLE IF NOT EXISTS audit_events (
    id BIGSERIAL PRIMARY KEY,

    actor_user_id BIGINT,

    event_type TEXT NOT NULL,

    resource_type TEXT,
    resource_id TEXT,

    metadata_json TEXT,

    ip_address TEXT,
    user_agent TEXT,

    created_at TEXT NOT NULL
);

-- OAUTH

CREATE TABLE IF NOT EXISTS oauth_accounts (
    id BIGSERIAL PRIMARY KEY,

    user_id BIGINT NOT NULL,

    provider TEXT NOT NULL,
    provider_user_id TEXT NOT NULL,
    provider_email TEXT,

    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,

    UNIQUE(provider, provider_user_id),

    FOREIGN KEY(user_id) REFERENCES users(id)
);

-- ORGANIZATION MEMBERS

CREATE TABLE IF NOT EXISTS organization_members (
    id BIGSERIAL PRIMARY KEY,

    organization_id BIGINT NOT NULL,
    user_id BIGINT NOT NULL,

    role TEXT NOT NULL DEFAULT 'viewer',
    status TEXT NOT NULL DEFAULT 'active',

    created_at TEXT,
    updated_at TEXT,

    FOREIGN KEY(organization_id) REFERENCES organizations(id),
    FOREIGN KEY(user_id) REFERENCES users(id),

    UNIQUE(organization_id, user_id)
);

-- ORGANIZATION INVITATIONS

CREATE TABLE IF NOT EXISTS organization_invitations (
    id BIGSERIAL PRIMARY KEY,

    organization_id BIGINT NOT NULL,

    email TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'member',

    token TEXT NOT NULL UNIQUE,
    status TEXT NOT NULL DEFAULT 'pending',

    invited_by_user_id BIGINT,
    accepted_by_user_id BIGINT,

    expires_at TEXT NOT NULL,
    accepted_at TEXT,

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

    UNIQUE(organization_id, email, status)
);

-- PASSWORD RESET TOKENS

CREATE TABLE IF NOT EXISTS password_reset_tokens (
    id BIGSERIAL PRIMARY KEY,

    user_id BIGINT NOT NULL,

    token_hash TEXT NOT NULL,

    expires_at TEXT NOT NULL,
    used_at TEXT,
    created_at TEXT NOT NULL,

    FOREIGN KEY(user_id) REFERENCES users(id)
);

-- CLIENTS

CREATE TABLE IF NOT EXISTS clients (
    id BIGSERIAL PRIMARY KEY,

    organization_id BIGINT NOT NULL,

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

-- PROJECTS

CREATE TABLE IF NOT EXISTS projects (
    id BIGSERIAL PRIMARY KEY,

    organization_id BIGINT NOT NULL,
    client_id BIGINT NOT NULL,

    name TEXT NOT NULL,

    start_date TEXT,
    description TEXT,
    end_date TEXT,

    created_at TEXT,
    updated_at TEXT,

    FOREIGN KEY(organization_id) REFERENCES organizations(id),
    FOREIGN KEY(client_id) REFERENCES clients(id)
);

-- CONTRACTS

CREATE TABLE IF NOT EXISTS contracts (
    id BIGSERIAL PRIMARY KEY,

    organization_id BIGINT NOT NULL,
    project_id BIGINT NOT NULL,

    title TEXT NOT NULL,
    status TEXT NOT NULL,

    signed_at TEXT,
    start_date TEXT,
    end_date TEXT,

    value DOUBLE PRECISION,
    currency TEXT,

    terms TEXT,
    notes TEXT,
    external_id TEXT,

    created_at TEXT NOT NULL,
    updated_at TEXT,

    FOREIGN KEY(organization_id) REFERENCES organizations(id),
    FOREIGN KEY(project_id) REFERENCES projects(id)
);

-- INVOICES

CREATE TABLE IF NOT EXISTS invoices (
    id BIGSERIAL PRIMARY KEY,

    organization_id BIGINT NOT NULL,
    contract_id BIGINT NOT NULL,

    invoice_number TEXT NOT NULL,
    status TEXT NOT NULL,

    issued_at TEXT,
    due_date TEXT,

    subtotal DOUBLE PRECISION,
    tax DOUBLE PRECISION,
    total DOUBLE PRECISION,

    currency TEXT,
    notes TEXT,

    created_at TEXT NOT NULL,
    updated_at TEXT,

    FOREIGN KEY(organization_id) REFERENCES organizations(id),
    FOREIGN KEY(contract_id) REFERENCES contracts(id)
);

-- PAYMENTS

CREATE TABLE IF NOT EXISTS payments (
    id BIGSERIAL PRIMARY KEY,

    organization_id BIGINT NOT NULL,
    invoice_id BIGINT NOT NULL,

    paid_at TEXT,

    amount DOUBLE PRECISION NOT NULL,
    currency TEXT,
    method TEXT,
    reference TEXT,
    notes TEXT,

    created_at TEXT NOT NULL,
    updated_at TEXT,

    FOREIGN KEY(organization_id) REFERENCES organizations(id),
    FOREIGN KEY(invoice_id) REFERENCES invoices(id)
);

-- ENGAGEMENTS

CREATE TABLE IF NOT EXISTS engagements (
    id BIGSERIAL PRIMARY KEY,

    organization_id BIGINT NOT NULL,
    project_id BIGINT NOT NULL,

    engagement_type TEXT NOT NULL DEFAULT 'software',

    contractor_name TEXT NOT NULL,
    contractor_email TEXT NOT NULL,

    role TEXT NOT NULL,
    title TEXT NOT NULL,

    scope_of_work TEXT NOT NULL,
    deliverables TEXT,
    repo_url TEXT,

    amount_cents BIGINT NOT NULL,

    currency TEXT NOT NULL DEFAULT 'usd',
    due_date TEXT,

    status TEXT NOT NULL DEFAULT 'draft',
    platform_fee_status TEXT NOT NULL DEFAULT 'pending',

    contract_id BIGINT,
    invoice_id BIGINT,
    payment_id BIGINT,

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- ENGAGEMENT MILESTONES

CREATE TABLE IF NOT EXISTS engagement_milestones (
    id BIGSERIAL PRIMARY KEY,

    engagement_id BIGINT NOT NULL,

    title TEXT NOT NULL,
    description TEXT,

    amount_cents BIGINT NOT NULL DEFAULT 0,

    due_date TEXT,

    status TEXT NOT NULL DEFAULT 'pending',

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT DEFAULT CURRENT_TIMESTAMP
);

-- ENGAGEMENT BILLING

CREATE TABLE IF NOT EXISTS engagement_billing (
    id BIGSERIAL PRIMARY KEY,

    engagement_id BIGINT NOT NULL,
    organization_id BIGINT NOT NULL,

    billing_type TEXT NOT NULL,

    amount_cents BIGINT NOT NULL,

    currency TEXT NOT NULL DEFAULT 'usd',

    status TEXT NOT NULL DEFAULT 'pending',

    stripe_checkout_session_id TEXT,
    stripe_payment_intent_id TEXT,

    paid_at TEXT,

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- OPERATIONAL EVENTS

CREATE TABLE IF NOT EXISTS operational_events (
    id BIGSERIAL PRIMARY KEY,

    organization_id BIGINT NOT NULL,
    actor_user_id BIGINT,

    entity_type TEXT NOT NULL,
    entity_id BIGINT NOT NULL,

    event_type TEXT NOT NULL,

    from_status TEXT,
    to_status TEXT,

    metadata TEXT NOT NULL DEFAULT '{}',

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- PARTIES

CREATE TABLE IF NOT EXISTS parties (
    id BIGSERIAL PRIMARY KEY,

    organization_id BIGINT NOT NULL,

    name TEXT NOT NULL,
    email TEXT,

    party_type TEXT NOT NULL,

    is_verified BIGINT NOT NULL DEFAULT 0,

    verification_status TEXT NOT NULL DEFAULT 'unverified',
    verified_at TEXT,
    verification_method TEXT,

    linked_user_id BIGINT,
    linked_client_id BIGINT,
    linked_organization_id BIGINT,

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- OPERATIONAL AGREEMENTS

CREATE TABLE IF NOT EXISTS operational_agreements (
    id BIGSERIAL PRIMARY KEY,

    organization_id BIGINT NOT NULL,
    engagement_id BIGINT,

    title TEXT NOT NULL,
    agreement_type TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft',

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- AGREEMENT PAYOUT RULES

CREATE TABLE IF NOT EXISTS agreement_payout_rules (
    id BIGSERIAL PRIMARY KEY,

    agreement_id BIGINT NOT NULL,

    from_party_id BIGINT NOT NULL,
    to_party_id BIGINT NOT NULL,

    rule_type TEXT NOT NULL,

    percent BIGINT,
    amount_cents BIGINT,

    trigger_event TEXT NOT NULL,

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- OPERATIONAL TRANSACTIONS

CREATE TABLE IF NOT EXISTS operational_transactions (
    id BIGSERIAL PRIMARY KEY,

    organization_id BIGINT NOT NULL,

    agreement_id BIGINT,
    engagement_id BIGINT,
    milestone_id BIGINT,

    from_party_id BIGINT NOT NULL,
    to_party_id BIGINT NOT NULL,

    transaction_type TEXT NOT NULL,

    amount_cents BIGINT NOT NULL,

    currency TEXT NOT NULL DEFAULT 'usd',

    status TEXT NOT NULL DEFAULT 'pending',

    trigger_event TEXT,

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- NOTIFICATIONS

CREATE TABLE IF NOT EXISTS notifications (
    id BIGSERIAL PRIMARY KEY,

    organization_id BIGINT NOT NULL,
    user_id BIGINT,

    recipient_email TEXT,

    notification_type TEXT NOT NULL,

    title TEXT NOT NULL,
    body TEXT NOT NULL,

    entity_type TEXT,
    entity_id BIGINT,

    read_at TEXT,

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- NOTIFICATION JOBS

CREATE TABLE IF NOT EXISTS notification_jobs (
    id BIGSERIAL PRIMARY KEY,

    notification_id BIGINT,

    channel TEXT NOT NULL DEFAULT 'email',

    status TEXT NOT NULL DEFAULT 'pending',

    attempts INTEGER NOT NULL DEFAULT 0,

    last_error TEXT,

    run_after TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- PARTY PAYMENT PROFILES

CREATE TABLE IF NOT EXISTS party_payment_profiles (
    id BIGSERIAL PRIMARY KEY,

    party_id BIGINT NOT NULL UNIQUE,
    organization_id BIGINT NOT NULL,

    payment_role TEXT NOT NULL,

    stripe_customer_id TEXT,
    stripe_payment_method_id TEXT,

    payer_authorization_status TEXT NOT NULL DEFAULT 'not_configured',
    payer_authorized_at TEXT,
    payer_authorization_scope TEXT,

    stripe_connect_account_id TEXT,
    stripe_connect_onboarding_status TEXT NOT NULL DEFAULT 'not_started',

    payout_status TEXT NOT NULL DEFAULT 'not_ready',
    payout_verified_at TEXT,

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY(party_id) REFERENCES parties(id),
    FOREIGN KEY(organization_id) REFERENCES organizations(id)
);

-- ORGANIZATION SUBSCRIPTIONS

CREATE TABLE IF NOT EXISTS organization_subscriptions (
    id BIGSERIAL PRIMARY KEY,

    organization_id BIGINT NOT NULL UNIQUE,

    subscription_status TEXT NOT NULL DEFAULT 'inactive',
    subscription_plan TEXT NOT NULL DEFAULT 'free',

    stripe_customer_id TEXT,
    stripe_subscription_id TEXT,

    current_period_start TEXT,
    current_period_end TEXT,

    cancel_at_period_end BOOLEAN NOT NULL DEFAULT FALSE,

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (organization_id) REFERENCES organizations(id)
);