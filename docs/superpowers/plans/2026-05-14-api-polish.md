# API Polish Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make salib feel native in the Rust ecosystem — serde on all types, Display for human-readable output, ndarray view acceptance, rayon feature-gated, and Arrow/Polars conversions.

**Architecture:** Feature-gated optional traits across all subcrates. serde stays unconditional in salib-core (load-bearing for Problem JSON). rayon gated behind `parallel` (default on) with serial fallbacks preserving bit-determinism. Arrow/Polars conversions live in the facade crate only.

**Tech Stack:** serde 1.x, ndarray 0.16 (serde feature), rayon 1.x, arrow 54.x, polars 0.46.x

---

## File Map

**Modified files:**

| File | Responsibility |
|------|---------------|
| `crates/salib-core/Cargo.toml` | Add `parallel` feature, make rayon optional |
| `crates/salib-core/src/lib.rs` | Conditional re-export of `par_*` functions |
| `crates/salib-core/src/reduce.rs` | Gate rayon import, add serial `par_*` fallbacks |
| `crates/salib-estimators/Cargo.toml` | Add `serde` feature |
| `crates/salib-estimators/src/sobol_indices.rs` | `cfg_attr(serde)` + Display |
| `crates/salib-estimators/src/morris.rs` | `cfg_attr(serde)` + Display |
| `crates/salib-estimators/src/fast.rs` | `cfg_attr(serde)` + Display |
| `crates/salib-estimators/src/borgonovo.rs` | `cfg_attr(serde)` + Display |
| `crates/salib-estimators/src/regression.rs` | `cfg_attr(serde)` + Display |
| `crates/salib-estimators/src/pawn.rs` | `cfg_attr(serde)` + Display |
| `crates/salib-estimators/src/dgsm.rs` | `cfg_attr(serde)` + Display |
| `crates/salib-estimators/src/qosa.rs` | `cfg_attr(serde)` + Display |
| `crates/salib-estimators/src/rbd_fast.rs` | `cfg_attr(serde)` + Display |
| `crates/salib-estimators/src/janon.rs` | `cfg_attr(serde)` + Display |
| `crates/salib-estimators/src/jansen.rs` | `cfg_attr(serde)` + Display |
| `crates/salib-estimators/src/owen.rs` | `cfg_attr(serde)` + Display |
| `crates/salib-estimators/src/given_data_sobol.rs` | `cfg_attr(serde)` + Display |
| `crates/salib-estimators/src/anova.rs` | `cfg_attr(serde)` + Display |
| `crates/salib-estimators/src/g_theory.rs` | `cfg_attr(serde)` + Display |
| `crates/salib-estimators/src/discrepancy.rs` | `cfg_attr(serde)` + Display |
| `crates/salib-estimators/src/fractional_factorial.rs` | `cfg_attr(serde)` + Display |
| `crates/salib-estimators/src/hdmr.rs` | `cfg_attr(serde)` + Display |
| `crates/salib-estimators/src/bootstrap_given_data.rs` | `cfg_attr(serde)` + Display |
| `crates/salib-samplers/Cargo.toml` | Add `serde` feature with `ndarray/serde` |
| `crates/salib-samplers/src/saltelli_matrix.rs` | `cfg_attr(serde)` |
| `crates/salib-samplers/src/morris.rs` | `cfg_attr(serde)` |
| `crates/salib-samplers/src/fast.rs` | `cfg_attr(serde)` |
| `crates/salib-samplers/src/owen_matrix.rs` | `cfg_attr(serde)` |
| `crates/salib-samplers/src/plackett_burman.rs` | `cfg_attr(serde)` |
| `crates/salib-samplers/src/lhs.rs` | Migrate unconditional Serialize to `cfg_attr` |
| `crates/salib-samplers/src/sobol.rs` | Migrate unconditional Serialize to `cfg_attr` |
| `crates/salib-surrogate/Cargo.toml` | Add `serde` feature with `ndarray/serde` |
| `crates/salib-surrogate/src/pce.rs` | `cfg_attr(serde)` + Display for SobolFromPce |
| `crates/salib-surrogate/src/active_subspace.rs` | `cfg_attr(serde)` |
| `crates/salib-surrogate/src/multi_index.rs` | `cfg_attr(serde)` |
| `crates/salib-surrogate/src/sparse_pce.rs` | `cfg_attr(serde)` |
| `crates/salib-surrogate/src/polynomial.rs` | `cfg_attr(serde)` |
| `crates/salib-shapley/Cargo.toml` | Add `serde` feature |
| `crates/salib-shapley/src/estimator.rs` | `cfg_attr(serde)` + Display |
| `crates/salib-validation/Cargo.toml` | Add `serde` feature |
| `crates/salib-validation/src/analytic.rs` | `cfg_attr(serde)` |
| `crates/salib/Cargo.toml` | Add serde/parallel/arrow/polars features |
| `crates/salib/src/lib.rs` | Add convert module |

**Created files:**

| File | Responsibility |
|------|---------------|
| `crates/salib-estimators/tests/serde_roundtrip.rs` | Serde round-trip tests for all estimator types |
| `crates/salib-estimators/tests/display_snapshot.rs` | Display output snapshot tests |
| `crates/salib-samplers/tests/serde_roundtrip.rs` | Serde round-trip tests for sampler types |
| `crates/salib-surrogate/tests/serde_roundtrip.rs` | Serde round-trip tests for surrogate types |
| `crates/salib-shapley/tests/serde_roundtrip.rs` | Serde round-trip test |
| `crates/salib-validation/tests/serde_roundtrip.rs` | Serde round-trip tests |
| `crates/salib/src/convert/arrow.rs` | Arrow RecordBatch conversions |
| `crates/salib/src/convert/polars.rs` | Polars DataFrame conversions |
| `crates/salib/src/convert/mod.rs` | Feature-gated convert module |
| `crates/salib/tests/arrow_roundtrip.rs` | Arrow conversion tests |
| `crates/salib/tests/polars_roundtrip.rs` | Polars conversion tests |

---

### Task 1: Rayon Feature Gate in salib-core

**Files:**
- Modify: `crates/salib-core/Cargo.toml`
- Modify: `crates/salib-core/src/reduce.rs`
- Modify: `crates/salib-core/src/lib.rs`

- [ ] **Step 1: Write the serial parity test**

Add to the bottom of `crates/salib-core/src/reduce.rs`, inside the existing `mod tests`:

```rust
#[test]
fn serial_par_parity_sum() {
    let data: Vec<f64> = (0..10_000).map(|i| (i as f64) * 0.0001).collect();
    let serial = tree_sum(&data);
    let parallel = par_tree_sum(&data);
    assert_eq!(serial.to_bits(), parallel.to_bits());
}

#[test]
fn serial_par_parity_dot() {
    let a: Vec<f64> = (0..10_000).map(|i| (i as f64) * 0.0001).collect();
    let b: Vec<f64> = (0..10_000).map(|i| 1.0 - (i as f64) * 0.00005).collect();
    let serial = tree_dot(&a, &b);
    let parallel = par_tree_dot(&a, &b);
    assert_eq!(serial.to_bits(), parallel.to_bits());
}

#[test]
fn serial_par_parity_var() {
    let data: Vec<f64> = (0..10_000).map(|i| (i as f64) * 0.0001).collect();
    let serial = tree_var(&data);
    let parallel = par_tree_var(&data);
    assert_eq!(serial.to_bits(), parallel.to_bits());
}
```

- [ ] **Step 2: Run parity tests to verify they pass (before gating)**

Run: `cargo test -p salib-core serial_par_parity -- --nocapture`
Expected: 3 tests PASS (serial and parallel are already bit-identical)

- [ ] **Step 3: Gate rayon behind `parallel` feature in Cargo.toml**

Replace the dependencies and add features section in `crates/salib-core/Cargo.toml`:

```toml
[dependencies]
rand = { version = "0.9", default-features = false }
rand_chacha = { version = "0.9", default-features = false }
sha2 = { version = "0.10", default-features = false }
serde = { version = "1", features = ["derive"] }
rayon = { version = "1", optional = true }
statrs = "0.18"
serde_json = "1"
thiserror = "2"

[features]
default = ["parallel"]
parallel = ["dep:rayon"]
```

- [ ] **Step 4: Gate rayon import and add serial fallbacks in reduce.rs**

Replace the entire `use rayon::prelude::*;` line and the three `par_*` functions in `crates/salib-core/src/reduce.rs`:

```rust
#[cfg(feature = "parallel")]
use rayon::prelude::*;
```

Replace `par_tree_sum` (lines 94-101):

```rust
#[must_use]
pub fn par_tree_sum(xs: &[f64]) -> f64 {
    if xs.len() <= BLOCK {
        return tree_sum(xs);
    }
    #[cfg(feature = "parallel")]
    let block_sums: Vec<f64> = xs.par_chunks(BLOCK).map(tree_sum).collect();
    #[cfg(not(feature = "parallel"))]
    let block_sums: Vec<f64> = xs.chunks(BLOCK).map(tree_sum).collect();
    tree_sum(&block_sums)
}
```

Replace `par_tree_dot` (lines 121-133):

```rust
#[must_use]
pub fn par_tree_dot(a: &[f64], b: &[f64]) -> f64 {
    assert_eq!(a.len(), b.len(), "par_tree_dot: length mismatch");
    if a.len() <= BLOCK {
        return tree_dot(a, b);
    }
    #[cfg(feature = "parallel")]
    let block_sums: Vec<f64> = a
        .par_chunks(BLOCK)
        .zip(b.par_chunks(BLOCK))
        .map(|(ac, bc)| tree_dot(ac, bc))
        .collect();
    #[cfg(not(feature = "parallel"))]
    let block_sums: Vec<f64> = a
        .chunks(BLOCK)
        .zip(b.chunks(BLOCK))
        .map(|(ac, bc)| tree_dot(ac, bc))
        .collect();
    tree_sum(&block_sums)
}
```

