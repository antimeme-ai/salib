Feature: D-study budget optimizer

  The D-study optimizer finds the cheapest measurement design that meets
  a target reliability coefficient. It extends the existing D-study
  projection from a four-point surface to an arbitrary grid, and adds
  constrained optimization with a user-supplied cost function.

  Scenario: D-study surface over item/rater grid
    Given G-theory variance components from a pilot study
    When I compute D-study surface for n_items in [2,4,8,16] and n_raters in [1,2,3,5]
    Then the surface has 16 points
    And Phi increases monotonically with n_items at fixed n_raters
    And Phi increases monotonically with n_raters at fixed n_items

  Scenario: find minimum design for target Phi
    Given G-theory variance components from a pilot study
    And a target Phi >= 0.80
    And a cost function cost(n_items, n_raters) = n_items * n_raters
    When I find the minimum design
    Then the result has Phi >= 0.80
    And no cheaper design with Phi >= 0.80 exists in the search grid

  Scenario: no feasible design returns None
    Given G-theory variance components from a pilot study
    And a target Phi >= 0.999
    And a search grid of max_items=5 max_raters=5
    When I find the minimum design
    Then no feasible design is found

  Scenario: minimum design at boundary
    Given G-theory variance components from a pilot study
    And a target Phi >= 0.50
    When I find the minimum design with max_items=20 max_raters=10
    Then the result has the smallest cost among all feasible designs
