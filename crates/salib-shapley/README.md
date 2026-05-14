# salib-shapley

[![crates.io](https://img.shields.io/crates/v/salib-shapley.svg)](https://crates.io/crates/salib-shapley)
[![docs.rs](https://img.shields.io/docsrs/salib-shapley)](https://docs.rs/salib-shapley)
[![license](https://img.shields.io/crates/l/salib-shapley.svg)](https://github.com/antimeme-ai/salib)

Shapley effects estimator for global sensitivity analysis. Part of the
[salib](https://crates.io/crates/salib) workspace.

Most users should depend on `salib` with the `shapley` feature. Use
`salib-shapley` directly when you want Shapley effects without pulling
in samplers or other estimators.

## What's inside

A single estimator: `estimate_shapley`, implementing the permutation-based
algorithm of Song, Nelson & Staum (2016) "Shapley effects for global
sensitivity analysis", *SIAM/ASA J. Uncertainty Quantification* 4(1).

Shapley effects allocate output variance fairly among correlated inputs,
unlike Sobol' indices which assume factor independence. When inputs are
independent, Shapley effects reduce to total-order Sobol' indices.

Returns `ShapleyIndices` with per-factor `sh` values summing to 1.

## Feature flags

| Flag | Default | Effect |
|---|---|---|
| `serde` | no | `Serialize`/`Deserialize` on `ShapleyIndices` |

## License

MIT OR Apache-2.0, at your option.
