FROM rust:1.71.1-bookworm as base

WORKDIR /opt/cowsay
RUN apt-get update && apt-get install -y protobuf-compiler
COPY . .

FROM base AS cowserve-builder

WORKDIR /opt/cowsay/cowserve
RUN unset DATABASE_URL && cargo build --release

FROM base AS cowserve-production

COPY --from=cowserve-builder /opt/cowsay/cowserve/target/release/cowserve /usr/local/bin/cowserve
CMD cowserve

