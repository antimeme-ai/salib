Feature: G-theory Gate 1 textbook reproduction

  Gate 1 validates variance-component estimation against a hand-verified
  synthetic dataset with known ANOVA decomposition. The dataset is a
  balanced 4 persons x 3 items x 2 raters crossed design.

  Data source: Hand-constructed synthetic dataset with analytically
  derived expected variance components via three-way ANOVA EMS
  (Expected Mean Squares) decomposition. See fixtures/gate1_pir_4x3x2.json
  for the data and full derivation.

  Scenario: Gate 1 variance components match hand-computed values
    Given the Gate 1 synthetic 4x3x2 p x i x r dataset
    When I estimate G-theory p x i x r components
    Then sigma_p matches the expected value within tolerance 0.0001
    And sigma_i matches the expected value within tolerance 0.0001
    And sigma_r matches the expected value within tolerance 0.0001
    And sigma_pi matches the expected value within tolerance 0.0001
    And sigma_pr matches the expected value within tolerance 0.0001
    And sigma_ir matches the expected value within tolerance 0.0001
    And sigma_pir matches the expected value within tolerance 0.0001
    And G matches the expected value within tolerance 0.0001
    And Phi matches the expected value within tolerance 0.0001

  Scenario: Gate 1 D-study projection at doubled design
    Given the Gate 1 variance components
    When I project D-study at n_items=6 and n_raters=4
    Then projected G exceeds the G-study G
    And projected Phi exceeds the G-study Phi
    And projected G matches the expected value within tolerance 0.0001
    And projected Phi matches the expected value within tolerance 0.0001
