# CRDT: G-Counter (Grow-Only Counter)

**A state-based Conflict-free Replicated Data Type (CRDT) for distributed counting** where the counter can only increase. Each replica maintains a local counter and merges by taking the per-node maximum — guaranteeing convergence without coordination.

## Why It Matters

In distributed systems, nodes often need to maintain shared counters (page views, like counts, inventory) even when network partitions prevent communication. Traditional approaches require distributed locks or consensus protocols (Paxos, Raft), which are slow and unavailable during partitions.

CRDTs solve this by exploiting mathematical properties that guarantee eventual consistency. The G-Counter, introduced by Shapiro et al. (2011), is the simplest state-based CRDT. It works by maintaining a separate count per replica, so merges are always safe.

**Real-world usage:** Riak uses G-Counters for distributed counters. Amazon DynamoDB's counters face similar challenges. Redis CRDTs (Redis Enterprise) implement this exact algorithm for geo-replicated counters.

**Key property:** The merge operation is **commutative** (order doesn't matter), **associative** (grouping doesn't matter), and **idempotent** (merging the same state twice is harmless). This means any network topology, any number of merges, any delivery order — the result always converges.

## How It Works

A G-Counter stores a `HashMap<String, u64>` mapping each node's identity to its local count. This is the key insight: rather than a single integer, the counter is a *vector* of per-node counts.

**Increment:** When a node increments, it adds to *only its own* entry in the map. Node "A" increments `counts["A"]`, never touching "B"'s entry. This ensures no information is lost — each node's contribution is preserved independently.

**Query (value):** The total count is the **sum** of all per-node entries: `counts.values().sum()`. This is O(n) where n is the number of replicas, typically small.

**Merge:** To merge two G-Counters, take the **element-wise maximum** of each node's entry: `merged[k] = max(a[k], b[k])`. The maximum is correct because each node's count only grows over time, so the larger value is always the more recent. This operation is O(n) in the number of nodes.

**Why maximum works:** Since each node only increments its own counter, node A's entry in any replica can only increase over time. If replica R1 saw A=5 and replica R2 saw A=8, then A must have incremented between those snapshots, so 8 is the correct (most recent) value. The maximum gives us the latest known state for each node.

## Quick Start

```rust
use crdt_gcounter::GCounter;

let mut node_a = GCounter::new();
let mut node_b = GCounter::new();

// Each node increments independently (network partition is fine)
node_a.increment("a", 3);
node_b.increment("b", 5);

// They merge when connectivity is restored
node_a.merge(&node_b);
assert_eq!(node_a.value(), 8); // 3 + 5 = 8

// Further increments merge correctly
node_a.increment("a", 2);
node_b.increment("b", 1);
node_a.merge(&node_b);
assert_eq!(node_a.value(), 11); // max(3+2, 3) + max(5, 5+1) = 5 + 6
```

## API

### `GCounter`
- `new() -> Self` — Create an empty counter
- `increment(&mut self, node: &str, delta: u64)` — Add `delta` to node's local count. O(1) amortized
- `value(&self) -> u64` — Total count across all nodes. O(n) where n = number of nodes
- `merge(&mut self, other: &Self)` — Merge another counter via per-node maximum. O(n)

## Architecture Notes

The G-Counter is one of several CRDT implementations in SuperInstance, alongside G-Set, PN-Vector, LWW-Register, and OR-Set. These power eventually-consistent state sharing across distributed nodes without requiring consensus protocols.

See the full architecture: [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md)

## License

MIT
