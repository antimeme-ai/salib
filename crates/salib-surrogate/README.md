# salib-surrogate

[![crates.io](https://img.shields.io/crates/v/salib-surrogate.svg)](https://crates.io/crates/salib-surrogate)
[![docs.rs](https://img.shields.io/docsrs/salib-surrogate)](https://docs.rs/salib-surrogate)
[![license](https://img.shields.io/crates/l/salib-surrogate.svg)](https://github.com/antimeme-ai/salib)

Surrogate models for sensitivity analysis: Polynomial Chaos Expansion
(PCE) and active subspaces. Part of the
[salib](https://crates.io/crates/salib) workspace.

Most users should depend on `salib` with the `surrogate` feature. Use
`salib-surrogate` directly when you only need PCE fitting without the
full estimator suite.

## What's inside

| Component | Function | Reference |
|---|---|---|
| Full PCE (OLS) | `fit_full_pce` | Xiu & Karniadakis (2002) |
| Sparse PCE (LARS, OMP) | `fit_sparse_pce` | Blatman & Sudret (2011) |
| Sobol' from PCE | `sobol_indices_from_pce` | Sudret (2008) |
| Active subspaces | `compute_active_subspace` | Constantine (2015) |

PCE fitting operates on canonical [-1, 1]^d inputs with Legendre or
Hermite polynomial families. Truncation schemes include total-degree
and hyperbolic cross.

`SparseSolver::Lars` and `SparseSolver::Omp` are both available; both
recover Ishigami Sobol' indices within 0.02 absolute using fewer than
80 active terms out of 286 candidates.

## Feature flags

| Flag | Default | Effect |
|---|---|---|
| `serde` | no | `Serialize`/`Deserialize` on all types (includes ndarray serde) |

## License

MIT OR Apache-2.0, at your option.
