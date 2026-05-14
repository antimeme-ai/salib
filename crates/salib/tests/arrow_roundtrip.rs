#![cfg(all(feature = "arrow", feature = "estimators", feature = "serde"))]
#![allow(clippy::unwrap_used)]

use arrow::array::Float64Array;
use salib::convert::arrow::sobol_to_batch;
use salib::estimators::SobolIndices;

#[test]
fn sobol_to_arrow_and_back() {
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
    let batch = sobol_to_batch(&idx, Some(&["x1", "x2", "x3"]));
    assert_eq!(batch.num_rows(), 3);
    assert_eq!(batch.num_columns(), 3); // factor, S1, ST

    let s1 = batch
        .column_by_name("S1")
        .unwrap()
        .as_any()
        .downcast_ref::<Float64Array>()
        .unwrap();
    assert_eq!(s1.value(0), 0.31);
    assert_eq!(s1.value(1), 0.44);
    assert_eq!(s1.value(2), 0.00);
}
