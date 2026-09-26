# rust-latency-gateway

[![Live on Vercel](https://img.shields.io/badge/Live_Demo-rust--latency--gateway.vercel.app-F97316?style=flat-square&logo=vercel)](https://rust-latency-gateway.vercel.app)
[![CI](https://github.com/Jawknee-builds/rust-latency-gateway/actions/workflows/ci.yml/badge.svg)](https://github.com/Jawknee-builds/rust-latency-gateway/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/rust-stable-orange?style=flat-square)](https://www.rust-lang.org/)
[![Deploy to Render](https://render.com/images/deploy-to-render-button.svg)](https://render.com/deploy?repo=https://github.com/Jawknee-builds/rust-latency-gateway)

> Low-latency API gateway with cache-aside, telemetry, and mock upstream. Built in Rust with zero external dependencies.

A small, runnable latency-gateway scaffold in Rust. The route dispatch and shared state are organized like an Axum application and can be migrated to Axum by replacing the tiny `http` module and TCP adapter. Every component has an explicit seam for swapping mock implementations with real ones.

## Why Rust for an API Gateway?

Python and Node gateways are fast enough for most workloads. The cases where they aren't:
- Sub-millisecond p99 requirements
- High fan-out (100+ concurrent upstream calls)
- Constrained memory budgets (embedded, edge, serverless cold starts)
- Predictable tail latency under GC pressure

This gateway demonstrates the Rust approach to those problems.

## Routes

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/healthz` | Liveness + telemetry snapshot |
| `GET` | `/v1/upstream/:key` | Cache-first upstream lookup |
| `GET` | `/v1/cache/:key` | Inspect cached value |

Responses include `x-cache: hit\|miss` header. The `healthz` endpoint returns live telemetry counters.

## Architecture

```
┌─────────────┐     ┌──────────────┐     ┌─────────────────┐
│   Client    │────▶│  Route Layer │────▶│  Cache (HashMap)│
│             │     │  (http.rs)   │     │  (cache.rs)     │
└─────────────┘     └──────┬───────┘     └────────┬────────┘
                           │ miss                  │
                    ┌──────▼───────┐               │
                    │   Upstream   │               │ hit
                    │ (upstream.rs)│               │
                    └──────┬───────┘     ┌─────────▼────────┐
                           │             │   Telemetry      │
                           └────────────▶│  (telemetry.rs)  │
                                         │  AtomicU64 ctrs  │
                                         └──────────────────┘
```

Components:
- `lib.rs` — shared `AppState`, `Upstream` trait, routing wiring
- `cache.rs` — `HashMap`-backed cache with TTL eviction strategy
- `upstream.rs` — `MockUpstream` (deterministic) + `Upstream` trait for real adapters
- `telemetry.rs` — `AtomicU64` counters: requests, errors, cache hits/misses, total μs
- `http.rs` — minimal TCP listener and request dispatcher (Axum-compatible shape)

## Performance Contracts

| Metric | Target | Methodology |
|--------|--------|-------------|
| Cache hit latency (p50) | < 1 ms | `hyperfine` 1000 sequential requests |
| Cache miss latency (p50) | < 5 ms | Mock upstream, no network |
| Cache miss latency (p99) | < 10 ms | 1000 requests, measure tail |
| Memory (idle, 1 worker) | < 10 MB | `heaptrack` or `/proc/self/status` |
| Throughput (cache hits) | > 10k req/s | `wrk -t4 -c100 -d10s` |

*Benchmarks run with `cargo build --release`. Debug builds are significantly slower.*
*Real measurements pending CI smoke test completion — methodology is documented, numbers are targets.*

## Run Locally

```bash
# Requires Rust stable (install via https://rustup.rs)
cargo build --release
./target/release/rust-latency-gateway

# Test routes
curl http://127.0.0.1:3000/healthz
curl http://127.0.0.1:3000/v1/upstream/demo
curl -v http://127.0.0.1:3000/v1/upstream/demo   # check x-cache: hit on second call

# Run tests
cargo test --verbose

# Format check
cargo fmt --check

# Clippy
cargo clippy --all-targets
```

Set `LISTEN_ADDR` env var to bind another address (default: `127.0.0.1:3000`).

## Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
cargo --version
```

## Extending to Real Upstreams

The `Upstream` trait is the seam for replacing `MockUpstream`:

```rust
pub trait Upstream: Send + Sync {
    fn fetch(&self, key: &str) -> Result<String, UpstreamError>;
}

// Implement for your real HTTP client
pub struct HttpUpstream { base_url: String }
impl Upstream for HttpUpstream {
    fn fetch(&self, key: &str) -> Result<String, UpstreamError> {
        // reqwest, hyper, or ureq call here
        todo!()
    }
}
```

## What's Next

- [ ] Axum migration (drop-in; same shape)
- [ ] Real p50/p99 benchmark results via `wrk`
- [ ] TTL eviction with background sweep
- [ ] Prometheus `/metrics` endpoint
- [ ] Circuit breaker on upstream errors
