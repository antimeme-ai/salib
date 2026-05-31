//! TCK: Saltelli2010 estimation from pre-computed outputs.
//!
//! Feature: saltelli2010_from_outputs.feature

#![allow(clippy::unwrap_used, clippy::float_cmp)]

use salib_core::RngState;
use salib_estimators::{
    estimate_saltelli2010, estimate_saltelli2010_from_outputs,
    estimate_saltelli2010_from_outputs_with_bootstrap, estimate_saltelli2010_with_bootstrap,
};
use salib_samplers::{build_saltelli_matrix, LhsSampler};

fn fresh_rng() -> RngState {
    RngState::from_seed([0x42; 32])
}

fn evaluate_rows(matrix: &ndarray::Array2<f64>, model: &dyn Fn(&[f64]) -> f64) -> Vec<f64> {
    matrix
        .rows()
        .into_iter()
        .map(|row| model(row.as_slice().unwrap()))
        .collect()
}

/// Scenario: from_outputs matches model-evaluated estimation
#[test]
fn from_outputs_matches_model_evaluated_estimation() {
    let sampler = LhsSampler::classic(4); // d = 2
    let mut rng = fresh_rng();
    let matrix = build_saltelli_matrix(&sampler, 256, false, &mut rng).unwrap();
    let model = |x: &[f64]| x[0] + 2.0 * x[1];

    let expected = estimate_saltelli2010(&matrix, &model);

    let fa = evaluate_rows(&matrix.a, &model);
    let fb = evaluate_rows(&matrix.b, &model);
    let fab: Vec<Vec<f64>> = matrix.a_b.iter().map(|m| evaluate_rows(m, &model)).collect();

    let actual = estimate_saltelli2010_from_outputs(&fa, &fb, &fab);

    assert_eq!(actual.n, expected.n);
    assert_eq!(actual.dim, expected.dim);
    assert_eq!(actual.total_variance, expected.total_variance);
    assert_eq!(actual.first_order, expected.first_order);
    assert_eq!(actual.total_order, expected.total_order);
}

/// Scenario: bootstrap from_outputs matches model-evaluated bootstrap
#[test]
fn bootstrap_from_outputs_matches_model_evaluated_bootstrap() {
    let sampler = LhsSampler::classic(4); // d = 2
    let mut rng = fresh_rng();
    let matrix = build_saltelli_matrix(&sampler, 256, false, &mut rng).unwrap();
    let model = |x: &[f64]| x[0] + 2.0 * x[1];

    let mut bootstrap_rng1 = RngState::from_seed([0xab; 32]);
    let expected =
        estimate_saltelli2010_with_bootstrap(&matrix, &model, 200, 0.05, &mut bootstrap_rng1);

    let fa = evaluate_rows(&matrix.a, &model);
    let fb = evaluate_rows(&matrix.b, &model);
    let fab: Vec<Vec<f64>> = matrix.a_b.iter().map(|m| evaluate_rows(m, &model)).collect();

    let mut bootstrap_rng2 = RngState::from_seed([0xab; 32]);
    let actual =
        estimate_saltelli2010_from_outputs_with_bootstrap(&fa, &fb, &fab, 200, 0.05, &mut bootstrap_rng2);

    assert_eq!(actual.indices.n, expected.indices.n);
    assert_eq!(actual.indices.dim, expected.indices.dim);
    assert_eq!(actual.indices.total_variance, expected.indices.total_variance);
    assert_eq!(actual.indices.first_order, expected.indices.first_order);
    assert_eq!(actual.indices.total_order, expected.indices.total_order);
    assert_eq!(actual.first_order_ci, expected.first_order_ci);
    assert_eq!(actual.total_order_ci, expected.total_order_ci);
}

/// Scenario: from_outputs handles constant output gracefully
#[test]
fn from_outputs_constant_output() {
    let n = 8;
    let d = 3;
    let fa = vec![0.5; n];
    let fb = vec![0.5; n];
    let fab = vec![vec![0.5; n]; d];

    let result = estimate_saltelli2010_from_outputs(&fa, &fb, &fab);

    assert!(
        result.total_variance.abs() < 1e-15,
        "variance should be ~0 for constant output"
    );
    for i in 0..d {
        assert_eq!(result.first_order[i], 0.0);
        assert_eq!(result.total_order[i], 0.0);
    }
}
