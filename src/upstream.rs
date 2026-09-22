/// Abstraction for an upstream data source, easy to replace with an HTTP client.
pub trait Upstream: Send + Sync + 'static { fn fetch(&self, key: &str) -> String; }

/// Deterministic mock upstream used by the runnable example and tests.
#[derive(Clone, Default)]
pub struct MockUpstream;

impl Upstream for MockUpstream {
    fn fetch(&self, key: &str) -> String { format!("mock:{key}") }
}