Replace `par_tree_var` (lines 167-186):

```rust
#[must_use]
pub fn par_tree_var(xs: &[f64]) -> f64 {
    let n = xs.len();
    if n < 2 {
        return 0.0;
    }
    #[allow(clippy::cast_precision_loss)]
    let n_f = n as f64;
    let mean = par_tree_sum(xs) / n_f;
    #[cfg(feature = "parallel")]
    let centered_sq: Vec<f64> = xs
        .par_iter()
        .map(|&x| {
            let d = x - mean;
            d * d
        })
        .collect();
    #[cfg(not(feature = "parallel"))]
    let centered_sq: Vec<f64> = xs
        .iter()
        .map(|&x| {
            let d = x - mean;
            d * d
        })
        .collect();
    #[allow(clippy::cast_precision_loss)]
    let denom = (n - 1) as f64;
    par_tree_sum(&centered_sq) / denom
}
```

- [ ] **Step 5: Forward `parallel` feature from facade crate**

In `crates/salib/Cargo.toml`, update the features section:

```toml
[features]
default = ["samplers", "estimators", "parallel"]
samplers = ["dep:salib-samplers"]
estimators = ["dep:salib-estimators"]
surrogate = ["dep:salib-surrogate", "salib-estimators?/surrogate"]
shapley = ["dep:salib-shapley"]
validation = ["dep:salib-validation"]
parallel = ["salib-core/parallel"]
full = ["samplers", "estimators", "surrogate", "shapley", "validation"]
```

- [ ] **Step 6: Run tests with parallel enabled (default)**

Run: `cargo test -p salib-core`
Expected: All tests PASS

- [ ] **Step 7: Run tests with parallel disabled**

Run: `cargo test -p salib-core --no-default-features`
Expected: All tests PASS with identical numerical results

- [ ] **Step 8: Run full workspace check with no default features**

Run: `cargo check --workspace --no-default-features`
Expected: Compiles clean

- [ ] **Step 9: Commit**

```bash
git add crates/salib-core/Cargo.toml crates/salib-core/src/reduce.rs crates/salib/Cargo.toml
git commit -m "feat(core): gate rayon behind 'parallel' feature (default on)

Serial fallbacks use identical tree-structured accumulation
for bit-determinism parity."
```

---

### Task 2: Serde Feature Flag in salib-estimators

**Files:**
- Modify: `crates/salib-estimators/Cargo.toml`
- Modify: 20 source files (all estimator types)
- Create: `crates/salib-estimators/tests/serde_roundtrip.rs`

- [ ] **Step 1: Add serde feature to Cargo.toml**

In `crates/salib-estimators/Cargo.toml`, add to `[dependencies]`:

```toml
serde = { version = "1", features = ["derive"], optional = true }
```

Update `[features]`:

```toml
[features]
default = []
surrogate = ["dep:salib-surrogate"]
serde = ["dep:serde"]
```

- [ ] **Step 2: Add cfg_attr serde derives to all estimator types**

In each file, add the `cfg_attr` line immediately after the existing `#[derive(...)]` line.

`crates/salib-estimators/src/sobol_indices.rs`:

On `SobolIndices` (line 13), change:
```rust
#[derive(Debug, Clone, PartialEq)]
```
to:
```rust
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

On `SobolIndicesWithCi` (line 60), change:
```rust
#[derive(Debug, Clone, PartialEq)]
```
to:
```rust
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

On `BootstrapMethod` (line 82), change:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
```
to:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-estimators/src/morris.rs` — on `MorrisEffects` (line 72):
```rust
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-estimators/src/fast.rs` — on `FastIndices` (line 64):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-estimators/src/borgonovo.rs` — on `BorgonovoIndices` (line 84):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-estimators/src/regression.rs` — on `RegressionIndices` (line 63):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-estimators/src/pawn.rs` — on `PawnIndices` (line 64):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-estimators/src/dgsm.rs` — on `DgsmIndices` (line 84):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

On `FdKind` (line 202):
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-estimators/src/qosa.rs` — on `QosaIndices` (line 106):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-estimators/src/rbd_fast.rs` — on `RbdFastIndices` (line 64):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-estimators/src/janon.rs` — on `JanonIndices` (line 84):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-estimators/src/jansen.rs` — on `JansenIndices` (line 56):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-estimators/src/owen.rs` — on `OwenIndices` (line 68):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-estimators/src/given_data_sobol.rs` — on `GivenDataSobolIndices` (line 71):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-estimators/src/anova.rs` — on `AnovaTwoWayResult` (line 61):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

On `AnovaThreeWayResult` (line 85):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-estimators/src/g_theory.rs` — on `GTheoryDesign` (line 16):
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

On `GTheoryResult` (line 61):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

On `DStudyPoint` (line 127):
```rust
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-estimators/src/discrepancy.rs` — on `DiscrepancyResult` (line 29):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-estimators/src/fractional_factorial.rs` — on `FractionalFactorialEffects` (line 23):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-estimators/src/bootstrap_given_data.rs` — on `BootstrapCi` (line 105):
```rust
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-estimators/src/hdmr.rs` — on `HdmrResult` (line 48). This type is only compiled when `surrogate` is active, and it contains `PolynomialChaos` from salib-surrogate. Gate serde on both features:
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

Also update `[features]` in Cargo.toml so serde propagates to surrogate when both are active:
```toml
[features]
default = []
surrogate = ["dep:salib-surrogate"]
serde = ["dep:serde", "salib-surrogate?/serde"]
```

- [ ] **Step 3: Verify it compiles without serde**

Run: `cargo check -p salib-estimators`
Expected: Compiles clean (no serde dep pulled in)

- [ ] **Step 4: Verify it compiles with serde**

Run: `cargo check -p salib-estimators --features serde`
Expected: Compiles clean

- [ ] **Step 5: Write serde round-trip tests**

Create `crates/salib-estimators/tests/serde_roundtrip.rs`:

