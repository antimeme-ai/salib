//! Sobol' index types — point estimates and bootstrap-CI extensions.
//!
//! Mirrors `salib_validation::SobolIndicesAnalytic`'s shape (per
//! ) so that
//! convergence-rate tests can compare estimator output directly
//! against analytic ground truth field-by-field.

use std::fmt;

/// Sobol' first-order and total-order indices, point-estimate.
/// Output of `estimate_saltelli2010` and friends.
///
/// `#[non_exhaustive]` — future fields (`dummy_floor: Option<f64>`,
/// etc.) land non-breaking.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct SobolIndices {
    /// Sample size used in the estimate (rows in each `SaltelliMatrix`
    /// matrix).
    pub n: usize,
    /// Factor count.
    pub dim: usize,
    /// `D = Var(Y)` — total output variance, sample-estimated.
    pub total_variance: f64,
    /// `Sᵢ` per factor.
    pub first_order: Vec<f64>,
    /// `S_Tᵢ` per factor.
    pub total_order: Vec<f64>,
    /// `S2_{ij}` second-order indices. `second_order[i][j]` for `i < j`.
    /// Indexed such that `second_order[i]` has length `dim - i - 1` and
    /// `second_order[i][k]` = `S2_{i, i+k+1}`.
    /// `None` when second-order was not requested.
    pub second_order: Option<Vec<Vec<f64>>>,
}

impl SobolIndices {
    /// Construct from raw values. No validation; estimator code is
    /// the only producer.
    #[must_use]
    pub fn new(
        n: usize,
        dim: usize,
        total_variance: f64,
        first_order: Vec<f64>,
        total_order: Vec<f64>,
        second_order: Option<Vec<Vec<f64>>>,
    ) -> Self {
        Self {
            n,
            dim,
            total_variance,
            first_order,
            total_order,
            second_order,
        }
    }
}

/// `SobolIndices` plus per-factor bootstrap confidence intervals.
///
/// `#[non_exhaustive]`.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub struct SobolIndicesWithCi {
    /// Point-estimate indices on the original (non-bootstrapped) data.
    pub indices: SobolIndices,
    /// 95% percentile CI per first-order index `(low, high)`.
    pub first_order_ci: Vec<(f64, f64)>,
    /// 95% percentile CI per total-order index `(low, high)`.
    pub total_order_ci: Vec<(f64, f64)>,
    /// Number of bootstrap resamples used.
    pub bootstrap_resamples: usize,
    /// Bootstrap method used.
    pub method: BootstrapMethod,
}

/// Bootstrap-CI estimation method. `#[non_exhaustive]`.
///
/// Today: `Percentile` only — matches `SALib`'s default.
/// `BCa` (bias-corrected accelerated, DiCiccio-Efron 1996) is
/// deferred;
/// default but the percentile method is sufficient for PR 7's
/// reviewer-affordance contract close.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum BootstrapMethod {
    /// Naive percentile bootstrap CI: take `α/2` and `1 - α/2`
    /// percentiles of the bootstrap distribution. `SALib`-compatible.
    Percentile,
}

impl fmt::Display for SobolIndices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Sobol' indices (N={}, d={})", self.n, self.dim)?;
        writeln!(f, "  Var[Y] = {:.4}", self.total_variance)?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}", "Factor", "S1", "ST")?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}", "------", "------", "------")?;
        for i in 0..self.dim {
            writeln!(
                f,
                "  {:>8}  {:>8.4}  {:>8.4}",
                i, self.first_order[i], self.total_order[i]
            )?;
        }
        Ok(())
    }
}

struct WithNames<'a> {
    indices: &'a SobolIndices,
    names: &'a [&'a str],
}

impl fmt::Display for WithNames<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let idx = self.indices;
        writeln!(f, "Sobol' indices (N={}, d={})", idx.n, idx.dim)?;
        writeln!(f, "  Var[Y] = {:.4}", idx.total_variance)?;
        writeln!(f)?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}", "Factor", "S1", "ST")?;
        writeln!(f, "  {:>8}  {:>8}  {:>8}", "------", "------", "------")?;
        for i in 0..idx.dim {
            let name = self.names.get(i).copied().unwrap_or("?");
            writeln!(
                f,
                "  {:>8}  {:>8.4}  {:>8.4}",
                name, idx.first_order[i], idx.total_order[i]
            )?;
        }
        Ok(())
    }
}

impl SobolIndices {
    /// Format the indices using factor names instead of numeric indices.
    pub fn display_with_names<'a>(&'a self, names: &'a [&'a str]) -> impl fmt::Display + 'a {
        WithNames {
            indices: self,
            names,
        }
    }
}

impl fmt::Display for SobolIndicesWithCi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let idx = &self.indices;
        writeln!(
            f,
            "Sobol' indices with CI (N={}, d={}, B={})",
            idx.n, idx.dim, self.bootstrap_resamples
        )?;
        writeln!(f, "  Var[Y] = {:.4}", idx.total_variance)?;
        writeln!(f)?;
        writeln!(
            f,
            "  {:>8}  {:>8}  {:>14}  {:>8}  {:>14}",
            "Factor", "S1", "S1 CI", "ST", "ST CI"
        )?;
        writeln!(
            f,
            "  {:>8}  {:>8}  {:>14}  {:>8}  {:>14}",
            "------", "------", "-----------", "------", "-----------"
        )?;
        for i in 0..idx.dim {
            let (s1_lo, s1_hi) = self.first_order_ci[i];
            let (st_lo, st_hi) = self.total_order_ci[i];
            writeln!(
                f,
                "  {:>8}  {:>8.4}  [{:.4},{:.4}]  {:>8.4}  [{:.4},{:.4}]",
                i, idx.first_order[i], s1_lo, s1_hi, idx.total_order[i], st_lo, st_hi
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn sobol_indices_new_preserves_fields() {
        let s = SobolIndices::new(64, 3, 12.5, vec![0.3, 0.5, 0.0], vec![0.4, 0.5, 0.2], None);
        assert_eq!(s.n, 64);
        assert_eq!(s.dim, 3);
        assert_eq!(s.total_variance, 12.5);
        assert_eq!(s.first_order, vec![0.3, 0.5, 0.0]);
        assert_eq!(s.total_order, vec![0.4, 0.5, 0.2]);
    }

    #[test]
    fn bootstrap_method_percentile_implements_eq() {
        assert_eq!(BootstrapMethod::Percentile, BootstrapMethod::Percentile);
    }
}
