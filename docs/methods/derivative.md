# Derivative-Based Methods

Sensitivity measures derived from model gradients — upper bounds on total-effect indices without the full Saltelli design.

> **When to use:** Gradient information is available or cheap (analytical, adjoint, or finite-difference). You want provable upper bounds on each factor's total contribution to output variance. Fewer model evaluations than Sobol' — $N \cdot (d + 1)$ for forward FD vs $N \cdot (d + 2)$ for Saltelli, but each evaluation includes a gradient.

---

## DGSM

Sobol' & Kucherenko (2009) *Math. Comp. Sim.* 79(10), 3009--3017. [[bib]](../bibliography.md#sobol-kucherenko2009)

### Theory

For a square-integrable model $f(\mathbf{x})$ with independent inputs, the **Derivative-based Global Sensitivity Measure** for factor $X_i$ is the expected squared partial derivative:

$$\nu_i = \mathbb{E}\!\left[\left(\frac{\partial f}{\partial x_i}\right)^2\right]$$

The Poincare inequality links $\nu_i$ to the total-effect Sobol' index. For an input $X_i$ with distribution $\mu_i$ and Poincare constant $C_P(\mu_i)$:

$$S_{Ti} \leq \frac{C_P(\mu_i) \cdot \nu_i}{\operatorname{Var}(Y)}$$

This bound is **provable**, not empirical. A factor with $\nu_i \cdot C_P / \operatorname{Var}(Y) < \delta$ contributes provably less than $\delta$ to total variance. The bound can be loose — it is one-sided and depends on the spectral gap of the input distribution — but it is sufficient for screening: factors below the threshold are safely fixed.

### Poincare constants

Per Roustant, Barthe & Iooss (2017), the closed-form constants for common distributions are:

| Distribution | $C_P$ |
|---|---|
| $\operatorname{Uniform}[a, b]$ | $(b - a)^2 / \pi^2$ |
| $\mathcal{N}(\mu, \sigma^2)$ | $\sigma^2$ |

Use `poincare_constant` to derive $C_P$ from a `Distribution`. Other distributions (Beta, Gamma, truncated families) require numerical Kummer-function solvers not yet implemented — supply the constant directly.

### Gradient computation

`estimate_dgsm` takes a pre-computed gradient matrix. The caller chooses the gradient source:

- **Analytical** — closed-form derivative of the model. Zero extra model evaluations.
- **Adjoint** — same interface as analytical; distinguished by the solver backend.
- **Finite-difference** — via `finite_difference_gradients`:
  - `FdKind::Forward`: $(f(x + \varepsilon e_i) - f(x)) / \varepsilon$, $O(\varepsilon)$ error, $N \cdot d$ extra evaluations.
  - `FdKind::Central`: $(f(x + \varepsilon e_i) - f(x - \varepsilon e_i)) / (2\varepsilon)$, $O(\varepsilon^2)$ error, $2 N d$ extra evaluations.

Typical step sizes: $\varepsilon = 10^{-6}$ for forward, $10^{-4}$ to $10^{-5}$ for central (balancing truncation vs round-off).

### Code

```rust
use salib::estimators::{
    estimate_dgsm, finite_difference_gradients,
    poincare_constant, FdKind,
};
use salib::{Distribution, tree_var};
use salib::samplers::{LhsSampler, Sampler};
use ndarray::Array2;

// Sample inputs (N = 4096, d = 3, Uniform[-pi, pi])
let dist = Distribution::Uniform { lo: -std::f64::consts::PI, hi: std::f64::consts::PI };
let cp = poincare_constant(&dist).unwrap();   // (2*pi)^2 / pi^2 = 4.0

// Compute gradients via central finite-difference
let gradients = finite_difference_gradients(
    x.view(), 1e-5, FdKind::Central, |xi| model(xi),
);

let var_y = tree_var(&y);
let indices = estimate_dgsm(
    gradients.view(),
    &[cp, cp, cp],   // one Poincare constant per factor
    var_y,
).unwrap();

println!("{indices}");
// Factor   nu     ST_upper
//      0  7.7721    2.1820
//      1 24.5001    6.8770
//      2 10.9184    3.0650
```

> **Verify** against Ishigami closed-form ($N = 4096$, seed `[0u8; 32]`, central FD with $\varepsilon = 10^{-5}$):
>
> | Factor | $\hat{\nu}_i$ | Analytic $\nu_i$ | $\hat{S}_{Ti}^{\text{upper}}$ | Analytic $S_{Ti}$ |
> |--------|--------------|------------------|-------------------------------|-------------------|
> | $x_1$  | 7.7721       | 7.72             | 2.1820                        | 0.5576            |
> | $x_2$  | 24.5001      | 24.50            | 6.8770                        | 0.4424            |
> | $x_3$  | 10.9184      | 10.99            | 3.0650                        | 0.2437            |
>
> All $S_{Ti}^{\text{upper}} \geq S_{Ti}$ — the Poincare bound holds. The bound is **loose** because $\operatorname{Uniform}[-\pi, \pi]$ has $C_P = 4$; the screening value is in the provable direction, not tight estimation.

---

## Choosing DGSM vs Sobol'

| Criterion | DGSM | Sobol' (Saltelli) |
|---|---|---|
| Output | Upper bound on $S_{Ti}$ | Direct estimate of $S_i$, $S_{Ti}$ |
| Cost | $N(d+1)$ to $2Nd$ (FD) | $N(d+2)$ |
| Gradient needed | Yes | No |
| Interactions | Bound only | Full decomposition |
| Correlated inputs | Same Poincare framework | Requires Shapley |

DGSM is a screening instrument. If the upper bound for a factor is below your tolerance, the factor is provably unimportant — fix it and reduce $d$ before running a full Sobol' analysis. If the bound is above the tolerance, DGSM cannot distinguish "genuinely important" from "loose bound" — proceed to variance-based methods.
