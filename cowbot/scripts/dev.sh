#!/bin/bash
source ../.env
export BOT_TOKEN
export CONTACT_URL
export DEV_SERVER_ID
export SERVER_URL

if [ "$1" = "standalone" ]; then
    cargo run --bin cowbot --features standalone
else
    cargo run --bin cowbot
fi
