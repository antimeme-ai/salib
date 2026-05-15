# Variance-Based Methods

Sobol' sensitivity indices — decompose output variance into input contributions.

> **When to use:** You want to know how much each input contributes to the uncertainty in the output. You can afford $(d + 2) \times N$ model evaluations, where $d$ is the number of factors and $N$ is typically 1024–16384. Your model is a black box — no gradients needed.

---

## Theory

For a square-integrable function $f(\mathbf{x})$ with independent inputs, the Sobol' decomposition writes the total variance as a sum of partial variances:

$$\operatorname{Var}(Y) = \sum_i V_i + \sum_{i < j} V_{ij} + \cdots + V_{1,2,\ldots,d}$$

where $V_i = \operatorname{Var}\!\big[\mathbb{E}(Y \mid X_i)\big]$ is the variance of the conditional expectation. The **first-order index** measures the fraction of variance explained by $X_i$ alone:

$$S_i = \frac{V_i}{\operatorname{Var}(Y)} = \frac{\operatorname{Var}\!\big[\mathbb{E}(Y \mid X_i)\big]}{\operatorname{Var}(Y)}$$

The **total-effect index** captures $X_i$'s contribution including all interactions:

$$S_{Ti} = 1 - \frac{\operatorname{Var}\!\big[\mathbb{E}(Y \mid \mathbf{X}_{\sim i})\big]}{\operatorname{Var}(Y)} = \frac{\mathbb{E}\!\big[\operatorname{Var}(Y \mid \mathbf{X}_{\sim i})\big]}{\operatorname{Var}(Y)}$$

where $\mathbf{X}_{\sim i}$ denotes all factors except $X_i$. If $S_{Ti} - S_i > 0$, the factor participates in interactions.

## Sampling design

