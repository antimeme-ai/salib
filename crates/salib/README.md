# salib

[![crates.io](https://img.shields.io/crates/v/salib.svg)](https://crates.io/crates/salib)
[![docs.rs](https://img.shields.io/docsrs/salib)](https://docs.rs/salib)
[![CI](https://github.com/antimeme-ai/salib/actions/workflows/ci.yml/badge.svg)](https://github.com/antimeme-ai/salib/actions)
[![license](https://img.shields.io/crates/l/salib.svg)](https://github.com/antimeme-ai/salib)

Global sensitivity analysis for Rust, implemented from the primary
literature. Bit-deterministic by construction.

## Quickstart

```toml
[dependencies]
salib = "0.1"
```

```rust
use std::f64::consts::PI;
use salib::*;
use salib::samplers::{SobolSampler, build_saltelli_matrix};
use salib::estimators::estimate_saltelli2010;

fn main() {
    let problem = ProblemBuilder::new()
        .factor("x1", Distribution::Uniform { lo: -PI, hi: PI })
        .factor("x2", Distribution::Uniform { lo: -PI, hi: PI })
        .factor("x3", Distribution::Uniform { lo: -PI, hi: PI })
        .build()
        .unwrap();

    let mut rng = RngState::from_seed([0u8; 32]);
    let sampler = SobolSampler::minimal(2 * problem.dim());
    let saltelli = build_saltelli_matrix(&sampler, 8192, false, &mut rng).unwrap();

    // Ishigami function: y = sin(x1) + 7*sin(x2)^2 + 0.1*x3^4*sin(x1)
    let indices = estimate_saltelli2010(&saltelli, |x| {
        x[0].sin() + 7.0 * x[1].sin().powi(2) + 0.1 * x[2].powi(4) * x[0].sin()
    });

    println!("{indices}");
}
```

## Methods

| Method | Function | Reference |
|---|---|---|
| **Variance-based (Sobol')** | | |
| Saltelli 2010 | `estimate_saltelli2010` | Saltelli et al. (2010) *Comp. Phys. Comm.* 181(2) |
| Jansen | `estimate_jansen` | Jansen (1999) *Comp. Phys. Comm.* 117(1-2) |
| Janon | `estimate_janon` | Janon et al. (2014) *Math. Comp. Sim.* 107 |
| Owen | `estimate_owen` | Owen (2013) *ACM Trans. Model. Comp. Sim.* 23(1) |
| Given-data Sobol' | `estimate_given_data_sobol` | Plischke et al. (2013) *Eur. J. Oper. Res.* 226(3) |
| **Elementary effects** | | |
| Morris | `estimate_morris_effects` | Morris (1991) *Technometrics* 33(2) |
| Grouped Morris | `estimate_grouped_morris_effects` | Campolongo et al. (2007) *Env. Mod. Soft.* 22(10) |
| **Frequency-based** | | |
| FAST / eFAST | `estimate_fast` | Cukier et al. (1973); Saltelli et al. (1999) |
| RBD-FAST | `estimate_rbd_fast` | Tarantola et al. (2006) *Rel. Eng. Sys. Safety* 91(6) |
| **Distribution-based** | | |
| Borgonovo delta | `estimate_borgonovo_delta` | Borgonovo (2007) *Rel. Eng. Sys. Safety* 92(6) |
| PAWN | `estimate_pawn` | Pianosi et al. (2015) *Env. Mod. Soft.* 67 |
| QOSA | `estimate_qosa` | Fort et al. (2016) *Stat. Comp.* 26(1-2) |
| **Derivative-based** | | |
| DGSM | `estimate_dgsm` | Sobol' & Kucherenko (2009) *Math. Comp. Sim.* 79(10) |
| **Regression** | | |
| SRC / SRRC / PCC / PRCC | `estimate_regression_indices` | Saltelli & Marivoet (1990) *Comp. Stat. Data Anal.* 9(1) |
| **Surrogate** | | |
| PCE (full OLS) | `fit_full_pce` | Xiu & Karniadakis (2002) *SIAM J. Sci. Comp.* 24(2) |
| PCE (sparse LARS/OMP) | `fit_sparse_pce` | Blatman & Sudret (2011) *J. Comp. Phys.* 230(6) |
| HDMR | `estimate_hdmr` | Li et al. (2002) *J. Phys. Chem. A* 106(37) |
| Active subspaces | `compute_active_subspace` | Constantine (2015) *Active Subspaces*, SIAM |
| **Game-theoretic** | | |
| Shapley effects | `estimate_shapley` | Song et al. (2016) *SIAM/ASA J. Unc. Quant.* 4(1) |
| **Experimental design** | | |
| ANOVA (two- and three-way) | `estimate_anova_two_way` | Fisher (1925) *Statistical Methods* |
| G-theory (D-study) | `estimate_g_theory_pir` | Brennan (2001) *Generalizability Theory*, Springer |
| Discrepancy (L2-star) | `compute_discrepancy` | Hickernell (1998) in *Monte Carlo and Quasi-Monte Carlo Methods* |
| Fractional factorial | `estimate_fractional_factorial` | Box et al. (1978) *Statistics for Experimenters* |

## Crate structure

`salib` is a facade that re-exports subcrates. Use it for convenience,
or depend on individual crates for finer control.

| Crate | Role |
|---|---|
| [`salib-core`](https://crates.io/crates/salib-core) | `Problem`, `Factor`, `Distribution`, `RngState`, deterministic reductions |
| [`salib-samplers`](https://crates.io/crates/salib-samplers) | LHS, Sobol' QMC, Saltelli, Morris trajectories, FAST/eFAST, Iman-Conover |
| [`salib-estimators`](https://crates.io/crates/salib-estimators) | All estimators listed above |
| [`salib-surrogate`](https://crates.io/crates/salib-surrogate) | PCE, sparse PCE, active subspaces |
| [`salib-shapley`](https://crates.io/crates/salib-shapley) | Shapley effects |
| [`salib-validation`](https://crates.io/crates/salib-validation) | Ishigami, Sobol' G, Morris test functions with closed-form indices |
| [`salib-cli`](https://crates.io/crates/salib-cli) | `salib sample`, `salib run`, `salib analyze` |

## Feature flags

| Flag | Default | Effect |
|---|---|---|
| `samplers` | yes | Sampling designs (LHS, Sobol', Saltelli, Morris, FAST) |
| `estimators` | yes | All sensitivity estimators |
| `parallel` | yes | Rayon-based parallel reductions |
| `surrogate` | no | PCE, HDMR, active subspaces |
| `shapley` | no | Shapley effects |
| `validation` | no | Analytic test functions |
| `serde` | no | `Serialize`/`Deserialize` on all result types |
| `arrow` | no | `RecordBatch` conversions for Arrow interop |
| `polars` | no | `DataFrame` conversions (implies `arrow`) |
| `full` | no | Everything except `serde`, `arrow`, `polars` |

## Bit-determinism

Identical `RngState` seeds produce identical results regardless of thread
count. Parallel reductions use tree-structured accumulation to eliminate
float-associativity nondeterminism under rayon. Disable `parallel` for
serial-only builds; results remain identical.

## Citation

If you use salib in published research, please cite:

```bibtex
@software{salib_rs,
  author  = {{antimeme.ai}},
  title   = {salib: Global Sensitivity Analysis for Rust},
  url     = {https://github.com/antimeme-ai/salib},
  version = {0.1.0},
  year    = {2026}
}
```

For individual methods, see the references in the method table above.

## MSRV

1.87

## License

MIT OR Apache-2.0, at your option.
