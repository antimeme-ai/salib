# salib-validation

[![crates.io](https://img.shields.io/crates/v/salib-validation.svg)](https://crates.io/crates/salib-validation)
[![docs.rs](https://img.shields.io/docsrs/salib-validation)](https://docs.rs/salib-validation)
[![license](https://img.shields.io/crates/l/salib-validation.svg)](https://github.com/antimeme-ai/salib)

Analytic test functions with closed-form sensitivity indices for
validating estimator implementations. Part of the
[salib](https://crates.io/crates/salib) workspace.

This crate is primarily for testing and benchmarking. Application code
rarely needs it directly.

## Test functions

| Function | Module | Factors | Closed-form indices |
|---|---|---|---|
| Ishigami | `ishigami` | 3 | First-order, total-order Sobol' |
| Sobol' G | `sobol_g` | *d* | First-order, total-order Sobol' |
| Morris additive | `morris_test` | *d* | mu, mu_star, sigma |
| Morris quadratic | `morris_test` | *d* | mu, mu_star, sigma |

Each module provides the function itself, `analytic_indices` (or
`analytic_effects`), and `input_distribution` returning the canonical
factor ranges.

The Ishigami function at (a=7, b=0.1) is the primary integration test
across the workspace. Its interaction structure (sin(X1) * X3^4 cross-term)
exercises both first-order and total-order estimators.

## Feature flags

| Flag | Default | Effect |
|---|---|---|
| `serde` | no | `Serialize`/`Deserialize` on `SobolIndicesAnalytic`, `MorrisEffectsAnalytic` |

## License

MIT OR Apache-2.0, at your option.
