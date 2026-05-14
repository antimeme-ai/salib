#![cfg(feature = "serde")]

// All types are `#[non_exhaustive]` and live in a separate crate scope from
// this integration test binary. Struct-literal construction is forbidden by
// `#[non_exhaustive]`. Strategy: deserialize an initial value from a known-
// good JSON string, round-trip it, and assert the two Debug representations
// match. This tests both Deserialize and Serialize paths end-to-end.

use salib_estimators::{
    anova::{AnovaThreeWayResult, AnovaTwoWayResult},
    bootstrap_given_data::BootstrapCi,
    borgonovo::BorgonovoIndices,
    discrepancy::DiscrepancyResult,
    dgsm::{DgsmIndices, FdKind},
    fast::FastIndices,
    fractional_factorial::FractionalFactorialEffects,
    g_theory::{DStudyPoint, GTheoryDesign, GTheoryResult},
    given_data_sobol::GivenDataSobolIndices,
    janon::JanonIndices,
    jansen::JansenIndices,
    morris::MorrisEffects,
    owen::OwenIndices,
    pawn::PawnIndices,
    qosa::QosaIndices,
    rbd_fast::RbdFastIndices,
    regression::RegressionIndices,
    sobol_indices::{BootstrapMethod, SobolIndices, SobolIndicesWithCi},
};

// ── Helpers ────────────────────────────────────────────────────────────

/// Deserialize from a JSON string, serialize back to JSON, deserialize again,
/// and assert the two Debug representations match.
fn roundtrip_json<T>(json: &str)
where
    T: serde::Serialize + serde::de::DeserializeOwned + std::fmt::Debug,
{
    let val: T = serde_json::from_str(json).expect("initial deserialize");
    let serialized = serde_json::to_string(&val).expect("serialize");
    let back: T = serde_json::from_str(&serialized).expect("re-deserialize");
    assert_eq!(format!("{val:?}"), format!("{back:?}"));
}

/// For types that implement PartialEq, assert value equality rather than Debug equality.
fn roundtrip_eq<T>(json: &str)
where
    T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
{
    let val: T = serde_json::from_str(json).expect("initial deserialize");
    let serialized = serde_json::to_string(&val).expect("serialize");
    let back: T = serde_json::from_str(&serialized).expect("re-deserialize");
    assert_eq!(val, back);
}

// ── sobol_indices.rs ───────────────────────────────────────────────────

#[test]
fn sobol_indices_roundtrip() {
    roundtrip_eq::<SobolIndices>(
        r#"{
            "n": 1024,
            "dim": 3,
            "total_variance": 13.84,
            "first_order": [0.31, 0.44, 0.0],
            "total_order": [0.56, 0.44, 0.24],
            "second_order": null
        }"#,
    );
}

#[test]
fn sobol_indices_with_second_order_roundtrip() {
    roundtrip_eq::<SobolIndices>(
        r#"{
            "n": 512,
            "dim": 3,
            "total_variance": 7.5,
            "first_order": [0.1, 0.2, 0.3],
            "total_order": [0.4, 0.5, 0.6],
            "second_order": [[0.05, 0.07], [0.03]]
        }"#,
    );
}

#[test]
fn sobol_indices_with_ci_roundtrip() {
    roundtrip_eq::<SobolIndicesWithCi>(
        r#"{
            "indices": {
                "n": 256,
                "dim": 2,
                "total_variance": 5.0,
                "first_order": [0.3, 0.7],
                "total_order": [0.35, 0.72],
                "second_order": null
            },
            "first_order_ci": [[0.20, 0.40], [0.60, 0.80]],
            "total_order_ci": [[0.25, 0.45], [0.62, 0.82]],
            "bootstrap_resamples": 500,
            "method": "Percentile"
        }"#,
    );
}

#[test]
fn bootstrap_method_roundtrip() {
    roundtrip_eq::<BootstrapMethod>(r#""Percentile""#);
}

// ── morris.rs ──────────────────────────────────────────────────────────

#[test]
fn morris_effects_roundtrip() {
    roundtrip_eq::<MorrisEffects>(
        r#"{
            "r": 20,
            "d": 3,
            "mu": [0.1, -0.5, 1.2],
            "mu_star": [0.1, 0.5, 1.2],
            "sigma": [0.05, 0.3, 0.8],
            "grouped_mu": null,
            "grouped_mu_star": null,
            "grouped_sigma": null,
            "group_names": null
        }"#,
    );
}

#[test]
fn morris_effects_grouped_roundtrip() {
    roundtrip_eq::<MorrisEffects>(
        r#"{
            "r": 10,
            "d": 4,
            "mu": [0.1, -0.2, 0.3, -0.4],
            "mu_star": [0.1, 0.2, 0.3, 0.4],
            "sigma": [0.01, 0.02, 0.03, 0.04],
            "grouped_mu": [0.05, -0.15],
            "grouped_mu_star": [0.05, 0.15],
            "grouped_sigma": [0.02, 0.03],
            "group_names": ["G1", "G2"]
        }"#,
    );
}

// ── fast.rs ────────────────────────────────────────────────────────────

