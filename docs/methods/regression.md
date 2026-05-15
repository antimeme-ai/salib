# Regression-Based Methods

Sensitivity indices from ordinary least squares — standardized coefficients and partial correlations on raw and rank-transformed data.

> **When to use:** Your model is approximately linear (SRC, PCC) or monotonic (SRRC, PRCC). You have an existing sample set from any sampler. Very cheap — just OLS on the data. Good first pass before committing to expensive Sobol'. **Always check the $R^2$ diagnostic before trusting the indices.**

---

## Theory

### SRC — Standardized Regression Coefficients

Saltelli & Marivoet (1990) *Comp. Stat. Data Anal.* 9(1), 55--64. [[bib]](../bibliography.md#saltelli-marivoet1990)

Fit an OLS regression $Y \approx \beta_0 + \boldsymbol{\beta} \cdot \mathbf{X}$ and standardize each coefficient by the input/output standard deviations:

$$\text{SRC}_i = \beta_i \cdot \frac{\sigma_{X_i}}{\sigma_Y}$$

For a truly linear model with independent inputs, $\text{SRC}_i^2 \approx S_i$ — the squared standardized coefficient recovers the first-order Sobol' index. The $R^2$ of the linear fit is the load-bearing diagnostic: if $R^2_{\text{linear}} > 0.7$, SRC indices are trustworthy; below that, the model is too nonlinear for linear regression to capture.

### SRRC — Standardized Rank Regression Coefficients

Replace both $\mathbf{X}$ and $Y$ with their ordinal ranks, then compute SRC on the rank-transformed data (Spearman regression). SRRC captures monotonic nonlinear relationships — the rank transform linearizes any monotonic function. Trust if $R^2_{\text{rank}} > 0.7$.

### PCC — Partial Correlation Coefficients

For each factor $X_i$, regress $X_i$ on all other factors and $Y$ on all other factors. The Pearson correlation of the residuals is the partial correlation:

$$\text{PCC}_i = \operatorname{corr}\!\big(X_i - \hat{X}_i^{(\sim i)},\; Y - \hat{Y}^{(\sim i)}\big)$$

PCC isolates $X_i$'s unique linear contribution after removing the linear effects of every other factor. In a correlated-input design, PCC separates individual influence from collinearity; with independent inputs, PCC and SRC agree in sign and rank ordering but differ in magnitude (PCC normalizes by residual variance, SRC by total variance).

### PRCC — Partial Rank Correlation Coefficients

PCC computed on rank-transformed data. Captures monotonic partial contribution. The standard tool for screening in Monte Carlo uncertainty analyses — Marino et al. (2008) *J. Theor. Biol.* cite PRCC as the default for biological models.

---

## Code

`estimate_regression_indices` returns all four indices plus both $R^2$ diagnostics in a single call. Sampler-agnostic — works on any $(X, Y)$ dataset.

```rust
use salib::estimators::estimate_regression_indices;
use ndarray::Array2;

// x: (N, d) input matrix, y: N-element output vector
let indices = estimate_regression_indices(x.view(), &y).unwrap();

println!("{indices}");
// Regression indices (d=3)
//   R²(linear) = 0.9987  R²(rank) = 0.9992
//
//   Factor      SRC      SRRC       PCC      PRCC
//   ------   ------    ------    ------    ------
//        0   0.8942    0.8951    0.9988    0.9992
//        1   0.4472    0.4476    0.9962    0.9976
//        2  -0.0012   -0.0018   -0.0084   -0.0121
```

> **Verify** on linear fixture $Y = 2 X_0 + X_1$ ($N = 1024$, seed `[0u8; 32]`):
>
> | Diagnostic | Value |
> |---|---|
> | $R^2_{\text{linear}}$ | $> 0.99$ |
> | $\text{SRC}_0 / \text{SRC}_1$ | $\approx 2.0$ (matches coefficient ratio) |
> | $\text{SRC}_2$ | $\approx 0$ (absent factor) |

> **Verify** on Ishigami ($N = 4096$, seed `[0u8; 32]`):
>
> | Diagnostic | Value | Interpretation |
> |---|---|---|
> | $R^2_{\text{linear}}$ | 0.19 | Well below 0.7 — SRC untrustworthy |
> | $R^2_{\text{rank}}$ | 0.19 | SRRC also untrustworthy |
> | $\text{SRRC}_1$ | $\approx 0$ | Correctly detects $\sin^2(x_2)$ non-monotonicity |

---

## When to use each variant

| Index | Captures | Trust signal | Best for |
|---|---|---|---|
| SRC | Linear effects | $R^2_{\text{linear}} > 0.7$ | Additive linear models |
| SRRC | Monotonic effects | $R^2_{\text{rank}} > 0.7$ | Monotonic nonlinearities |
| PCC | Linear partial contribution | $R^2_{\text{linear}} > 0.7$ | Correlated inputs — isolates individual factors |
| PRCC | Monotonic partial contribution | $R^2_{\text{rank}} > 0.7$ | Monotonic + correlated inputs |

All four are sampler-agnostic and cost $O(N d^2 + d^3)$ — negligible compared to any model-evaluation budget. None recover Sobol' indices unless the model is linear (SRC) or monotonic (SRRC/PRCC). The $R^2$ diagnostic is the load-bearing trust signal.

---

## Workflow

1. Run `estimate_regression_indices` on your existing $(X, Y)$ data.
2. Check $R^2_{\text{linear}}$ and $R^2_{\text{rank}}$.
3. If $R^2 > 0.7$: trust the corresponding indices. Identify dominant factors by $|\text{SRC}|$ or $|\text{PRCC}|$ rank ordering.
4. If $R^2 < 0.7$: the model is too nonlinear for regression-based analysis. Proceed to variance-based (Sobol') or distribution-based (Borgonovo, PAWN) methods.
