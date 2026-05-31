//! D-study budget optimizer tests.
//!
//! Validates d_study_surface() and find_minimum_design() against the
//! Gate 1 synthetic dataset's variance components.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use ndarray::Array3;
use salib_estimators::{
    d_study_surface, estimate_g_theory_pir, find_minimum_design, project_g_theory_d_study,
    GTheoryDesign,
};

/// Gate 1 synthetic dataset: 4p x 3i x 2r
fn gate1_grid() -> Array3<f64> {
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

#[test]
fn d_study_surface_has_correct_point_count() {
    let result = estimate_g_theory_pir(gate1_grid().view(), GTheoryDesign::Crossed).unwrap();
    let items = [2, 4, 8, 16];
    let raters = [1, 2, 3, 5];
    let surface = d_study_surface(&result, &items, &raters).unwrap();
    assert_eq!(surface.points.len(), 16);
}

#[test]
fn d_study_surface_phi_increases_with_items_at_fixed_raters() {
    let result = estimate_g_theory_pir(gate1_grid().view(), GTheoryDesign::Crossed).unwrap();
    let items = [2, 4, 8, 16];
    let raters = [1, 2, 3, 5];
    let surface = d_study_surface(&result, &items, &raters).unwrap();

    // For each fixed rater count, Phi should increase with items
    for (r_idx, &_nr) in raters.iter().enumerate() {
        for i in 1..items.len() {
            let prev = &surface.points[(i - 1) * raters.len() + r_idx];
            let curr = &surface.points[i * raters.len() + r_idx];
            assert!(
                curr.phi_coefficient >= prev.phi_coefficient,
                "Phi should increase with items: ni={} Phi={:.4} vs ni={} Phi={:.4}",
                prev.n_items,
                prev.phi_coefficient,
                curr.n_items,
                curr.phi_coefficient
            );
        }
    }
}

#[test]
fn d_study_surface_phi_increases_with_raters_at_fixed_items() {
    let result = estimate_g_theory_pir(gate1_grid().view(), GTheoryDesign::Crossed).unwrap();
    let items = [2, 4, 8, 16];
    let raters = [1, 2, 3, 5];
    let surface = d_study_surface(&result, &items, &raters).unwrap();

    // For each fixed item count, Phi should increase with raters
    for (i_idx, &_ni) in items.iter().enumerate() {
        for r in 1..raters.len() {
            let prev = &surface.points[i_idx * raters.len() + (r - 1)];
            let curr = &surface.points[i_idx * raters.len() + r];
            assert!(
                curr.phi_coefficient >= prev.phi_coefficient,
                "Phi should increase with raters: nr={} Phi={:.4} vs nr={} Phi={:.4}",
                prev.n_raters,
                prev.phi_coefficient,
                curr.n_raters,
                curr.phi_coefficient
            );
        }
    }
}

#[test]
fn d_study_surface_matches_individual_projections() {
    let result = estimate_g_theory_pir(gate1_grid().view(), GTheoryDesign::Crossed).unwrap();
    let items = [2, 4, 8];
    let raters = [1, 3];
    let surface = d_study_surface(&result, &items, &raters).unwrap();

    for point in &surface.points {
        let direct = project_g_theory_d_study(&result, point.n_items, point.n_raters).unwrap();
        assert!(
            (point.g_coefficient - direct.g_coefficient).abs() < 1e-12,
            "Surface G({},{}) should match direct projection",
            point.n_items,
            point.n_raters
        );
        assert!(
            (point.phi_coefficient - direct.phi_coefficient).abs() < 1e-12,
            "Surface Phi({},{}) should match direct projection",
            point.n_items,
            point.n_raters
        );
    }
}

#[test]
fn find_minimum_design_finds_feasible_solution() {
    let result = estimate_g_theory_pir(gate1_grid().view(), GTheoryDesign::Crossed).unwrap();
    let target_phi = 0.80;
    let cost_fn = |ni: usize, nr: usize| (ni * nr) as f64;
    let design = find_minimum_design(&result, target_phi, 20, 10, cost_fn)
        .unwrap()
        .expect("should find a feasible design for Phi >= 0.80");

    assert!(
        design.phi_coefficient >= target_phi,
        "design Phi={:.4} should be >= {target_phi}",
        design.phi_coefficient
    );
}

#[test]
fn find_minimum_design_is_cheapest() {
    let result = estimate_g_theory_pir(gate1_grid().view(), GTheoryDesign::Crossed).unwrap();
    let target_phi = 0.80;
    let cost_fn = |ni: usize, nr: usize| (ni * nr) as f64;
    let design = find_minimum_design(&result, target_phi, 20, 10, &cost_fn)
        .unwrap()
        .expect("should find a feasible design");

    let design_cost = cost_fn(design.n_items, design.n_raters);

    // Verify no cheaper feasible design exists
    for ni in 1..=20 {
        for nr in 1..=10 {
            let point = project_g_theory_d_study(&result, ni, nr).unwrap();
            if point.phi_coefficient >= target_phi {
                let c = cost_fn(ni, nr);
                assert!(
                    c >= design_cost,
                    "found cheaper feasible design: ({ni},{nr}) cost={c} vs ({},{}) cost={design_cost}",
                    design.n_items, design.n_raters
                );
            }
        }
    }
}

#[test]
fn find_minimum_design_returns_none_for_infeasible_target() {
    let result = estimate_g_theory_pir(gate1_grid().view(), GTheoryDesign::Crossed).unwrap();
    let target_phi = 0.999;
    let cost_fn = |ni: usize, nr: usize| (ni * nr) as f64;
    let design = find_minimum_design(&result, target_phi, 5, 5, cost_fn).unwrap();
    assert!(
        design.is_none(),
        "should return None for infeasible Phi >= {target_phi} with max 5x5"
    );
}

#[test]
fn find_minimum_design_with_asymmetric_cost() {
    let result = estimate_g_theory_pir(gate1_grid().view(), GTheoryDesign::Crossed).unwrap();
    let target_phi = 0.80;
    // Raters are 5x more expensive than items
    let cost_fn = |ni: usize, nr: usize| ni as f64 + 5.0 * nr as f64;
    let design = find_minimum_design(&result, target_phi, 30, 10, cost_fn)
        .unwrap()
        .expect("should find a feasible design");

    assert!(design.phi_coefficient >= target_phi);
    // With expensive raters, the optimizer should favor more items over more raters
    // (This is a soft check -- depends on variance structure)
}
