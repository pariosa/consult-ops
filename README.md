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

3. setup local mailpit

docker volume create consult_ops_mailpit_data

docker run -d \
 --name consult_ops_mailpit \
 -p 127.0.0.1:8025:8025 \
 -p 127.0.0.1:1025:1025 \
 -e MP_DATABASE=/data/mailpit.db \
 -v consult_ops_mailpit_data:/data \
 axllent/mailpit

a. stop start restart or discard a mailpit server:

# Stop receiving mail; keep stored messages

docker stop consult_ops_mailpit

# Start again; old messages return

docker start consult_ops_mailpit

# Delete the container; keep stored messages

docker rm -f consult_ops_mailpit

# Permanently delete the stored mail

# Only after removing the container:

docker volume rm consult_ops_mailpit_data
