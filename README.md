# CRDT-GCounter — Grow-Only Counter Conflict-Free Replicated Data Type

A Rust implementation of the **G-Counter** (Grow-Only Counter), one of the fundamental state-based Conflict-Free Replicated Data Types (CRDTs). Each replica maintains a vector of per-node counters and convergence is achieved by element-wise max on merge.

## Why It Matters

Distributed systems with weak consistency guarantees — edge databases, collaborative editors, IoT sensor meshes, offline-first mobile apps — need data structures that converge without coordination. CRDTs provide **mathematically guaranteed eventual consistency**: given that all updates eventually propagate, all replicas converge to the same state deterministically.

The G-Counter is the simplest non-trivial CRDT. It only increments (never decrements), making it suitable for:

- **View/download counters** across CDN edge nodes
- **Sensor readings** in IoT networks with intermittent connectivity
- **Vote tallies** in decentralized voting systems
- **Access logs** distributed across geo-replicated servers

## How It Works

### Data Model

Each node *i* maintains a local counter **c[i]** within a vector (represented as `HashMap<String, u64>` in this implementation). The total value is the sum of all entries:

$$V = \sum_{i \in N} c[i]$$

### Increment

When node *i* increments by Δ:

$$c[i] \leftarrow c[i] + \Delta$$

This is a **local mutation** — no network round-trip required.

### Merge

When two replicas *A* and *B* exchange state, they compute the element-wise maximum:

$$c_{\text{merged}}[i] = \max(c_A[i],\ c_B[i]) \quad \forall\ i \in N$$

Since **max** is commutative, associative, and idempotent, the merge operation forms a **semilattice** — the mathematical structure that guarantees convergence.

### Convergence Proof (Sketch)

**Claim:** If all pairs of replicas eventually exchange states, then all replicas converge to the same value.

**Proof:** Each increment is monotonic (counter only grows). The merge function (element-wise max) is idempotent: `merge(s, s) = s`. By the partial order defined by pointwise ≤ on vectors, every sequence of merges converges to the least upper bound (join) of all observed states. ∎

### Complexity

| Operation | Time | Space |
|---|---|---|
| `increment(node, δ)` | O(1) amortized (HashMap insert) | O(1) per entry |
| `value()` | O(n) where n = number of nodes | — |
| `merge(other)` | O(n) | O(n) |

## Quick Start

```toml
[dependencies]
crdt-gcounter = "0.1"
```

```rust
use crdt_gcounter::GCounter;

let mut node_a = GCounter::new();
let mut node_b = GCounter::new();

// Independent local increments
node_a.increment("server-1", 5);
node_b.increment("server-2", 3);

// Merge — no conflict resolution needed
node_a.merge(&node_b);
assert_eq!(node_a.value(), 8);

// Idempotent: merging again changes nothing
node_a.merge(&node_b);
assert_eq!(node_a.value(), 8);
```

## API

### `GCounter`

```rust
pub struct GCounter { /* internal HashMap<String, u64> */ }

impl GCounter {
    pub fn new() -> Self;
    pub fn increment(&mut self, node: &str, delta: u64);
    pub fn value(&self) -> u64;
    pub fn merge(&mut self, other: &Self);
}
```

| Method | Description |
|---|---|
| `new()` | Create an empty counter with no nodes. |
| `increment(node, delta)` | Add `delta` to node `node`'s local counter. |
| `value()` | Return the total: sum of all node counters. |
| `merge(other)` | Pointwise-max merge with another G-Counter. Idempotent, commutative, associative. |

## Architecture Notes

The G-Counter implements the **γ + η = C** principle central to this crate ecosystem:

- **γ (gamma)**: The merge semilattice — the mathematical specification of how states combine (element-wise max). This is the *design contract*.
- **η (eta)**: The `HashMap`-based implementation — the *realization* in code, with all its concrete allocation, hashing, and iteration behavior.
- **C (Configuration)**: The emergent property — **eventual consistency** — that holds when the implementation faithfully realizes the semilattice contract.

When γ (the math) and η (the code) are aligned, C (convergence) is guaranteed. If either is broken — say, a merge that doesn't take the max, or a hash collision causing lost entries — C fails.

The implementation derives `Clone` for state snapshots and `Debug` for observability. The `HashMap` representation supports sparse node sets efficiently, avoiding the fixed-size vector allocation of classical CVCRDT descriptions.

## References

- **Shapiro, M., Preguiça, N., Baquero, C., & Zawirski, M. (2011).** "Conflict-Free Replicated Data Types." *Proc. 17th Int. Symp. on Stabilization, Safety, and Security of Distributed Systems (SSS)*, LNCS 6976, pp. 386–400. Springer. — The seminal CRDT paper defining G-Counter, PN-Counter, and the semilattice framework.
- **Shapiro, M., et al. (2011).** "Convergent and Commutative Replicated Data Types." *Bulletin of the EATCS*, 104, 67–88. — Extended treatment of state-based vs. operation-based CRDTs.
- **Baquero, C., Preguiça, N., & Shapiro, M. (2014).** "Making Operation-Based CRDTs Operation-Based." *Proc. 14th Int. Conf. on Distributed Applications and Interoperable Systems (DAIS)*, LNCS 8460, pp. 126–140. — Distinguishes state-based and op-based merge semantics.
- **Terry, D. B., Theimer, M. M., Petersen, K., Demers, A. J., Spreitzer, M. J., & Hauser, C. H. (1995).** "Managing Update Conflicts: Bayou, a Weakly Connected Replicated Storage System." *Proc. 15th ACM SOSP*, pp. 172–182. — Precursor work on eventual consistency models.
- **Lamport, L. (1978).** "Time, Clocks, and the Ordering of Events in a Distributed System." *Comm. ACM*, 21(7), 558–565. — Logical clocks underpin the vector-clock semantics used in CRDTs.
- **Cormen, T. H., et al. (2022).** *Introduction to Algorithms*, 4th ed., Ch. 11 (Hash Tables). MIT Press. — HashMap complexity analysis.

## License

MIT
