use crate::http::{Request, Response};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

#[derive(Default)]
pub struct Telemetry { requests: AtomicU64, errors: AtomicU64, total_micros: AtomicU64 }

impl Telemetry {
    pub fn record(&self, _request: &Request, response: &Response, elapsed: Duration) {
        self.requests.fetch_add(1, Ordering::Relaxed);
        if response.status.0 >= 400 { self.errors.fetch_add(1, Ordering::Relaxed); }
        self.total_micros.fetch_add(elapsed.as_micros() as u64, Ordering::Relaxed);
    }
    pub fn snapshot(&self) -> (u64, u64, u64) { (self.requests.load(Ordering::Relaxed), self.errors.load(Ordering::Relaxed), self.total_micros.load(Ordering::Relaxed)) }
}
