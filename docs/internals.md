# Internals

How salib guarantees bit-deterministic results regardless of thread count, and the engineering decisions behind the parallel implementation.

---

## Bit-determinism

salib produces identical floating-point results for a given seed regardless of how many threads are available. This is not approximate reproducibility — it is exact, bitwise identity.

Most parallel numerical libraries sacrifice determinism because floating-point addition is not associative: $(a + b) + c \neq a + (b + c)$ in IEEE 754. When a thread pool splits work differently across runs, the reduction order changes, and the sum changes.

salib avoids this by never allowing the thread pool to determine reduction order.

## Tree-structured reductions

All parallel reductions in salib use a fixed binary tree structure. Given $N$ elements:

1. Partition into fixed-size leaves (determined by $N$, not by thread count)
2. Reduce each leaf sequentially
3. Merge leaf results pairwise up the tree, always left-to-right

The tree shape is determined entirely by $N$. Whether rayon uses 1 thread or 128, the same pairs get merged in the same order. The result is bitwise identical.

$$\text{reduce}([a, b, c, d, e, f, g, h]) = \text{merge}\!\Big(\text{merge}\big(\text{sum}(a,b), \text{sum}(c,d)\big),\, \text{merge}\big(\text{sum}(e,f), \text{sum}(g,h)\big)\Big)$$

This applies to every accumulation in the library: variance estimates, mean computations, Fourier coefficient sums, KDE evaluations.

## The rayon contract

salib uses rayon behind the `parallel` feature flag (on by default). The contract:

- **rayon decides which threads run which work.** salib does not pin tasks to threads or assume any scheduling order.
- **salib decides which values get combined with which.** The reduction tree is fixed before rayon sees it.
- **Disabling `parallel` produces identical results.** The serial fallback uses the same tree structure with the same merge order. The only difference is wall-clock time.

```toml
# Serial-only build (identical results, single-threaded)
[dependencies]
salib = { version = "0.1", default-features = false, features = ["estimators", "samplers"] }
```

## RNG architecture

All randomness flows through `RngState`, a wrapper around ChaCha20. The design:

- **Seed → RNG → samples** is a pure function. Same seed, same samples, always.
- **Splitting**: when parallel work needs independent streams, `RngState::split()` derives child RNGs deterministically from the parent state. The split is positional — child $k$ always gets the same stream regardless of thread assignment.
- **No thread-local RNG.** Every random draw comes from a specific, deterministic position in the ChaCha stream.

```rust
let mut rng = RngState::from_seed([0u8; 32]);

// These two calls always produce the same samples,
// regardless of thread count or scheduling:
let a = sampler.unit_sample(1024, &mut rng);
let b = sampler.unit_sample(1024, &mut rng);
```

## Variance computation

Numerical stability matters. salib uses Welford's online algorithm for single-pass variance, and the parallel merge variant (Chan et al., 1979) for combining partial results:

$$\bar{x}_{AB} = \frac{n_A \bar{x}_A + n_B \bar{x}_B}{n_A + n_B}$$

$$M_{2,AB} = M_{2,A} + M_{2,B} + \frac{(\bar{x}_B - \bar{x}_A)^2 \cdot n_A \cdot n_B}{n_A + n_B}$$

This avoids catastrophic cancellation in the naive $\mathbb{E}[X^2] - \mathbb{E}[X]^2$ formula and composes correctly in the binary reduction tree.

## Content hashing

`Problem::content_hash()` and `Sampler::config_hash()` produce SHA-256 digests of the problem/sampler configuration. These serve as cache keys and reproducibility checksums — if two runs have the same content hash and the same RNG seed, they produce bitwise identical results.

The hash is computed over a canonical JSON serialization of the configuration, so it is stable across Rust compiler versions and struct field reordering.

## What is NOT guaranteed

- **Cross-platform determinism.** Different CPU architectures may produce different results due to differences in FMA (fused multiply-add) availability. salib is deterministic within a given binary on a given platform.
- **Cross-version determinism.** A future version of salib may change algorithms or reduction order. The guarantee is within a single version.
- **Determinism across feature flags.** Enabling `surrogate` or `shapley` does not change the behavior of core estimators, but the surrogate methods themselves (PCE, HDMR) may use different internal reduction strategies.
