//! `salib-validation` — analytic test functions with closed-form
//! sensitivity indices, plus frozen reference data.
//!
//! # Test functions
//!
//! | Function                  | Source                               |
//! |---------------------------|--------------------------------------|
//! | Ishigami                  | Ishigami-Homma 1990                  |
//! | Sobol' G (tunable)        | Saltelli-Sobol' 1995                 |
//! | Morris test (20-factor)   | Morris 1991                          |
//!
//! Each function module exports:
//! - `fn name(x: &[f64]) -> f64` — the function evaluation.
//! - `fn analytic_indices(params...) -> SobolIndicesAnalytic` — closed-form
//!   Sobol' decomposition (ground truth).
//! - `fn input_distribution(...) -> Problem` — the canonical input space.

#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

pub mod analytic;
pub mod ishigami;
pub mod morris_test;
pub mod sobol_g;

pub use analytic::{MorrisEffectsAnalytic, SobolIndicesAnalytic};
