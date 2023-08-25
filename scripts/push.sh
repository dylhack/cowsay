#!/bin/bash
if [ -z "$1" ]; then
    echo "Usage: $0 <version>"
    exit 1
fi

docker push ghcr.io/dylhack/cowbot-standalone:$1
docker push ghcr.io/dylhack/cowbot:$1
docker push ghcr.io/dylhack/cowserve:$1
