FROM rust:1.76 as toolchain
RUN cargo install cargo-chef --locked
RUN cargo install sqlx-cli --no-default-features --features sqlite
WORKDIR /app

FROM toolchain as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM toolchain as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV DATABASE_URL=sqlite:db.sqlite
RUN sqlx database create
RUN sqlx migrate run
RUN cargo build --release

FROM debian:bookworm-slim as runtime
WORKDIR /app
ENV DATABASE_URL=sqlite:db.sqlite
ENV SERVER_ADDRESS=0.0.0.0
ENV SERVER_PORT=8000
COPY --from=builder /app/target/release/axum-web-test /app/axum-web-test
COPY --from=builder /app/db.sqlite /app/db.sqlite
EXPOSE 8000
ENTRYPOINT [ "/app/axum-web-test" ]
