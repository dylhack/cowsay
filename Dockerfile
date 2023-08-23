FROM rust:1.71.1-bookworm as base

WORKDIR /opt/cowsay
RUN apt-get update && apt-get install -y protobuf-compiler
COPY . .

FROM base AS cowbot-builder

WORKDIR /opt/cowsay/cowbot
RUN cargo build --release

FROM base AS cowbot-standalone-builder

WORKDIR /opt/cowsay/cowbot
RUN cargo build --release --no-default-features

FROM base AS cowserve-builder

WORKDIR /opt/cowsay/cowserve
RUN unset DATABASE_URL && cargo build --release

FROM base AS cowserve-production

COPY --from=cowserve-builder /opt/cowsay/cowserve/target/release/cowserve /usr/local/bin/cowserve
CMD cowserve

FROM base AS cowbot-production

COPY --from=cowbot-builder /opt/cowsay/cowbot/target/release/cowbot /usr/local/bin/cowbot
CMD cowbot

FROM base AS cowbot-standalone-production

COPY --from=cowbot-standalone-builder /opt/cowsay/cowbot/target/release/cowbot /usr/local/bin/cowbot
CMD cowbot
