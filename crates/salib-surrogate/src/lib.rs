//! `salib-surrogate` — surrogate models for sensitivity analysis.
//!
//! - Polynomial Chaos Expansion (PCE) with full OLS and sparse LARS
//!   coefficient selection (Blatman-Sudret 2011).
//! - Analytic Sobol' indices from PCE coefficients (Sudret 2008).
//! - Active subspace dimension reduction (Constantine 2014).
//!
//! Surrogate models build a function approximation and then derive
//! sensitivity indices analytically — a fundamentally different
//! dataflow from the direct-MC estimators in `salib-estimators`.
//!
//! # Determinism
//!
//! Polynomial evaluation and multi-index enumeration are pure;
//! same input → bit-identical output.

#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

pub mod active_subspace;
pub mod multi_index;
pub mod pce;
pub mod polynomial;
pub mod sparse_pce;

pub use active_subspace::{compute_active_subspace, ActiveSubspace, ActiveSubspaceError};
pub use multi_index::{enumerate_hyperbolic, enumerate_total_degree, MultiIndex, MultiIndexError};
pub use pce::{fit_full_pce, sobol_indices_from_pce, PceError, PolynomialChaos, SobolFromPce};
pub use polynomial::{evaluate, norm_squared, PolynomialFamily};
pub use sparse_pce::{fit_sparse_pce, SparseFitDiagnostic, SparseSolver, TruncationScheme};
