# Workstream 1: API Polish — Design Spec

**Goal:** Make salib feel native in the Rust ecosystem. Users should not have to
fight the library to get data in, results out, or types serialized.

**Scope:** 0.1.x patch release. No new analysis methods, no new samplers. Pure
ergonomic surface work on existing public API.

**Non-goals:** PyO3 bindings, gRPC service layer, documentation site, benchmarks.
Those are separate workstreams.

---

## 1. Serde on All Public Types

### Problem

34 of 38 public result/output types lack `Serialize`/`Deserialize`. Users cannot
persist analysis results to JSON, YAML, MessagePack, or any serde-compatible
format. Only 4 sampler config types (`LhsKind`, `LhsSampler`, `SobolDimSet`,
`SobolSampler`) currently derive serde traits.

### Design

Add `Serialize`/`Deserialize` behind an optional `serde` feature flag on each
subcrate. This keeps serde out of the dependency tree for users who don't need
it.

**Per-crate feature flags:**

```toml
# crates/salib-estimators/Cargo.toml
[features]
serde = ["dep:serde", "salib-core/serde"]

[dependencies]
serde = { version = "1", features = ["derive"], optional = true }
```

Same pattern for `salib-surrogate`, `salib-shapley`, `salib-validation`,
`salib-samplers`. The facade crate (`salib`) forwards:

```toml
[features]
serde = [
    "salib-core/serde",
    "salib-samplers?/serde",
    "salib-estimators?/serde",
    "salib-surrogate?/serde",
    "salib-shapley?/serde",
    "salib-validation?/serde",
]
```

**Conditional derives via cfg_attr:**

```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SobolIndices { ... }
```

**Types requiring special serde handling (contain ndarray arrays):**

7 types contain `Array2<f64>` or `Array3<f64>`:

| Type | Crate | ndarray fields |
|------|-------|----------------|
| `SaltelliMatrix` | salib-samplers | a, b, a_b, b_a |
| `MorrisTrajectories` | salib-samplers | trajectories (3D), deltas, factor_order, group_order |
| `FastDesign` | salib-samplers | samples, omegas, phases |
| `OwenMatrix` | salib-samplers | a, b, c, a_c, b_a |
| `PlackettBurmanDesign` | salib-samplers | matrix |
| `ActiveSubspace` | salib-surrogate | eigenvectors |

These require `ndarray` feature `serde` (which uses `ndarray-serde`). Add:

```toml
# crates/salib-samplers/Cargo.toml
[features]
serde = ["dep:serde", "ndarray/serde", "salib-core/serde"]
```

No custom serde impls needed — `ndarray/serde` handles `ArrayBase` types via
the standard derive path.

**Types to add serde derives to (complete list):**

*salib-estimators (22 types):*
- SobolIndices, SobolIndicesWithCi, BootstrapMethod
- JansenIndices, JanonIndices, OwenIndices
- MorrisEffects
- FastIndices, RbdFastIndices
- BorgonovoIndices, PawnIndices, QosaIndices
- DgsmIndices, RegressionIndices
- GivenDataSobolIndices, BootstrapCi
- AnovaTwoWayResult, AnovaThreeWayResult
- GTheoryResult, DStudyPoint
- HdmrResult, DiscrepancyResult
- FractionalFactorialEffects

*salib-surrogate (7 types):*
- PolynomialChaos, SobolFromPce, ActiveSubspace
- MultiIndex, SparseFitDiagnostic
- TruncationScheme, SparseSolver, PolynomialFamily

*salib-shapley (1 type):*
- ShapleyIndices

*salib-validation (2 types):*
- SobolIndicesAnalytic, MorrisEffectsAnalytic

*salib-samplers (5 types already have it, 5 more need it):*
- SaltelliMatrix, MorrisTrajectories, FastDesign, OwenMatrix, PlackettBurmanDesign

**Note on HdmrResult:** Contains a `PolynomialChaos` field from `salib-surrogate`.
When `serde` is enabled on `salib-estimators`, it must forward to
`salib-surrogate/serde`. This only applies when the `surrogate` feature is also
active. Use:

```rust
#[cfg_attr(
    all(feature = "serde", feature = "surrogate"),
    derive(serde::Serialize, serde::Deserialize)
)]
```

Or restructure `HdmrResult` so the `pce` field is behind `#[cfg(feature = "surrogate")]`.
The latter is already the case — `HdmrResult` is only available when `surrogate`
is enabled, so gating its serde derive on both features is natural.

---

## 2. Display for Result Types

