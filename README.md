[![Build & Publish Docker Image](https://github.com/dylhack/cowsay-bot/actions/workflows/main.yml/badge.svg)](https://github.com/dylhack/cowsay-bot/actions/workflows/main.yml)

Cowsay bot for Discord with color! (WIP) ([invite URL](https://discord.com/api/oauth2/authorize?client_id=1135038990081347605&permissions=0&scope=applications.commands%20bot))

# Setup

Docker is recommended for setup.

```sh
docker run -d -e BOT_TOKEN=token ghcr.io/dylhack/cowsay-bot:main
```

**TODO**

 - [ ] `/cowsay fortune` command (utilize fortune-mod database)
 - [ ] Give the images true transparency instead of a black background
 - [x] Support all characters ([resolved](https://github.com/dylhack/cowsay-bot/commit/f97684a20aef158a7546a540939b0d12054702b8))
 - [x] Publish for ARMv7 ([resolved](https://github.com/dylhack/cowsay-bot/commit/514503e68ec78ed4d3fb3c4d4b839725210bb650))
