FROM rust as builder
WORKDIR /app

COPY . .

RUN cargo build --release


FROM rust
WORKDIR /app

COPY --from=builder /app/target/release/subway_api .
COPY --from=builder /app/target/release/migration .
COPY --from=builder /app/target/release/subway_cli .
COPY --from=builder /app/.env .

USER 1000:1000

CMD ["./subway_api"]