#!/bin/bash
source ../.env

if [ "$1" = "standalone" ]; then
    cargo run --bin cowbot --features standalone
else
    cargo run --bin cowbot
fi
