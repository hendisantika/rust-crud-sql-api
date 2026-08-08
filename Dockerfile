# ---- base image with the toolchain and native build deps --------------------
FROM rust:1-slim-bookworm AS chef
LABEL authors="hendisantika"
WORKDIR /app
# argonautica builds libargon2 from C sources and needs clang/bindgen
RUN apt-get update && apt-get install -y --no-install-recommends \
        build-essential pkg-config libssl-dev clang llvm-dev libclang-dev \
    && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-chef --locked

# ---- work out the dependency graph -----------------------------------------
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ---- build dependencies (cached), then the app -----------------------------
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
# queries are resolved from the committed .sqlx cache, so no database is needed
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin rust-crud-sql

# ---- runtime ----------------------------------------------------------------
FROM debian:bookworm-slim AS runtime
WORKDIR /app
ENV TZ=Asia/Jakarta
RUN apt-get update && apt-get install -y --no-install-recommends \
        ca-certificates tzdata \
    && ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/rust-crud-sql /usr/local/bin/rust-crud-sql
EXPOSE 8000
CMD ["rust-crud-sql"]
