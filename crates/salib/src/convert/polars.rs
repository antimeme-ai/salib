//! Polars `DataFrame` conversions for SALib result types.
//!
//! Polars uses `polars-arrow` internally rather than the `arrow` crate, so
//! these functions build `DataFrame`s directly from SALib types using
//! native Polars Series constructors.

use polars::prelude::*;

fn factor_series(d: usize, names: Option<&[&str]>) -> Column {
    match names {
        Some(n) => {
            let s: Series = Series::new("factor".into(), &n[..d]);
            s.into()
        }
        None => {
            let indices: Vec<u32> = (0..d as u32).collect();
            let s: Series = Series::new("factor".into(), &indices);
            s.into()
        }
    }
}

fn f64_series(name: &str, data: &[f64]) -> Column {
    let s: Series = Series::new(name.into(), data);
    s.into()
}

fn build_df(cols: Vec<Column>) -> DataFrame {
    DataFrame::new(cols).unwrap_or_else(|e| panic!("polars DataFrame error: {e}"))
}

// ---------------------------------------------------------------------------
// Estimator types
// ---------------------------------------------------------------------------

/// Convert `SobolIndices` to a Polars `DataFrame` with columns
/// `[factor, S1, ST]`.
#[cfg(feature = "estimators")]
#[must_use]
pub fn sobol_to_df(idx: &crate::estimators::SobolIndices, names: Option<&[&str]>) -> DataFrame {
    let d = idx.dim;
    build_df(vec![
        factor_series(d, names),
        f64_series("S1", &idx.first_order),
        f64_series("ST", &idx.total_order),
    ])
}

/// Convert `MorrisEffects` to a Polars `DataFrame` with columns
/// `[factor, mu, mu_star, sigma]`.
#[cfg(feature = "estimators")]
#[must_use]
pub fn morris_to_df(eff: &crate::estimators::MorrisEffects, names: Option<&[&str]>) -> DataFrame {
    let d = eff.d;
    build_df(vec![
        factor_series(d, names),
        f64_series("mu", &eff.mu),
        f64_series("mu_star", &eff.mu_star),
        f64_series("sigma", &eff.sigma),
    ])
}

/// Convert `FastIndices` to a Polars `DataFrame` with columns
/// `[factor, S, ST]`.
#[cfg(feature = "estimators")]
#[must_use]
pub fn fast_to_df(idx: &crate::estimators::FastIndices, names: Option<&[&str]>) -> DataFrame {
    let d = idx.d();
    build_df(vec![
        factor_series(d, names),
        f64_series("S", &idx.s),
        f64_series("ST", &idx.st),
    ])
}

/// Convert `RegressionIndices` to a Polars `DataFrame` with columns
/// `[factor, SRC, SRRC, PCC, PRCC]`.
#[cfg(feature = "estimators")]
#[must_use]
pub fn regression_to_df(
    idx: &crate::estimators::RegressionIndices,
    names: Option<&[&str]>,
) -> DataFrame {
    let d = idx.d();
    build_df(vec![
        factor_series(d, names),
        f64_series("SRC", &idx.src),
        f64_series("SRRC", &idx.srrc),
        f64_series("PCC", &idx.pcc),
        f64_series("PRCC", &idx.prcc),
    ])
}

/// Convert `BorgonovoIndices` to a Polars `DataFrame` with columns
/// `[factor, delta]`.
#[cfg(feature = "estimators")]
#[must_use]
pub fn borgonovo_to_df(
    idx: &crate::estimators::BorgonovoIndices,
    names: Option<&[&str]>,
) -> DataFrame {
    let d = idx.d();
    build_df(vec![
        factor_series(d, names),
        f64_series("delta", &idx.delta),
    ])
}

/// Convert `PawnIndices` to a Polars `DataFrame` with columns
/// `[factor, median, mean, maximum, minimum, cv]`.
#[cfg(feature = "estimators")]
#[must_use]
pub fn pawn_to_df(idx: &crate::estimators::PawnIndices, names: Option<&[&str]>) -> DataFrame {
    let d = idx.d();
    build_df(vec![
        factor_series(d, names),
        f64_series("median", &idx.median),
        f64_series("mean", &idx.mean),
        f64_series("maximum", &idx.maximum),
        f64_series("minimum", &idx.minimum),
        f64_series("cv", &idx.cv),
    ])
}

/// Convert `DgsmIndices` to a Polars `DataFrame` with columns
/// `[factor, vi, st_upper]`.
#[cfg(feature = "estimators")]
#[must_use]
pub fn dgsm_to_df(idx: &crate::estimators::DgsmIndices, names: Option<&[&str]>) -> DataFrame {
    let d = idx.d();
    build_df(vec![
        factor_series(d, names),
        f64_series("vi", &idx.vi),
        f64_series("st_upper", &idx.st_upper),
    ])
}

// ---------------------------------------------------------------------------
// Shapley
// ---------------------------------------------------------------------------

/// Convert `ShapleyIndices` to a Polars `DataFrame` with columns
/// `[factor, Sh]`.
#[cfg(feature = "shapley")]
#[must_use]
pub fn shapley_to_df(idx: &crate::shapley::ShapleyIndices, names: Option<&[&str]>) -> DataFrame {
    let d = idx.k();
    build_df(vec![factor_series(d, names), f64_series("Sh", &idx.sh)])
}
