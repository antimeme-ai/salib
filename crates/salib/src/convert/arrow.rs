//! Arrow `RecordBatch` conversions for SALib result types.

use arrow::array::{Float64Array, StringArray, UInt32Array};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use std::sync::Arc;

fn factor_column(d: usize, names: Option<&[&str]>) -> (Field, Arc<dyn arrow::array::Array>) {
    match names {
        Some(n) => {
            let arr = StringArray::from(n[..d].to_vec());
            (Field::new("factor", DataType::Utf8, false), Arc::new(arr))
        }
        None => {
            let arr = UInt32Array::from((0..d as u32).collect::<Vec<_>>());
            (Field::new("factor", DataType::UInt32, false), Arc::new(arr))
        }
    }
}

fn f64_col(name: &str, data: &[f64]) -> (Field, Arc<dyn arrow::array::Array>) {
    (
        Field::new(name, DataType::Float64, false),
        Arc::new(Float64Array::from(data.to_vec())),
    )
}

fn build_batch(cols: Vec<(Field, Arc<dyn arrow::array::Array>)>) -> RecordBatch {
    let (fields, arrays): (Vec<_>, Vec<_>) = cols.into_iter().unzip();
    let schema = Arc::new(Schema::new(fields));
    RecordBatch::try_new(schema, arrays).unwrap_or_else(|e| panic!("arrow schema error: {e}"))
}

// ---------------------------------------------------------------------------
// Estimator types
// ---------------------------------------------------------------------------

/// Convert `SobolIndices` to an Arrow `RecordBatch` with columns
/// `[factor, S1, ST]`.
#[cfg(feature = "estimators")]
#[must_use]
pub fn sobol_to_batch(
    idx: &crate::estimators::SobolIndices,
    names: Option<&[&str]>,
) -> RecordBatch {
    let d = idx.dim;
    build_batch(vec![
        factor_column(d, names),
        f64_col("S1", &idx.first_order),
        f64_col("ST", &idx.total_order),
    ])
}

/// Convert `MorrisEffects` to an Arrow `RecordBatch` with columns
/// `[factor, mu, mu_star, sigma]`.
#[cfg(feature = "estimators")]
#[must_use]
pub fn morris_to_batch(
    eff: &crate::estimators::MorrisEffects,
    names: Option<&[&str]>,
) -> RecordBatch {
    let d = eff.d;
    build_batch(vec![
        factor_column(d, names),
        f64_col("mu", &eff.mu),
        f64_col("mu_star", &eff.mu_star),
        f64_col("sigma", &eff.sigma),
    ])
}

/// Convert `FastIndices` to an Arrow `RecordBatch` with columns
/// `[factor, S, ST]`.
#[cfg(feature = "estimators")]
#[must_use]
pub fn fast_to_batch(idx: &crate::estimators::FastIndices, names: Option<&[&str]>) -> RecordBatch {
    let d = idx.d();
    build_batch(vec![
        factor_column(d, names),
        f64_col("S", &idx.s),
        f64_col("ST", &idx.st),
    ])
}

/// Convert `RegressionIndices` to an Arrow `RecordBatch` with columns
/// `[factor, SRC, SRRC, PCC, PRCC]`.
#[cfg(feature = "estimators")]
#[must_use]
pub fn regression_to_batch(
    idx: &crate::estimators::RegressionIndices,
    names: Option<&[&str]>,
) -> RecordBatch {
    let d = idx.d();
    build_batch(vec![
        factor_column(d, names),
        f64_col("SRC", &idx.src),
        f64_col("SRRC", &idx.srrc),
        f64_col("PCC", &idx.pcc),
        f64_col("PRCC", &idx.prcc),
    ])
}

/// Convert `BorgonovoIndices` to an Arrow `RecordBatch` with columns
/// `[factor, delta]`.
#[cfg(feature = "estimators")]
#[must_use]
pub fn borgonovo_to_batch(
    idx: &crate::estimators::BorgonovoIndices,
    names: Option<&[&str]>,
) -> RecordBatch {
    let d = idx.d();
    build_batch(vec![factor_column(d, names), f64_col("delta", &idx.delta)])
}

/// Convert `PawnIndices` to an Arrow `RecordBatch` with columns
/// `[factor, median, mean, maximum, minimum, cv]`.
#[cfg(feature = "estimators")]
#[must_use]
pub fn pawn_to_batch(idx: &crate::estimators::PawnIndices, names: Option<&[&str]>) -> RecordBatch {
    let d = idx.d();
    build_batch(vec![
        factor_column(d, names),
        f64_col("median", &idx.median),
        f64_col("mean", &idx.mean),
        f64_col("maximum", &idx.maximum),
        f64_col("minimum", &idx.minimum),
        f64_col("cv", &idx.cv),
    ])
}

/// Convert `DgsmIndices` to an Arrow `RecordBatch` with columns
/// `[factor, vi, st_upper]`.
#[cfg(feature = "estimators")]
#[must_use]
pub fn dgsm_to_batch(idx: &crate::estimators::DgsmIndices, names: Option<&[&str]>) -> RecordBatch {
    let d = idx.d();
    build_batch(vec![
        factor_column(d, names),
        f64_col("vi", &idx.vi),
        f64_col("st_upper", &idx.st_upper),
    ])
}

// ---------------------------------------------------------------------------
// Shapley
// ---------------------------------------------------------------------------

/// Convert `ShapleyIndices` to an Arrow `RecordBatch` with columns
/// `[factor, Sh]`.
#[cfg(feature = "shapley")]
#[must_use]
pub fn shapley_to_batch(
    idx: &crate::shapley::ShapleyIndices,
    names: Option<&[&str]>,
) -> RecordBatch {
    let d = idx.k();
    build_batch(vec![factor_column(d, names), f64_col("Sh", &idx.sh)])
}
