# Surrogate-Based Methods

Build a cheap polynomial approximation, extract Sobol' indices analytically from the coefficients. No additional model evaluations once the surrogate is fitted.

> **When to use:** Model evaluations are expensive. You can afford $N$ runs to build the surrogate, then extract all sensitivity information from the fitted coefficients at negligible cost. Works best when the model is smooth and the effective dimensionality is moderate.

---

## Polynomial Chaos Expansion

Xiu & Karniadakis (2002) *SIAM J. Sci. Comp.* 24(2), 619--644 (generalized PCE). [[bib]](../bibliography.md#xiu-karniadakis2002)

Blatman & Sudret (2011) *J. Comp. Phys.* 230(6), 2345--2367 (sparse PCE). [[bib]](../bibliography.md#blatman-sudret2011)

### Theory

Approximate the model output as a truncated series in orthogonal polynomial basis functions:

$$f(\mathbf{x}) \approx \sum_{|\alpha| \leq p} c_\alpha \, \Psi_\alpha(\boldsymbol{\xi})$$

where $\boldsymbol{\xi}$ is the input mapped to each factor's polynomial-canonical domain, $\alpha$ is a multi-index, and $\Psi_\alpha(\boldsymbol{\xi}) = \prod_k \Psi_{\alpha_k}(\xi_k)$ is the tensor-product basis. The polynomial families are matched to the input distributions per the Wiener-Askey scheme:

| Input distribution | Polynomial family | Canonical domain |
|---|---|---|
| $\operatorname{Uniform}[a, b]$ | Legendre | $[-1, 1]$ |
| $\mathcal{N}(\mu, \sigma^2)$ | Hermite | $\mathbb{R}$ |

Coefficients $\{c_\alpha\}$ are fitted via OLS on the $(N, P)$ basis matrix, where $P = \binom{d + p}{p}$ is the number of basis terms at total degree $p$.

### Sobol' indices from coefficients

Per Sudret (2008), the orthogonality of the tensor-product basis makes Sobol' indices analytic (Eq 36--39):

$$S_i = \frac{\sum_{\alpha \in \mathcal{A}_i} c_\alpha^2 \, \langle \Psi_\alpha, \Psi_\alpha \rangle}{\sum_{\alpha \neq 0} c_\alpha^2 \, \langle \Psi_\alpha, \Psi_\alpha \rangle}$$

where $\mathcal{A}_i = \{\alpha : \alpha_i > 0,\; \alpha_j = 0 \;\forall j \neq i\}$ is the set of main-effect multi-indices for factor $i$. The total-order index sums over all multi-indices where factor $i$ is active:

$$S_{Ti} = \frac{\sum_{\alpha : \alpha_i > 0} c_\alpha^2 \, \langle \Psi_\alpha, \Psi_\alpha \rangle}{\sum_{\alpha \neq 0} c_\alpha^2 \, \langle \Psi_\alpha, \Psi_\alpha \rangle}$$

These are **exact given exact coefficients**; finite-$N$ OLS introduces estimation error in the coefficients, which propagates to the indices.

### Full OLS: `fit_full_pce`

Use when $N \gg P$. Fits all $P$ basis terms via Cholesky-factored normal equations. Cost: $O(P^3 + P^2 N)$ for the OLS solve, plus $O(N P d)$ for basis-matrix construction.

```rust
use salib::surrogate::{
    fit_full_pce, sobol_indices_from_pce, PolynomialFamily,
};

// samples_canonical: (N, d) in [-1, 1]^d (Legendre domain)
let pce = fit_full_pce(
    samples_canonical.view(),
    &y,
    &[PolynomialFamily::Legendre; 3],
    5,   // max_degree p = 5
).unwrap();

let sobol = sobol_indices_from_pce(&pce).unwrap();
println!("{sobol}");
```

> **Verify** on additive model $Y = \xi_0 + 2\xi_1$ over $[-1, 1]^2$ ($N = 256$, Legendre, $p = 3$):
>
> $\operatorname{Var}(\xi) = 1/3$ for each factor. $\operatorname{Var}(Y) = 1/3 + 4/3 = 5/3$.
>
> | Factor | $\hat{S}_i$ | Analytic $S_i$ | $\hat{S}_{Ti}$ | Analytic $S_{Ti}$ |
> |--------|-------------|----------------|-----------------|-------------------|
> | $\xi_0$ | 0.2000 | 0.2 | 0.2000 | 0.2 |
> | $\xi_1$ | 0.8000 | 0.8 | 0.8000 | 0.8 |
>
> Additive model: $S_{Ti} = S_i$ (no interactions). PCE recovers the exact split because a linear function is in the span of any degree-$p \geq 1$ basis.

### Sparse LARS/OMP: `fit_sparse_pce`

Use when $P$ is large and most coefficients are near zero. Forward-selection solvers with leave-one-out cross-validation stopping:

- **OMP** (Orthogonal Matching Pursuit) — greedily pick the basis column most correlated with the residual, refit OLS on the active set, stop on LOO-CV upturn.
- **LARS** (Least Angle Regression, Efron 2004) — equiangular forward selection; gentler column additions, same LOO stopping criterion.

Both produce a `PolynomialChaos` compatible with `sobol_indices_from_pce` — the sparse and full paths share the same downstream analysis.

```rust
use salib::surrogate::{
    fit_sparse_pce, sobol_indices_from_pce,
    SparseSolver, TruncationScheme, PolynomialFamily,
};

let (pce, diagnostic) = fit_sparse_pce(
    samples_canonical.view(),
    &y,
    &[PolynomialFamily::Legendre; 5],
    4,   // max_degree
    TruncationScheme::Hyperbolic { q: 0.75 },
    SparseSolver::Omp,
    None,   // max_terms: auto
).unwrap();

let sobol = sobol_indices_from_pce(&pce).unwrap();
println!("Active terms: {} / {}", diagnostic.num_active, diagnostic.candidate_basis_size);
println!("{sobol}");
```

### Basis truncation

- **Total-degree** (`TruncationScheme::TotalDegree`): all $\alpha$ with $|\alpha| = \sum \alpha_j \leq p$. Basis size $P = \binom{d+p}{p}$.
- **Hyperbolic $q$-norm** (`TruncationScheme::Hyperbolic { q }`): $\left(\sum \alpha_j^q\right)^{1/q} \leq p$ with $q \in (0, 1]$. At $q < 1$, high-interaction terms are suppressed before the solver sees them. Blatman & Sudret (2011) recommend $q = 0.75$.

---

## HDMR

Li, Rosenthal & Rabitz (2001) *J. Phys. Chem. A* 105(33), 7765--7777. [[bib]](../bibliography.md#li2001)

### Theory

High-Dimensional Model Representation decomposes $f$ into component functions of increasing interaction order:

$$f(\mathbf{x}) = f_0 + \sum_i f_i(x_i) + \sum_{i < j} f_{ij}(x_i, x_j) + \cdots$$

The RS-HDMR (Random Sampling) implementation fits a full PCE to the $(X, Y)$ data, then groups the PCE coefficients by interaction order. Each component function's variance contribution is accumulated from the basis functions whose multi-index has the matching active-factor set.

This produces the same first-order and total-order Sobol' indices as `sobol_indices_from_pce`, plus:

- **Second-order pairwise indices** $S_{ij}$ — variance attributed to the $X_i \times X_j$ interaction.
- **Per-order variance fractions** — how much variance lives at each interaction order.

### Code

```rust
use salib::estimators::estimate_hdmr;

let result = estimate_hdmr(
    x.view(),       // (N, d) physical-domain inputs
    &y,             // N-element model output
    &problem,       // defines factor distributions
    2,              // max_order: up to pairwise interactions
    4,              // max_degree: PCE truncation degree
).unwrap();

println!("{result}");
// HDMR decomposition (d=3)
//   Var[Y] = 13.8446
//   Order 1 variance fraction: 0.7563
//   Order 2 variance fraction: 0.2437
//
//   Factor      S1        ST
//        0  0.3139    0.5576
//        1  0.4424    0.4424
//        2  0.0000    0.2437
```

> **When to use:** You want the full interaction structure (which pairs of factors interact and how much variance each pair explains) at the cost of a PCE fit. For first-order and total-order indices alone, `sobol_indices_from_pce` on a full or sparse PCE is simpler.

---

## Active Subspaces

Constantine (2015) *Active Subspaces: Emerging Ideas for Dimension Reduction in Parameter Studies*, SIAM. [[bib]](../bibliography.md#constantine2015)

### Theory

Find the directions in input space along which $f$ varies most. Define the uncentered gradient covariance matrix:

$$C = \mathbb{E}\!\left[\nabla f \, (\nabla f)^T\right]$$

and eigendecompose $C = W \Lambda W^T$ with $\lambda_1 \geq \cdots \geq \lambda_d \geq 0$. Each eigenvalue $\lambda_i = \mathbb{E}\!\left[(({\nabla f})^T w_i)^2\right]$ is the mean-squared directional derivative along eigenvector $w_i$ (Constantine 2014 Lemma 2.1).

The **active subspace** is the span of the leading $k$ eigenvectors — directions where the model varies most. A gap in the eigenvalue spectrum ($\lambda_k \gg \lambda_{k+1}$) signals a clean reduced-dimension representation: $f(\mathbf{x}) \approx g(W_k^T \mathbf{x})$ for some lower-dimensional function $g$.

### Monte Carlo estimator

Given $M$ gradient samples $\nabla f(x_1), \ldots, \nabla f(x_M)$ stacked into an $(M, d)$ matrix, the estimator is:

$$\tilde{C} = \frac{1}{M} \sum_{j=1}^{M} (\nabla f_j)(\nabla f_j)^T$$

Eigendecomposed via `nalgebra::SymmetricEigen` at cost $O(d^3 + M d^2)$.

### Active-dimension detection

The largest-gap heuristic: $k = \operatorname{argmax}_j (\lambda_j / \lambda_{j+1})$. A perfect gap ($\lambda_{j+1} \leq 10^{-12} \lambda_{\max}$) commits $k$ immediately — this handles ridge functions and low-rank models deterministically.

Optional `gap_threshold`: require the ratio to exceed a caller-specified threshold $t > 1$ before accepting a gap. If no ratio qualifies, $k = d$ (all directions retained).

### Code

```rust
use salib::surrogate::compute_active_subspace;

// gradients: (M, d) matrix of sampled gradients
let result = compute_active_subspace(
    gradients.view(),
    None,   // gap_threshold: use default heuristic
).unwrap();

println!("Active dimension: {}", result.k_active);
println!("Eigenvalues: {:?}", result.eigenvalues);
// Leading eigenvectors define the active subspace:
// result.eigenvectors columns 0..k_active
```

> **Verify** on ridge function $f(\mathbf{x}) = \mathbf{a}^T \mathbf{x}$ with $\mathbf{a} = (3, 0, 4)$ ($M = 50$):
>
> | Eigenvalue | Value | Interpretation |
> |---|---|---|
> | $\lambda_1$ | 25.0 | $= \|\mathbf{a}\|^2$ |
> | $\lambda_2$ | 0.0 | Numerically zero |
> | $\lambda_3$ | 0.0 | Numerically zero |
>
> $k_{\text{active}} = 1$. Leading eigenvector $= \pm \mathbf{a} / \|\mathbf{a}\|$. Rank-1 gradient covariance recovered exactly.

> **When to use:** High-dimensional inputs ($d > 10$) where you suspect the model really depends on a few linear combinations of the inputs. Active subspaces identify those combinations and their relative importance. Pair with DGSM or surrogate methods — the gradient samples serve double duty.

---

## Choosing among surrogates

| Method | Produces | Cost | Best for |
|---|---|---|---|
| Full PCE | $S_i$, $S_{Ti}$ (analytic) | $O(P^3 + NP)$ | Moderate $d$ and $p$, smooth models, $N \gg P$ |
| Sparse PCE | $S_i$, $S_{Ti}$ (analytic) | $O(K \cdot NP)$ for $K$ steps | Large basis ($P \gg N$), sparse coefficient structure |
| HDMR | $S_i$, $S_{ij}$, $S_{Ti}$, order fractions | Full PCE + grouping | Interaction structure analysis |
| Active Subspaces | Dominant directions, eigenvalue spectrum | $O(d^3 + Md^2)$ | Dimension reduction, $d > 10$ |

Start with sparse PCE if $d \cdot p$ is large. Use full PCE when $N$ is generous relative to $P$. Add HDMR when you need pairwise interaction indices. Use active subspaces when you want to reduce the input dimension before fitting any surrogate.
