# TCK — salib sensitivity analysis

Behavioral spec layer for the `salib-*` crates. Every architectural commitment lands as a Gherkin `.feature` file before code. The spec is intentionally implementation-language-agnostic so it survives reimplementation.

Harness: `salib-tck` (in `crates/salib-tck/`), a lightweight Gherkin parser + synchronous scenario runner.

## Sub-domain layout

One directory per architectural commitment, each containing a `features/` directory with `.feature` files:

```
tck/salib/
├── rng-determinism/
├── problem/
├── lhs-sampler/
├── sobol-sampler/
├── saltelli-matrix/
├── sobol-estimator/
├── morris-estimator/
├── fast-estimator/
├── fast-sampler/
├── borgonovo-estimator/
├── pawn-estimator/
├── dgsm-estimator/
├── regression-estimator/
├── given-data-sobol-estimator/
├── rbd-fast-estimator/
├── shapley-estimator/
├── pce-estimator/
├── sparse-pce-estimator/
├── active-subspace/
├── anova-estimator/
├── hdmr/
├── qosa-estimator/
├── iman-conover/
├── validation/
└── ...
```

Step definitions live in each salib crate's `tests/` directory (e.g., `crates/salib-estimators/tests/saltelli2010_tck.rs`), not under `tck/`.
