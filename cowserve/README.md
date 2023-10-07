
Requirements for this project. 

- sqlx-cli
- rustc ~v1.71
- Postgres 15
- RabbitMQ 3

If you have rustup and cargo already...

```sh 
rustup default 1.71.1
cargo install sqlx-cli
```

If you use Docker...

```sh 
docker compose up queue database
```

# Setup 

1. Configure .env
2. Migrate the DB
3. Launch cowserve

```sh
cp .env.example .env
sqlx migrate run
bash scripts/dev.sh
```

