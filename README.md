## Dev

```sh
docker compose up --build --watch
```

```sh
curl -X POST -H "Content-Type: application/json" -d '{"message":"hello"}' localhost:8080/publish
```

## Test

```sh
cargo install cargo-watch
```

```sh
cargo watch --exec check --exec test
```
