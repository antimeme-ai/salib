#![cfg(feature = "serde")]
#![allow(clippy::unwrap_used)]

use salib_validation::*;

#[test]
fn sobol_indices_analytic_roundtrip() {
    let s = SobolIndicesAnalytic::new(10.0, vec![0.3, 0.7], vec![0.5, 0.9], None);
    let json = serde_json::to_string(&s).unwrap();
    let back: SobolIndicesAnalytic = serde_json::from_str(&json).unwrap();
    assert_eq!(s, back);
}

#[test]
fn morris_effects_analytic_roundtrip() {
    let m = MorrisEffectsAnalytic::new(vec![1.0, 2.0], vec![1.0, 2.0], vec![0.5, 0.5]);
    let json = serde_json::to_string(&m).unwrap();
    let back: MorrisEffectsAnalytic = serde_json::from_str(&json).unwrap();
    assert_eq!(m, back);
}
