# Distribution-Based Methods

Sensitivity through the lens of the full output distribution, not just its variance.

> **When to use:** You suspect a factor changes the output distribution's shape, location, or tails in ways that variance alone cannot capture. Two factors with identical $S_i$ can have very different distributional importance — one shifts the mean while the other fattens the tails. Distribution-based indices detect both. All three methods below are sampler-agnostic: they work on any `(X, Y)` dataset, including observational data.

---

## Borgonovo $\delta$

Borgonovo (2007) *Rel. Eng. Sys. Safety* 92(6), 771--784. [[bib]](../bibliography.md#borgonovo2007)

Estimated via the Plischke-Borgonovo-Smith (2013) given-data algorithm. [[bib]](../bibliography.md#plischke2013)

### Definition

The moment-independent importance measure $\delta_i$ is the expected $L^1$ distance between the unconditional output density $f_Y$ and the conditional density $f_{Y|X_i}$:

$$\delta_i = \frac{1}{2}\,\mathbb{E}_{X_i}\!\left[\int \big|f_Y(y) - f_{Y|X_i}(y)\big|\,dy\right]$$

$\delta_i \in [0, 1]$. It equals zero if and only if $Y$ and $X_i$ are independent. Unlike Sobol' indices, $\delta$ responds to *any* distributional change — location, scale, shape, modality — not just variance.

### Algorithm

Plischke-Borgonovo-Smith 2013, Eq 26:

1. Build the unconditional density $\hat{f}_Y$ via Gaussian KDE with Silverman's bandwidth.
2. Partition $X_i$ into $M$ equal-frequency classes by ordinal rank. $M = \min\!\big(\lceil N^{\text{exp}}\rceil,\, 48\big)$ where $\text{exp} = 2 / (7 + \tanh((1500 - N)/500))$.
3. For each class $j$: build conditional $\hat{f}_{Y|\text{class}_j}$ via KDE on $Y$ values in the class; integrate $|f_Y - f_{Y|\text{class}_j}|$ by trapezoidal rule over a 100-point grid.
4. $\hat{\delta}_i = \sum_j \frac{n_j}{2N} \int |\hat{f}_Y - \hat{f}_{Y|\text{class}_j}|\,dy$.

### Code

```rust
use salib::estimators::estimate_borgonovo_delta;
use ndarray::Array2;

// x: (N, d) input matrix, y: N-element output vector
let indices = estimate_borgonovo_delta(x.view(), &y).unwrap();

// indices.delta[i] is δ for factor i
println!("{indices}");
```

> **Verify** on Ishigami ($a = 7$, $b = 0.1$, LHS $N = 4096$, seed `[0u8; 32]`):
>
> | Factor | $\hat{\delta}_i$ | Analytic $\delta_i$ |
> |--------|-------------------|---------------------|
> | $x_1$  | 0.210             | 0.214               |
> | $x_2$  | 0.368             | 0.371               |
> | $x_3$  | 0.144             | 0.157               |
>
> Analytic values from Plischke-Borgonovo-Smith (2013).

> **Caveat:** The estimator ships the raw Plischke 2013 Eq 26 form without the Eq 30 jackknife bias correction that `SALib` applies by default. At $N = 4096$ the difference is roughly 0.02--0.04 per factor. KDE convergence on multimodal outputs (like Ishigami) is slow; use $N \geq 4096$ for tight estimates.

---

## PAWN

Pianosi & Wagener (2015) *Env. Mod. Soft.* 67, 1--11. [[bib]](../bibliography.md#pianosi2015)

Generalized to arbitrary samplers via Pianosi & Wagener (2018). [[bib]](../bibliography.md#pianosi2018)

### Definition

PAWN conditions on $X_i$ by slicing it into $S$ equal-frequency bins, then measures the Kolmogorov-Smirnov distance between the unconditional CDF $F_Y$ and the conditional CDF $F_{Y|X_i \in \text{slice}_k}$:

$$\text{KS}_k = \sup_y \big|F_Y(y) - F_{Y|\text{slice}_k}(y)\big|$$

The per-factor PAWN index aggregates the $S$ slice-wise KS values. Pianosi 2018 recommends the **median**; the 2015 formulation uses the **maximum**:

$$T_i^{\text{(median)}} = \text{median}_k\,\text{KS}_k, \qquad T_i^{\text{(max)}} = \max_k\,\text{KS}_k$$

The estimator also returns `mean`, `minimum`, and `cv` (coefficient of variation) across slices.

### Why CDF, not PDF

Borgonovo $\delta$ compares densities via KDE — bandwidth selection and integration grids are required. PAWN compares CDFs directly from sample order statistics. No kernel, no tuning parameter beyond the slice count $S$. Trade-off: the KS statistic is less sensitive to subtle density changes (e.g., distributions that differ only in higher moments while having similar CDFs).

### Code

```rust
use salib::estimators::estimate_pawn;

// n_slices: conditioning slice count (SALib default 10;
// Pianosi 2020 recommends S in [10, 20])
let indices = estimate_pawn(x.view(), &y, 10).unwrap();

// indices.median[i], indices.maximum[i], etc.
println!("{indices}");
```

> **Verify** on Ishigami ($a = 7$, $b = 0.1$, LHS $N = 4096$, $S = 10$, seed `[0u8; 32]`):
>
> | Factor | median | max   |
> |--------|--------|-------|
> | $x_1$  | 0.245  | 0.280 |
> | $x_2$  | 0.390  | 0.499 |
> | $x_3$  | 0.088  | 0.199 |
>
> SALib differential at the same $N$: max absolute difference 0.007 across all factors and statistics.

> **Caveat:** PAWN has no closed-form analytic value for standard test functions. Validation is against SALib and by ranking agreement with Sobol' total-order indices. The ranking is stable across $S \in [8, 16]$, but exact index values shift with the slice count. Require $N \geq 2S$ samples; $N \geq 1024$ with $S = 10$ for stable estimates.

---

## QOSA

Fort, Klein & Rachdi (2016) *Comm. Stat. Theory Methods* 45(15), 4349--4364. [[bib]](../bibliography.md#fort2016)

Estimated via the partition-based form of Maume-Deschamps & Niang (2018). [[bib]](../bibliography.md#maumedeschamps2018)

### Definition

Quantile-Oriented Sensitivity Analysis answers: **which input drives the $\alpha$-quantile of $Y$?** This is the right question when tail behavior matters — exceedance probabilities, VaR-style measures, 95th-percentile latency.

The index is derived from the $\alpha$-quantile contrast (check loss) function $\psi_\alpha(y, \theta) = (y - \theta)(\alpha - \mathbf{1}_{y \leq \theta})$. Via the Conditional Tail Expectation (Maume-Deschamps & Niang 2018, Prop 3.1):

$$S_i^\alpha = 1 - \frac{\mathbb{E}\!\big[Y \mid Y > F_{Y|X_i}^{-1}(\alpha)\big] - \mathbb{E}[Y]}{\text{CTE}_\alpha(Y) - \mathbb{E}[Y]}$$

where $\text{CTE}_\alpha(Y) = \mathbb{E}[Y \mid Y > F_Y^{-1}(\alpha)]$ is the tail mean above the $\alpha$-quantile.

$S_i^\alpha \in [0, 1]$. It equals zero if $Y \perp X_i$ (the factor has no influence on the $\alpha$-quantile) and one if $Y$ is $X_i$-measurable (the factor fully determines the output).

### Partition-based estimator

The implementation uses ordinal-class partitioning (same adaptive class count as Borgonovo $\delta$) rather than the kernel-conditional-quantile estimator of the original paper:

1. Sort $Y$; take the $\lceil \alpha N \rceil$-th value as $\hat{\theta}^*$ (empirical $\alpha$-quantile).
2. Compute $\bar{Y}$ and $\widehat{\text{CTE}}_\alpha(Y) = \frac{1}{N(1-\alpha)} \sum_j Y_j \cdot \mathbf{1}_{Y_j > \hat{\theta}^*}$.
3. For each factor $i$: partition $X_i$ into $K$ classes; compute the conditional $\alpha$-quantile per class; evaluate the conditional CTE; apply the Prop 3.1 formula.

### Code

```rust
use salib::estimators::estimate_qosa;

// alpha: quantile level in (0, 1)
// 0.5 = median, 0.95 = tail, 0.99 = extreme tail
let indices = estimate_qosa(x.view(), &y, 0.95).unwrap();

// indices.s[i] is S^alpha for factor i
// indices.alpha, indices.global_quantile, indices.global_cte
// are diagnostic echoes
println!("{indices}");
```

> **Verify** on Ishigami-derived tail model ($N = 4096$, seed `[0u8; 32]`):
>
> For $Y = X_1 + 5 X_2 \cdot \mathbf{1}_{X_3 > 0.95}$:
> - At $\alpha = 0.5$: $X_1$ dominates (the indicator fires only 5% of the time; the median is driven by $X_1$).
> - At $\alpha = 0.95$: $X_3$ dominates (it gates the tail-driving term).
>
> QOSA is the only method in this library that can distinguish median-sensitive from tail-sensitive factors on the same model.

> **Caveat:** The partition-based estimator trades the original paper's kernel-conditional-quantile fit for a piecewise-constant class approximation. The two converge asymptotically; at finite $N$ the partition variant's bias is bounded by class-mean variance. Use $N \geq 1024$ and $\alpha$ not too close to 0 or 1 (the tail shrinks to $N(1 - \alpha)$ effective samples). If $\text{CTE}_\alpha(Y) \approx \bar{Y}$, the denominator vanishes and the estimator returns `QosaError::DegenerateTail`.

---

## Choosing among distribution-based methods

| Method       | Measures                     | Internals             | Best for                                          |
|--------------|------------------------------|-----------------------|---------------------------------------------------|
| Borgonovo $\delta$ | Full density shift     | KDE + trapezoidal integration | Detecting any distributional change, including shape and modality. |
| PAWN         | CDF shift (KS distance)     | Empirical CDFs, no tuning | Robust ranking with small samples; no density estimation needed. |
| QOSA         | Quantile-specific sensitivity | Partition + CTE       | Tail-focused analysis: which factor drives the 95th percentile?    |

If you want a single distribution-based index and don't care about tails specifically, start with **PAWN** — it is the simplest to interpret and the most robust to sample size. Use **Borgonovo $\delta$** when you need sensitivity to density shape changes that PAWN's CDF comparison might miss. Use **QOSA** when the question is explicitly about a quantile or tail.

All three methods complement variance-based Sobol' indices rather than replacing them. Run Sobol' first for variance decomposition, then distribution-based methods if you suspect factors that matter for risk or distributional shape but not variance.
