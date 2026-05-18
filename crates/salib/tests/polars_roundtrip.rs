#![cfg(all(feature = "polars", feature = "estimators", feature = "serde"))]
#![allow(clippy::unwrap_used)]

#[allow(unused_imports)]
use polars::prelude::*;
use salib::convert::polars::sobol_to_df;
use salib::estimators::SobolIndices;

#[test]
fn sobol_to_polars_df() {
    let idx: SobolIndices = serde_json::from_str(
        r#"{
        "n": 1024,
        "dim": 3,
        "total_variance": 13.84,
        "first_order": [0.31, 0.44, 0.00],
        "total_order": [0.56, 0.44, 0.24],
        "second_order": null
    }"#,
    )
    .unwrap();
    let df = sobol_to_df(&idx, Some(&["x1", "x2", "x3"]));
    assert_eq!(df.height(), 3);
    assert_eq!(df.width(), 3); // factor, S1, ST

    let s1 = df.column("S1").unwrap().f64().unwrap();
    assert_eq!(s1.get(0).unwrap(), 0.31);
    assert_eq!(s1.get(1).unwrap(), 0.44);
    assert_eq!(s1.get(2).unwrap(), 0.00);
}
