# Use official Rust image with full toolchain
FROM rust:1.74

# Install SQLite client and dev libs (needed for sqlx compile-time checks)
RUN apt-get update && \
    apt-get install -y sqlite3 libsqlite3-dev && \
    rm -rf /var/lib/apt/lists/*

# Install SQLx CLI for prepare step
RUN cargo install sqlx-cli --no-default-features --features sqlite

# Set working directory
WORKDIR /usr/src/app

# Copy the source code
COPY . .

# Set the database URL for compile-time SQLx macros
ENV DATABASE_URL="sqlite:consult_ops.db"

# Prepare SQLx offline cache (or online if you have a DB file)
RUN cargo sqlx prepare

# Build the release binary
RUN cargo build --release

# Expose the port your backend listens on
EXPOSE 8000

# Run the backend
CMD ["./target/release/backend"]