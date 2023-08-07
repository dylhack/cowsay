Cowsay bot for Discord with color! (WIP) ([invite URL](https://discord.com/api/oauth2/authorize?client_id=1135038990081347605&permissions=0&scope=applications.commands%20bot))

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

