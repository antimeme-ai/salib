# Frequency-Based Methods

Spectral sensitivity analysis -- decompose output variance via Fourier analysis of search curves or permuted samples.

> **When to use:** You want first-order (and optionally total-effect) indices but prefer spectral methods over Monte Carlo variance decomposition. FAST/eFAST costs $d \times N$ evaluations. RBD-FAST costs only $N$ -- use it when the budget is tight and first-order is enough.

---

## Theory

Cukier et al. (1973) *J. Chem. Phys.* 59(8), 3873--3878. [[bib]](../bibliography.md#cukier1973)

FAST assigns each factor a characteristic frequency $\omega_i$ and samples the model along a search curve in the input space. The curve sweeps a single parameter $s$ through $[0, 2\pi)$ and converts it to a $d$-dimensional input via

$$x_j(s) = \tfrac{1}{2} + \tfrac{1}{\pi}\arcsin\!\big(\sin(\omega_j\, s + \varphi_j)\big)$$

where $\varphi_j \sim \mathrm{Uniform}[0, 2\pi)$ is a random phase shift. Because each factor oscillates at a different frequency, the model output's Fourier spectrum at frequency $\omega_i$ reveals $X_i$'s first-order contribution.

The one-sided power spectrum is $\mathrm{Sp}[k] = |Y[k]|^2 / N^2$ for $k \in [1, \lfloor N/2 \rfloor]$, where $Y[k]$ is the DFT of the model output at frequency $k$. The first-order index extracts variance at harmonics of $\omega_i$:

$$S_i = \frac{\sum_{p=1}^{M} \mathrm{Sp}[p\,\omega_i]}{\sum_{k=1}^{\lfloor N/2 \rfloor} \mathrm{Sp}[k]}$$

where $M$ is the harmonic truncation order (typically 4).

---

## FAST / eFAST

Saltelli, Tarantola & Chan (1999) *Technometrics* 41(1), 39–56. [[bib]](../bibliography.md#saltelli1999)

Classic FAST (Cukier 1973) computes first-order indices only. The extended variant (eFAST, Saltelli 1999) adds total-effect indices by partitioning the spectrum into "factor $i$" and "everything else" bands.

**First-order.** Factor $i$ gets the maximum frequency $\omega_{\max} = \lfloor(N-1)/(2M)\rfloor$. First-order variance is the spectral power at harmonics $\omega_{\max}, 2\omega_{\max}, \ldots, M\omega_{\max}$:

$$V_{1,i} = 2 \sum_{p=1}^{M} \mathrm{Sp}[p\,\omega_i]$$

**Total-effect.** The complementary variance collects all spectral power in the low-frequency band $[1, \lfloor\omega_i/2\rfloor]$, which excludes factor $i$'s harmonics:

$$V_{\sim i} = 2 \sum_{k=1}^{\lfloor\omega_i/2\rfloor} \mathrm{Sp}[k]$$

$$S_{Ti} = 1 - \frac{V_{\sim i}}{V}$$

where $V = 2\sum_{k=1}^{\lfloor N/2\rfloor} \mathrm{Sp}[k]$ is the total spectral variance.

**Frequency assignment.** The remaining $d - 1$ factors receive complementary frequencies bounded above by $\omega_{\max} / (2M)$ so their spectra do not overlap $\omega_{\max}$'s harmonic band. When $m = \lfloor\omega_{\max}/(2M)\rfloor \geq d - 1$, complementary frequencies are drawn from $\mathrm{linspace}(1, m, d-1)$; otherwise they cycle $1, 2, \ldots, m, 1, 2, \ldots$ (collisions are tolerable because the FFT bins remain well-separated).

**Cost.** One search curve per factor, $N$ points each -- $d \times N$ total model evaluations.

```rust
use salib::samplers::{build_fast_design, FastDesign};
use salib::estimators::estimate_fast;
use salib::RngState;

let mut rng = RngState::from_seed([0u8; 32]);
let design: FastDesign = build_fast_design(
    3,    // d = 3 factors
    1025, // N per factor
    4,    // harmonic order M
    &mut rng,
).unwrap();

let indices = estimate_fast(&design, |x| {
    x[0].sin()
        + 7.0 * x[1].sin().powi(2)
        + 0.1 * x[2].powi(4) * x[0].sin()
}).unwrap();

println!("{indices}");
```

> **Verify** against Ishigami closed-form ($N = 1025$, seed `[0u8; 32]`):
>
> | Factor | $\hat{S}_i$ | Analytic $S_i$ | $\hat{S}_{Ti}$ | Analytic $S_{Ti}$ |
> |--------|-------------|----------------|-----------------|-------------------|
> | $x_1$  | 0.312       | 0.314          | 0.539           | 0.558             |
> | $x_2$  | 0.444       | 0.442          | 0.489           | 0.442             |
> | $x_3$  | 0.020       | 0.000          | 0.241           | 0.244             |
>
> eFAST exhibits a known systematic bias on interaction-heavy functions like Ishigami. The $\sin(x_1) \cdot x_3^4$ term creates spectral content at sums and differences of $\omega_1$ and $\omega_3$ that aliases into both factors' harmonic bands. The result: $S_3$ shows a small false signal, $S_{T_1}$ underestimates, and $S_{T_2}$ overestimates. This is bias, not MC noise -- it persists as $N$ grows.

> **Sample size:** $N$ must satisfy $N \geq 4M^2 + 1$ (i.e. $N \geq 65$ at $M = 4$). Use $N \geq 257$ for stable estimates; $N = 1025$ for publication-grade results with $d \leq 10$.

---

## RBD-FAST

Tarantola et al. (2006) *Rel. Eng. Sys. Safety* 91(6), 717--727. [[bib]](../bibliography.md#tarantola2006)
Plischke (2010) *Rel. Eng. Sys. Safety* 95(4), 354–360. [[bib]](../bibliography.md#plischke2010)

Key insight: use a single random sample for all factors instead of one search curve per factor.

For each factor $i$, sort the sample rows by $X_i$ and apply the permutation to $Y$. Sorting creates a "fundamental frequency 1" pattern in the reordered output for any function of $x_i$ alone. The FAST spectral analysis then extracts first-order variance from the low-frequency band:

$$V_{1,i} = 2 \sum_{k=1}^{M} \mathrm{Sp}[k]$$

where $\mathrm{Sp}$ is the power spectrum of the permuted $Y$ vector.

**Plischke bias correction.** The naive estimate $\hat{S}_{\mathrm{naive}} = V_{1,i}/V$ overestimates small effects. The 2010 correction subtracts the expected bias:

$$\lambda = \frac{2M}{N}, \qquad S_i = \frac{\hat{S}_{\mathrm{naive}} - \lambda}{1 - \lambda}$$

The corrected estimate can be slightly negative for true-zero factors -- that is a feature of unbiased estimation, not a bug.

**First-order only.** RBD-FAST does not produce total-effect indices. The permutation-based spectral landscape does not cleanly separate the complementary band needed for $S_{Ti}$.

**Cost.** $N$ evaluations total (not $d \times N$). The input matrix can come from any sampler -- LHS, Sobol' sequence, or user-provided data.

```rust
use salib::samplers::{LhsSampler, Sampler};
use salib::estimators::estimate_rbd_fast;
use salib::RngState;

let mut rng = RngState::from_seed([0u8; 32]);
let sampler = LhsSampler::classic(3);
let x = sampler.unit_sample(4096, &mut rng);

// Map [0, 1]^3 to [-pi, pi]^3 and evaluate Ishigami
let y: Vec<f64> = (0..4096).map(|i| {
    let u = [x[[i, 0]], x[[i, 1]], x[[i, 2]]];
    let v: Vec<f64> = u.iter()
        .map(|&u| -std::f64::consts::PI + 2.0 * std::f64::consts::PI * u)
        .collect();
    v[0].sin() + 7.0 * v[1].sin().powi(2) + 0.1 * v[2].powi(4) * v[0].sin()
}).collect();

let indices = estimate_rbd_fast(x.view(), &y, 10).unwrap();
println!("{indices}");
```

> **Verify** against Ishigami closed-form ($N = 4096$, $M = 10$, seed `[0u8; 32]`):
>
> | Factor | $\hat{S}_i$ | Analytic $S_i$ |
> |--------|-------------|----------------|
> | $x_1$  | 0.326       | 0.314          |
> | $x_2$  | 0.467       | 0.442          |
> | $x_3$  | -0.002      | 0.000          |

> **Caveat:** RBD-FAST uses $M = 10$ by default (vs $M = 4$ for eFAST) because the permutation creates a different spectral landscape than the search curve. The minimum sample size is $N \geq 2M + 1$.

---

## Choosing between methods

| Method    | $S_i$ | $S_{Ti}$ | Cost               | Best for                                                  |
|-----------|--------|----------|--------------------|------------------------------------------------------------|
| eFAST     | yes    | yes      | $d \times N$       | Both indices via spectral decomposition.                   |
| RBD-FAST  | yes    | no       | $N$                | First-order only, tight budget, any existing sample design. |

If you need total-effect indices, use eFAST (or variance-based Sobol' methods, which cost $(d+2) \times N$ but avoid FAST's harmonic-aliasing bias). If you only need first-order indices and have an existing sample set or want the cheapest possible design, use RBD-FAST.