#[test]
fn fast_indices_roundtrip() {
    roundtrip_json::<FastIndices>(
        r#"{
            "s": [0.31, 0.44, 0.0],
            "st": [0.56, 0.44, 0.24]
        }"#,
    );
}

// ── borgonovo.rs ───────────────────────────────────────────────────────

#[test]
fn borgonovo_indices_roundtrip() {
    roundtrip_json::<BorgonovoIndices>(
        r#"{"delta": [0.1, 0.2, 0.3]}"#,
    );
}

// ── regression.rs ──────────────────────────────────────────────────────

#[test]
fn regression_indices_roundtrip() {
    roundtrip_json::<RegressionIndices>(
        r#"{
            "src": [0.5, -0.3, 0.1],
            "srrc": [0.52, -0.31, 0.12],
            "pcc": [0.48, -0.29, 0.09],
            "prcc": [0.50, -0.30, 0.11],
            "r2_linear": 0.92,
            "r2_rank": 0.89
        }"#,
    );
}

// ── pawn.rs ────────────────────────────────────────────────────────────

#[test]
fn pawn_indices_roundtrip() {
    roundtrip_json::<PawnIndices>(
        r#"{
            "median": [0.4, 0.1, 0.05],
            "maximum": [0.6, 0.2, 0.1],
            "mean": [0.45, 0.12, 0.07],
            "minimum": [0.3, 0.05, 0.02],
            "cv": [0.2, 0.3, 0.5]
        }"#,
    );
}

// ── dgsm.rs ────────────────────────────────────────────────────────────

#[test]
fn dgsm_indices_roundtrip() {
    roundtrip_json::<DgsmIndices>(
        r#"{"vi": [1.5, 0.3, 0.0], "st_upper": [0.8, 0.15, 0.0]}"#,
    );
}

#[test]
fn fd_kind_forward_roundtrip() {
    roundtrip_eq::<FdKind>(r#""Forward""#);
}

#[test]
fn fd_kind_central_roundtrip() {
    roundtrip_eq::<FdKind>(r#""Central""#);
}

// ── qosa.rs ────────────────────────────────────────────────────────────

#[test]
fn qosa_indices_roundtrip() {
    roundtrip_json::<QosaIndices>(
        r#"{
            "s": [0.55, 0.30, 0.05],
            "alpha": 0.95,
            "global_quantile": 12.3,
            "global_cte": 15.8
        }"#,
    );
}

// ── rbd_fast.rs ────────────────────────────────────────────────────────

#[test]
fn rbd_fast_indices_roundtrip() {
    roundtrip_json::<RbdFastIndices>(r#"{"s": [0.6, 0.2, 0.05]}"#);
}

// ── janon.rs ───────────────────────────────────────────────────────────

#[test]
fn janon_indices_roundtrip() {
    roundtrip_json::<JanonIndices>(
        r#"{
            "first_order": [0.31, 0.44, 0.0],
            "total_variance": 13.84,
            "second_order": null
        }"#,
    );
}

#[test]
fn janon_indices_with_second_order_roundtrip() {
    roundtrip_json::<JanonIndices>(
        r#"{
            "first_order": [0.31, 0.44, 0.0],
            "total_variance": 13.84,
            "second_order": [[0.05, 0.07], [0.03]]
        }"#,
    );
}

// ── jansen.rs ──────────────────────────────────────────────────────────

#[test]
fn jansen_indices_roundtrip() {
    roundtrip_json::<JansenIndices>(
        r#"{
            "first_order": [0.29, 0.42, 0.01],
            "total_variance": 14.0,
            "second_order": null
        }"#,
    );
}

// ── owen.rs ────────────────────────────────────────────────────────────

#[test]
fn owen_indices_roundtrip() {
    roundtrip_json::<OwenIndices>(
        r#"{
            "first_order": [0.30, 0.43, 0.0],
            "total_variance": 13.9,
            "second_order": null
        }"#,
    );
}

// ── given_data_sobol.rs ────────────────────────────────────────────────

#[test]
fn given_data_sobol_indices_roundtrip() {
    roundtrip_json::<GivenDataSobolIndices>(r#"{"s1": [0.28, 0.41, 0.02]}"#);
}

// ── anova.rs ───────────────────────────────────────────────────────────

#[test]
fn anova_two_way_result_roundtrip() {
    roundtrip_json::<AnovaTwoWayResult>(
        r#"{
            "v_row": 10.5,
            "v_column": 5.2,
            "v_interaction": 1.1,
            "v_residual": 0.0,
            "ms_row": 5.25,
            "ms_column": 2.6,
            "ms_interaction": 0.55,
            "ms_residual": 0.0,
            "f_row": 9.55,
            "f_column": 4.73,
            "f_interaction": null,
            "p_row": 0.01,
            "p_column": 0.05,
            "p_interaction": null,
            "variance_fraction_ci_low": null,
            "variance_fraction_ci_high": null,
            "bootstrap_iterations": null,
            "bootstrap_alpha": null
        }"#,
    );
}