```rust
#![cfg(feature = "serde")]
#![allow(clippy::unwrap_used)]

use salib_estimators::*;

fn roundtrip<T>(val: &T)
where
    T: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug + PartialEq,
{
    let json = serde_json::to_string(val).unwrap();
    let back: T = serde_json::from_str(&json).unwrap();
    assert_eq!(*val, back, "round-trip failed for {json}");
}

fn roundtrip_debug<T>(val: &T)
where
    T: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug,
{
    let json = serde_json::to_string(val).unwrap();
    let back: T = serde_json::from_str(&json).unwrap();
    assert_eq!(format!("{val:?}"), format!("{back:?}"));
}

#[test]
fn sobol_indices_roundtrip() {
    let idx = SobolIndices {
        n: 1024,
        dim: 3,
        total_variance: 13.84,
        first_order: vec![0.31, 0.44, 0.00],
        total_order: vec![0.56, 0.44, 0.24],
        second_order: None,
    };
    roundtrip(&idx);
}

#[test]
fn sobol_indices_with_ci_roundtrip() {
    let idx = SobolIndicesWithCi {
        indices: SobolIndices {
            n: 1024,
            dim: 2,
            total_variance: 10.0,
            first_order: vec![0.5, 0.5],
            total_order: vec![0.5, 0.5],
            second_order: None,
        },
        first_order_ci: vec![(0.4, 0.6), (0.4, 0.6)],
        total_order_ci: vec![(0.4, 0.6), (0.4, 0.6)],
        bootstrap_resamples: 100,
        method: BootstrapMethod::Percentile,
    };
    roundtrip(&idx);
}

#[test]
fn bootstrap_method_roundtrip() {
    roundtrip(&BootstrapMethod::Percentile);
}

#[test]
fn morris_effects_roundtrip() {
    let m = MorrisEffects {
        r: 10,
        d: 3,
        mu: vec![1.0, 2.0, 3.0],
        mu_star: vec![1.0, 2.0, 3.0],
        sigma: vec![0.5, 0.5, 0.5],
        grouped_mu: None,
        grouped_mu_star: None,
        grouped_sigma: None,
        group_names: None,
    };
    roundtrip(&m);
}

#[test]
fn fast_indices_roundtrip() {
    let f = FastIndices {
        s: vec![0.3, 0.4],
        st: vec![0.5, 0.6],
    };
    roundtrip_debug(&f);
}

#[test]
fn borgonovo_indices_roundtrip() {
    let b = BorgonovoIndices {
        delta: vec![0.1, 0.2, 0.3],
    };
    roundtrip_debug(&b);
}

#[test]
fn regression_indices_roundtrip() {
    let r = RegressionIndices {
        src: vec![0.1, 0.2],
        srrc: vec![0.1, 0.2],
        pcc: vec![0.3, 0.4],
        prcc: vec![0.3, 0.4],
        r2_linear: 0.95,
        r2_rank: 0.93,
    };
    roundtrip_debug(&r);
}

#[test]
fn pawn_indices_roundtrip() {
    let p = PawnIndices {
        median: vec![0.1],
        maximum: vec![0.5],
        mean: vec![0.2],
        minimum: vec![0.0],
        cv: vec![0.3],
    };
    roundtrip_debug(&p);
}

#[test]
fn dgsm_indices_roundtrip() {
    let d = DgsmIndices {
        vi: vec![1.0, 2.0],
        st_upper: vec![0.5, 0.8],
    };
    roundtrip_debug(&d);
}

#[test]
fn qosa_indices_roundtrip() {
    let q = QosaIndices {
        s: vec![0.3, 0.7],
        alpha: 0.95,
        global_quantile: 5.0,
        global_cte: 6.0,
    };
    roundtrip_debug(&q);
}

#[test]
fn rbd_fast_indices_roundtrip() {
    let r = RbdFastIndices { s: vec![0.4, 0.6] };
    roundtrip_debug(&r);
}

#[test]
fn jansen_indices_roundtrip() {
    let j = JansenIndices {
        first_order: vec![0.3, 0.7],
        total_variance: 10.0,
        second_order: None,
    };
    roundtrip_debug(&j);
}

#[test]
fn janon_indices_roundtrip() {
    let j = JanonIndices {
        first_order: vec![0.3, 0.7],
        total_variance: 10.0,
        second_order: None,
    };
    roundtrip_debug(&j);
}

#[test]
fn owen_indices_roundtrip() {
    let o = OwenIndices {
        first_order: vec![0.3, 0.7],
        total_variance: 10.0,
        second_order: None,
    };
    roundtrip_debug(&o);
}

#[test]
fn given_data_sobol_roundtrip() {
    let g = GivenDataSobolIndices {
        s1: vec![0.3, 0.7],
    };
    roundtrip_debug(&g);
}

#[test]
fn discrepancy_result_roundtrip() {
    let d = DiscrepancyResult {
        centered: 0.01,
        wrap_around: 0.02,
        modified: 0.015,
        l2_star: 0.005,
    };
    roundtrip_debug(&d);
}

#[test]
fn fractional_factorial_roundtrip() {
    let f = FractionalFactorialEffects {
        dim: 3,
        n_runs: 8,
        main_effects: vec![1.0, -0.5, 0.3],
        main_effects_abs: vec![1.0, 0.5, 0.3],
    };
    roundtrip_debug(&f);
}

#[test]
fn bootstrap_ci_roundtrip() {
    let b = BootstrapCi {
        ci_low: vec![0.1, 0.2],
        ci_high: vec![0.5, 0.8],
        n_resamples: 1000,
        alpha: 0.05,
        n_skipped: 0,
    };
    roundtrip(&b);
}

#[test]
fn g_theory_result_roundtrip() {
    let g = GTheoryResult::from_components(
        1.0, 0.5, 0.3, 0.1, 0.05, 0.02, 0.01, 0.85, 0.80,
    );
    roundtrip_debug(&g);
}

#[test]
fn d_study_point_roundtrip() {
    let d = DStudyPoint {
        n_items: 10,
        n_raters: 3,
        g_coefficient: 0.85,
        phi_coefficient: 0.80,
    };
    roundtrip(&d);
}

#[test]
fn g_theory_design_roundtrip() {
    roundtrip(&GTheoryDesign::Crossed);
}

#[test]
fn fd_kind_roundtrip() {
    roundtrip(&FdKind::Forward);
    roundtrip(&FdKind::Central);
}

#[test]
fn anova_two_way_roundtrip() {
    let a = AnovaTwoWayResult {
        v_row: 1.0,
        v_column: 2.0,
        v_interaction: 0.5,
        v_residual: 0.1,
        ms_row: 0.5,
        ms_column: 1.0,
        ms_interaction: 0.25,
        ms_residual: 0.05,
        f_row: Some(10.0),
        f_column: Some(20.0),
        f_interaction: Some(5.0),
        p_row: Some(0.001),
        p_column: Some(0.0001),
        p_interaction: Some(0.01),
        variance_fraction_ci_low: None,
        variance_fraction_ci_high: None,
        bootstrap_iterations: None,
        bootstrap_alpha: None,
    };
    roundtrip_debug(&a);
}

#[test]
fn anova_three_way_roundtrip() {
    let a = AnovaThreeWayResult {
        v_data: 1.0,
        v_brittleness: 0.5,
        v_inference: 0.3,
        v_data_brittleness: 0.1,
        v_data_inference: 0.05,
        v_brittleness_inference: 0.02,
        v_data_brittleness_inference: 0.01,
        v_residual: 0.005,
        ms_data: 0.5,
        ms_brittleness: 0.25,
        ms_inference: 0.15,
        ms_data_brittleness: 0.05,
        ms_data_inference: 0.025,
        ms_brittleness_inference: 0.01,
        ms_data_brittleness_inference: 0.005,
        ms_residual: 0.0025,
        f_data: Some(200.0),
        f_brittleness: Some(100.0),
        f_inference: Some(60.0),
        f_data_brittleness: Some(20.0),
        f_data_inference: Some(10.0),
        f_brittleness_inference: Some(4.0),
        f_data_brittleness_inference: Some(2.0),
        p_data: Some(0.001),
        p_brittleness: Some(0.01),
        p_inference: Some(0.02),
        p_data_brittleness: Some(0.05),
        p_data_inference: Some(0.1),
        p_brittleness_inference: Some(0.2),
        p_data_brittleness_inference: Some(0.3),
        variance_fraction_ci_low: None,
        variance_fraction_ci_high: None,
        bootstrap_iterations: None,
        bootstrap_alpha: None,
    };
    roundtrip_debug(&a);
}
```

- [ ] **Step 6: Add serde_json dev-dependency**

In `crates/salib-estimators/Cargo.toml`, add to `[dev-dependencies]`:

```toml
serde_json = "1"
```

- [ ] **Step 7: Run serde round-trip tests**

Run: `cargo test -p salib-estimators --features serde serde_roundtrip`
Expected: All tests PASS

- [ ] **Step 8: Verify default build still works (no serde)**

Run: `cargo check -p salib-estimators`
Expected: Compiles clean without serde

- [ ] **Step 9: Commit**

```bash
git add crates/salib-estimators/
git commit -m "feat(estimators): add serde feature flag with Serialize/Deserialize on all result types"
```

---

### Task 3: Serde Feature Flag in salib-samplers

**Files:**
- Modify: `crates/salib-samplers/Cargo.toml`
- Modify: `crates/salib-samplers/src/saltelli_matrix.rs`
- Modify: `crates/salib-samplers/src/morris.rs`
- Modify: `crates/salib-samplers/src/fast.rs`
- Modify: `crates/salib-samplers/src/owen_matrix.rs`
- Modify: `crates/salib-samplers/src/plackett_burman.rs`
- Modify: `crates/salib-samplers/src/lhs.rs`
- Modify: `crates/salib-samplers/src/sobol.rs`
- Create: `crates/salib-samplers/tests/serde_roundtrip.rs`

- [ ] **Step 1: Add serde feature to Cargo.toml**

`salib-samplers` already has `serde` as an unconditional dependency (used for `config_hash`). The feature needs to gate only the derive macros on output types, and enable `ndarray/serde` for types with array fields. Add to Cargo.toml:

```toml
[features]
serde = ["ndarray/serde"]
```

Note: `serde` the crate is already in `[dependencies]` unconditionally (needed for `Sampler::config_hash`). The `serde` *feature flag* only controls whether ndarray gets its serde support and whether output types get derives.

- [ ] **Step 2: Add cfg_attr serde derives to sampler output types**

`crates/salib-samplers/src/saltelli_matrix.rs` — on `SaltelliMatrix` (line 57):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-samplers/src/morris.rs` — on `MorrisTrajectories` (line 50):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-samplers/src/fast.rs` — on `FastDesign` (line 73):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-samplers/src/owen_matrix.rs` — on `OwenMatrix` (line 57):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-samplers/src/plackett_burman.rs` — on `PlackettBurmanDesign` (line 14):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

- [ ] **Step 3: Migrate LhsKind and LhsSampler to conditional serde**

Currently `LhsKind` and `LhsSampler` derive `Serialize` unconditionally. Change them to conditional:

`crates/salib-samplers/src/lhs.rs` — on `LhsKind` (line 49), change:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[non_exhaustive]
#[serde(tag = "kind")]
```
to:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
#[cfg_attr(feature = "serde", serde(tag = "kind"))]
```

On `LhsSampler` (line 66), change:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
```
to:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

Remove any `use serde::Serialize;` import at the top of `lhs.rs` that is no longer needed for the derive (it may still be needed for `config_hash` — check and keep if so).

- [ ] **Step 4: Migrate SobolDimSet and SobolSampler similarly**

`crates/salib-samplers/src/sobol.rs` — on `SobolDimSet`, change:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
```
to:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

On `SobolSampler`, same change.

Check: if `config_hash` uses `serde_json::to_string(&self)`, the `Serialize` impl is needed unconditionally for sampler types. In that case, keep `Serialize` in the unconditional derive for sampler config types only, and add `Deserialize` conditionally:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
```

- [ ] **Step 5: Verify compilation**

Run: `cargo check -p salib-samplers`
Expected: Compiles clean without serde feature

Run: `cargo check -p salib-samplers --features serde`
Expected: Compiles clean with serde feature

- [ ] **Step 6: Write serde round-trip tests**

Create `crates/salib-samplers/tests/serde_roundtrip.rs`:

```rust
#![cfg(feature = "serde")]
#![allow(clippy::unwrap_used)]

use ndarray::Array2;
use salib_samplers::*;

#[test]
fn saltelli_matrix_roundtrip() {
    use salib_core::RngState;
    let sampler = SobolSampler::minimal(6);
    let mut rng = RngState::from_seed([0u8; 32]);
    let sm = build_saltelli_matrix(&sampler, 64, false, &mut rng).unwrap();
    let json = serde_json::to_string(&sm).unwrap();
    let back: SaltelliMatrix = serde_json::from_str(&json).unwrap();
    assert_eq!(sm.n, back.n);
    assert_eq!(sm.dim, back.dim);
    assert_eq!(sm.a, back.a);
    assert_eq!(sm.b, back.b);
}

#[test]
fn lhs_sampler_roundtrip() {
    let s = LhsSampler::classic(3);
    let json = serde_json::to_string(&s).unwrap();
    let back: LhsSampler = serde_json::from_str(&json).unwrap();
    assert_eq!(s, back);
}