### Problem

Zero `Display` implementations exist. Users must manually format results or rely
on `Debug` output, which is noisy and not presentation-ready.

### Design

Implement `fmt::Display` for all index/result types that a user would want to
print. Output is a clean table, one factor per row.

**Example output for `SobolIndices`:**

```
Sobol' indices (N=8192, d=3)
  Var[Y] = 13.8446

  Factor    S1        ST
  ──────    ──────    ──────
  0         0.3139    0.5576
  1         0.4424    0.4424
  2         0.0000    0.2437
```

When factor names are not embedded in the result type (they live in `Problem`),
the Display impl uses integer indices. A separate method
`display_with_names(&[&str]) -> impl Display` returns a wrapper that uses
the provided names:

```rust
impl SobolIndices {
    pub fn display_with_names<'a>(&'a self, names: &'a [&str]) -> impl fmt::Display + 'a { ... }
}
```

**Types that get Display impls:**

| Type | Format |
|------|--------|
| `SobolIndices` | Table: factor, S1, ST, (S2 if present) |
| `SobolIndicesWithCi` | Table: factor, S1 [CI], ST [CI] |
| `MorrisEffects` | Table: factor, μ, μ*, σ |
| `FastIndices` | Table: factor, S, ST |
| `RbdFastIndices` | Table: factor, S |
| `BorgonovoIndices` | Table: factor, δ |
| `PawnIndices` | Table: factor, median, mean, max, min, CV |
| `DgsmIndices` | Table: factor, ν, ST_upper |
| `QosaIndices` | Table: factor, S (+ α, quantile, CTE header) |
| `RegressionIndices` | Table: factor, SRC, SRRC, PCC, PRCC (+ R² footer) |
| `GivenDataSobolIndices` | Table: factor, S1 |
| `ShapleyIndices` | Table: factor, Sh |
| `JansenIndices` | Table: factor, S1 (+ S2 if present) |
| `JanonIndices` | Table: factor, S1 (+ S2 if present) |
| `OwenIndices` | Table: factor, S1 (+ S2 if present) |
| `DiscrepancyResult` | Key-value: CD, WD, MD, L2* |
| `GTheoryResult` | Variance components + G/Φ coefficients |
| `AnovaTwoWayResult` | ANOVA table: source, SS, MS, F, p |
| `AnovaThreeWayResult` | ANOVA table: source, SS, MS, F, p |
| `FractionalFactorialEffects` | Table: factor, main effect, |main effect| |
| `SobolFromPce` | Table: factor, S1, ST |
| `HdmrResult` | Table: factor, S1, ST (+ order variances) |
| `BootstrapCi` | Table: factor, CI_low, CI_high |

**Not getting Display:** `PolynomialChaos` (too complex for a table),
`ActiveSubspace` (eigenvector matrix is not tabular), `MultiIndex` (internal),
sampler output types (matrices, not summary results). These keep `Debug` only.

---

## 3. ndarray View Acceptance

### Problem

All 23 public estimator/surrogate functions accept `&Array2<f64>` or
`&Array3<f64>`, requiring owned arrays. Users with existing `ArrayView2` data
must clone into owned arrays.

### Design

Change parameter types from `&ArrayN<f64>` to `ArrayViewN<'_, f64>`. This is
**backward compatible** — `&Array2<f64>` auto-coerces to `ArrayView2` via ndarray's
`Deref`/`AsRef` implementations. Existing user code compiles unchanged.

**23 functions to update:**

*salib-estimators (20 functions):*
- `estimate_anova_two_way(grid: ArrayView2<'_, f64>)`
- `estimate_anova_two_way_with_bootstrap(grid: ArrayView2<'_, f64>, ...)`
- `bootstrap_anova_two_way(grid: ArrayView2<'_, f64>, ...)`
- `estimate_anova_three_way(grid: ArrayView3<'_, f64>)`
- `estimate_anova_three_way_with_bootstrap(grid: ArrayView3<'_, f64>, ...)`
- `bootstrap_anova_three_way(grid: ArrayView3<'_, f64>, ...)`
- `estimate_borgonovo_delta(x: ArrayView2<'_, f64>, y: &[f64])`
- `estimate_rbd_fast(x: ArrayView2<'_, f64>, y: &[f64], ...)`
- `estimate_regression_indices(x: ArrayView2<'_, f64>, y: &[f64])`
- `estimate_dgsm(gradients: ArrayView2<'_, f64>, ...)`
- `finite_difference_gradients(samples: ArrayView2<'_, f64>, ...)`
- `estimate_pawn(x: ArrayView2<'_, f64>, y: &[f64], ...)`
- `estimate_hdmr(x: ArrayView2<'_, f64>, y: &[f64], ...)`
- `estimate_given_data_sobol(x: ArrayView2<'_, f64>, y: &[f64])`
- `bootstrap_given_data(x: ArrayView2<'_, f64>, y: &[f64], ...)`
- `estimate_qosa(x: ArrayView2<'_, f64>, y: &[f64], ...)`
- `estimate_g_theory_pir(grid: ArrayView3<'_, f64>, ...)`
- `estimate_g_theory_pir_with_bootstrap(grid: ArrayView3<'_, f64>, ...)`
- `bootstrap_g_theory_pir(grid: ArrayView3<'_, f64>, ...)`
- `compute_discrepancy(sample: ArrayView2<'_, f64>)`

