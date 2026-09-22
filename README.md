# rust-latency-gateway

A small, runnable latency-gateway scaffold in Rust. It intentionally has **zero external dependencies**, so it builds offline with only `rustc`/Cargo's standard tooling. The route dispatch and shared state are organized like an Axum application and can be migrated to Axum by replacing the tiny `http` module and TCP adapter.

## Routes

- `GET /healthz` - liveness response.
- `GET /v1/upstream/:key` - cache-first lookup against the deterministic mock upstream.
- `GET /v1/cache/:key` - inspect a cached value.

Responses include `x-cache: hit|miss` for upstream lookups. Telemetry tracks request count, error count, and aggregate microseconds.

## Run

```sh
cargo run --manifest-path rust-latency-gateway/Cargo.toml
curl http://127.0.0.1:3000/v1/upstream/demo
```

Set `LISTEN_ADDR` to bind another address. Run tests with:

```sh
cargo test --manifest-path rust-latency-gateway/Cargo.toml
```

The `Upstream` trait is the seam for replacing `MockUpstream` with a real client when dependencies are available.