#[test]
fn sobol_sampler_roundtrip() {
    let s = SobolSampler::minimal(4);
    let json = serde_json::to_string(&s).unwrap();
    let back: SobolSampler = serde_json::from_str(&json).unwrap();
    assert_eq!(s, back);
}
```

- [ ] **Step 7: Run tests**

Run: `cargo test -p salib-samplers --features serde serde_roundtrip`
Expected: All PASS

- [ ] **Step 8: Commit**

```bash
git add crates/salib-samplers/
git commit -m "feat(samplers): add serde feature flag with Serialize/Deserialize on output types"
```

---

### Task 4: Serde Feature Flag in salib-surrogate, salib-shapley, salib-validation

**Files:**
- Modify: `crates/salib-surrogate/Cargo.toml`, 5 source files
- Modify: `crates/salib-shapley/Cargo.toml`, 1 source file
- Modify: `crates/salib-validation/Cargo.toml`, 1 source file
- Create: `crates/salib-surrogate/tests/serde_roundtrip.rs`
- Create: `crates/salib-shapley/tests/serde_roundtrip.rs`
- Create: `crates/salib-validation/tests/serde_roundtrip.rs`

- [ ] **Step 1: salib-surrogate Cargo.toml**

Add to `crates/salib-surrogate/Cargo.toml`:

```toml
serde = { version = "1", features = ["derive"], optional = true }

[features]
serde = ["dep:serde", "ndarray/serde"]
```

- [ ] **Step 2: Add cfg_attr to all surrogate types**

`crates/salib-surrogate/src/pce.rs` — on `PolynomialChaos` (line 77):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

On `SobolFromPce` (line 154):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-surrogate/src/active_subspace.rs` — on `ActiveSubspace` (line 78):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-surrogate/src/multi_index.rs` — on `MultiIndex` (line 35):
```rust
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-surrogate/src/sparse_pce.rs` — on `TruncationScheme` (line 90):
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

On `SparseSolver` (line 102):
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

On `SparseFitDiagnostic` (line 117):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

`crates/salib-surrogate/src/polynomial.rs` — on `PolynomialFamily` (line 49):
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

- [ ] **Step 3: salib-shapley Cargo.toml and type**

Add to `crates/salib-shapley/Cargo.toml`:

```toml
serde = { version = "1", features = ["derive"], optional = true }

[features]
serde = ["dep:serde"]
```

`crates/salib-shapley/src/estimator.rs` — on `ShapleyIndices` (line 44):
```rust
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

- [ ] **Step 4: salib-validation Cargo.toml and types**

Add to `crates/salib-validation/Cargo.toml`:

```toml
serde = { version = "1", features = ["derive"], optional = true }

[features]
serde = ["dep:serde"]
```

`crates/salib-validation/src/analytic.rs` — on `SobolIndicesAnalytic` (line 52):
```rust
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

On `MorrisEffectsAnalytic` (line 112):
```rust
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
```

- [ ] **Step 5: Verify compilation for all three**

Run: `cargo check -p salib-surrogate -p salib-shapley -p salib-validation`
Expected: Compiles clean

Run: `cargo check -p salib-surrogate --features serde -p salib-shapley --features serde -p salib-validation --features serde`
Expected: Compiles clean

- [ ] **Step 6: Write serde tests for all three**

Create `crates/salib-surrogate/tests/serde_roundtrip.rs`:

```rust
#![cfg(feature = "serde")]
#![allow(clippy::unwrap_used)]

use salib_surrogate::*;

#[test]
fn polynomial_chaos_roundtrip() {
    let pc = PolynomialChaos {
        coefficients: vec![1.0, 0.5, 0.25],
        multi_indices: vec![
            MultiIndex::new(vec![0, 0]).unwrap(),
            MultiIndex::new(vec![1, 0]).unwrap(),
            MultiIndex::new(vec![0, 1]).unwrap(),
        ],
        families: vec![PolynomialFamily::Legendre, PolynomialFamily::Legendre],
        max_degree: 2,
    };
    let json = serde_json::to_string(&pc).unwrap();
    let back: PolynomialChaos = serde_json::from_str(&json).unwrap();
    assert_eq!(format!("{pc:?}"), format!("{back:?}"));
}

#[test]
fn sobol_from_pce_roundtrip() {
    let s = SobolFromPce {
        first_order: vec![0.3, 0.7],
        total_order: vec![0.5, 0.9],
        total_variance: 10.0,
    };
    let json = serde_json::to_string(&s).unwrap();
    let back: SobolFromPce = serde_json::from_str(&json).unwrap();
    assert_eq!(format!("{s:?}"), format!("{back:?}"));
}

#[test]
fn sparse_solver_roundtrip() {
    let json = serde_json::to_string(&SparseSolver::Lars).unwrap();
    let back: SparseSolver = serde_json::from_str(&json).unwrap();
    assert_eq!(SparseSolver::Lars, back);
}

#[test]
fn polynomial_family_roundtrip() {
    let f = PolynomialFamily::Jacobi { alpha: 0.5, beta: 1.0 };
    let json = serde_json::to_string(&f).unwrap();
    let back: PolynomialFamily = serde_json::from_str(&json).unwrap();
    assert_eq!(f, back);
}
```

Create `crates/salib-shapley/tests/serde_roundtrip.rs`:

```rust
#![cfg(feature = "serde")]
#![allow(clippy::unwrap_used)]

use salib_shapley::ShapleyIndices;

#[test]
fn shapley_indices_roundtrip() {
    let s = ShapleyIndices {
        sh: vec![0.3, 0.5, 0.2],
        var_y: 10.0,
        n_perm: 1000,
    };
    let json = serde_json::to_string(&s).unwrap();
    let back: ShapleyIndices = serde_json::from_str(&json).unwrap();
    assert_eq!(format!("{s:?}"), format!("{back:?}"));
}
```

Create `crates/salib-validation/tests/serde_roundtrip.rs`:

```rust
#![cfg(feature = "serde")]
#![allow(clippy::unwrap_used)]

use salib_validation::*;

#[test]
fn sobol_indices_analytic_roundtrip() {
    let s = SobolIndicesAnalytic::new(
        10.0,
        vec![0.3, 0.7],
        vec![0.5, 0.9],
        None,
    );
    let json = serde_json::to_string(&s).unwrap();
    let back: SobolIndicesAnalytic = serde_json::from_str(&json).unwrap();
    assert_eq!(s, back);
}

#[test]
fn morris_effects_analytic_roundtrip() {
    let m = MorrisEffectsAnalytic::new(
        vec![1.0, 2.0],
        vec![1.0, 2.0],
        vec![0.5, 0.5],
    );
    let json = serde_json::to_string(&m).unwrap();
    let back: MorrisEffectsAnalytic = serde_json::from_str(&json).unwrap();
    assert_eq!(m, back);
}
```

Add `serde_json = "1"` to `[dev-dependencies]` in all three Cargo.toml files.

- [ ] **Step 7: Run all serde tests**

Run: `cargo test -p salib-surrogate --features serde serde_roundtrip`
Run: `cargo test -p salib-shapley --features serde serde_roundtrip`
Run: `cargo test -p salib-validation --features serde serde_roundtrip`
Expected: All PASS

- [ ] **Step 8: Commit**

```bash
git add crates/salib-surrogate/ crates/salib-shapley/ crates/salib-validation/
git commit -m "feat(surrogate,shapley,validation): add serde feature flag with Serialize/Deserialize"
```

---

### Task 5: Serde Forwarding in Facade Crate

**Files:**
- Modify: `crates/salib/Cargo.toml`

- [ ] **Step 1: Add serde feature forwarding**

In `crates/salib/Cargo.toml`, update `[features]`:

```toml
[features]
default = ["samplers", "estimators", "parallel"]
samplers = ["dep:salib-samplers"]
estimators = ["dep:salib-estimators"]
surrogate = ["dep:salib-surrogate", "salib-estimators?/surrogate"]
shapley = ["dep:salib-shapley"]
validation = ["dep:salib-validation"]
parallel = ["salib-core/parallel"]
serde = [
    "salib-samplers?/serde",
    "salib-estimators?/serde",
    "salib-surrogate?/serde",
    "salib-shapley?/serde",
    "salib-validation?/serde",
]
full = ["samplers", "estimators", "surrogate", "shapley", "validation"]
```

Note: `salib-core/serde` is a no-op right now (serde is unconditional in core) but future-proofs if core ever gates it.

- [ ] **Step 2: Verify full feature matrix compiles**

Run: `cargo check -p salib`
Run: `cargo check -p salib --features serde`
Run: `cargo check -p salib --features full,serde`
Run: `cargo check -p salib --no-default-features`
Expected: All compile clean

- [ ] **Step 3: Commit**

```bash
git add crates/salib/Cargo.toml
git commit -m "feat(salib): add serde feature forwarding to all subcrates"
```

---

### Task 6: Display Impls for Factor-Indexed Types

**Files:**
- Modify: `crates/salib-estimators/src/sobol_indices.rs`
- Modify: `crates/salib-estimators/src/morris.rs`
- Modify: `crates/salib-estimators/src/fast.rs`
- Modify: `crates/salib-estimators/src/borgonovo.rs`
- Modify: `crates/salib-estimators/src/regression.rs`
- Modify: `crates/salib-estimators/src/pawn.rs`
- Modify: `crates/salib-estimators/src/dgsm.rs`
- Modify: `crates/salib-estimators/src/qosa.rs`
- Modify: `crates/salib-estimators/src/rbd_fast.rs`
- Modify: `crates/salib-estimators/src/janon.rs`
- Modify: `crates/salib-estimators/src/jansen.rs`
- Modify: `crates/salib-estimators/src/owen.rs`
- Modify: `crates/salib-estimators/src/given_data_sobol.rs`
- Modify: `crates/salib-estimators/src/fractional_factorial.rs`
- Modify: `crates/salib-estimators/src/bootstrap_given_data.rs`

- [ ] **Step 1: Add Display for SobolIndices and display_with_names**

Add to `crates/salib-estimators/src/sobol_indices.rs`:

```rust
use std::fmt;

impl fmt::Display for SobolIndices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Sobol' indices (N={}, d={})", self.n, self.dim)?;
        writeln!(f, "  Var[Y] = {:.4}", self.total_variance)?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}", "Factor", "S1", "ST")?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}", "------", "------", "------")?;
        for i in 0..self.dim {
            writeln!(
                f,
                "  {:>8}  {:>8.4}  {:>8.4}",
                i, self.first_order[i], self.total_order[i]
            )?;
        }
        Ok(())
    }
}

