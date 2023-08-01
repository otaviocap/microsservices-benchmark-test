FROM rust:1.67 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin db-consumer

FROM debian:bullseye-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/db-consumer /app
ENTRYPOINT ["/app/db-consumer"]