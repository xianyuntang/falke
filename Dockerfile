FROM rust as builder
WORKDIR /app

COPY . .

RUN cargo build --release


FROM rust
WORKDIR /app

COPY --from=builder /app/target/release/api .
COPY --from=builder /app/target/release/reverse_proxy .
COPY --from=builder /app/target/release/migration .
COPY --from=builder /app/target/release/cli .
COPY --from=builder /app/.env .

USER 1000:1000

CMD ["./subway_api"]