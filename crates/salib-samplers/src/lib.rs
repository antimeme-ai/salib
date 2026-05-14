//! `salib-samplers` — sampling designs for sensitivity analysis.
//!
//! Two surfaces:
//!
//! - **`Sampler` trait** — i.i.d.-shaped samplers producing unit-cube
//!   samples in `[0, 1)^d` (LHS, Sobol'). Downstream code maps the
//!   unit samples through factor distributions via
//!   `Distribution::quantile`.
//! - **Free-function constructors** for designs whose output is
//!   intrinsically blocked or carries load-bearing metadata:
//!   `build_saltelli_matrix` (the `(A, B, A_Bⁱ)` construction),
//!   `build_morris_trajectories` (OAT trajectories with factor-
//!   permutation order), and `build_fast_design` (search-curve
//!   blocks with frequency / phase metadata).
//!
//! # Determinism
//!
//! Every sampler advances the input `RngState`'s `word_pos` by the
//! exact number of bytes consumed; same `RngState` in → bit-identical
//! `Array2<f64>` out.

#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

pub mod fast;
pub mod iman_conover;
pub mod lhs;
pub mod morris;
pub mod owen_matrix;
pub mod plackett_burman;
pub mod saltelli_matrix;
pub mod sampler;
pub mod sobol;

pub use fast::{build_fast_design, FastDesign, FastError};
pub use iman_conover::{iman_conover_transform, ImanConoverError};
pub use lhs::{LhsKind, LhsSampler};
pub use morris::{
    build_grouped_morris_trajectories, build_morris_trajectories, MorrisError, MorrisTrajectories,
};
pub use owen_matrix::{build_owen_matrix, OwenMatrix, OwenMatrixError};
pub use plackett_burman::{build_plackett_burman, PbError, PlackettBurmanDesign};
pub use saltelli_matrix::{
    build_grouped_saltelli_matrix, build_saltelli_matrix, SaltelliError, SaltelliMatrix,
};
pub use sampler::Sampler;
pub use sobol::{SobolDimSet, SobolSampler};
