#![cfg(feature = "serde")]
#![allow(clippy::unwrap_used)]

use salib_samplers::*;

#[test]
fn saltelli_matrix_roundtrip() {
    use salib_core::RngState;
    let sampler = SobolSampler::minimal(6);
    let mut rng = RngState::from_seed([0u8; 32]);
    let sm = build_saltelli_matrix(&sampler, 64, false, &mut rng).unwrap();
    let json = serde_json::to_string(&sm).unwrap();
    let back: SaltelliMatrix = serde_json::from_str(&json).unwrap();
    assert_eq!(sm.n, back.n);
    assert_eq!(sm.dim, back.dim);
    assert_eq!(sm.a, back.a);
    assert_eq!(sm.b, back.b);
}

#[test]
fn lhs_sampler_roundtrip() {
    let s = LhsSampler::classic(3);
    let json = serde_json::to_string(&s).unwrap();
    let back: LhsSampler = serde_json::from_str(&json).unwrap();
    assert_eq!(s, back);
}

#[test]
fn sobol_sampler_roundtrip() {
    let s = SobolSampler::minimal(4);
    let json = serde_json::to_string(&s).unwrap();
    let back: SobolSampler = serde_json::from_str(&json).unwrap();
    assert_eq!(s, back);
}
