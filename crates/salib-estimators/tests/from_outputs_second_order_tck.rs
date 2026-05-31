//! Tests for estimate_saltelli2010_from_outputs_with_second_order.
//!
//! Validates that the from-outputs S2 computation matches the model-
//! evaluated estimate_saltelli2010 with second_order=true.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use salib_core::RngState;
use salib_estimators::{
    estimate_saltelli2010, estimate_saltelli2010_from_outputs_with_second_order,
};
use salib_samplers::{build_saltelli_matrix, LhsSampler};

fn fresh_rng() -> RngState {
    RngState::from_seed([0x42; 32])
}

/// S2 from cached outputs matches S2 from model evaluation.
#[test]
fn from_outputs_s2_matches_model_evaluated() {
    let s = LhsSampler::classic(6); // d = 3
    let mut rng = fresh_rng();
    // second_order = true to get B_A matrices
    let m = build_saltelli_matrix(&s, 512, true, &mut rng).unwrap();

    // Model with interactions: Y = X0 + X1 + X0*X1 + 0.5*X2
    let model = |x: &[f64]| x[0] + x[1] + x[0] * x[1] + 0.5 * x[2];

    // Model-evaluated (the reference)
    let reference = estimate_saltelli2010(&m, model);

    // Compute model outputs manually
    let n = m.n;
    let d = m.dim;
    let fa: Vec<f64> = (0..n).map(|i| model(m.a.row(i).as_slice().unwrap())).collect();
    let fb: Vec<f64> = (0..n).map(|i| model(m.b.row(i).as_slice().unwrap())).collect();
    let fab: Vec<Vec<f64>> = (0..d)
        .map(|j| {
            (0..n)
                .map(|i| model(m.a_b[j].row(i).as_slice().unwrap()))
                .collect()
        })
        .collect();
    let b_a = m.b_a.as_ref().expect("second_order=true should produce b_a");
    let fba: Vec<Vec<f64>> = (0..d)
        .map(|j| {
            (0..n)
                .map(|i| model(b_a[j].row(i).as_slice().unwrap()))
                .collect()
        })
        .collect();

    let from_outputs =
        estimate_saltelli2010_from_outputs_with_second_order(&fa, &fb, &fab, &fba);

    // S1 and ST should match exactly
    for i in 0..d {
        assert!(
            (from_outputs.first_order[i] - reference.first_order[i]).abs() < 1e-10,
            "S1[{i}] mismatch: {} vs {}",
            from_outputs.first_order[i],
            reference.first_order[i]
        );
        assert!(
            (from_outputs.total_order[i] - reference.total_order[i]).abs() < 1e-10,
            "ST[{i}] mismatch: {} vs {}",
            from_outputs.total_order[i],
            reference.total_order[i]
        );
    }

    // S2 should match
    let ref_s2 = reference.second_order.expect("reference should have S2");
    let out_s2 = from_outputs
        .second_order
        .expect("from_outputs should have S2");

    assert_eq!(ref_s2.len(), out_s2.len());
    for i in 0..ref_s2.len() {
        assert_eq!(ref_s2[i].len(), out_s2[i].len());
        for j in 0..ref_s2[i].len() {
            assert!(
                (ref_s2[i][j] - out_s2[i][j]).abs() < 1e-10,
                "S2[{i}][{j}] mismatch: {} vs {}",
                ref_s2[i][j],
                out_s2[i][j]
            );
        }
    }
}

/// S2 shape is correct: d rows, upper triangle.
#[test]
fn from_outputs_s2_shape_is_upper_triangle() {
    let s = LhsSampler::classic(8); // d = 4
    let mut rng = fresh_rng();
    let m = build_saltelli_matrix(&s, 128, true, &mut rng).unwrap();
    let model = |x: &[f64]| x[0] + x[1] * x[2] + x[3];
    let n = m.n;
    let d = m.dim;
    let fa: Vec<f64> = (0..n).map(|i| model(m.a.row(i).as_slice().unwrap())).collect();
    let fb: Vec<f64> = (0..n).map(|i| model(m.b.row(i).as_slice().unwrap())).collect();
    let fab: Vec<Vec<f64>> = (0..d)
        .map(|j| (0..n).map(|i| model(m.a_b[j].row(i).as_slice().unwrap())).collect())
        .collect();
    let b_a = m.b_a.as_ref().unwrap();
    let fba: Vec<Vec<f64>> = (0..d)
        .map(|j| (0..n).map(|i| model(b_a[j].row(i).as_slice().unwrap())).collect())
        .collect();

    let result = estimate_saltelli2010_from_outputs_with_second_order(&fa, &fb, &fab, &fba);
    let s2 = result.second_order.expect("should have S2");

    // Upper triangle: s2[i] has d - i - 1 elements
    assert_eq!(s2.len(), 4);
    assert_eq!(s2[0].len(), 3); // S2_{0,1}, S2_{0,2}, S2_{0,3}
    assert_eq!(s2[1].len(), 2); // S2_{1,2}, S2_{1,3}
    assert_eq!(s2[2].len(), 1); // S2_{2,3}
    assert_eq!(s2[3].len(), 0); // no pairs starting at last factor
}

/// For a model with known X0*X1 interaction, S2_{0,1} should be
/// substantially positive while S2_{0,2} and S2_{1,2} should be near zero.
#[test]
fn from_outputs_s2_detects_known_interaction() {
    let s = LhsSampler::classic(6); // d = 3
    let mut rng = fresh_rng();
    let m = build_saltelli_matrix(&s, 4096, true, &mut rng).unwrap();
    // Y = X0*X1 + 0.1*X2 — strong interaction between factors 0 and 1
    let model = |x: &[f64]| x[0] * x[1] + 0.1 * x[2];
    let n = m.n;
    let d = m.dim;
    let fa: Vec<f64> = (0..n).map(|i| model(m.a.row(i).as_slice().unwrap())).collect();
    let fb: Vec<f64> = (0..n).map(|i| model(m.b.row(i).as_slice().unwrap())).collect();
    let fab: Vec<Vec<f64>> = (0..d)
        .map(|j| (0..n).map(|i| model(m.a_b[j].row(i).as_slice().unwrap())).collect())
        .collect();
    let b_a = m.b_a.as_ref().unwrap();
    let fba: Vec<Vec<f64>> = (0..d)
        .map(|j| (0..n).map(|i| model(b_a[j].row(i).as_slice().unwrap())).collect())
        .collect();

    let result = estimate_saltelli2010_from_outputs_with_second_order(&fa, &fb, &fab, &fba);
    let s2 = result.second_order.expect("should have S2");

    // S2_{0,1} should be positive and substantial (interaction term)
    let s2_01 = s2[0][0]; // S2_{0,1}
    assert!(
        s2_01 > 0.1,
        "S2_{{0,1}} = {s2_01:.4} should be > 0.1 (X0*X1 interaction)"
    );

    // S2_{0,2} and S2_{1,2} should be near zero (no interaction)
    let s2_02 = s2[0][1]; // S2_{0,2}
    let s2_12 = s2[1][0]; // S2_{1,2}
    assert!(
        s2_02.abs() < 0.1,
        "S2_{{0,2}} = {s2_02:.4} should be near 0"
    );
    assert!(
        s2_12.abs() < 0.1,
        "S2_{{1,2}} = {s2_12:.4} should be near 0"
    );
}
