# salib

Global sensitivity analysis for Rust. A port of Python's
[SALib](https://salib.readthedocs.io/) built for correctness and
reproducibility.

**Bit-deterministic**: identical `RngState` produces identical results
regardless of thread count. Parallel reductions use a tree-structured
accumulation strategy to eliminate float-associativity nondeterminism under
[rayon](https://docs.rs/rayon).

## Quickstart

```toml
# Cargo.toml
[dependencies]
salib = "0.1"
```

```rust
use std::f64::consts::PI;
use salib::*;
use salib::samplers::{SobolSampler, build_saltelli_matrix};
use salib::estimators::estimate_saltelli2010;

fn main() {
    // 1. Define the problem: 3 factors, Uniform(-pi, pi)
    let problem = ProblemBuilder::new()
        .factor("x1", Distribution::Uniform { lo: -PI, hi: PI })
        .factor("x2", Distribution::Uniform { lo: -PI, hi: PI })
        .factor("x3", Distribution::Uniform { lo: -PI, hi: PI })
        .build()
        .unwrap();

    // 2. Build a Saltelli sample matrix (N=8192 base samples, 3 factors → 6-dim sampler)
    let mut rng = RngState::from_seed([0u8; 32]);
    let sampler = SobolSampler::minimal(2 * problem.dim());
    let saltelli = build_saltelli_matrix(&sampler, 8192, false, &mut rng).unwrap();

    // 3. Estimate Sobol' indices — the estimator calls the model internally
    //    Ishigami: y = sin(x1) + 7*sin(x2)^2 + 0.1*x3^4*sin(x1)
    let indices = estimate_saltelli2010(&saltelli, |x| {
        x[0].sin() + 7.0 * x[1].sin().powi(2) + 0.1 * x[2].powi(4) * x[0].sin()
    });

    // 4. Print results
    for (i, f) in problem.factors().iter().enumerate() {
        println!("{}: S1 = {:.4}, ST = {:.4}", f.name, indices.first_order[i], indices.total_order[i]);
    }
}
```

## Crate structure

`salib` is a facade that re-exports subcrates. Use it for convenience, or
depend on individual crates for finer control.

| Crate | Contents |
|---|---|
| `salib-core` | `Problem`, `Factor`, `Distribution`, `RngState`, deterministic reductions |
| `salib-samplers` | LHS, Sobol' QMC, Halton, Saltelli (A/B/A\_Bi), Morris trajectories, FAST/eFAST/RBD-FAST designs |
| `salib-estimators` | Variance-based Sobol' (Saltelli2010, Jansen, Janon, Owen), Morris EE, FAST/eFAST, RBD-FAST, Borgonovo delta, PAWN, DGSM, regression (SRC/SRRC/PCC/PRCC), given-data Sobol', ANOVA, HDMR, G-theory, fractional factorial, discrepancy |
| `salib-surrogate` | PCE (full + sparse LARS), active subspaces |
| `salib-shapley` | Shapley effects (Song-Nelson-Staum 2016) |
| `salib-validation` | Analytic test functions (Ishigami, Sobol' G, etc.) with closed-form indices |
| `salib-cli` | CLI binary: `sample`, `run`, `analyze` subcommands |

## Feature flags

```toml
[features]
default = ["samplers", "estimators"]
samplers   = ["dep:salib-samplers"]
estimators = ["dep:salib-estimators"]
surrogate  = ["dep:salib-surrogate"]
shapley    = ["dep:salib-shapley"]
validation = ["dep:salib-validation"]
full       = ["samplers", "estimators", "surrogate", "shapley", "validation"]
```

## Requirements

- Edition 2021
- MSRV 1.87

## License

Licensed under MIT OR Apache-2.0, at your option.
