# Quickstart

Sobol' indices for the Ishigami function in five minutes.

---

The Ishigami function is the standard test problem for sensitivity analysis methods. It has three inputs, nonlinear interactions, and closed-form Sobol' indices — so you know immediately whether your estimates are right.

## The model

The Ishigami function maps three independent uniform inputs $x_i \sim \mathcal{U}(-\pi, \pi)$ to a scalar:

$$f(\mathbf{x}) = \sin(x_1) + 7\sin^2(x_2) + 0.1\, x_3^4 \sin(x_1)$$

The $x_3^4 \sin(x_1)$ interaction term makes this a hard test: $x_3$ contributes nothing to first-order variance but has a large total-effect index through its interaction with $x_1$.

## Closed-form indices

With $a = 7$ and $b = 0.1$, the analytic Sobol' indices are:

| Factor | $S_i$ (first-order) | $S_{Ti}$ (total-effect) |
|--------|---------------------|-------------------------|
| $x_1$  | 0.3139              | 0.5576                  |
| $x_2$  | 0.4424              | 0.4424                  |
| $x_3$  | 0.0000              | 0.2437                  |

$x_3$ has zero first-order effect but 24% total effect — all of it through the $x_1 x_3$ interaction. Any estimator that handles this correctly is working.

## Dependencies

```toml
[dependencies]
salib = "0.1"
```

This pulls in `salib-core`, `salib-samplers`, and `salib-estimators` by default.

## Code

```rust
use std::f64::consts::PI;
use salib::*;
use salib::samplers::{SobolSampler, build_saltelli_matrix};
use salib::estimators::estimate_saltelli2010;

fn main() {
    // 1. Define the problem: three uniform factors on [-π, π].
    let problem = ProblemBuilder::new()
        .factor("x1", Distribution::Uniform { lo: -PI, hi: PI })
        .factor("x2", Distribution::Uniform { lo: -PI, hi: PI })
        .factor("x3", Distribution::Uniform { lo: -PI, hi: PI })
        .build()
        .unwrap();

    // 2. Generate quasi-random samples.
    //    Sobol' sequence, 8192 base samples, Saltelli cross-matrix design.
    let mut rng = RngState::from_seed([0u8; 32]);
    let sampler = SobolSampler::minimal(2 * problem.dim());
    let saltelli = build_saltelli_matrix(
        &sampler, 8192, false, &mut rng
    ).unwrap();

    // 3. Evaluate the model and estimate indices.
    let indices = estimate_saltelli2010(&saltelli, |x| {
        x[0].sin()
            + 7.0 * x[1].sin().powi(2)
            + 0.1 * x[2].powi(4) * x[0].sin()
    });

    println!("{indices}");
}
```

## Output

```
Sobol' Indices (Saltelli 2010)
──────────────────────────────────────────
Factor          S1        ST
x1          0.3143    0.5559
x2          0.4427    0.4420
x3         -0.0042    0.2455
```

> **Verify:** Compare against the closed-form values above. $S_1$ estimates are within 0.005 of the analytic values at $N = 8192$. $S_T$ estimates are within 0.002. $S_1$ for $x_3$ is slightly negative — expected Monte Carlo noise around the true value of zero.

## What just happened

Three things, each handled by a different crate.

**Sampling** (`salib-samplers`): A Sobol' quasi-random sequence generated $N = 8192$ base samples in the unit hypercube, then the Saltelli cross-matrix design produced the $A$, $B$, and $A_B^{(i)}$ matrices needed for the variance decomposition — $(d + 2) \times N = 40{,}960$ model evaluations total.

**Evaluation**: The closure ran the Ishigami function on each row of the combined matrix. salib does not own your model — any `Fn(&[f64]) -> f64` works.

**Estimation** (`salib-estimators`): The Saltelli 2010 estimator computed first-order indices $S_i$ and total-effect indices $S_{Ti}$ from the $A$, $B$, $A_B^{(i)}$ output vectors using the formulas in [Saltelli et al. (2010)](bibliography.md#saltelli2010).

## Next

- [Choosing a method](choosing.md) — when to use Morris screening vs. Sobol' indices vs. distribution-based measures.
- [Variance-based methods](methods/variance-based.md) — the full theory behind the Saltelli, Jansen, Janon, and Owen estimators.
- [Internals](internals.md) — how salib guarantees identical results regardless of thread count.
