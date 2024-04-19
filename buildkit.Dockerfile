FROM rust:1.76 as builder
# check query macros
ENV DATABASE_URL=sqlite:db.sqlite
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo install sqlx-cli --no-default-features --features sqlite
WORKDIR /app
COPY . .
RUN sqlx database create
RUN sqlx migrate run
RUN --mount=type=cache,target=/usr/local/cargo/registry --mount=type=cache,target=/app/target cargo build --release && \
    mv /app/target/release/axum-web-test /app


FROM debian:bookworm-slim as runtime
WORKDIR /app
ENV SERVER_ADDRESS=0.0.0.0
ENV SERVER_PORT=8000
COPY --from=builder /app/axum-web-test /app
EXPOSE 8000
ENTRYPOINT [ "/app/axum-web-test" ]
