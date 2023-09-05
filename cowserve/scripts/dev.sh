#!/bin/bash
source ../.env
export DATABASE_URL
export QUEUE_URL
export SERVER_PORT

cargo run --bin cowserve $1