struct WithNames<'a> {
    indices: &'a SobolIndices,
    names: &'a [&'a str],
}

impl fmt::Display for WithNames<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let idx = self.indices;
        writeln!(f, "Sobol' indices (N={}, d={})", idx.n, idx.dim)?;
        writeln!(f, "  Var[Y] = {:.4}", idx.total_variance)?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}", "Factor", "S1", "ST")?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}", "------", "------", "------")?;
        for i in 0..idx.dim {
            let name = self.names.get(i).copied().unwrap_or("?");
            writeln!(
                f,
                "  {:>8}  {:>8.4}  {:>8.4}",
                name, idx.first_order[i], idx.total_order[i]
            )?;
        }
        Ok(())
    }
}

impl SobolIndices {
    pub fn display_with_names<'a>(&'a self, names: &'a [&'a str]) -> impl fmt::Display + 'a {
        WithNames { indices: self, names }
    }
}
```

- [ ] **Step 2: Add Display for SobolIndicesWithCi**

Add to `crates/salib-estimators/src/sobol_indices.rs`:

```rust
impl fmt::Display for SobolIndicesWithCi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let idx = &self.indices;
        writeln!(f, "Sobol' indices with CI (N={}, d={}, B={})", idx.n, idx.dim, self.bootstrap_resamples)?;
        writeln!(f, "  Var[Y] = {:.4}", idx.total_variance)?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}  {:>14}  {:>8}  {:>14}", "Factor", "S1", "S1 CI", "ST", "ST CI")?;
        writeln!(f, "  {:>8}  {:>8}  {:>14}  {:>8}  {:>14}", "------", "------", "-----------", "------", "-----------")?;
        for i in 0..idx.dim {
            let (s1_lo, s1_hi) = self.first_order_ci[i];
            let (st_lo, st_hi) = self.total_order_ci[i];
            writeln!(
                f,
                "  {:>8}  {:>8.4}  [{:.4},{:.4}]  {:>8.4}  [{:.4},{:.4}]",
                i, idx.first_order[i], s1_lo, s1_hi, idx.total_order[i], st_lo, st_hi
            )?;
        }
        Ok(())
    }
}
```

- [ ] **Step 3: Add Display for MorrisEffects**

Add to `crates/salib-estimators/src/morris.rs`:

```rust
use std::fmt;

impl fmt::Display for MorrisEffects {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Morris effects (r={}, d={})", self.r, self.d)?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}  {:>8}", "Factor", "\u{03bc}", "\u{03bc}*", "\u{03c3}")?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}  {:>8}", "------", "------", "------", "------")?;
        for i in 0..self.d {
            writeln!(
                f,
                "  {:>8}  {:>8.4}  {:>8.4}  {:>8.4}",
                i, self.mu[i], self.mu_star[i], self.sigma[i]
            )?;
        }
        Ok(())
    }
}
```

- [ ] **Step 4: Add Display for remaining factor-indexed types**

Add to `crates/salib-estimators/src/fast.rs`:

```rust
use std::fmt;

impl fmt::Display for FastIndices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "FAST indices (d={})", self.d())?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}", "Factor", "S", "ST")?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}", "------", "------", "------")?;
        for i in 0..self.d() {
            writeln!(f, "  {:>8}  {:>8.4}  {:>8.4}", i, self.s[i], self.st[i])?;
        }
        Ok(())
    }
}
```

Add to `crates/salib-estimators/src/borgonovo.rs`:

```rust
use std::fmt;

impl fmt::Display for BorgonovoIndices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Borgonovo \u{03b4} indices (d={})", self.d())?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}", "Factor", "\u{03b4}")?;
        writeln!(f, "  {:>8}  {:>8}", "------", "------")?;
        for i in 0..self.d() {
            writeln!(f, "  {:>8}  {:>8.4}", i, self.delta[i])?;
        }
        Ok(())
    }
}
```

Add to `crates/salib-estimators/src/regression.rs`:

```rust
use std::fmt;

impl fmt::Display for RegressionIndices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Regression indices (d={})", self.d())?;
        writeln!(f, "  R\u{00b2}(linear) = {:.4}  R\u{00b2}(rank) = {:.4}", self.r2_linear, self.r2_rank)?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}  {:>8}  {:>8}", "Factor", "SRC", "SRRC", "PCC", "PRCC")?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}  {:>8}  {:>8}", "------", "------", "------", "------", "------")?;
        for i in 0..self.d() {
            writeln!(
                f,
                "  {:>8}  {:>8.4}  {:>8.4}  {:>8.4}  {:>8.4}",
                i, self.src[i], self.srrc[i], self.pcc[i], self.prcc[i]
            )?;
        }
        Ok(())
    }
}
```

Add to `crates/salib-estimators/src/pawn.rs`:

```rust
use std::fmt;

impl fmt::Display for PawnIndices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PAWN indices (d={})", self.d())?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}  {:>8}  {:>8}  {:>8}", "Factor", "median", "mean", "max", "min", "CV")?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}  {:>8}  {:>8}  {:>8}", "------", "------", "------", "------", "------", "------")?;
        for i in 0..self.d() {
            writeln!(
                f,
                "  {:>8}  {:>8.4}  {:>8.4}  {:>8.4}  {:>8.4}  {:>8.4}",
                i, self.median[i], self.mean[i], self.maximum[i], self.minimum[i], self.cv[i]
            )?;
        }
        Ok(())
    }
}
```

Add to `crates/salib-estimators/src/dgsm.rs`:

```rust
use std::fmt;

impl fmt::Display for DgsmIndices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "DGSM indices (d={})", self.d())?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}  {:>10}", "Factor", "\u{03bd}", "ST_upper")?;
        writeln!(f, "  {:>8}  {:>8}  {:>10}", "------", "------", "--------")?;
        for i in 0..self.d() {
            writeln!(f, "  {:>8}  {:>8.4}  {:>10.4}", i, self.vi[i], self.st_upper[i])?;
        }
        Ok(())
    }
}
```

Add to `crates/salib-estimators/src/qosa.rs`:

```rust
use std::fmt;

impl fmt::Display for QosaIndices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "QOSA indices (d={}, \u{03b1}={:.2})", self.d(), self.alpha)?;
        writeln!(f, "  quantile = {:.4}  CTE = {:.4}", self.global_quantile, self.global_cte)?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}", "Factor", "S")?;
        writeln!(f, "  {:>8}  {:>8}", "------", "------")?;
        for i in 0..self.d() {
            writeln!(f, "  {:>8}  {:>8.4}", i, self.s[i])?;
        }
        Ok(())
    }
}
```

Add to `crates/salib-estimators/src/rbd_fast.rs`:

```rust
use std::fmt;

impl fmt::Display for RbdFastIndices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "RBD-FAST indices (d={})", self.d())?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}", "Factor", "S")?;
        writeln!(f, "  {:>8}  {:>8}", "------", "------")?;
        for i in 0..self.d() {
            writeln!(f, "  {:>8}  {:>8.4}", i, self.s[i])?;
        }
        Ok(())
    }
}
```

Add to `crates/salib-estimators/src/janon.rs`:

```rust
use std::fmt;

impl fmt::Display for JanonIndices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Janon indices (d={})", self.d())?;
        writeln!(f, "  Var[Y] = {:.4}", self.total_variance)?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}", "Factor", "S1")?;
        writeln!(f, "  {:>8}  {:>8}", "------", "------")?;
        for i in 0..self.d() {
            writeln!(f, "  {:>8}  {:>8.4}", i, self.first_order[i])?;
        }
        Ok(())
    }
}
```

Add to `crates/salib-estimators/src/jansen.rs`:

```rust
use std::fmt;

impl fmt::Display for JansenIndices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Jansen indices (d={})", self.d())?;
        writeln!(f, "  Var[Y] = {:.4}", self.total_variance)?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}", "Factor", "S1")?;
        writeln!(f, "  {:>8}  {:>8}", "------", "------")?;
        for i in 0..self.d() {
            writeln!(f, "  {:>8}  {:>8.4}", i, self.first_order[i])?;
        }
        Ok(())
    }
}
```

Add to `crates/salib-estimators/src/owen.rs`:

```rust
use std::fmt;

impl fmt::Display for OwenIndices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Owen indices (d={})", self.d())?;
        writeln!(f, "  Var[Y] = {:.4}", self.total_variance)?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}", "Factor", "S1")?;
        writeln!(f, "  {:>8}  {:>8}", "------", "------")?;
        for i in 0..self.d() {
            writeln!(f, "  {:>8}  {:>8.4}", i, self.first_order[i])?;
        }
        Ok(())
    }
}
```

Add to `crates/salib-estimators/src/given_data_sobol.rs`:

```rust
use std::fmt;

impl fmt::Display for GivenDataSobolIndices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Given-data Sobol' indices (d={})", self.d())?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}", "Factor", "S1")?;
        writeln!(f, "  {:>8}  {:>8}", "------", "------")?;
        for i in 0..self.d() {
            writeln!(f, "  {:>8}  {:>8.4}", i, self.s1[i])?;
        }
        Ok(())
    }
}
```

Add to `crates/salib-estimators/src/fractional_factorial.rs`:

```rust
use std::fmt;

impl fmt::Display for FractionalFactorialEffects {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Fractional factorial effects (d={}, runs={})", self.dim, self.n_runs)?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>10}  {:>10}", "Factor", "Effect", "|Effect|")?;
        writeln!(f, "  {:>8}  {:>10}  {:>10}", "------", "--------", "--------")?;
        for i in 0..self.dim {
            writeln!(
                f,
                "  {:>8}  {:>10.4}  {:>10.4}",
                i, self.main_effects[i], self.main_effects_abs[i]
            )?;
        }
        Ok(())
    }
}
```

Add to `crates/salib-estimators/src/bootstrap_given_data.rs`:

```rust
use std::fmt;

