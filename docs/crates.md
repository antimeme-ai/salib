# Crate Map

salib is a workspace of nine crates. The `salib` facade re-exports the most common types and functions; depend on individual crates for finer control over compile times and dependencies.

---

## Dependency graph

```
salib  (facade)
├── salib-core           types, distributions, RngState, deterministic reductions
├── salib-samplers       sampling designs (LHS, Sobol', Saltelli, Morris, FAST)
│   └── salib-core
├── salib-estimators     all sensitivity estimators
│   ├── salib-core
│   └── salib-samplers
├── salib-surrogate      PCE, HDMR, active subspaces  [optional: "surrogate"]
│   └── salib-core
├── salib-shapley        Shapley effects              [optional: "shapley"]
│   ├── salib-core
│   ├── salib-samplers
│   └── salib-estimators
├── salib-validation     test functions (Ishigami, Sobol' G, Morris)  [optional: "validation"]
│   └── salib-core
├── salib-tck            Gherkin test harness          [dev only]
└── salib-cli            CLI binary                    [separate install]
    ├── salib-core
    ├── salib-samplers
    └── salib-estimators
```

## Crates

### salib-core

Types that everything else depends on.

| Type | Role |
|------|------|
| `Problem` | Factor names, distributions, dimension |
| `ProblemBuilder` | Fluent builder for `Problem` |
| `Factor` | Name + distribution for one input |
| `Distribution` | `Uniform`, `Normal`, `LogNormal`, `Triangular`, `Beta` |
| `RngState` | Deterministic ChaCha20 RNG with `split()` for parallelism |
| `tree_sum`, `tree_dot`, `tree_var` | Binary-tree accumulation for bit-deterministic parallel sums |

### salib-samplers

Sampling designs that produce the input matrices for estimators.

| Design | Function / Type |
|--------|----------------|
| Latin Hypercube | `LhsSampler` |
| Sobol' QMC | `SobolSampler` |
| Saltelli cross-matrix | `build_saltelli_matrix` |
| Morris trajectories | `build_morris_trajectories` |
| FAST/eFAST search curves | `build_fast_design` |
| Iman–Conover correlation | `iman_conover_transform` |

### salib-estimators

All sensitivity estimators. Each takes a sample matrix (or design) and a model closure, returns a result struct with `Display` and optional `serde`.

| Family | Function |
|--------|----------|
| Sobol' (Saltelli 2010) | `estimate_saltelli2010` |
| Sobol' (Jansen) | `estimate_jansen` |
| Sobol' (Janon) | `estimate_janon` |
| Sobol' (Owen) | `estimate_owen` |
| Given-data Sobol' | `estimate_given_data_sobol` |
| Morris | `estimate_morris_effects` |
| Grouped Morris | `estimate_grouped_morris_effects` |
| FAST / eFAST | `estimate_fast` |
| RBD-FAST | `estimate_rbd_fast` |
| Borgonovo δ | `estimate_borgonovo_delta` |
| PAWN | `estimate_pawn` |
| QOSA | `estimate_qosa` |
| DGSM | `estimate_dgsm` |
| SRC / SRRC / PCC / PRCC | `estimate_regression_indices` |
| HDMR | `estimate_hdmr` |
| ANOVA | `estimate_anova_two_way` |
| G-Theory | `estimate_g_theory_pir` |
| Discrepancy | `compute_discrepancy` |
| Fractional factorial | `estimate_fractional_factorial` |

### salib-surrogate

Surrogate models. Optional — enable with `features = ["surrogate"]` on the facade.

| Component | Function |
|-----------|----------|
| Full PCE (OLS) | `fit_full_pce` |
| Sparse PCE (LARS/OMP) | `fit_sparse_pce` |
| Active subspaces | `compute_active_subspace` |

### salib-shapley

Shapley effects. Optional — enable with `features = ["shapley"]`.

| Component | Function |
|-----------|----------|
| Shapley effects | `estimate_shapley` |

### salib-validation

Analytic test functions with closed-form sensitivity indices. Optional — enable with `features = ["validation"]`.

| Function | Factors | Closed-form |
|----------|---------|-------------|
| Ishigami | 3 | $S_i$, $S_{Ti}$ |
| Sobol' G | $d$ | $S_i$, $S_{Ti}$ |
| Morris | $d$ | $\mu^*$, $\sigma$ classification |
| Linear | $d$ | exact SRC |

### salib-tck

Internal test-contract-kit. Gherkin-based integration tests. Not published for external use.

### salib-cli

Command-line interface. Install with `cargo install salib-cli`.

```
salib sample   # generate samples to CSV
salib run      # evaluate a model on samples
salib analyze  # compute sensitivity indices
```

---

## Feature flags

All flags on the `salib` facade crate:

| Flag | Default | Effect |
|------|---------|--------|
| `samplers` | yes | Sampling designs |
| `estimators` | yes | All sensitivity estimators |
| `parallel` | yes | Rayon-based parallel reductions |
| `surrogate` | no | PCE, HDMR, active subspaces |
| `shapley` | no | Shapley effects |
| `validation` | no | Analytic test functions |
| `serde` | no | `Serialize`/`Deserialize` on result types |
| `arrow` | no | Arrow `RecordBatch` conversions |
| `polars` | no | Polars `DataFrame` conversions |
| `full` | no | Everything except `serde`, `arrow`, `polars` |

```toml
# Kitchen sink
salib = { version = "0.1", features = ["full", "serde"] }

# Minimal: just Sobol' indices, no parallelism
salib = { version = "0.1", default-features = false, features = ["samplers", "estimators"] }
```
