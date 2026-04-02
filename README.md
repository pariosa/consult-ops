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