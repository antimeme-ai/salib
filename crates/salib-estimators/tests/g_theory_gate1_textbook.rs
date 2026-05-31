//! Gate 1 textbook reproduction for G-theory (p x i x r).
//!
//! Validates variance-component estimation against a hand-verified
//! synthetic 4p x 3i x 2r crossed design with analytically derived
//! expected values via three-way ANOVA EMS decomposition.
//!
//! Fixture: tests/fixtures/gate1_pir_4x3x2.json
//!
//! All expected values were computed by hand using the standard EMS
//! formulas for a fully crossed random-effects ANOVA (see fixture
//! file for complete derivation including SS, df, MS, and EMS formulas).

#![allow(clippy::unwrap_used, clippy::expect_used)]

use ndarray::Array3;
use salib_estimators::{
    estimate_g_theory_pir, project_g_theory_d_study, GTheoryDesign,
};

/// Build the 4x3x2 grid from the fixture data.
fn gate1_grid() -> Array3<f64> {
    // 4 persons x 3 items x 2 raters
    // Data from fixtures/gate1_pir_4x3x2.json
    let data: [[[f64; 2]; 3]; 4] = [
        [[4.0, 3.0], [6.0, 5.0], [5.0, 4.0]],
        [[7.0, 6.0], [8.0, 8.0], [6.0, 5.0]],
        [[3.0, 4.0], [5.0, 6.0], [4.0, 5.0]],
        [[8.0, 7.0], [9.0, 8.0], [7.0, 7.0]],
    ];

    let mut grid = Array3::<f64>::zeros((4, 3, 2));
    for (pp, person) in data.iter().enumerate() {
        for (ii, item) in person.iter().enumerate() {
            for (rr, &value) in item.iter().enumerate() {
                grid[[pp, ii, rr]] = value;
            }
        }
    }
    grid
}

/// Expected values from the fixture (hand-computed via ANOVA EMS).
struct Expected {
    sigma_p: f64,
    sigma_i: f64,
    sigma_r: f64,
    sigma_pi: f64,
    sigma_pr: f64,
    sigma_ir: f64,
    sigma_pir: f64,
    g_coefficient: f64,
    phi_coefficient: f64,
}

fn expected() -> Expected {
    Expected {
        sigma_p: 2.250_000,
        sigma_i: 0.750_000,
        sigma_r: -0.041_667,
        sigma_pi: 0.250_000,
        sigma_pr: 0.375_000,
        sigma_ir: -0.013_889,
        sigma_pir: 0.097_222,
        g_coefficient: 0.886_861,
        phi_coefficient: 0.814_070,
    }
}

const TOL: f64 = 0.0001;

fn assert_close(name: &str, actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() < tolerance,
        "Gate 1 mismatch for {name}: actual={actual:.6}, expected={expected:.6}, diff={:.6}",
        (actual - expected).abs()
    );
}

/// Gate 1: Variance components match hand-computed values.
#[test]
fn gate1_variance_components_match_hand_computed() {
    let grid = gate1_grid();
    let result = estimate_g_theory_pir(grid.view(), GTheoryDesign::Crossed).unwrap();
    let exp = expected();

    assert_close("sigma_p", result.sigma_p, exp.sigma_p, TOL);
    assert_close("sigma_i", result.sigma_i, exp.sigma_i, TOL);
    assert_close("sigma_r", result.sigma_r, exp.sigma_r, TOL);
    assert_close("sigma_pi", result.sigma_pi, exp.sigma_pi, TOL);
    assert_close("sigma_pr", result.sigma_pr, exp.sigma_pr, TOL);
    assert_close("sigma_ir", result.sigma_ir, exp.sigma_ir, TOL);
    assert_close("sigma_pir", result.sigma_pir, exp.sigma_pir, TOL);
}

/// Gate 1: G and Phi coefficients match hand-computed values.
#[test]
fn gate1_reliability_coefficients_match_hand_computed() {
    let grid = gate1_grid();
    let result = estimate_g_theory_pir(grid.view(), GTheoryDesign::Crossed).unwrap();
    let exp = expected();

    assert_close("G", result.g_coefficient, exp.g_coefficient, TOL);
    assert_close("Phi", result.phi_coefficient, exp.phi_coefficient, TOL);
}

/// Gate 1: D-study projection at (6, 4) matches hand-computed values.
#[test]
fn gate1_d_study_projection_matches_hand_computed() {
    let grid = gate1_grid();
    let result = estimate_g_theory_pir(grid.view(), GTheoryDesign::Crossed).unwrap();

    let projected = project_g_theory_d_study(&result, 6, 4).unwrap();

    // Expected D-study values from fixture
    let exp_g = 0.941_632;
    let exp_phi = 0.898_752;

    assert_close("D-study G(6,4)", projected.g_coefficient, exp_g, TOL);
    assert_close("D-study Phi(6,4)", projected.phi_coefficient, exp_phi, TOL);

    // D-study with more items/raters should yield higher reliability
    assert!(
        projected.g_coefficient > result.g_coefficient,
        "D-study G({},{}) = {:.4} should exceed G-study G = {:.4}",
        projected.n_items, projected.n_raters,
        projected.g_coefficient, result.g_coefficient
    );
    assert!(
        projected.phi_coefficient > result.phi_coefficient,
        "D-study Phi({},{}) = {:.4} should exceed G-study Phi = {:.4}",
        projected.n_items, projected.n_raters,
        projected.phi_coefficient, result.phi_coefficient
    );
}

/// Structural property: sigma_p is the dominant variance component
/// (persons should differ more than items or raters in this dataset).
#[test]
fn gate1_sigma_p_is_dominant() {
    let grid = gate1_grid();
    let result = estimate_g_theory_pir(grid.view(), GTheoryDesign::Crossed).unwrap();

    assert!(
        result.sigma_p > result.sigma_i,
        "sigma_p={:.4} should exceed sigma_i={:.4}",
        result.sigma_p, result.sigma_i
    );
    assert!(
        result.sigma_p > result.sigma_pi,
        "sigma_p={:.4} should exceed sigma_pi={:.4}",
        result.sigma_p, result.sigma_pi
    );
}

/// Structural property: G >= Phi always (relative decisions are at
/// least as reliable as absolute decisions).
#[test]
fn gate1_g_exceeds_phi() {
    let grid = gate1_grid();
    let result = estimate_g_theory_pir(grid.view(), GTheoryDesign::Crossed).unwrap();

    assert!(
        result.g_coefficient >= result.phi_coefficient,
        "G={:.4} should be >= Phi={:.4}",
        result.g_coefficient, result.phi_coefficient
    );
}
