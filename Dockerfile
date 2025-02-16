# ========= builder stage =========
FROM rust:1 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# ========= runtime stage =========
FROM debian:bookworm-slim AS runtime

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates openssl && \
    update-ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

RUN useradd -ms /bin/bash falke && \
    mkdir -p /data /certs && \
    chown falke:falke /data


COPY --from=builder /app/target/release/api .
COPY --from=builder /app/target/release/gateway .
COPY --from=builder /app/target/release/migration .
COPY --from=builder /app/target/release/cli .
COPY --from=builder /app/entrypoint.sh .

USER falke
ENTRYPOINT ["./entrypoint.sh"]
