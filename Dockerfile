# Prepare the build environment
FROM lukemathwalker/cargo-chef:latest-rust-1.79.0 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
# Copy the project files & check the lock file
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json

# Build our project dependencies
RUN cargo chef cook --release --recipe-path recipe.json
#RUN cargo sqlx prepare --workspace
COPY . .
ENV SQLX_OFFLINE true
# Build our project
RUN cargo build --release --bin app

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/app app
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./app"]