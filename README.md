# Consult-Ops - Full-Stack Consultancy Engine

## Overview

Consult-Ops is a self-hosted consultant management platform:

- Rust backend with Actix-Web
- SQLite database
- Nuxt3 + Vue3 frontend
- Local PDF + invoice management
- Fully self-contained, zero-cloud hosting

## Local Setup

1. Clone repo:

   ```bash
   git clone https://github.com/yourusername/consult-ops.git
   cd consult-ops

   npm install -g concurrently
    concurrently "cd backend && cargo run" "cd frontend && npm run dev"

   ```

2. setup database

   docker compose up -d postgres

   docker exec -i consult_ops_postgres psql -U consult_ops -d consult_ops < backend/db/schema.postgres.sql

if this happens in the test environment:
database "consult_ops_test" does not exist

make sure to run these docker commands :

docker exec -it consult_ops_postgres createdb -U consult_ops consult_ops_test

docker exec -i consult_ops_postgres psql \
 -U consult_ops \
 -d consult_ops_test \
 < backend/db/schema.postgres.sql
