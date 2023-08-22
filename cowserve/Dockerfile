FROM rust:1.71.1-slim-buster as builder

WORKDIR /opt/app

COPY . .
RUN cargo build --release

FROM builder AS production

COPY --from=builder /opt/app/target/release/cowsay-bot /usr/local/bin/cowsay-bot
CMD cowsay-bot