*salib-surrogate (3 functions):*
- `fit_full_pce(samples_canonical: ArrayView2<'_, f64>, y: &[f64], ...)`
- `fit_sparse_pce(samples_canonical: ArrayView2<'_, f64>, y: &[f64], ...)`
- `compute_active_subspace(gradients: ArrayView2<'_, f64>, ...)`

**`&[f64]` parameters stay as-is.** They already accept `Vec<f64>`, arrays, and
slices naturally. Changing to `ArrayView1` would hurt ergonomics.

**`bootstrap_given_data` closure parameter** also accepts `&Array2<f64>` in the
closure signature. This needs to change to `ArrayView2` as well for consistency.

**Internal call sites:** Functions that internally call `.view()`, `.slice()`,
or index into the array will need minor adjustments — `ArrayView2` methods are
nearly identical to `&Array2` methods, but some method chains may need `.reborrow()`.

---

## 4. Rayon Feature Gate

### Problem

`rayon` is a mandatory dependency of `salib-core`. The `par_tree_sum`,
`par_tree_dot`, and `par_tree_var` functions use `rayon::slice::ParallelSlice`.
This blocks WASM and embedded targets.

### Design

Gate rayon behind a `parallel` feature flag, **default on**.

```toml
# crates/salib-core/Cargo.toml
[features]
default = ["parallel"]
parallel = ["dep:rayon"]

[dependencies]
rayon = { version = "1", optional = true }
```

**Serial fallbacks:** Each `par_tree_*` function gets a serial equivalent. Use
`cfg` to dispatch:

```rust
#[cfg(feature = "parallel")]
pub fn par_tree_sum(data: &[f64]) -> f64 {
    // existing rayon implementation
}

#[cfg(not(feature = "parallel"))]
pub fn par_tree_sum(data: &[f64]) -> f64 {
    // sequential tree-structured sum (same accumulation order for determinism)
}
```

The serial fallback must use the **same tree-structured accumulation** as the
parallel version to preserve bit-determinism. The only difference is sequential
vs parallel traversal of the tree levels.

**Facade forwarding:**

```toml
# crates/salib/Cargo.toml
[features]
default = ["samplers", "estimators", "parallel"]
parallel = ["salib-core/parallel"]
```

**Testing:** All existing tests must pass with `--no-default-features` (parallel
disabled) and produce **identical numerical results** to the parallel path.

---

## 5. Arrow and Polars Conversions

### Problem

Researchers working with columnar data (Arrow, Parquet, Polars DataFrames) must
manually extract fields from result types into their preferred format.

### Design

Optional `arrow` and `polars` feature flags on the **facade crate only**. These
provide `From`/`Into` conversions from result types to Arrow `RecordBatch` and
Polars `DataFrame`.

```toml
# crates/salib/Cargo.toml
[features]
arrow = ["dep:arrow"]
polars = ["dep:polars", "arrow"]

[dependencies]
arrow = { version = "54", optional = true, default-features = false, features = ["ffi"] }
polars = { version = "0.46", optional = true, default-features = false, features = ["lazy"] }
```

**Conversion module:** `salib/src/convert.rs`

```rust
#[cfg(feature = "arrow")]
pub mod arrow;

#[cfg(feature = "polars")]
pub mod polars;
```

**Arrow conversions** produce a `RecordBatch` with one column per index/metric.
For factor-indexed results (most estimator types), rows correspond to factors:

```rust
// SobolIndices → RecordBatch
// Columns: "factor" (UInt32), "S1" (Float64), "ST" (Float64)
// Optional: "S2_i_j" columns if second_order is present

impl From<&SobolIndices> for RecordBatch { ... }
```

