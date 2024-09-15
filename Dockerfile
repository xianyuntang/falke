FROM rust:1 AS chef
RUN cargo install cargo-chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
RUN useradd -ms /bin/bash falke

RUN mkdir /data
RUN mkdir /certs
RUN chown falke:falke /data

RUN apt update
RUN apt install openssl -y
RUN rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/api .
COPY --from=builder /app/target/release/reverse_proxy .
COPY --from=builder /app/target/release/migration .
COPY --from=builder /app/target/release/cli .
COPY --from=builder /app/entrypoint.sh .

USER falke

ENTRYPOINT ["./entrypoint.sh"]
