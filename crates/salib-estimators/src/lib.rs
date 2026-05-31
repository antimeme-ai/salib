//! `salib-estimators` — sensitivity-index estimators.
//!
//! Variance-based Sobol' (Saltelli2010, Jansen, Janon, Owen), Morris
//! elementary effects, FAST/eFAST/RBD-FAST, Borgonovo δ, PAWN, DGSM,
//! regression (SRC/SRRC/PCC/PRCC), given-data Sobol', ANOVA, G-theory,
//! fractional factorial, and discrepancy measures.
//!
//! # Feature flags
//!
//! - **`surrogate`** — enables the [`hdmr`] module (RS-HDMR via PCE
//!   decomposition). Pulls in `salib-surrogate`.
//!
//! # Determinism
//!
//! All sums route through `salib_core::reduce::tree_*` —
//! bit-identical regardless of rayon partitioning. Bootstrap RNG
//! draws use `ChaCha20Rng` derived from the caller's `RngState`.

#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

pub mod anova;
pub mod bootstrap;
pub mod bootstrap_given_data;
pub mod borgonovo;
pub mod dgsm;
pub mod discrepancy;
pub mod fast;
pub mod fractional_factorial;
pub mod g_theory;
pub mod given_data_sobol;
#[cfg(feature = "surrogate")]
pub mod hdmr;
pub mod janon;
pub mod jansen;
pub mod morris;
pub mod owen;
pub mod pawn;
pub mod qosa;
pub mod rbd_fast;
pub mod regression;
pub mod saltelli2010;
pub mod sobol_indices;

pub use anova::{
    bootstrap_anova_three_way, bootstrap_anova_two_way, estimate_anova_three_way,
    estimate_anova_three_way_with_bootstrap, estimate_anova_two_way,
    estimate_anova_two_way_with_bootstrap, AnovaBootstrapError, AnovaError, AnovaThreeWayResult,
    AnovaTwoWayResult,
};
pub use bootstrap::{
    estimate_saltelli2010_from_outputs_with_bootstrap, estimate_saltelli2010_with_bootstrap,
};
pub use bootstrap_given_data::{
    bootstrap_given_data, BootstrapCi, BootstrapGivenDataError, BoxedEstimatorError,
};
pub use borgonovo::{estimate_borgonovo_delta, BorgonovoError, BorgonovoIndices};
pub use dgsm::{
    estimate_dgsm, finite_difference_gradients, poincare_constant, DgsmError, DgsmIndices, FdKind,
    PoincareError,
};
pub use discrepancy::{compute_discrepancy, DiscrepancyError, DiscrepancyResult};
pub use fast::{estimate_fast, FastEstimatorError, FastIndices};
pub use fractional_factorial::{estimate_fractional_factorial, FractionalFactorialEffects};
pub use g_theory::{
    bootstrap_g_theory_pir, estimate_g_theory_pir, estimate_g_theory_pir_with_bootstrap,
    project_g_theory_d_study, DStudyPoint, GTheoryBootstrapError, GTheoryDesign, GTheoryError,
    GTheoryResult,
};
pub use given_data_sobol::{estimate_given_data_sobol, GivenDataSobolError, GivenDataSobolIndices};
#[cfg(feature = "surrogate")]
pub use hdmr::{estimate_hdmr, HdmrError, HdmrResult};
pub use janon::{estimate_janon, JanonIndices};
pub use jansen::{estimate_jansen, JansenIndices};
pub use morris::{
    estimate_grouped_morris_effects, estimate_morris_effects, EmptyError, MorrisEffects,
};
pub use owen::{estimate_owen, OwenIndices};
pub use pawn::{estimate_pawn, PawnError, PawnIndices};
pub use qosa::{estimate_qosa, QosaError, QosaIndices};
pub use rbd_fast::{estimate_rbd_fast, RbdFastError, RbdFastIndices};
pub use regression::{estimate_regression_indices, RegressionError, RegressionIndices};
pub use saltelli2010::{estimate_saltelli2010, estimate_saltelli2010_from_outputs};
pub use sobol_indices::{BootstrapMethod, SobolIndices, SobolIndicesWithCi};
