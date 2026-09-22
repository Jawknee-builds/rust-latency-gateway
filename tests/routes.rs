use rust_latency_gateway::{http::{Method, Request}, upstream::{MockUpstream, Upstream}, App};

fn get(path: &str) -> Request { Request { method: Method::Get, path: path.into() } }

#[test]
fn health_route_is_available() { let app = App::new(2, MockUpstream); assert_eq!(app.routes(get("/healthz")).status.0, 200); }

#[test]
fn upstream_miss_is_cached() {
    let app = App::new(2, MockUpstream);
    let first = app.routes(get("/v1/upstream/alpha"));
    let second = app.routes(get("/v1/upstream/alpha"));
    assert_eq!(first.headers["x-cache"], "miss");
    assert_eq!(second.headers["x-cache"], "hit");
    assert_eq!(second.body, "mock:alpha");
}

#[test]
fn cache_eviction_is_bounded() {
    let app = App::new(1, MockUpstream);
    app.routes(get("/v1/upstream/a")); app.routes(get("/v1/upstream/b"));
    assert_eq!(app.cache.len(), 1);
    assert_eq!(app.routes(get("/v1/cache/a")).status.0, 404);
}

#[derive(Default)] struct CountingUpstream(std::sync::atomic::AtomicUsize);
impl Upstream for CountingUpstream { fn fetch(&self, key: &str) -> String { self.0.fetch_add(1, std::sync::atomic::Ordering::Relaxed); key.into() } }
#[test]
fn telemetry_counts_errors() { let app = App::new(1, CountingUpstream::default()); app.routes(get("/nope")); assert_eq!(app.telemetry.snapshot().0, 1); assert_eq!(app.telemetry.snapshot().1, 1); }
