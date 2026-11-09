FROM rust:1.85-slim AS builder

WORKDIR /app

COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/roblox-cloud-pc /usr/local/bin/roblox-cloud-pc
COPY web ./web

ENV PORT=10000

EXPOSE 10000

CMD ["roblox-cloud-pc"]
