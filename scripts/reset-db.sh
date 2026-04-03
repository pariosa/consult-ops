#!/bin/bash

# Path to your SQLite DB
DB_FILE="../backend/db/consult_ops.db"
SCHEMA_FILE="../backend/db/schema.sql"

echo "Deleting existing database..."
rm -f "$DB_FILE"

echo "Creating new database and applying schema..."
sqlite3 "$DB_FILE" < "$SCHEMA_FILE"

sudo chown $USER:$USER ../backend/db/consult_ops.db
chmod 644 ../backend/db/consult_ops.db

echo "Database reset complete!"