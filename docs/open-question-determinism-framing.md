# Open question: how bit-determinism is framed (2026-05-21)

- **Status**: open — flagged in an antimeme-stack architecture review,
  parked here for a dedicated pass. Not yet acted on.
- **Scope**: framing and documentation only. No engineering is to be removed.

## The observation

salib leads with bit-determinism as a headline property — `README.md` line 4
("Bit-deterministic by construction"), positioned beside "implemented from the
primary literature" as a co-equal virtue.

Bit-determinism (tree-structured parallel reduction, identical `RngState` →
identical output regardless of thread count) is real, correct, well-built
engineering. The question is **not** whether to keep it — keep it — but whether
its current *framing* is honest about what it buys.

It does **not** improve the accuracy of a sensitivity estimate. The accuracy of
a Sobol' index is governed by estimator choice, sample size, and the QMC
sequence — not by whether the parallel fold is bit-reproducible.
Bit-determinism is a **reproducibility / testability / debuggability**
property:

- it makes frozen reference-value tests possible (CI can pin an exact value);
- it makes regression detection exact ("did this refactor move the numbers");
- it makes bug bisection deterministic.

Those are worth having. But framed as a top-line *scientific* virtue it invites
a misuse: a downstream consumer treating bit-stability as *result*-stability.
Concretely — mojave feeds `salib-estimators` eval outputs (LLM-judge scores,
stochastic sampling). The Monte Carlo estimator's bit-reproducibility sits far
below the sampling-noise floor of the model under analysis. Two runs are not
comparable just because each is internally bit-stable.

## The reframe to consider

Do not remove the property — **re-shelve it**:

1. Move bit-determinism out of the headline. The headline claim should be the
   scientific one: GSA implemented from the primary literature and validated
   against closed-form analytic indices (`salib-validation`). Bit-determinism
   becomes a listed *engineering guarantee* under a "Reproducibility" or
   "Testing guarantees" heading, described as what it actually buys (above).
2. State the noise-floor boundary explicitly: bit-determinism is a property of
   *salib's computation*, and it sits below the statistical (Monte Carlo) error
   of any estimate. Document that two estimates are comparable on statistical
   grounds, never on bit-identity grounds.
3. In the TCK, classify `tree_fold_invariance` / `multi_stream_chacha` as the
   *reproducibility / regression* tier — distinct from the *correctness vs.
   literature* tier (`salib-validation` analytic indices). Both are legitimate;
   they answer different questions. (Ties into the org-wide TCK
   conformance-tier ADR — `antimeme-ai/.github`,
   `docs/adr/0001-tck-conformance-tiers.md`.)

## Prompt for the pick-up session

- Audit every place "deterministic" / "bit-deterministic" appears (`README.md`,
  `docs/index.md`, `docs/internals.md`, crate-level docs) and re-classify each
  mention as either a scientific claim (it is not) or a reproducibility
  guarantee (it is).
- Draft the `README.md` reframe per (1)–(2) above.
- Decide whether the noise-floor boundary deserves its own short doc
  (`docs/reproducibility.md`) or a section in `docs/internals.md`.
- Confirm `salib-validation`'s reference-value tests read as the *primary*
  correctness gate and the determinism tests read as clearly secondary.
