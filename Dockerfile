FROM rust AS builder
WORKDIR /app

COPY . .

RUN cargo build --release


FROM rust
RUN useradd -ms /bin/bash falke


WORKDIR /app

RUN mkdir /data
RUN chown falke:falke /data

COPY --from=builder /app/target/release/api .
COPY --from=builder /app/target/release/reverse_proxy .
COPY --from=builder /app/target/release/migration .
COPY --from=builder /app/target/release/cli .
COPY --from=builder /app/.env .
COPY --from=builder /app/entrypoint.sh .



USER falke
ENTRYPOINT ["./entrypoint.sh"]
