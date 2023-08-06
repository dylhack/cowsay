FROM rust:1.71.1-slim-buster as builder

WORKDIR /opt/app

COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build --release
COPY . .

FROM builder AS production

COPY --from=builder /opt/app/target/release/cowsay-bot /usr/local/bin/cowsay-bot
CMD cowsay-bot
