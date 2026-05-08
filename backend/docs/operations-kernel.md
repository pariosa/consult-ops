# Operations Kernel

All meaningful business lifecycle changes must pass through the operations kernel.

No handler, frontend PATCH route, Stripe webhook, or admin action should directly mutate engagement status.

The Kernel is responsible for:

1. validating the current state
2. applying the allowed transition
3. enforcing payment/contract/milestone rules
4. recording an operational event
5. dispatching notifications later

Tonight’s backend progress

We shifted the app from basic CRUD toward an operational certainty system.

Core idea

The app should not just store engagements, contracts, payments, and milestones. It should validate whether important business actions are allowed, record what happened, and create an auditable operational history.

Added / started

1. Engagement state machine

Added a central lifecycle model for engagements.

Example states:

draft
pending_signature
awaiting_payment
active
milestone_review
overdue
suspended
completed
cancelled
disputed

Example events:

contract_sent
contract_signed
payment_received
milestone_submitted
milestone_approved
complete
cancel
dispute

Purpose:

No engagement should move to a new lifecycle state unless the transition is valid. 2. Operations kernel service

Added an operations kernel layer so lifecycle changes go through one central business logic path.

New intended flow:

handler
→ operations kernel
→ state transition validation
→ event recording
→ database update
→ response

This prevents random handlers or frontend PATCH requests from directly mutating important lifecycle status.

3. Operational event logging

Added the operational_events table to create an audit/event history.

SQLite-safe version:

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
created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

Purpose:

Record who/what changed, from which status, to which status, and when. 4. New event timeline endpoints

Added/started handlers for:

GET /api/engagements/{id}/events
GET /api/organizations/{id}/events

These will power future timelines, admin dashboards, activity feeds, audit logs, and operational reporting.

5. Lifecycle route strategy

We decided to keep existing lifecycle-style routes but route them through the kernel.

Important routes:

POST /api/engagements/{id}/mark-contract-sent
POST /api/engagements/{id}/mark-signed
POST /api/engagements/{id}/activate
POST /api/engagements/{id}/complete
POST /api/engagements/{id}/cancel
POST /api/engagements/{id}/dispute
POST /api/milestones/{id}/submit
POST /api/milestones/{id}/approve

Important rule:

PATCH /api/engagements/{id} should not directly change lifecycle status.

PATCH should be for editable fields only.

This is the first step toward making the app more than a workflow tracker.

The app is starting to become:

a system of record for consulting operations

and eventually:

an operational validation layer for contracts, payments, projects, engagements, milestones, and audits
Practical value unlocked

This architecture can support:

audit timelines
state validation
payment enforcement
contract lifecycle tracking
milestone approval history
organization-level operational history
admin dashboards
notifications
deadline automation
dispute protection
Next best steps
Test each lifecycle route manually.
Confirm operational events are inserted.
Add frontend timeline display.
Wire milestone submit/approve into the engagement event system.
Make Stripe webhook call the operations kernel on successful payment.
Add basic organization admin dashboard showing recent operational
