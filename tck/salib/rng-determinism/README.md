# TCK — saltelli RNG determinism

The Layer-1 outer Gherkin gate for `saltelli_core::rng` (multi-stream
ChaCha20 with deterministic salt-derived forking) and
`saltelli_core::reduce` (fixed-tree pairwise reductions whose output
is bit-identical regardless of rayon partitioning).

Mirrors the TCK posture in
§ "Layer 1 — Outer
Gherkin TCK." The other three layers (inner property + identity
tests, frozen-CSV SALib differential, convergence-rate + criterion +
cargo-mutants) live inside `crates/saltelli-core/src/` and
`tests/`, not here.

## What this directory covers

- **`multi_stream_chacha.feature`** — `RngState` invariants. Same
  `(seed, stream, word_pos)` produces identical bytes; same parent
  + same salt produces identical fork; distinct salts produce
  distinct forks; `word_pos` snapshot enables resumption byte-for-
  byte. Pins
  § "RngState
  shape and forking semantics."

- **`tree_fold_invariance.feature`** — `tree_sum` / `par_tree_sum` /
  `tree_dot` / `par_tree_dot` are bit-identical regardless of rayon
  thread count `{1, 2, 8, 32}`. Pins the same ADR § "Tree-fold
  reductions and the float-associativity defense."

## What this directory does NOT cover

- **Performance** — covered by criterion benches under
  `crates/saltelli-core/benches/` (Layer 4).
- **Cross-platform reproducibility under FMA / AVX-512** — planned for a future release
  `prior-project-i8q`. The local `cargo xtask reference-ci` builds
  the workspace under `RUSTFLAGS="-C target-feature=-fma"`; the
  GitHub Actions matrix wiring is planned for a future release.
- **`Problem` / `Distribution` / inverse-CDF** — out of scope for
  this PR; covered by `tck/saltelli/problem/` once of
  opens it.
- **Sampler-level determinism** — Sobol' / Saltelli matrix
  determinism is a downstream property covered by
  `tck/saltelli/sobol-sampler/` and `tck/saltelli/saltelli-sampler/`
  in PRs 5–6.

## Step-definition home

Per the workspace convention, step definitions live in the consuming
crate, not under `tck/`. Specifically:

- `crates/saltelli-core/tests/rng_determinism_tck.rs` wires
  `multi_stream_chacha.feature`.
- `crates/saltelli-core/tests/tree_fold_tck.rs` wires
  `tree_fold_invariance.feature`.

## See also

- the ADR
  this directory operationalizes.
- the four-layer
  validation strategy.
- `tck/saltelli/README.md` — saltelli-wide TCK layout.
