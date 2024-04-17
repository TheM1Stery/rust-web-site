FROM rust:1.76 as toolchain
RUN cargo install sqlx-cli --no-default-features --features sqlite
WORKDIR /app

FROM toolchain as builder
# check query macros
ENV DATABASE_URL=sqlite:db.sqlite
COPY . .
RUN sqlx database create
RUN sqlx migrate run
RUN cargo build --release


FROM debian:bookworm-slim as runtime
WORKDIR /app
ENV SERVER_ADDRESS=0.0.0.0
ENV SERVER_PORT=8000
COPY --from=builder /app/target/release/axum-web-test /app/axum-web-test
EXPOSE 8000
ENTRYPOINT [ "/app/axum-web-test" ]
