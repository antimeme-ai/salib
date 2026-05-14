# salib-estimators

[![crates.io](https://img.shields.io/crates/v/salib-estimators.svg)](https://crates.io/crates/salib-estimators)
[![docs.rs](https://img.shields.io/docsrs/salib-estimators)](https://docs.rs/salib-estimators)
[![license](https://img.shields.io/crates/l/salib-estimators.svg)](https://github.com/antimeme-ai/salib)

Sensitivity analysis estimators. Part of the
[salib](https://crates.io/crates/salib) workspace.

Most users should depend on `salib` with the `estimators` feature (on by
default). Use `salib-estimators` directly for a lighter dependency tree
without samplers.

## Estimators

| Category | Method | Function |
|---|---|---|
| Variance-based | Saltelli 2010 | `estimate_saltelli2010` |
| | Jansen | `estimate_jansen` |
| | Janon | `estimate_janon` |
| | Owen | `estimate_owen` |
| | Given-data Sobol' | `estimate_given_data_sobol` |
| Elementary effects | Morris | `estimate_morris_effects` |
| | Grouped Morris | `estimate_grouped_morris_effects` |
| Frequency-based | FAST / eFAST | `estimate_fast` |
| | RBD-FAST | `estimate_rbd_fast` |
| Distribution-based | Borgonovo delta | `estimate_borgonovo_delta` |
| | PAWN | `estimate_pawn` |
| | QOSA | `estimate_qosa` |
| Derivative-based | DGSM | `estimate_dgsm` |
| Regression | SRC / SRRC / PCC / PRCC | `estimate_regression_indices` |
| Experimental design | ANOVA (2- and 3-way) | `estimate_anova_two_way` |
| | G-theory | `estimate_g_theory_pir` |
| | Discrepancy (L2-star) | `compute_discrepancy` |
| | Fractional factorial | `estimate_fractional_factorial` |

Bootstrap confidence intervals are available via `estimate_saltelli2010_with_bootstrap`,
`estimate_anova_*_with_bootstrap`, and `estimate_g_theory_pir_with_bootstrap`.

All result types implement `Display` for human-readable output.

## Feature flags

| Flag | Default | Effect |
|---|---|---|
| `surrogate` | no | Enables HDMR (depends on `salib-surrogate` for PCE) |
| `serde` | no | `Serialize`/`Deserialize` on all result types |

## Inputs

Public functions accept `ArrayView2<f64>` / `ArrayView3<f64>` rather
than owned arrays. Pass `arr.view()` or let auto-coercion handle it.

## License

MIT OR Apache-2.0, at your option.
