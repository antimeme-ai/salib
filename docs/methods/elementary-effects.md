# Elementary Effects

OAT (one-at-a-time) trajectories through the input space. Screening: which factors matter, cheaply.

> **When to use:** You have many factors (10--100+) and a limited computational budget. Morris elementary effects tell you which factors are influential and which can be fixed without the cost of a full Sobol' analysis. Typical budget: $r \times (d + 1)$ model evaluations, with $r = 10$--$50$. Compare this to Sobol', which costs $N \times (d + 2)$ with $N$ typically 1024--16384.

---

## Morris 1991

Morris (1991) *Technometrics* 33(2), 161--174. [[bib]](../bibliography.md#morris1991)

The method constructs $r$ random trajectories through a $p$-level grid on the unit hypercube $[0, 1]^d$. Each trajectory is a sequence of $d + 1$ points where consecutive points differ in exactly one coordinate (the OAT property). At each step, a single factor $x_i$ is perturbed by a fixed step $\Delta$, and the resulting change in output defines the **elementary effect** for that factor at that base point:

$$EE_i = \frac{f(x_1, \ldots, x_i + \Delta, \ldots, x_d) - f(\mathbf{x})}{\Delta}$$

The step size follows Saltelli's convention:

$$\Delta = \frac{p}{2(p - 1)}$$

For $p = 4$ (the default), $\Delta = 2/3$. Base points are drawn from the lower half of the grid so that adding $\Delta$ stays on a grid point.

Each trajectory visits each factor exactly once. With $r$ trajectories, every factor accumulates $r$ elementary effects. Three statistics are computed per factor $i$:

$$\mu_i = \frac{1}{r} \sum_{k=1}^{r} EE_i^{(k)}$$

$$\mu_i^* = \frac{1}{r} \sum_{k=1}^{r} \lvert EE_i^{(k)} \rvert$$

$$\sigma_i = \sqrt{\frac{1}{r - 1} \sum_{k=1}^{r} (EE_i^{(k)} - \mu_i)^2}$$

$\mu_i^*$ was introduced by Campolongo et al. (2007) [[bib]](../bibliography.md#campolongo2007) to fix a cancellation problem with the signed mean $\mu_i$: for non-monotonic factors, positive and negative elementary effects average toward zero even when the factor is influential. $\mu_i^*$ uses absolute values and is the modern standard for screening.

$\sigma_i$ indicates the degree of non-linearity or interaction effects. A factor with high $\mu_i^*$ and low $\sigma_i$ has a strong, approximately linear, additive effect. A factor with high $\sigma_i$ relative to $\mu_i^*$ participates in interactions or has a strongly non-linear effect.

### Sampling

```rust
use salib::samplers::build_morris_trajectories;
use salib::*;

let mut rng = RngState::from_seed([0u8; 32]);

// d=8 factors, r=20 trajectories, p=4 grid levels
let trajectories = build_morris_trajectories(
    8, 20, 4, &mut rng
).unwrap();
// Total cost: 20 * (8 + 1) = 180 model evaluations.
```

Grid levels (`p`) must be even (Saltelli convention). `p = 4` is the standard choice; `p = 8` gives finer resolution at the cost of a denser grid but does not change the number of model evaluations.

### Estimation

```rust
use salib::estimators::estimate_morris_effects;

let effects = estimate_morris_effects(&trajectories, |x| {
    // Ishigami function
    x[0].sin()
        + 7.0 * x[1].sin().powi(2)
        + 0.1 * x[2].powi(4) * x[0].sin()
}).unwrap();

println!("{effects}");
// Prints μ, μ*, σ per factor.
```

> **Verify** against the additive-linear test function $Y = \sum_{i=1}^{d} i \cdot x_i$ ($d = 8$, $r = 50$, $p = 4$, seed `[0u8; 32]`):
>
> | Factor | $\mu_i$ | $\mu_i^*$ | $\sigma_i$ | Analytic |
> |--------|---------|-----------|------------|----------|
> | 0      | 1.0000  | 1.0000    | 0.0000     | $\mu = 1$, $\sigma = 0$ |
> | 1      | 2.0000  | 2.0000    | 0.0000     | $\mu = 2$, $\sigma = 0$ |
> | ...    | ...     | ...       | ...        | ... |
> | 7      | 8.0000  | 8.0000    | 0.0000     | $\mu = 8$, $\sigma = 0$ |
>
> For a purely linear function, elementary effects are constant across all trajectories: $EE_i = b_i$ regardless of the base point. The estimator recovers each coefficient exactly (to floating-point precision), and $\sigma_i = 0$. This is the degenerate case. For non-linear functions, $\sigma_i > 0$ and $\mu_i$ converges at rate $O(1/\sqrt{r})$.

> **Caveat:** The identity $\mu_i^* \geq |\mu_i|$ always holds (mean of absolute values is at least the absolute value of the mean). If your results violate this, there is a bug.

---

## Grouped Morris

Campolongo et al. (2007) *Env. Mod. Soft.* 22(10), 1509--1518. [[bib]](../bibliography.md#campolongo2007)

When inputs are naturally grouped (all parameters of a subsystem, all coefficients of one physical process), grouped Morris perturbs all factors in a group simultaneously rather than one at a time. Instead of $d + 1$ points per trajectory, there are $n_{\text{groups}} + 1$ points. Each step moves every factor in the selected group by $\Delta$.

The elementary effect for a group $g$ is:

$$EE_g = \frac{f(\mathbf{x}_{\text{after}}) - f(\mathbf{x}_{\text{before}})}{\Delta}$$

where the step perturbs all $|g|$ factors in the group. The same $\mu$, $\mu^*$, $\sigma$ statistics are computed per group.

Total cost: $r \times (n_{\text{groups}} + 1)$ model evaluations. When $n_{\text{groups}} \ll d$, this is substantially cheaper than ungrouped Morris.

### Sampling

```rust
use salib::samplers::build_grouped_morris_trajectories;
use salib::*;

let groups = vec![
    Group { name: "subsystem_A".into(), factor_indices: vec![0, 1] },
    Group { name: "subsystem_B".into(), factor_indices: vec![2, 3] },
];

let mut rng = RngState::from_seed([0u8; 32]);

// d=4 total factors, r=50 trajectories, p=4 levels
let trajectories = build_grouped_morris_trajectories(
    &groups, 4, 50, 4, &mut rng
).unwrap();
// Total cost: 50 * (2 + 1) = 150 evaluations (vs 50 * 5 = 250 ungrouped).
```

### Estimation

```rust
use salib::estimators::estimate_grouped_morris_effects;

let effects = estimate_grouped_morris_effects(
    &trajectories,
    &groups,
    |x| x[0] + x[1] + 3.0 * x[2] + 3.0 * x[3],
).unwrap();

// Per-group statistics
let grouped_mu_star = effects.grouped_mu_star
    .as_ref().unwrap();
let group_names = effects.group_names
    .as_ref().unwrap();
for (name, ms) in group_names.iter().zip(grouped_mu_star) {
    println!("{name}: μ* = {ms:.4}");
}
```

The output `MorrisEffects` contains both per-factor and per-group statistics. Per-factor effects in grouped Morris record the group-level elementary effect for each member factor (individual factor contributions cannot be separated when factors move simultaneously).

> **Verify:** With singleton groups (one factor per group), grouped Morris produces the same $\mu^*$ values as ungrouped Morris under the same seed.

> **Caveat:** Grouped Morris answers "does this subsystem matter?" not "which factor within the subsystem matters?" If a group screens as important, follow up with an ungrouped analysis on its member factors.

---

## Interpreting the ($\mu^*$, $\sigma$) plane

| Region | Interpretation |
|--------|---------------|
| High $\mu_i^*$, low $\sigma_i$ | Important factor, approximately linear and additive. |
| High $\mu_i^*$, high $\sigma_i$ | Important factor involved in interactions or with strong non-linearity. |
| Low $\mu_i^*$, low $\sigma_i$ | Non-influential factor. Candidate for fixing at its nominal value. |
| Low $\mu_i^*$, high $\sigma_i$ | Rare. Factor with nearly cancelling effects that vary with context. Investigate. |

The standard screening decision: factors with $\mu_i^*$ below some threshold are non-influential and can be excluded from a subsequent full analysis (e.g., Sobol').