impl fmt::Display for BootstrapCi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = self.ci_low.len();
        writeln!(f, "Bootstrap CI (B={}, \u{03b1}={:.2}, skipped={})", self.n_resamples, self.alpha, self.n_skipped)?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>10}  {:>10}", "Factor", "CI_low", "CI_high")?;
        writeln!(f, "  {:>8}  {:>10}  {:>10}", "------", "--------", "--------")?;
        for i in 0..d {
            writeln!(f, "  {:>8}  {:>10.4}  {:>10.4}", i, self.ci_low[i], self.ci_high[i])?;
        }
        Ok(())
    }
}
```

- [ ] **Step 5: Verify it compiles**

Run: `cargo check -p salib-estimators`
Expected: Compiles clean

- [ ] **Step 6: Write Display snapshot tests**

Create `crates/salib-estimators/tests/display_snapshot.rs`:

```rust
#![allow(clippy::unwrap_used)]

use salib_estimators::*;

#[test]
fn sobol_indices_display() {
    let idx = SobolIndices {
        n: 8192,
        dim: 3,
        total_variance: 13.8446,
        first_order: vec![0.3139, 0.4424, 0.0000],
        total_order: vec![0.5576, 0.4424, 0.2437],
        second_order: None,
    };
    let output = format!("{idx}");
    assert!(output.contains("Sobol' indices (N=8192, d=3)"));
    assert!(output.contains("Var[Y] = 13.8446"));
    assert!(output.contains("0.3139"));
    assert!(output.contains("0.5576"));
}

#[test]
fn sobol_indices_display_with_names() {
    let idx = SobolIndices {
        n: 8192,
        dim: 3,
        total_variance: 13.8446,
        first_order: vec![0.3139, 0.4424, 0.0000],
        total_order: vec![0.5576, 0.4424, 0.2437],
        second_order: None,
    };
    let output = format!("{}", idx.display_with_names(&["x1", "x2", "x3"]));
    assert!(output.contains("x1"));
    assert!(output.contains("x2"));
    assert!(output.contains("x3"));
}

#[test]
fn morris_effects_display() {
    let m = MorrisEffects {
        r: 10,
        d: 2,
        mu: vec![1.0, -0.5],
        mu_star: vec![1.0, 0.5],
        sigma: vec![0.3, 0.2],
        grouped_mu: None,
        grouped_mu_star: None,
        grouped_sigma: None,
        group_names: None,
    };
    let output = format!("{m}");
    assert!(output.contains("Morris effects (r=10, d=2)"));
    assert!(output.contains("1.0000"));
}

#[test]
fn regression_indices_display() {
    let r = RegressionIndices {
        src: vec![0.8, 0.2],
        srrc: vec![0.7, 0.3],
        pcc: vec![0.9, 0.1],
        prcc: vec![0.85, 0.15],
        r2_linear: 0.95,
        r2_rank: 0.93,
    };
    let output = format!("{r}");
    assert!(output.contains("R\u{00b2}(linear) = 0.9500"));
    assert!(output.contains("SRC"));
    assert!(output.contains("PRCC"));
}

#[test]
fn discrepancy_result_display() {
    let d = DiscrepancyResult {
        centered: 0.0123,
        wrap_around: 0.0234,
        modified: 0.0156,
        l2_star: 0.0045,
    };
    let output = format!("{d}");
    assert!(output.contains("0.0123"));
    assert!(output.contains("L2*"));
}

#[test]
fn bootstrap_ci_display() {
    let b = BootstrapCi {
        ci_low: vec![0.1, 0.2],
        ci_high: vec![0.5, 0.8],
        n_resamples: 1000,
        alpha: 0.05,
        n_skipped: 3,
    };
    let output = format!("{b}");
    assert!(output.contains("B=1000"));
    assert!(output.contains("skipped=3"));
}
```

- [ ] **Step 7: Run display tests**

Run: `cargo test -p salib-estimators display_snapshot`
Expected: All PASS

- [ ] **Step 8: Commit**

```bash
git add crates/salib-estimators/
git commit -m "feat(estimators): add Display impls for all result types"
```

---

### Task 7: Display Impls for ANOVA, G-Theory, Discrepancy, Surrogate, Shapley

**Files:**
- Modify: `crates/salib-estimators/src/anova.rs`
- Modify: `crates/salib-estimators/src/g_theory.rs`
- Modify: `crates/salib-estimators/src/discrepancy.rs`
- Modify: `crates/salib-estimators/src/hdmr.rs`
- Modify: `crates/salib-surrogate/src/pce.rs`
- Modify: `crates/salib-shapley/src/estimator.rs`

- [ ] **Step 1: Add Display for ANOVA and G-theory types**

Add to `crates/salib-estimators/src/anova.rs`:

```rust
use std::fmt;

impl fmt::Display for AnovaTwoWayResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Two-way ANOVA")?;
        writeln!(f)?;
        writeln!(f, "  {:>14}  {:>10}  {:>10}  {:>10}  {:>10}", "Source", "SS", "MS", "F", "p")?;
        writeln!(f, "  {:>14}  {:>10}  {:>10}  {:>10}  {:>10}", "-----------", "--------", "--------", "--------", "--------")?;
        let fmt_opt = |v: Option<f64>| v.map_or("     ---".to_string(), |x| format!("{x:>10.4}"));
        writeln!(f, "  {:>14}  {:>10.4}  {:>10.4}  {}  {}", "Row", self.v_row, self.ms_row, fmt_opt(self.f_row), fmt_opt(self.p_row))?;
        writeln!(f, "  {:>14}  {:>10.4}  {:>10.4}  {}  {}", "Column", self.v_column, self.ms_column, fmt_opt(self.f_column), fmt_opt(self.p_column))?;
        writeln!(f, "  {:>14}  {:>10.4}  {:>10.4}  {}  {}", "Interaction", self.v_interaction, self.ms_interaction, fmt_opt(self.f_interaction), fmt_opt(self.p_interaction))?;
        writeln!(f, "  {:>14}  {:>10.4}  {:>10.4}", "Residual", self.v_residual, self.ms_residual)?;
        Ok(())
    }
}

impl fmt::Display for AnovaThreeWayResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Three-way ANOVA")?;
        writeln!(f)?;
        writeln!(f, "  {:>20}  {:>10}  {:>10}  {:>10}  {:>10}", "Source", "SS", "MS", "F", "p")?;
        writeln!(f, "  {:>20}  {:>10}  {:>10}  {:>10}  {:>10}", "-----------------", "--------", "--------", "--------", "--------")?;
        let fmt_opt = |v: Option<f64>| v.map_or("     ---".to_string(), |x| format!("{x:>10.4}"));
        writeln!(f, "  {:>20}  {:>10.4}  {:>10.4}  {}  {}", "Data", self.v_data, self.ms_data, fmt_opt(self.f_data), fmt_opt(self.p_data))?;
        writeln!(f, "  {:>20}  {:>10.4}  {:>10.4}  {}  {}", "Brittleness", self.v_brittleness, self.ms_brittleness, fmt_opt(self.f_brittleness), fmt_opt(self.p_brittleness))?;
        writeln!(f, "  {:>20}  {:>10.4}  {:>10.4}  {}  {}", "Inference", self.v_inference, self.ms_inference, fmt_opt(self.f_inference), fmt_opt(self.p_inference))?;
        writeln!(f, "  {:>20}  {:>10.4}  {:>10.4}  {}  {}", "Data*Brittleness", self.v_data_brittleness, self.ms_data_brittleness, fmt_opt(self.f_data_brittleness), fmt_opt(self.p_data_brittleness))?;
        writeln!(f, "  {:>20}  {:>10.4}  {:>10.4}  {}  {}", "Data*Inference", self.v_data_inference, self.ms_data_inference, fmt_opt(self.f_data_inference), fmt_opt(self.p_data_inference))?;
        writeln!(f, "  {:>20}  {:>10.4}  {:>10.4}  {}  {}", "Britt*Inference", self.v_brittleness_inference, self.ms_brittleness_inference, fmt_opt(self.f_brittleness_inference), fmt_opt(self.p_brittleness_inference))?;
        writeln!(f, "  {:>20}  {:>10.4}  {:>10.4}  {}  {}", "D*B*I", self.v_data_brittleness_inference, self.ms_data_brittleness_inference, fmt_opt(self.f_data_brittleness_inference), fmt_opt(self.p_data_brittleness_inference))?;
        writeln!(f, "  {:>20}  {:>10.4}  {:>10.4}", "Residual", self.v_residual, self.ms_residual)?;
        Ok(())
    }
}
```

Add to `crates/salib-estimators/src/g_theory.rs`:

```rust
use std::fmt;

impl fmt::Display for GTheoryResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "G-theory (p x i x r)")?;
        writeln!(f)?;
        writeln!(f, "  Variance components:")?;
        writeln!(f, "    \u{03c3}\u{00b2}(p)   = {:.4}", self.sigma_p)?;
        writeln!(f, "    \u{03c3}\u{00b2}(i)   = {:.4}", self.sigma_i)?;
        writeln!(f, "    \u{03c3}\u{00b2}(r)   = {:.4}", self.sigma_r)?;
        writeln!(f, "    \u{03c3}\u{00b2}(pi)  = {:.4}", self.sigma_pi)?;
        writeln!(f, "    \u{03c3}\u{00b2}(pr)  = {:.4}", self.sigma_pr)?;
        writeln!(f, "    \u{03c3}\u{00b2}(ir)  = {:.4}", self.sigma_ir)?;
        writeln!(f, "    \u{03c3}\u{00b2}(pir) = {:.4}", self.sigma_pir)?;
        writeln!(f)?;
        writeln!(f, "  G = {:.4}  \u{03a6} = {:.4}", self.g_coefficient, self.phi_coefficient)?;
        Ok(())
    }
}
```

Add to `crates/salib-estimators/src/discrepancy.rs`:

```rust
use std::fmt;

