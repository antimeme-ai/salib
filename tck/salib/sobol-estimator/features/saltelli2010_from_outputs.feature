Feature: Saltelli2010 estimation from pre-computed outputs

  Saltelli2010 indices can be computed from pre-evaluated model outputs
  (fa, fb, fab arrays) without requiring a model function. This supports
  pipelines where model evaluation happens externally (e.g., LLM evals
  on GPU infrastructure) and Sobol analysis runs post-hoc on cached
  outputs.

  Scenario: from_outputs matches model-evaluated estimation
    Given a SaltelliMatrix with N=256 and d=2
    And a linear model Y = X_0 + 2*X_1
    When I compute indices via estimate_saltelli2010
    And I extract fa/fb/fab from the same matrix and model
    And I compute indices via estimate_saltelli2010_from_outputs
    Then both SobolIndices are bit-identical

  Scenario: bootstrap from_outputs matches model-evaluated bootstrap
    Given a SaltelliMatrix with N=256 and d=2
    And a linear model Y = X_0 + 2*X_1
    When I compute CIs via estimate_saltelli2010_with_bootstrap with B=200
    And I extract fa/fb/fab from the same matrix and model
    And I compute CIs via estimate_saltelli2010_from_outputs_with_bootstrap with B=200
    Then both SobolIndicesWithCi are bit-identical

  Scenario: from_outputs handles constant output gracefully
    Given fa/fb/fab arrays where all values equal 0.5
    When I compute indices via estimate_saltelli2010_from_outputs
    Then total_variance is approximately zero
    And all S1 and ST indices are zero