#[test]
fn anova_three_way_result_roundtrip() {
    roundtrip_json::<AnovaThreeWayResult>(
        r#"{
            "v_data": 8.0,
            "v_brittleness": 3.0,
            "v_inference": 2.5,
            "v_data_brittleness": 0.5,
            "v_data_inference": 0.3,
            "v_brittleness_inference": 0.2,
            "v_data_brittleness_inference": 0.1,
            "v_residual": 0.0,
            "ms_data": 4.0,
            "ms_brittleness": 1.5,
            "ms_inference": 1.25,
            "ms_data_brittleness": 0.25,
            "ms_data_inference": 0.15,
            "ms_brittleness_inference": 0.1,
            "ms_data_brittleness_inference": 0.05,
            "ms_residual": 0.0,
            "f_data": 32.0,
            "f_brittleness": 12.0,
            "f_inference": 10.0,
            "f_data_brittleness": null,
            "f_data_inference": null,
            "f_brittleness_inference": null,
            "f_data_brittleness_inference": null,
            "p_data": 0.001,
            "p_brittleness": 0.01,
            "p_inference": 0.02,
            "p_data_brittleness": null,
            "p_data_inference": null,
            "p_brittleness_inference": null,
            "p_data_brittleness_inference": null,
            "variance_fraction_ci_low": null,
            "variance_fraction_ci_high": null,
            "bootstrap_iterations": null,
            "bootstrap_alpha": null
        }"#,
    );
}

// ── g_theory.rs ────────────────────────────────────────────────────────

#[test]
fn g_theory_design_crossed_roundtrip() {
    roundtrip_eq::<GTheoryDesign>(r#""Crossed""#);
}

#[test]
fn g_theory_design_nested_roundtrip() {
    roundtrip_eq::<GTheoryDesign>(r#""Nested""#);
}

#[test]
fn g_theory_design_mixed_roundtrip() {
    roundtrip_eq::<GTheoryDesign>(r#""Mixed""#);
}

#[test]
fn g_theory_result_roundtrip() {
    roundtrip_json::<GTheoryResult>(
        r#"{
            "sigma_p": 1.5,
            "sigma_i": 0.3,
            "sigma_r": 0.2,
            "sigma_pi": 0.1,
            "sigma_pr": 0.05,
            "sigma_ir": 0.04,
            "sigma_pir": 0.02,
            "g_coefficient": 0.88,
            "phi_coefficient": 0.82,
            "variance_component_ci_low": null,
            "variance_component_ci_high": null,
            "g_coefficient_ci_low": null,
            "g_coefficient_ci_high": null,
            "phi_coefficient_ci_low": null,
            "phi_coefficient_ci_high": null,
            "bootstrap_iterations": null,
            "bootstrap_alpha": null,
            "bootstrap_skipped": null
        }"#,
    );
}

#[test]
fn d_study_point_roundtrip() {
    roundtrip_eq::<DStudyPoint>(
        r#"{
            "n_items": 10,
            "n_raters": 5,
            "g_coefficient": 0.91,
            "phi_coefficient": 0.87
        }"#,
    );
}

// ── discrepancy.rs ─────────────────────────────────────────────────────

#[test]
fn discrepancy_result_roundtrip() {
    roundtrip_json::<DiscrepancyResult>(
        r#"{
            "centered": 0.123,
            "wrap_around": 0.098,
            "modified": 0.145,
            "l2_star": 0.201
        }"#,
    );
}

// ── fractional_factorial.rs ────────────────────────────────────────────

#[test]
fn fractional_factorial_effects_roundtrip() {
    roundtrip_json::<FractionalFactorialEffects>(
        r#"{
            "dim": 3,
            "n_runs": 4,
            "main_effects": [1.2, -0.5, 0.3],
            "main_effects_abs": [1.2, 0.5, 0.3]
        }"#,
    );
}

// ── bootstrap_given_data.rs ────────────────────────────────────────────

#[test]
fn bootstrap_ci_roundtrip() {
    roundtrip_eq::<BootstrapCi>(
        r#"{
            "ci_low": [0.20, 0.35, 0.01],
            "ci_high": [0.42, 0.55, 0.09],
            "n_resamples": 1000,
            "alpha": 0.05,
            "n_skipped": 3
        }"#,
    );
}

// ── hdmr.rs (requires surrogate feature) ──────────────────────────────

#[cfg(feature = "surrogate")]
#[test]
fn hdmr_result_roundtrip() {
    use salib_estimators::hdmr::HdmrResult;
    roundtrip_json::<HdmrResult>(
        r#"{
            "dim": 2,
            "total_variance": 0.34,
            "order_variance": [0.9, 0.1],
            "first_order": [0.6, 0.3],
            "second_order": [[0.1]],
            "total_order": [0.7, 0.4],
            "pce": {
                "coefficients": [1.0, 0.5, -0.3],
                "multi_indices": [
                    {"indices": [0, 0]},
                    {"indices": [1, 0]},
                    {"indices": [0, 1]}
                ],
                "families": ["Legendre", "Legendre"],
                "max_degree": 1
            }
        }"#,
    );
}
