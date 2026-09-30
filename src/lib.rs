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

/// FNV-1a 64 — the digest every substrate in the SuperInstance fleet agrees on.
pub const FNV_OFFSET: u64 = 0xcbf29ce484222325;
pub const FNV_PRIME: u64 = 0x100000001b3;

#[inline]
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h = FNV_OFFSET;
    for &b in bytes {
        h = (h ^ b as u64).wrapping_mul(FNV_PRIME);
    }
    h
}

/// True if this crate's FNV-1a still agrees with the rest of the fleet.
pub fn canary_holds() -> bool {
    fnv1a64("café Δ 日本語".as_bytes()) == 0x024a555471370b18d
}
