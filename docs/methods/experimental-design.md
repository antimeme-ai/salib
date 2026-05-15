# Experimental Design Methods

Classical design-of-experiments instruments — variance decomposition, measurement reliability, space-filling quality, and screening designs.

> **When to use:** You have a structured experimental layout (balanced grid, crossed facets, factorial design) or need to evaluate the quality of a sampling plan. These methods operate on observed data arranged in known designs, not on black-box model evaluations with random sampling.

---

## ANOVA

Fisher (1925) *Statistical Methods for Research Workers*. [[bib]](../bibliography.md#fisher1925)

### Theory

Analysis of variance partitions the total sum of squares of a balanced design into components attributable to each factor and their interactions. For a two-way layout with factors $A$ (rows, $a$ levels) and $B$ (columns, $b$ levels):

$$SS_{\text{total}} = SS_A + SS_B + SS_{AB} + SS_{\text{error}}$$

where $SS_A = b \sum_{i=1}^{a} (\bar{Y}_{i\cdot} - \bar{Y}_{\cdot\cdot})^2$ measures the row (factor $A$) effect, $SS_B = a \sum_{j=1}^{b} (\bar{Y}_{\cdot j} - \bar{Y}_{\cdot\cdot})^2$ measures the column (factor $B$) effect, and $SS_{AB}$ captures the interaction residual.

Each mean square $MS = SS / df$ is tested against an appropriate denominator via the $F$-statistic:

$$F_A = \frac{MS_A}{MS_{AB}}, \quad F_B = \frac{MS_B}{MS_{AB}}$$

with $p$-values from the Fisher-Snedecor distribution. A significant $F$-ratio means the factor's effect is larger than expected from the interaction noise alone.

The three-way extension adds factors $A \times B \times C$ with all two-way and three-way interaction terms. The implementation uses the three-way interaction $MS_{ABC}$ as the error denominator for all $F$-tests (unreplicated balanced design — no pure-error term exists).

### Code

```rust
use salib::estimators::estimate_anova_two_way;
use ndarray::arr2;

// 3 x 4 balanced grid: 3 levels of factor A, 4 levels of factor B
let grid = arr2(&[
    [23.0, 27.0, 21.0, 25.0],
    [30.0, 34.0, 28.0, 32.0],
    [18.0, 22.0, 16.0, 20.0],
]);

let result = estimate_anova_two_way(grid.view()).unwrap();
println!("{result}");
```

Three-way ANOVA:

```rust
use salib::estimators::estimate_anova_three_way;
use ndarray::Array3;

// A x B x C balanced grid
let grid = Array3::<f64>::from_shape_fn((3, 4, 2), |(i, j, k)| {
    10.0 * i as f64 + 3.0 * j as f64 + 1.5 * k as f64
});

let result = estimate_anova_three_way(grid.view()).unwrap();
println!("{result}");
```

Bootstrap confidence intervals on variance fractions:

```rust
use salib::estimators::estimate_anova_two_way_with_bootstrap;
use salib::RngState;
use ndarray::arr2;

let grid = arr2(&[
    [23.0, 27.0, 21.0, 25.0],
    [30.0, 34.0, 28.0, 32.0],
    [18.0, 22.0, 16.0, 20.0],
]);
let mut rng = RngState::from_seed([0u8; 32]);

let result = estimate_anova_two_way_with_bootstrap(
    grid.view(), 2000, 0.05, &mut rng
).unwrap();
```

> **Note:** The implementation operates on unreplicated balanced grids. The residual sum of squares is identically zero — inferential $F$-tests use the interaction mean square as the denominator, following the classical unreplicated two-way design convention.

---

## G-Theory

Brennan (2001) *Generalizability Theory*, Springer. [[bib]](../bibliography.md#brennan2001)

### Theory

Generalizability theory extends classical reliability (Cronbach's alpha) to designs with multiple facets. A crossed $p \times i \times r$ design (persons $\times$ items $\times$ raters) decomposes the total variance into seven components:

$$\sigma^2_{\text{total}} = \sigma^2_p + \sigma^2_i + \sigma^2_r + \sigma^2_{pi} + \sigma^2_{pr} + \sigma^2_{ir} + \sigma^2_{pir}$$

Each component is estimated from the corresponding mean square via the standard EMS equations (Brennan 2001 Chapter 2). The two reliability coefficients are:

**Generalizability coefficient** (relative decisions):

$$\rho^2 = \frac{\sigma^2_p}{\sigma^2_p + \sigma^2_\delta}$$

where $\sigma^2_\delta = \sigma^2_{pi}/n_i + \sigma^2_{pr}/n_r + \sigma^2_{pir}/(n_i \cdot n_r)$ is the relative error variance.

**Dependability coefficient** (absolute decisions):

$$\Phi = \frac{\sigma^2_p}{\sigma^2_p + \sigma^2_\Delta}$$

where $\sigma^2_\Delta$ includes all non-person variance components scaled by their facet sample sizes.

The D-study projects $\rho^2$ and $\Phi$ to hypothetical designs with different numbers of items and raters — answering "how many raters/items do I need to reach $\rho^2 \geq 0.80$?"

### Code

```rust
use salib::estimators::{estimate_g_theory_pir, GTheoryDesign};
use ndarray::Array3;

// p x i x r balanced grid: 10 persons, 5 items, 3 raters
let grid = Array3::<f64>::from_shape_fn((10, 5, 3), |(p, i, r)| {
    50.0 + 6.0 * p as f64 + 2.0 * i as f64 + 0.5 * r as f64
});

let result = estimate_g_theory_pir(grid.view(), GTheoryDesign::Crossed).unwrap();
println!("{result}");
```

D-study projection:

```rust
use salib::estimators::project_g_theory_d_study;

// What if we used 8 items and 4 raters instead?
let projected = project_g_theory_d_study(&result, 8, 4).unwrap();
println!("G = {:.4}, Phi = {:.4}", projected.g_coefficient, projected.phi_coefficient);
```

Bootstrap confidence intervals:

```rust
use salib::estimators::{estimate_g_theory_pir_with_bootstrap, GTheoryDesign};
use salib::RngState;

let mut rng = RngState::from_seed([0u8; 32]);
let result = estimate_g_theory_pir_with_bootstrap(
    grid.view(), GTheoryDesign::Crossed, 2000, 0.05, &mut rng
).unwrap();
```

> **When to use:** Measurement study design — deciding how many raters, items, or occasions are needed to achieve a target reliability. The D-study projection avoids costly pilot studies by extrapolating from a single G-study.

> **Scope:** Only the fully crossed $p \times i \times r$ design is implemented. Nested and mixed designs are planned for a future release.

---

## Discrepancy

Hickernell (1998) in *Monte Carlo and Quasi-Monte Carlo Methods 1996*, Springer. [[bib]](../bibliography.md#hickernell1998)

### Theory

Discrepancy measures how uniformly a point set fills the unit hypercube $[0, 1]^d$. Lower discrepancy implies better space-filling and, by the Koksma-Hlawka inequality, lower quasi-Monte Carlo integration error:

$$\left|\int_{[0,1]^d} f(\mathbf{x})\,d\mathbf{x} - \frac{1}{N}\sum_{i=1}^{N} f(\mathbf{x}_i)\right| \leq D^*(P_N) \cdot V(f)$$

where $D^*(P_N)$ is the star discrepancy of the point set and $V(f)$ is the variation of $f$ in the sense of Hardy and Krause.

Four discrepancy measures are computed simultaneously:

| Measure | Description |
|---------|-------------|
| **Centered (CD)** | Hickernell 1998 Eq 3.8. Symmetric around 0.5. |
| **Wrap-around (WD)** | Hickernell 1998 Eq 3.10. Invariant to coordinate shifts modulo 1. |
| **Modified (MD)** | Fang et al. 2006. Penalizes boundary clustering more heavily. |
| **L2-star** | Niederreiter 1992. The classical L2 star discrepancy. |

### Code

```rust
use salib::estimators::compute_discrepancy;
use ndarray::Array2;

// Evaluate a 100-point Sobol' sequence in 3 dimensions
let sample = Array2::<f64>::from_shape_fn((100, 3), |(i, j)| {
    // (placeholder — use your actual sample matrix)
    ((i * 7 + j * 13) % 100) as f64 / 100.0
});

let result = compute_discrepancy(sample.view()).unwrap();
println!("{result}");
```

> **Use case:** Evaluate and compare the quality of sampling designs — LHS vs Sobol' sequence vs Halton sequence vs random. Lower discrepancy values indicate better space-filling. All input values must lie in $[0, 1]$; rescale if needed.

---

## Fractional Factorial

Box, Hunter & Hunter (1978) *Statistics for Experimenters*, Wiley. [[bib]](../bibliography.md#box1978)

### Theory

A $2^{k-p}$ fractional factorial runs a fraction of the full factorial design to estimate main effects and (when resolution permits) two-factor interactions. The implementation uses Plackett-Burman designs — a family of two-level screening designs where $N$ runs (a multiple of 4) can screen up to $N - 1$ factors.

For each factor $i$, the main effect is the difference in mean response between the high (+1) and low (-1) levels:

$$\text{effect}_i = \bar{Y}_{x_i = +1} - \bar{Y}_{x_i = -1}$$

The coded levels $\{-1, +1\}$ are mapped to the physical factor bounds via $x_{\text{phys}} = \text{lo} + \frac{x_{\text{coded}} + 1}{2}(\text{hi} - \text{lo})$.

### Code

```rust
use salib::estimators::estimate_fractional_factorial;
use salib::samplers::build_plackett_burman;
use salib::*;

let problem = ProblemBuilder::new()
    .factor("x1", Distribution::Uniform { lo: 0.0, hi: 1.0 })
    .factor("x2", Distribution::Uniform { lo: 0.0, hi: 1.0 })
    .factor("x3", Distribution::Uniform { lo: 0.0, hi: 1.0 })
    .build()
    .unwrap();

let design = build_plackett_burman(problem.dim()).unwrap();

let effects = estimate_fractional_factorial(&design, &problem, |x| {
    3.0 * x[0] + 1.0 * x[1] + 0.5 * x[2]
});

println!("{effects}");
```

> **When to use:** Classical DoE screening — identify which factors have large main effects using the fewest runs possible. Most useful when you have a physical experiment (not just a computer model) and runs are expensive. For computer models with cheap evaluations, variance-based methods (Sobol') or Morris screening are usually preferable.

---

## Choosing among experimental design methods

| Method | Input | Output | Best for |
|--------|-------|--------|----------|
| ANOVA | Balanced grid | Variance fractions + $F$-tests | Factor significance in structured experiments. |
| G-Theory | Crossed $p \times i \times r$ grid | Variance components + $\rho^2$, $\Phi$ | Measurement reliability, D-study projections. |
| Discrepancy | Point set in $[0, 1]^d$ | CD, WD, MD, L2* | Evaluating space-filling quality of sampling plans. |
| Fractional Factorial | Plackett-Burman design | Main effects | Screening with minimal runs in physical experiments. |