All four estimators below use the Saltelli cross-matrix design. Two independent $N \times d$ base matrices $A$ and $B$ are drawn (Sobol' quasi-random sequence by default). For each factor $i$, a hybrid matrix $A_B^{(i)}$ is constructed: all columns from $A$ except column $i$, which comes from $B$.

This produces $N(d + 2)$ model evaluations: $N$ for $A$, $N$ for $B$, and $N$ for each of $d$ cross-matrices.

```rust
use salib::samplers::{SobolSampler, build_saltelli_matrix};
use salib::*;

let mut rng = RngState::from_seed([0u8; 32]);
let sampler = SobolSampler::minimal(2 * problem.dim());

// N = 8192 base samples, skip = false
let saltelli = build_saltelli_matrix(
    &sampler, 8192, false, &mut rng
).unwrap();
```

> **Sample size:** $N = 1024$ is the practical minimum for stable first-order estimates with $d \leq 10$. For total-effect indices or models with strong interactions, use $N \geq 4096$. Doubling $N$ roughly halves the standard error.

---

## Saltelli 2010

Saltelli et al. (2010) *Comp. Phys. Comm.* 181(2), 259–270. [[bib]](../bibliography.md#saltelli2010)

The default estimator. First-order:

$$\hat{S}_i = \frac{\frac{1}{N}\sum_{j=1}^{N} f(B)_j \Big(f(A_B^{(i)})_j - f(A)_j\Big)}{\operatorname{Var}(Y)}$$

Total-effect:

$$\hat{S}_{Ti} = \frac{\frac{1}{2N}\sum_{j=1}^{N} \Big(f(A)_j - f(A_B^{(i)})_j\Big)^2}{\operatorname{Var}(Y)}$$

```rust
use salib::estimators::estimate_saltelli2010;

let indices = estimate_saltelli2010(&saltelli, |x| {
    x[0].sin()
        + 7.0 * x[1].sin().powi(2)
        + 0.1 * x[2].powi(4) * x[0].sin()
});

println!("{indices}");
```

> **Verify** against Ishigami closed-form ($N = 8192$, seed `[0u8; 32]`):
>
> | Factor | $\hat{S}_i$ | Analytic $S_i$ | $\hat{S}_{Ti}$ | Analytic $S_{Ti}$ |
> |--------|-------------|----------------|-----------------|-------------------|
> | $x_1$  | 0.3143      | 0.3139         | 0.5559          | 0.5576            |
> | $x_2$  | 0.4427      | 0.4424         | 0.4420          | 0.4424            |
> | $x_3$  | −0.0042     | 0.0000         | 0.2455          | 0.2437            |

---

## Jansen 1999

Jansen (1999) *Comp. Phys. Comm.* 117(1–2), 35–43. [[bib]](../bibliography.md#jansen1999)

An alternative total-effect estimator with empirically better convergence for small $N$.

First-order (same as Saltelli 2010):

$$\hat{S}_i = \frac{\frac{1}{N}\sum_{j=1}^{N} f(B)_j \Big(f(A_B^{(i)})_j - f(A)_j\Big)}{\operatorname{Var}(Y)}$$

Total-effect (Jansen formulation):

$$\hat{S}_{Ti} = \frac{\frac{1}{2N}\sum_{j=1}^{N} \Big(f(B)_j - f(A_B^{(i)})_j\Big)^2}{\operatorname{Var}(Y)}$$

The difference from Saltelli is subtle: Jansen compares $f(B)$ to $f(A_B^{(i)})$ instead of $f(A)$ to $f(A_B^{(i)})$ in the total-effect formula. Both are consistent estimators; Jansen converges faster when the model has high-order interactions.

```rust
use salib::estimators::estimate_jansen;

let indices = estimate_jansen(&saltelli, |x| {
    x[0].sin()
        + 7.0 * x[1].sin().powi(2)
        + 0.1 * x[2].powi(4) * x[0].sin()
});
```

---

## Janon 2014

Janon et al. (2014) *ESAIM: Probability and Statistics* 18, 342–364. [[bib]](../bibliography.md#janon2014)

Asymptotically efficient first-order estimator. Uses the cross-matrix outputs differently to reduce bias:

$$\hat{S}_i = \frac{\frac{1}{N}\sum_{j=1}^{N} f(A)_j\, f(A_B^{(i)})_j - \left(\frac{1}{2N}\sum_{j=1}^{N}\big(f(A)_j + f(A_B^{(i)})_j\big)\right)^2}{\frac{1}{2N}\sum_{j=1}^{N}\big(f(A)_j^2 + f(A_B^{(i)})_j^2\big) - \left(\frac{1}{2N}\sum_{j=1}^{N}\big(f(A)_j + f(A_B^{(i)})_j\big)\right)^2}$$

Janon's estimator achieves the semiparametric efficiency bound for $S_i$ — no other estimator based on the same design can have lower asymptotic variance. In practice the difference matters most for small $N$ and models with near-zero first-order indices.

```rust
use salib::estimators::estimate_janon;

let indices = estimate_janon(&saltelli, |x| {
    x[0].sin()
        + 7.0 * x[1].sin().powi(2)
        + 0.1 * x[2].powi(4) * x[0].sin()
});
```

---

## Owen 2013

Owen (2013) *ACM Trans. Model. Comp. Sim.* 23(2), Article 11. [[bib]](../bibliography.md#owen2013)

Three-matrix design that produces improved estimates for second-order interaction indices $S_{ij}$. Owen introduces a third base matrix $C$ and defines cross-matrices $A_B^{(i)}$ and $A_C^{(i)}$ to separate first-order and interaction effects more cleanly.

Owen requires its own sampling design — the sampler dimension must be $3d$ (for the $A$, $B$, $C$ matrices):

```rust
use salib::samplers::{SobolSampler, build_owen_matrix};
use salib::estimators::estimate_owen;
use salib::*;

let mut rng = RngState::from_seed([0u8; 32]);
// 3 * dim for the three-matrix design
let sampler = SobolSampler::minimal(3 * problem.dim());
let owen = build_owen_matrix(&sampler, 8192, &mut rng).unwrap();

let indices = estimate_owen(&owen, |x| {
    x[0].sin()
        + 7.0 * x[1].sin().powi(2)
        + 0.1 * x[2].powi(4) * x[0].sin()
});
```

---

## Given-Data Sobol'

Plischke et al. (2013) *Eur. J. Oper. Res.* 226(3), 536–550. [[bib]](../bibliography.md#plischke2013)

Estimate first-order Sobol' indices from an existing dataset without a designed experiment. The method partitions the data by binning each input variable and estimates $V_i$ from the between-bin variance of conditional means.

> **When to use:** You have observational data (not from a designed experiment) and want sensitivity indices. You cannot re-run the model. The estimate is first-order only — no total-effect indices.

```rust
use salib::estimators::estimate_given_data_sobol;
use ndarray::Array2;

// x: (N, d) matrix of inputs, y: (N,) vector of outputs
let indices = estimate_given_data_sobol(x.view(), &y);
```

> **Caveat:** Given-data estimates are biased when inputs are correlated. The method assumes independence — if your factors are dependent, the indices measure a blend of sensitivity and dependence. Consider Shapley effects for correlated inputs.

---

## Choosing among estimators

| Estimator     | $S_i$ | $S_{Ti}$ | Best for                                              |
|---------------|--------|----------|-------------------------------------------------------|
| Saltelli 2010 | yes    | yes      | General use. Start here.                              |
| Jansen 1999   | yes    | yes      | Better $S_T$ convergence at small $N$.                |
| Janon 2014    | yes    | yes      | Most efficient $S_i$ at the semiparametric bound.     |
| Owen 2013     | yes    | yes      | Second-order interactions $S_{ij}$.                   |
| Given-data    | yes    | no       | Observational data, no designed experiment.           |

If you're unsure, use Saltelli 2010. It's the most widely cited, has stable behavior across problem types, and gives you both $S_i$ and $S_{Ti}$. Switch to Jansen if you're budget-constrained on model evaluations, or to Janon if you need the tightest possible confidence intervals on first-order indices.
