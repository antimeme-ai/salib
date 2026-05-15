# salib

Global sensitivity analysis for Rust, implemented from the primary literature.
Bit-deterministic by construction.

[crates.io](https://crates.io/crates/salib) · [API reference](https://docs.rs/salib) · [source](https://github.com/antimeme-ai/salib)

---

## Start here

- **[Quickstart](quickstart.md)** — Ishigami function, Saltelli sampling, Sobol' indices. Five minutes.
- **[Choosing a method](choosing.md)** — which estimator for which question.

## Methods

### Variance-based (Sobol')

Decompose output variance into contributions from each input.
The workhorse of global SA.

- [Saltelli 2010](methods/variance-based.md#saltelli-2010) — improved estimator for first-order and total-effect indices. The default choice.
- [Jansen 1999](methods/variance-based.md#jansen-1999) — total-effect estimator with better convergence for small samples.
- [Janon 2014](methods/variance-based.md#janon-2014) — asymptotically efficient first-order estimator.
- [Owen 2013](methods/variance-based.md#owen-2013) — three-matrix design for improved second-order estimates.
- [Given-data Sobol'](methods/variance-based.md#given-data-sobol) — Sobol' indices from observational data without designed experiments. Plischke et al. 2013.

### Elementary effects

OAT trajectories through the input space. Screening: which factors matter, cheaply.

- [Morris 1991](methods/elementary-effects.md#morris-1991) — mean and standard deviation of elementary effects. The original screening method.
- [Grouped Morris](methods/elementary-effects.md#grouped-morris) — Morris with factor groups. Campolongo et al. 2007.

### Frequency-based

Probe model response at characteristic frequencies per factor.

- [FAST / eFAST](methods/frequency.md#fast--efast) — Fourier Amplitude Sensitivity Test. First-order via spectral decomposition; extended variant adds total-effect. Cukier 1973, Saltelli 1999.
- [RBD-FAST](methods/frequency.md#rbd-fast) — Random Balance Designs. Reuses a single random sample for all factors. Tarantola et al. 2006.

### Distribution-based

Sensitivity beyond variance: shift in the entire output distribution.

- [Borgonovo δ](methods/distribution.md#borgonovo-delta) — moment-independent importance measure. Captures any distributional shift. Borgonovo 2007.
- [PAWN](methods/distribution.md#pawn) — CDF-based sensitivity via Kolmogorov–Smirnov statistic. Pianosi et al. 2015.
- [QOSA](methods/distribution.md#qosa) — quantile-oriented sensitivity analysis. Fort et al. 2016.

### Derivative-based

- [DGSM](methods/derivative.md#dgsm) — Derivative-based Global Sensitivity Measures. Upper bounds on total-effect indices from gradients. Sobol' & Kucherenko 2009.

### Regression

- [SRC / SRRC / PCC / PRCC](methods/regression.md) — standardized regression and partial correlation coefficients. Linear and rank-transformed. Saltelli & Marivoet 1990.

### Surrogate

Build a cheap approximation, extract indices analytically.

- [Polynomial Chaos Expansion](methods/surrogate.md#polynomial-chaos-expansion) — full OLS and sparse LARS/OMP. Analytic Sobol' indices from coefficients. Xiu & Karniadakis 2002, Blatman & Sudret 2011.
- [HDMR](methods/surrogate.md#hdmr) — High-Dimensional Model Representation. Cut-HDMR with PCE component functions. Li et al. 2002.
- [Active Subspaces](methods/surrogate.md#active-subspaces) — gradient-based dimension reduction. Eigendecomposition of the uncentered gradient covariance. Constantine 2015.

### Game-theoretic

- [Shapley Effects](methods/game-theoretic.md) — cost-sharing of output variance via coalitional game theory. Song, Nelson & Staum 2016.

### Experimental design

- [ANOVA](methods/experimental-design.md#anova) — two-way and three-way analysis of variance. Fisher 1925.
- [G-Theory](methods/experimental-design.md#g-theory) — generalizability theory D-study. Variance components for measurement designs. Brennan 2001.
- [Discrepancy](methods/experimental-design.md#discrepancy) — L2-star discrepancy of point sets. Hickernell 1998.
- [Fractional Factorial](methods/experimental-design.md#fractional-factorial) — main effects and interactions from designed experiments. Box, Hunter & Hunter 1978.

## Reference

- **[Bibliography](bibliography.md)** — annotated references for every method.
- **[Internals](internals.md)** — bit-determinism, tree-structured reductions, the rayon contract.
- **[Crate map](crates.md)** — which crate owns what, dependency graph, feature flags.
