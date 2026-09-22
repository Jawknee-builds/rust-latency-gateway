use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

/// Small thread-safe FIFO cache suitable for the scaffold. Replace with an LRU
/// implementation when cache policy becomes a production requirement.
#[derive(Clone)]
pub struct Cache {
    inner: Arc<Mutex<Inner>>,
    capacity: usize,
}

struct Inner { values: HashMap<String, String>, order: VecDeque<String> }

impl Cache {
    pub fn new(capacity: usize) -> Self {
        Self { inner: Arc::new(Mutex::new(Inner { values: HashMap::new(), order: VecDeque::new() })), capacity }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.inner.lock().expect("cache mutex poisoned").values.get(key).cloned()
    }

    pub fn insert(&self, key: String, value: String) {
        if self.capacity == 0 { return; }
        let mut inner = self.inner.lock().expect("cache mutex poisoned");
        if !inner.values.contains_key(&key) { inner.order.push_back(key.clone()); }
        inner.values.insert(key, value);
        while inner.values.len() > self.capacity {
            if let Some(oldest) = inner.order.pop_front() { inner.values.remove(&oldest); }
        }
    }

    pub fn len(&self) -> usize { self.inner.lock().expect("cache mutex poisoned").values.len() }
}
