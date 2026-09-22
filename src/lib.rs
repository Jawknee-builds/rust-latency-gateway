//! Core components for a small, dependency-free latency gateway.
//!
//! The route table mirrors an Axum application (`Router::new().route(...)`) while
//! using standard-library request/response types so the project builds offline.

pub mod cache;
pub mod http;
pub mod telemetry;
pub mod upstream;

use cache::Cache;
use http::{Method, Request, Response, StatusCode};
use telemetry::Telemetry;
use upstream::Upstream;

/// Application state shared by route handlers, analogous to Axum's `State`.
pub struct App<U: Upstream> {
    pub cache: Cache,
    pub upstream: U,
    pub telemetry: Telemetry,
}

impl<U: Upstream> App<U> {
    pub fn new(cache_capacity: usize, upstream: U) -> Self {
        Self { cache: Cache::new(cache_capacity), upstream, telemetry: Telemetry::default() }
    }

    /// Axum-style route dispatch for `GET /healthz`, `GET /v1/cache/:key`, and
    /// `GET /v1/upstream/:key`. Unknown paths return 404.
    pub fn routes(&self, request: Request) -> Response {
        let started = std::time::Instant::now();
        let response = match (&request.method, request.path.as_str()) {
            (Method::Get, "/healthz") => Response::ok("ok\n"),
            (Method::Get, path) if path.starts_with("/v1/cache/") => {
                let key = &path[11..];
                match self.cache.get(key) {
                    Some(value) => Response::ok(value),
                    None => Response::new(StatusCode::NotFound, "cache miss\n"),
                }
            }
            (Method::Get, path) if path.starts_with("/v1/upstream/") => {
                let key = &path[14..];
                match self.cache.get(key) {
                    Some(value) => Response::ok(value).header("x-cache", "hit"),
                    None => {
                        let value = self.upstream.fetch(key);
                        self.cache.insert(key.to_string(), value.clone());
                        Response::ok(value).header("x-cache", "miss")
                    }
                }
            }
            _ => Response::new(StatusCode::NotFound, "not found\n"),
        };
        self.telemetry.record(&request, &response, started.elapsed());
        response
    }
}
