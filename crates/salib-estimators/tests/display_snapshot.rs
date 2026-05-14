#![cfg(feature = "serde")]
#![allow(clippy::unwrap_used)]

use salib_estimators::*;

#[test]
fn sobol_indices_display() {
    let json = r#"{"n":8192,"dim":3,"total_variance":13.8446,"first_order":[0.3139,0.4424,0.0],"total_order":[0.5576,0.4424,0.2437],"second_order":null}"#;
    let idx: SobolIndices = serde_json::from_str(json).unwrap();
    let output = format!("{idx}");
    assert!(output.contains("Sobol' indices (N=8192, d=3)"));
    assert!(output.contains("Var[Y] = 13.8446"));
    assert!(output.contains("0.3139"));
    assert!(output.contains("0.5576"));
}

#[test]
fn sobol_indices_display_with_names() {
    let json = r#"{"n":8192,"dim":3,"total_variance":13.8446,"first_order":[0.3139,0.4424,0.0],"total_order":[0.5576,0.4424,0.2437],"second_order":null}"#;
    let idx: SobolIndices = serde_json::from_str(json).unwrap();
    let output = format!("{}", idx.display_with_names(&["x1", "x2", "x3"]));
    assert!(output.contains("x1"));
    assert!(output.contains("x2"));
    assert!(output.contains("x3"));
}

#[test]
fn morris_effects_display() {
    let json = r#"{"r":10,"d":2,"mu":[1.0,-0.5],"mu_star":[1.0,0.5],"sigma":[0.3,0.2],"grouped_mu":null,"grouped_mu_star":null,"grouped_sigma":null,"group_names":null}"#;
    let m: MorrisEffects = serde_json::from_str(json).unwrap();
    let output = format!("{m}");
    assert!(output.contains("Morris effects (r=10, d=2)"));
    assert!(output.contains("1.0000"));
}

#[test]
fn regression_indices_display() {
    let json = r#"{"src":[0.8,0.2],"srrc":[0.7,0.3],"pcc":[0.9,0.1],"prcc":[0.85,0.15],"r2_linear":0.95,"r2_rank":0.93}"#;
    let r: RegressionIndices = serde_json::from_str(json).unwrap();
    let output = format!("{r}");
    assert!(output.contains("R\u{00b2}(linear) = 0.9500"));
    assert!(output.contains("SRC"));
    assert!(output.contains("PRCC"));
}

#[test]
fn bootstrap_ci_display() {
    let json = r#"{"ci_low":[0.1,0.2],"ci_high":[0.5,0.8],"n_resamples":1000,"alpha":0.05,"n_skipped":3}"#;
    let b: BootstrapCi = serde_json::from_str(json).unwrap();
    let output = format!("{b}");
    assert!(output.contains("B=1000"));
    assert!(output.contains("skipped=3"));
}