impl fmt::Display for DiscrepancyResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Discrepancy measures")?;
        writeln!(f)?;
        writeln!(f, "  CD   = {:.6}", self.centered)?;
        writeln!(f, "  WD   = {:.6}", self.wrap_around)?;
        writeln!(f, "  MD   = {:.6}", self.modified)?;
        writeln!(f, "  L2*  = {:.6}", self.l2_star)?;
        Ok(())
    }
}
```

Add to `crates/salib-estimators/src/hdmr.rs` (gated behind `surrogate`):

```rust
use std::fmt;

impl fmt::Display for HdmrResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "HDMR decomposition (d={})", self.dim)?;
        writeln!(f, "  Var[Y] = {:.4}", self.total_variance)?;
        for (order, frac) in self.order_variance.iter().enumerate() {
            writeln!(f, "  Order {} variance fraction: {:.4}", order + 1, frac)?;
        }
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}", "Factor", "S1", "ST")?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}", "------", "------", "------")?;
        for i in 0..self.dim {
            writeln!(
                f,
                "  {:>8}  {:>8.4}  {:>8.4}",
                i, self.first_order[i], self.total_order[i]
            )?;
        }
        Ok(())
    }
}
```

- [ ] **Step 2: Add Display for SobolFromPce and ShapleyIndices**

Add to `crates/salib-surrogate/src/pce.rs`:

```rust
use std::fmt;

impl fmt::Display for SobolFromPce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Sobol' indices from PCE (d={})", self.d())?;
        writeln!(f, "  Var[Y] = {:.4}", self.total_variance)?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}", "Factor", "S1", "ST")?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}", "------", "------", "------")?;
        for i in 0..self.d() {
            writeln!(
                f,
                "  {:>8}  {:>8.4}  {:>8.4}",
                i, self.first_order[i], self.total_order[i]
            )?;
        }
        Ok(())
    }
}
```

Add to `crates/salib-shapley/src/estimator.rs`:

```rust
use std::fmt;