**Polars conversions** wrap the Arrow conversion:

```rust
impl From<&SobolIndices> for DataFrame {
    fn from(indices: &SobolIndices) -> Self {
        let batch: RecordBatch = indices.into();
        DataFrame::from(batch)
    }
}
```

**Types that get conversions (factor-indexed results):**

- SobolIndices, SobolIndicesWithCi
- JansenIndices, JanonIndices, OwenIndices
- MorrisEffects
- FastIndices, RbdFastIndices
- BorgonovoIndices, PawnIndices, QosaIndices
- DgsmIndices, RegressionIndices
- GivenDataSobolIndices, BootstrapCi
- ShapleyIndices
- SobolFromPce, HdmrResult
- FractionalFactorialEffects

**Types that do NOT get conversions:**

- ANOVA/G-theory results (table structure, not factor-indexed)
- DiscrepancyResult (scalar, not tabular)
- PolynomialChaos, ActiveSubspace (model objects, not index tables)
- Sampler output types (large matrices, not summary results)

**With-names variant:** Like Display, conversions accept optional factor names:

```rust
pub fn sobol_to_record_batch(indices: &SobolIndices, names: Option<&[&str]>) -> RecordBatch { ... }
```

When names are provided, the "factor" column is `Utf8` instead of `UInt32`.

---

## 6. salib-core Serde Feature (Prerequisite)

`salib-core` public types (`Problem`, `Factor`, `Distribution`, `RngState`)
already derive `Serialize`/`Deserialize` unconditionally. To make serde optional
across the workspace, `salib-core` needs the same feature-gate treatment:

```toml
# crates/salib-core/Cargo.toml
[features]
default = ["parallel"]
parallel = ["dep:rayon"]
serde = ["dep:serde"]

[dependencies]
serde = { version = "1", features = ["derive"], optional = true }
```

This is a **breaking change** for anyone currently relying on `salib-core`'s
unconditional serde derives. Since we're at 0.1.x and just published, this is
acceptable. The facade crate's `serde` feature enables it transitively.

**Alternative:** Keep serde unconditional in `salib-core` (it's small, and Problem
serialization is a core use case). Only gate it in the heavier crates. This avoids
the breaking change.

**Recommendation:** Keep serde unconditional in `salib-core`. Gate it in estimators,
surrogate, shapley, validation, and samplers. The core types are small and serde
is genuinely load-bearing for Problem definition (JSON config files, etc.).

---

## Dependency Summary

| Feature flag | Added deps | Which crates |
|-------------|-----------|--------------|
| `serde` (default off) | serde 1.x | estimators, surrogate, shapley, validation, samplers |
| `parallel` (default on) | rayon 1.x | core |
| `arrow` (default off) | arrow 54.x | facade only |
| `polars` (default off) | polars 0.46.x | facade only |

No new mandatory dependencies. All new deps are optional and feature-gated.

---

## Testing Strategy

1. **Serde round-trip tests:** For every type that gains serde derives, add a test
   that serializes to JSON and deserializes back, asserting equality. These tests
   live in each subcrate, gated behind `#[cfg(feature = "serde")]`.

2. **Display snapshot tests:** For each Display impl, assert the formatted string
   matches an expected snapshot. Use a deterministic fixture (e.g., Ishigami with
   seed `[0u8; 32]`, N=1024).

3. **View compatibility:** Existing tests already pass owned arrays. Add one test
   per function that passes an `ArrayView2` explicitly to verify the new signatures
   work.

4. **Parallel parity:** Run the full test suite with `--no-default-features` and
   diff numerical results against the parallel run. Bit-identical required.

5. **Arrow/Polars round-trip:** Serialize a result to RecordBatch/DataFrame, read
   back column values, assert they match the original struct fields.

6. **Feature matrix CI:** Test these combinations:
   - `--no-default-features` (minimal: no serde, no parallel, no samplers, no estimators)
   - default features
   - `--all-features`
   - `--features serde`
   - `--features serde,arrow,polars`

---

## Migration / Breaking Changes

- **ndarray views:** Backward compatible. No user code changes needed.
- **Display:** Additive. No breaking changes.
- **Serde:** Additive (feature-gated). No breaking changes.
- **Rayon feature gate:** Breaking for `salib-core` users who don't use the facade
  and rely on `par_tree_*` functions without enabling `parallel`. Mitigated by
  making `parallel` default-on.
- **Arrow/Polars:** Additive (feature-gated). No breaking changes.

**Overall: semver-compatible 0.1.x patch.**
