BEGIN;

ALTER TABLE organizations
    ADD COLUMN IF NOT EXISTS visibility TEXT NOT NULL DEFAULT 'private';

ALTER TABLE organizations
    ADD CONSTRAINT organizations_visibility_check
    CHECK (visibility IN ('public', 'private'));

CREATE INDEX IF NOT EXISTS idx_organizations_public
    ON organizations (name, id)
    WHERE visibility = 'public';

CREATE TABLE IF NOT EXISTS organization_join_requests (
    id BIGSERIAL PRIMARY KEY,
    organization_id BIGINT NOT NULL REFERENCES organizations(id),
    user_id BIGINT NOT NULL REFERENCES users(id),
    requested_role TEXT NOT NULL DEFAULT 'member',
    status TEXT NOT NULL DEFAULT 'pending'
        CHECK (status IN ('pending', 'approved', 'rejected', 'cancelled')),
    reviewed_by_user_id BIGINT REFERENCES users(id),
    reviewed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS
    idx_organization_join_requests_one_pending
    ON organization_join_requests (organization_id, user_id)
    WHERE status = 'pending';

CREATE INDEX IF NOT EXISTS
    idx_organization_join_requests_org_status
    ON organization_join_requests (organization_id, status);

CREATE INDEX IF NOT EXISTS
    idx_organization_join_requests_user
    ON organization_join_requests (user_id);

COMMIT;
