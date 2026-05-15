# Choosing a Method

Which sensitivity analysis method for which question.

---

## The short version

| Situation | Method | Page |
|-----------|--------|------|
| General-purpose, black-box model | **Saltelli 2010** | [variance-based](methods/variance-based.md) |
| Many factors, limited budget (screening) | **Morris** | [elementary effects](methods/elementary-effects.md) |
| Need interactions + total effects | **Saltelli 2010** or **Jansen** | [variance-based](methods/variance-based.md) |
| Very tight budget, first-order only | **RBD-FAST** | [frequency](methods/frequency.md) |
| Care about distributional shift, not just variance | **Borgonovo δ** or **PAWN** | [distribution](methods/distribution.md) |
| Tail risk / exceedance probability | **QOSA** | [distribution](methods/distribution.md) |
| Gradients available, want cheap screening | **DGSM** | [derivative](methods/derivative.md) |
| Model is approximately linear | **SRC / SRRC** | [regression](methods/regression.md) |
| Want analytic indices from a surrogate | **PCE** | [surrogate](methods/surrogate.md) |
| Correlated inputs | **Shapley effects** | [game-theoretic](methods/game-theoretic.md) |
| Observational data, can't re-run model | **Given-data Sobol'** | [variance-based](methods/variance-based.md) |
| Physical experiment (not computer model) | **Fractional factorial** or **ANOVA** | [experimental design](methods/experimental-design.md) |

## Decision tree

Start here and follow the branches.

**Can you run the model?**

- **No** → you have observational data only.
  - Inputs independent? → [Given-data Sobol'](methods/variance-based.md#given-data-sobol)
  - Inputs correlated? → [Shapley effects](methods/game-theoretic.md)

- **Yes** → how many evaluations can you afford?

  - **Very few** ($< 100$) → gradients available?
    - Yes → [DGSM](methods/derivative.md)
    - No → [SRC/SRRC](methods/regression.md) on whatever data you have

  - **Moderate** ($r \times (d+1)$, hundreds to low thousands) →
    [Morris screening](methods/elementary-effects.md). Identifies which factors matter, cheaply.

  - **Many** (thousands+) → what do you want to measure?
    - Variance contribution → [Saltelli 2010](methods/variance-based.md#saltelli-2010) or [FAST/eFAST](methods/frequency.md)
    - Distributional shift → [Borgonovo δ](methods/distribution.md#borgonovo-delta) or [PAWN](methods/distribution.md#pawn)
    - Tail behavior → [QOSA](methods/distribution.md#qosa)
    - Analytic indices from a surrogate → [PCE](methods/surrogate.md#polynomial-chaos-expansion)

## Cost comparison

All costs in terms of model evaluations, where $d$ is the number of factors and $N$ is the base sample size.

| Method | Total evaluations | First-order | Total-effect | Interactions |
|--------|-------------------|-------------|--------------|--------------|
| Saltelli 2010 | $N(d + 2)$ | yes | yes | via $S_{Ti} - S_i$ |
| Jansen | $N(d + 2)$ | yes | yes | via $S_{Ti} - S_i$ |
| Janon | $N(d + 2)$ | yes | yes | via $S_{Ti} - S_i$ |
| Owen | $N(d + 2)$ | yes | yes | $S_{ij}$ directly |
| Morris | $r(d + 1)$ | $\mu^*$ (screening) | no | $\sigma$ (screening) |
| FAST/eFAST | $dN$ | yes | yes (eFAST) | no |
| RBD-FAST | $N$ | yes | no | no |
| Borgonovo δ | $N$ (+ KDE) | moment-independent | no | no |
| PAWN | $N$ (+ conditioning) | CDF-based | no | no |
| DGSM | $N(d + 1)$ (finite diff) | upper bound on $S_T$ | bound | no |
| SRC/SRRC | $N$ (OLS) | linear approx | no | no |
| PCE | $N$ (training) | analytic | analytic | analytic |
| Shapley | $N \times 2^d$ subsets | full attribution | implicit | implicit |

## Common workflows

### "I have no idea which factors matter"

Start with **Morris screening** ($r = 20$, $p = 4$). It costs $20 \times (d + 1)$ evaluations and tells you which factors to investigate further. Then run **Saltelli 2010** on the important factors only, with $N = 4096$ or more.

### "I need publication-quality Sobol' indices"

Use **Saltelli 2010** with $N = 8192$ or $N = 16384$. Report both $S_i$ and $S_{Ti}$. If $\sum S_i < 0.9$, interactions are significant — consider reporting $S_{ij}$ via **Owen**.

Bootstrap confidence intervals: resample the $A$, $B$, $A_B^{(i)}$ output vectors and recompute the estimator. 1000 bootstrap replicates is standard.

### "My model takes hours per evaluation"

Build a **PCE surrogate** from a small training set ($N = 2d$ to $5d$ with sparse LARS). Extract analytic Sobol' indices from the polynomial coefficients. Verify against a handful of direct Saltelli estimates to confirm the surrogate is adequate.

### "My inputs are correlated"

Sobol' indices assume independence — they are ambiguous when inputs are dependent. Use **Shapley effects**, which provide a unique, complete variance attribution regardless of input dependence.
