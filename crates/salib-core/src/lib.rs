//! `salib-core` — foundational types for the salib sensitivity-analysis
//! library.
//!
//! Owns the experiment-level vocabulary that every other salib crate
//! composes against: [`Problem`], [`Factor`], the closed [`Distribution`]
//! enum, [`rng::RngState`] (multi-stream ChaCha20 with deterministic
//! salt-derived forking), and the [`reduce`] primitives
//! ([`reduce::tree_sum`], [`reduce::tree_dot`], [`reduce::tree_var`] and
//! their `par_` variants).
//!
//! Same architectural family as Python's `SALib`, R's `sensitivity`, and
//! MATLAB's `UQLab`.
//!
//! # Determinism
//!
//! `salib-core` is the determinism floor every sampler and estimator
//! stands on:
//!
//! - [`rng::RngState`] is a serializable RNG identity. Recording it
//!   lets a verifier reconstruct any SA campaign's RNG stream.
//! - [`reduce`] reductions defeat the float-associativity
//!   nondeterminism that naive `par_iter().sum()` over `f64` produces
//!   under rayon. Bit-identical regardless of thread count.

#![forbid(unsafe_code)]
#![deny(clippy::disallowed_methods)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

pub mod distribution;
pub mod problem;
pub mod reduce;
pub mod rng;

pub use distribution::Distribution;
pub use problem::{BuildError, Factor, FactorKind, Group, Problem, ProblemBuilder};
pub use reduce::{par_tree_dot, par_tree_sum, par_tree_var, tree_dot, tree_sum, tree_var, BLOCK};
pub use rng::{RngAlgorithm, RngState};
