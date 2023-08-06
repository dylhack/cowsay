Cowsay bot for Discord with color! (WIP)

# Setup

Docker is recommended for setup.

```sh
docker build . -t cowsay-bot:latest
cp .env.example .env
# fill out the ENV before running.
docker run --env-file ./.env -it cowsay-bot:latest
```

**TODO**

 - [ ] `/cowsay fortune` command (utilize fortune-mod database)
 - [ ] Give the images true transparency instead of a black background

