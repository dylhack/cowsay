#!/bin/bash
if [ -z "$1" ]; then
    echo "Usage: $0 <version>"
    exit 1
fi

docker build --target cowbot-standalone-production -t ghcr.io/dylhack/cowbot-standalone:$1 .
docker build --target cowbot-production -t ghcr.io/dylhack/cowbot:$1 .
docker build --target cowserve-production -t ghcr.io/dylhack/cowserve:$1 .
