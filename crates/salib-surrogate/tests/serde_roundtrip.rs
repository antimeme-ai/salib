#![cfg(feature = "serde")]
#![allow(clippy::unwrap_used)]

use salib_surrogate::*;

#[test]
fn polynomial_chaos_roundtrip() {
    let json = r#"{"coefficients":[1.0,0.5,0.25],"multi_indices":[{"indices":[0,0]},{"indices":[1,0]},{"indices":[0,1]}],"families":["Legendre","Legendre"],"max_degree":2}"#;
    let pc: PolynomialChaos = serde_json::from_str(json).unwrap();
    let re_json = serde_json::to_string(&pc).unwrap();
    let back: PolynomialChaos = serde_json::from_str(&re_json).unwrap();
    assert_eq!(format!("{pc:?}"), format!("{back:?}"));
}

#[test]
fn sobol_from_pce_roundtrip() {
    let json = r#"{"first_order":[0.3,0.7],"total_order":[0.5,0.9],"total_variance":10.0}"#;
    let s: SobolFromPce = serde_json::from_str(json).unwrap();
    let re_json = serde_json::to_string(&s).unwrap();
    let back: SobolFromPce = serde_json::from_str(&re_json).unwrap();
    assert_eq!(format!("{s:?}"), format!("{back:?}"));
}

#[test]
fn sparse_solver_roundtrip() {
    let json = serde_json::to_string(&SparseSolver::Lars).unwrap();
    let back: SparseSolver = serde_json::from_str(&json).unwrap();
    assert_eq!(SparseSolver::Lars, back);
}

#[test]
fn polynomial_family_roundtrip() {
    let json = r#"{"Jacobi":{"alpha":0.5,"beta":1.0}}"#;
    let f: PolynomialFamily = serde_json::from_str(json).unwrap();
    let re_json = serde_json::to_string(&f).unwrap();
    let back: PolynomialFamily = serde_json::from_str(&re_json).unwrap();
    assert_eq!(f, back);
}
