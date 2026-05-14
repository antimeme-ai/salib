#![cfg(feature = "serde")]
#![allow(clippy::unwrap_used)]

use salib_shapley::ShapleyIndices;

#[test]
fn shapley_indices_roundtrip() {
    let json = r#"{"sh":[0.3,0.5,0.2],"var_y":10.0,"n_perm":1000}"#;
    let s: ShapleyIndices = serde_json::from_str(json).unwrap();
    let re_json = serde_json::to_string(&s).unwrap();
    let back: ShapleyIndices = serde_json::from_str(&re_json).unwrap();
    assert_eq!(format!("{s:?}"), format!("{back:?}"));
}
