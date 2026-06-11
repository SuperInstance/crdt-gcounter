use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GCounter {
    counts: HashMap<String, u64>,
}

impl GCounter {
    pub fn new() -> Self {
        Self { counts: HashMap::new() }
    }

    pub fn increment(&mut self, node: &str, delta: u64) {
        *self.counts.entry(node.to_string()).or_insert(0) += delta;
    }

    pub fn value(&self) -> u64 {
        self.counts.values().sum()
    }

    pub fn merge(&mut self, other: &Self) {
        for (k, &v) in &other.counts {
            let entry = self.counts.entry(k.clone()).or_insert(0);
            *entry = (*entry).max(v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_increment_and_value() {
        let mut c = GCounter::new();
        c.increment("a", 3);
        c.increment("b", 5);
        assert_eq!(c.value(), 8);
    }
}