impl fmt::Display for ShapleyIndices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Shapley effects (k={}, perms={})", self.k(), self.n_perm)?;
        writeln!(f, "  Var[Y] = {:.4}", self.var_y)?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}", "Factor", "Sh")?;
        writeln!(f, "  {:>8}  {:>8}", "------", "------")?;
        for i in 0..self.k() {
            writeln!(f, "  {:>8}  {:>8.4}", i, self.sh[i])?;
        }
        Ok(())
    }
}
```

- [ ] **Step 3: Verify it compiles**

Run: `cargo check --workspace --all-features`
Expected: Compiles clean

- [ ] **Step 4: Commit**

```bash
git add crates/salib-estimators/src/anova.rs crates/salib-estimators/src/g_theory.rs crates/salib-estimators/src/discrepancy.rs crates/salib-estimators/src/hdmr.rs crates/salib-surrogate/src/pce.rs crates/salib-shapley/src/estimator.rs
git commit -m "feat(estimators,surrogate,shapley): Display impls for ANOVA, G-theory, discrepancy, HDMR, PCE, Shapley"
```

---

### Task 8: ndarray View Acceptance in salib-estimators

**Files:**
- Modify: 14 source files in `crates/salib-estimators/src/`

- [ ] **Step 1: Add ndarray view imports**

In each file that accepts `&Array2<f64>` or `&Array3<f64>`, add `ArrayView2` and/or `ArrayView3` to the ndarray imports. For example, in `crates/salib-estimators/src/borgonovo.rs`, change:
```rust
use ndarray::Array2;
```
to:
```rust
use ndarray::{Array2, ArrayView2};
```

Files and their needed view imports:
- `anova.rs`: add `ArrayView2, ArrayView3`
- `borgonovo.rs`: add `ArrayView2`
- `rbd_fast.rs`: add `ArrayView2`
- `regression.rs`: add `ArrayView2`
- `dgsm.rs`: add `ArrayView2`
- `pawn.rs`: add `ArrayView2`
- `hdmr.rs`: add `ArrayView2`
- `given_data_sobol.rs`: add `ArrayView2`
- `bootstrap_given_data.rs`: add `ArrayView2`
- `qosa.rs`: add `ArrayView2`
- `discrepancy.rs`: add `ArrayView2`
- `g_theory.rs`: add `ArrayView3`
- `fast.rs` (estimator): add `ArrayView2`

- [ ] **Step 2: Change all function signatures**

In each file, replace `grid: &Array2<f64>` with `grid: ArrayView2<'_, f64>` (or `x:`, `gradients:`, `samples:`, `sample:` as appropriate). Replace `&Array3<f64>` with `ArrayView3<'_, f64>`.

The internal body code should work unchanged in most cases because `ArrayView2` supports the same indexing and slicing operations as `&Array2`. Where the body calls methods that require `&Array2` specifically, use `.to_owned()` only if unavoidable, or use `.view()` on intermediate results.

Key signature changes (all 23 functions):

`anova.rs`:
- `estimate_anova_two_way(grid: ArrayView2<'_, f64>)`
- `estimate_anova_two_way_with_bootstrap(grid: ArrayView2<'_, f64>, ...)`
- `bootstrap_anova_two_way(grid: ArrayView2<'_, f64>, ...)`
- `estimate_anova_three_way(grid: ArrayView3<'_, f64>)`
- `estimate_anova_three_way_with_bootstrap(grid: ArrayView3<'_, f64>, ...)`
- `bootstrap_anova_three_way(grid: ArrayView3<'_, f64>, ...)`

`borgonovo.rs`:
- `estimate_borgonovo_delta(x: ArrayView2<'_, f64>, y: &[f64])`

`rbd_fast.rs`:
- `estimate_rbd_fast(x: ArrayView2<'_, f64>, y: &[f64], harmonic: u32)`

`regression.rs`:
- `estimate_regression_indices(x: ArrayView2<'_, f64>, y: &[f64])`

`dgsm.rs`:
- `estimate_dgsm(gradients: ArrayView2<'_, f64>, ...)`
- `finite_difference_gradients(samples: ArrayView2<'_, f64>, ...)`

`pawn.rs`:
- `estimate_pawn(x: ArrayView2<'_, f64>, y: &[f64], n_slices: usize)`

`hdmr.rs`:
- `estimate_hdmr(x: ArrayView2<'_, f64>, y: &[f64], ...)`

`given_data_sobol.rs`:
- `estimate_given_data_sobol(x: ArrayView2<'_, f64>, y: &[f64])`

`bootstrap_given_data.rs`:
- `bootstrap_given_data(x: ArrayView2<'_, f64>, y: &[f64], ... estimator_fn: F)`
  - Also change the closure type `F: FnMut(ArrayView2<'_, f64>, &[f64]) -> ...`

`qosa.rs`:
- `estimate_qosa(x: ArrayView2<'_, f64>, y: &[f64], alpha: f64)`

`discrepancy.rs`:
- `compute_discrepancy(sample: ArrayView2<'_, f64>)`

`g_theory.rs`:
- `estimate_g_theory_pir(grid: ArrayView3<'_, f64>, ...)`
- `estimate_g_theory_pir_with_bootstrap(grid: ArrayView3<'_, f64>, ...)`
- `bootstrap_g_theory_pir(grid: ArrayView3<'_, f64>, ...)`

`fast.rs` (estimator, not sampler):
- `estimate_fast` — check signature; it may take `&FastDesign` rather than raw arrays. If so, skip.

- [ ] **Step 3: Fix internal compilation errors**

After changing signatures, `cargo check -p salib-estimators --all-features` will reveal any internal uses that break. Common fixes:

- `grid.view()` is redundant when `grid` is already a view — remove it
- `grid.to_owned()` if a function internally needs an owned array (should be rare)
- `.reborrow()` if a view is passed to multiple functions (shouldn't be needed in practice)
- Update any internal calls between functions that now take views

- [ ] **Step 4: Verify compilation**

Run: `cargo check -p salib-estimators --all-features`
Expected: Compiles clean

- [ ] **Step 5: Run existing tests (backward compat)**

Run: `cargo test -p salib-estimators --all-features`
Expected: All existing tests PASS unchanged (owned arrays coerce to views)

- [ ] **Step 6: Commit**

```bash
git add crates/salib-estimators/
git commit -m "feat(estimators): accept ArrayView2/ArrayView3 in all public APIs

Backward compatible — &Array2 auto-coerces to ArrayView2."
```

---

### Task 9: ndarray View Acceptance in salib-surrogate

**Files:**
- Modify: `crates/salib-surrogate/src/pce.rs`
- Modify: `crates/salib-surrogate/src/sparse_pce.rs`
- Modify: `crates/salib-surrogate/src/active_subspace.rs`

- [ ] **Step 1: Update imports and signatures**

`crates/salib-surrogate/src/pce.rs` — change:
```rust
use ndarray::Array2;
```
to:
```rust
use ndarray::{Array2, ArrayView2};
```

Change `fit_full_pce` signature:
```rust
pub fn fit_full_pce(
    samples_canonical: ArrayView2<'_, f64>,
    y: &[f64],
    families: &[PolynomialFamily],
    max_degree: usize,
) -> Result<PolynomialChaos, PceError>
```

`crates/salib-surrogate/src/sparse_pce.rs` — same import change, update `fit_sparse_pce`:
```rust
pub fn fit_sparse_pce(
    samples_canonical: ArrayView2<'_, f64>,
    y: &[f64],
    families: &[PolynomialFamily],
    max_degree: usize,
    truncation: TruncationScheme,
    solver: SparseSolver,
    max_terms: Option<usize>,
) -> Result<(PolynomialChaos, SparseFitDiagnostic), PceError>
```

`crates/salib-surrogate/src/active_subspace.rs` — update `compute_active_subspace`:
```rust
pub fn compute_active_subspace(
    gradients: ArrayView2<'_, f64>,
    gap_threshold: Option<f64>,
) -> Result<ActiveSubspace, ActiveSubspaceError>
```

- [ ] **Step 2: Fix internal compilation and run tests**

Run: `cargo check -p salib-surrogate`
Run: `cargo test -p salib-surrogate`
Expected: Compiles clean, all tests pass

- [ ] **Step 3: Commit**

```bash
git add crates/salib-surrogate/
git commit -m "feat(surrogate): accept ArrayView2 in fit_full_pce, fit_sparse_pce, compute_active_subspace"
```

---

### Task 10: Arrow and Polars Conversions

**Files:**
- Modify: `crates/salib/Cargo.toml`
- Modify: `crates/salib/src/lib.rs`
- Create: `crates/salib/src/convert/mod.rs`
- Create: `crates/salib/src/convert/arrow.rs`
- Create: `crates/salib/src/convert/polars.rs`
- Create: `crates/salib/tests/arrow_roundtrip.rs`
- Create: `crates/salib/tests/polars_roundtrip.rs`

- [ ] **Step 1: Add arrow and polars to facade Cargo.toml**

In `crates/salib/Cargo.toml`, add to `[dependencies]`:

```toml
arrow = { version = "54", optional = true, default-features = false, features = ["ffi"] }
polars = { version = "0.46", optional = true, default-features = false, features = ["lazy"] }
```

Add to `[features]`:

```toml
arrow = ["dep:arrow"]
polars = ["dep:polars", "arrow"]
```

Add to `[dev-dependencies]`:

```toml
serde_json = "1"
```

- [ ] **Step 2: Create convert module structure**

Create `crates/salib/src/convert/mod.rs`:

```rust
#[cfg(feature = "arrow")]
pub mod arrow;

#[cfg(feature = "polars")]
pub mod polars;
```

- [ ] **Step 3: Create Arrow conversions**

Create `crates/salib/src/convert/arrow.rs`:

```rust
use arrow::array::{Float64Array, StringArray, UInt32Array};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use std::sync::Arc;

fn factor_column(d: usize, names: Option<&[&str]>) -> (Field, Arc<dyn arrow::array::Array>) {
    match names {
        Some(n) => {
            let arr = StringArray::from(n[..d].to_vec());
            (Field::new("factor", DataType::Utf8, false), Arc::new(arr))
        }
        None => {
            let arr = UInt32Array::from((0..d as u32).collect::<Vec<_>>());
            (Field::new("factor", DataType::UInt32, false), Arc::new(arr))
        }
    }
}

fn f64_col(name: &str, data: &[f64]) -> (Field, Arc<dyn arrow::array::Array>) {
    (
        Field::new(name, DataType::Float64, false),
        Arc::new(Float64Array::from(data.to_vec())),
    )
}

fn build_batch(cols: Vec<(Field, Arc<dyn arrow::array::Array>)>) -> RecordBatch {
    let (fields, arrays): (Vec<_>, Vec<_>) = cols.into_iter().unzip();
    let schema = Arc::new(Schema::new(fields));
    RecordBatch::try_new(schema, arrays).unwrap_or_else(|e| panic!("arrow schema error: {e}"))
}

pub fn sobol_to_batch(
    indices: &crate::estimators::SobolIndices,
    names: Option<&[&str]>,
) -> RecordBatch {
    let d = indices.dim;
    let mut cols = vec![factor_column(d, names)];
    cols.push(f64_col("S1", &indices.first_order));
    cols.push(f64_col("ST", &indices.total_order));
    build_batch(cols)
}

pub fn morris_to_batch(
    effects: &crate::estimators::MorrisEffects,
    names: Option<&[&str]>,
) -> RecordBatch {
    let d = effects.d;
    let mut cols = vec![factor_column(d, names)];
    cols.push(f64_col("mu", &effects.mu));
    cols.push(f64_col("mu_star", &effects.mu_star));
    cols.push(f64_col("sigma", &effects.sigma));
    build_batch(cols)
}

pub fn fast_to_batch(
    indices: &crate::estimators::FastIndices,
    names: Option<&[&str]>,
) -> RecordBatch {
    let d = indices.d();
    let mut cols = vec![factor_column(d, names)];
    cols.push(f64_col("S", &indices.s));
    cols.push(f64_col("ST", &indices.st));
    build_batch(cols)
}

pub fn regression_to_batch(
    indices: &crate::estimators::RegressionIndices,
    names: Option<&[&str]>,
) -> RecordBatch {
    let d = indices.d();
    let mut cols = vec![factor_column(d, names)];
    cols.push(f64_col("SRC", &indices.src));
    cols.push(f64_col("SRRC", &indices.srrc));
    cols.push(f64_col("PCC", &indices.pcc));
    cols.push(f64_col("PRCC", &indices.prcc));
    build_batch(cols)
}

pub fn borgonovo_to_batch(
    indices: &crate::estimators::BorgonovoIndices,
    names: Option<&[&str]>,
) -> RecordBatch {
    let d = indices.d();
    let mut cols = vec![factor_column(d, names)];
    cols.push(f64_col("delta", &indices.delta));
    build_batch(cols)
}

pub fn pawn_to_batch(
    indices: &crate::estimators::PawnIndices,
    names: Option<&[&str]>,
) -> RecordBatch {
    let d = indices.d();
    let mut cols = vec![factor_column(d, names)];
    cols.push(f64_col("median", &indices.median));
    cols.push(f64_col("mean", &indices.mean));
    cols.push(f64_col("max", &indices.maximum));
    cols.push(f64_col("min", &indices.minimum));
    cols.push(f64_col("CV", &indices.cv));
    build_batch(cols)
}

pub fn dgsm_to_batch(
    indices: &crate::estimators::DgsmIndices,
    names: Option<&[&str]>,
) -> RecordBatch {
    let d = indices.d();
    let mut cols = vec![factor_column(d, names)];
    cols.push(f64_col("vi", &indices.vi));
    cols.push(f64_col("ST_upper", &indices.st_upper));
    build_batch(cols)
}

pub fn shapley_to_batch(
    indices: &crate::shapley::ShapleyIndices,
    names: Option<&[&str]>,
) -> RecordBatch {
    let d = indices.k();
    let mut cols = vec![factor_column(d, names)];
    cols.push(f64_col("Sh", &indices.sh));
    build_batch(cols)
}
```

- [ ] **Step 4: Create Polars conversions**

Create `crates/salib/src/convert/polars.rs`:

```rust
use polars::prelude::*;

pub fn batch_to_dataframe(batch: arrow::record_batch::RecordBatch) -> DataFrame {
    let schema = batch.schema();
    let mut columns: Vec<Column> = Vec::with_capacity(batch.num_columns());
    for (i, field) in schema.fields().iter().enumerate() {
        let col = batch.column(i);
        let series = Series::from_arrow(field.name().into(), col.clone())
            .unwrap_or_else(|e| panic!("polars conversion error: {e}"));
        columns.push(series.into());
    }
    DataFrame::new(columns).unwrap_or_else(|e| panic!("polars DataFrame error: {e}"))
}
```

- [ ] **Step 5: Wire up convert module in lib.rs**

In `crates/salib/src/lib.rs`, add at the end:

```rust
#[cfg(any(feature = "arrow", feature = "polars"))]
pub mod convert;
```

- [ ] **Step 6: Verify compilation**

Run: `cargo check -p salib --features arrow`
Run: `cargo check -p salib --features polars`
Expected: Both compile clean

- [ ] **Step 7: Write Arrow roundtrip test**

Create `crates/salib/tests/arrow_roundtrip.rs`:

```rust
#![cfg(feature = "arrow")]
#![allow(clippy::unwrap_used)]

use arrow::array::Float64Array;
use salib::convert::arrow::*;
use salib::estimators::SobolIndices;

#[test]
fn sobol_to_arrow_and_back() {
    let idx = SobolIndices {
        n: 1024,
        dim: 3,
        total_variance: 13.84,
        first_order: vec![0.31, 0.44, 0.00],
        total_order: vec![0.56, 0.44, 0.24],
        second_order: None,
    };
    let batch = sobol_to_batch(&idx, Some(&["x1", "x2", "x3"]));
    assert_eq!(batch.num_rows(), 3);
    assert_eq!(batch.num_columns(), 3);

    let s1 = batch
        .column_by_name("S1")
        .unwrap()
        .as_any()
        .downcast_ref::<Float64Array>()
        .unwrap();
    assert_eq!(s1.value(0), 0.31);
    assert_eq!(s1.value(1), 0.44);
    assert_eq!(s1.value(2), 0.00);
}
```

- [ ] **Step 8: Run tests**

Run: `cargo test -p salib --features arrow arrow_roundtrip`
Expected: PASS

- [ ] **Step 9: Commit**

```bash
git add crates/salib/
git commit -m "feat(salib): add Arrow and Polars conversion modules behind feature flags"
```

---

### Task 11: Full Feature Matrix Verification

**Files:** None (verification only)

- [ ] **Step 1: Run no-default-features**

Run: `cargo check --workspace --no-default-features`
Expected: Compiles clean

- [ ] **Step 2: Run default features**

Run: `cargo check --workspace`
Expected: Compiles clean

- [ ] **Step 3: Run all features**

Run: `cargo check --workspace --all-features`
Expected: Compiles clean

- [ ] **Step 4: Run serde feature alone**

Run: `cargo check -p salib --features serde`
Expected: Compiles clean

- [ ] **Step 5: Run full test suite with all features**

Run: `cargo test --workspace --all-features`
Expected: All tests pass

- [ ] **Step 6: Run clippy with all features**

Run: `cargo clippy --workspace --all-features -- -D warnings`
Expected: Zero warnings

- [ ] **Step 7: Run fmt check**

Run: `cargo fmt --all -- --check`
Expected: No formatting issues

- [ ] **Step 8: Commit if any fmt fixes needed**

```bash
cargo fmt --all
git add -A
git commit -m "chore: cargo fmt"
```
