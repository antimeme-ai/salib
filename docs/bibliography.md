# Bibliography

Annotated references for every method implemented in salib, organized by family. Each entry links back to the method page that uses it.

---

## Variance-based (Sobol')

<a id="sobol1993"></a>
**Sobol' (1993)**
Sobol', I. M. "Sensitivity estimates for nonlinear mathematical models." *Mathematical Modelling and Computational Experiments* 1(4), 407–414.
The foundational paper. Introduces the ANOVA-like functional decomposition of a square-integrable function into terms of increasing dimensionality, and defines the variance-based sensitivity indices $S_i$ and $S_{Ti}$ that all subsequent estimators target.

<a id="saltelli2010"></a>
**Saltelli et al. (2010)** → [method page](methods/variance-based.md#saltelli-2010)
Saltelli, A., Annoni, P., Azzini, I., Campolongo, F., Ratto, M., & Tarantola, S. "Variance based sensitivity analysis of model output. Design and estimator for the total sensitivity index." *Computer Physics Communications* 181(2), 259–270.
The improved estimator for both $S_i$ and $S_{Ti}$ that superseded Sobol' (2001). Uses the $f(B)(f(A_B^{(i)}) - f(A))$ formulation for first-order and the squared-difference formulation for total-effect. The default choice in most SA software.

<a id="jansen1999"></a>
**Jansen (1999)** → [method page](methods/variance-based.md#jansen-1999)
Jansen, M. J. W. "Analysis of variance designs for model output." *Computer Physics Communications* 117(1–2), 35–43.
Alternative total-effect estimator that compares $f(B)$ to $f(A_B^{(i)})$ rather than $f(A)$ to $f(A_B^{(i)})$. Empirically better convergence for small $N$ and models with high-order interactions.

<a id="janon2014"></a>
**Janon et al. (2014)** → [method page](methods/variance-based.md#janon-2014)
Janon, A., Klein, T., Lagnoux, A., Nodet, M., & Prieur, C. "Asymptotic normality and efficiency of two Sobol index estimators." *ESAIM: Probability and Statistics* 18, 342–364.
Proves that their estimator achieves the semiparametric efficiency bound for $S_i$ — no other estimator from the same Saltelli design can have lower asymptotic variance.

<a id="owen2013"></a>
**Owen (2013)** → [method page](methods/variance-based.md#owen-2013)
Owen, A. B. "Better estimation of small Sobol' sensitivity indices." *ACM Transactions on Modeling and Computer Simulation* 23(2), Article 11.
Three-matrix design for improved second-order interaction estimates $S_{ij}$. Introduces a third base matrix to separate first-order and interaction effects more cleanly.

<a id="plischke2013"></a>
**Plischke et al. (2013)** → [method page](methods/variance-based.md#given-data-sobol)
Plischke, E., Borgonovo, E., & Smith, C. L. "Global sensitivity measures from given data." *European Journal of Operational Research* 226(3), 536–550.
Estimate first-order Sobol' indices from observational data by binning inputs and computing between-bin variance of conditional means. No designed experiment required.

<a id="homma1996"></a>
**Homma & Saltelli (1996)**
Homma, T., & Saltelli, A. "Importance measures in global sensitivity analysis of nonlinear models." *Reliability Engineering & System Safety* 52(1), 1–17.
Introduces the total-effect index $S_{Ti}$ as a complement to first-order indices. Shows that $S_{Ti} = 0$ is necessary and sufficient for $X_i$ to be non-influential.

---

## Elementary effects

<a id="morris1991"></a>
**Morris (1991)** → [method page](methods/elementary-effects.md#morris-1991)
Morris, M. D. "Factorial sampling plans for preliminary computational experiments." *Technometrics* 33(2), 161–174.
The original OAT screening design. Defines elementary effects on a $p$-level grid and proposes $r$ random trajectories to estimate $\mu_i$ and $\sigma_i$.

<a id="campolongo2007"></a>
**Campolongo et al. (2007)** → [method page](methods/elementary-effects.md#grouped-morris)
Campolongo, F., Cariboni, J., & Saltelli, A. "An effective screening design for sensitivity analysis of large models." *Environmental Modelling & Software* 22(10), 1509–1518.
Introduces $\mu_i^*$ (mean of absolute elementary effects) to fix the cancellation problem in Morris's signed mean. Also extends Morris to grouped factors.

---

## Frequency-based

<a id="cukier1973"></a>
**Cukier et al. (1973)** → [method page](methods/frequency.md#fast--efast)
Cukier, R. I., Fortuin, C. M., Shuler, K. E., Petschek, A. G., & Schaibly, J. H. "Study of the sensitivity of coupled reaction systems to uncertainties in rate coefficients. I. Theory." *Journal of Chemical Physics* 59(8), 3873–3878.
The original FAST method. Maps each input to a characteristic frequency and uses Fourier decomposition to extract first-order sensitivity from the model output's power spectrum.

<a id="saltelli1999"></a>
**Saltelli et al. (1999)** → [method page](methods/frequency.md#fast--efast)
Saltelli, A., Tarantola, S., & Chan, K. P.-S. "A quantitative model-independent method for global sensitivity analysis of model output." *Technometrics* 41(1), 39–56.
Extended FAST (eFAST): adds total-effect indices by computing the complementary variance at all non-$\omega_i$ frequencies.

<a id="tarantola2006"></a>
**Tarantola et al. (2006)** → [method page](methods/frequency.md#rbd-fast)
Tarantola, S., Gatelli, D., & Mara, T. A. "Random balance designs for the estimation of first order global sensitivity indices." *Reliability Engineering & System Safety* 91(6), 717–727.
RBD-FAST: uses a single random sample for all factors by permuting columns independently, reducing cost from $d \times N$ to $N$.

<a id="plischke2010"></a>
**Plischke (2010)**
Plischke, E. "An effective algorithm for computing global sensitivity indices (EASI)." *Reliability Engineering & System Safety* 95(4), 354–360.
Bias correction for RBD-FAST via the $\lambda = 2M/N$ bandwidth factor.

---

## Distribution-based

<a id="borgonovo2007"></a>
**Borgonovo (2007)** → [method page](methods/distribution.md#borgonovo-delta)
Borgonovo, E. "A new uncertainty importance measure." *Reliability Engineering & System Safety* 92(6), 771–784.
The $\delta$ moment-independent importance measure: the expected $L^1$ distance between conditional and unconditional output densities. Captures distributional shifts that variance-based indices miss.

<a id="pianosi2015"></a>
**Pianosi & Wagener (2015)** → [method page](methods/distribution.md#pawn)
Pianosi, F., & Wagener, T. "A simple and efficient method for global sensitivity analysis based on cumulative distribution functions." *Environmental Modelling & Software* 67, 1–11.
PAWN: CDF-based sensitivity using Kolmogorov–Smirnov distance between conditional and unconditional output CDFs. Simpler and more robust than kernel-density-based measures.

<a id="pianosi2018"></a>
**Pianosi & Wagener (2018)**
Pianosi, F., & Wagener, T. "Distribution-based sensitivity analysis from a generic input-output sample." *Environmental Modelling & Software* 108, 197–207.
Refinements to the PAWN methodology with improved conditioning strategies.

<a id="maumedeschamps2018"></a>
**Maume-Deschamps & Niang (2018)**
Maume-Deschamps, V., & Niang, I. "Estimation of quantile oriented sensitivity indices." *Statistics & Probability Letters* 134, 122–127.
Partition-based estimator for QOSA with convergence analysis.

<a id="fort2016"></a>
**Fort et al. (2016)** → [method page](methods/distribution.md#qosa)
Fort, J.-C., Klein, T., & Rachdi, N. "New sensitivity analysis subordinated to a contrast." *Communications in Statistics — Theory and Methods* 45(15), 4349–4364.
QOSA: quantile-oriented sensitivity analysis. Measures factor importance at a specific quantile level, capturing tail sensitivity that variance-based methods average away.

---

## Derivative-based

<a id="sobol-kucherenko2009"></a>
<a id="sobolkucherenko2009"></a>
**Sobol' & Kucherenko (2009)** → [method page](methods/derivative.md#dgsm)
Sobol', I. M., & Kucherenko, S. "Derivative based global sensitivity measures and their link with global sensitivity indices." *Mathematics and Computers in Simulation* 79(10), 3009–3017.
DGSM: defines $\nu_i = \mathbb{E}[(\partial f / \partial x_i)^2]$ and proves upper bounds linking $\nu_i$ to total-effect indices $S_{Ti}$. Cheap when gradients are available.

---

## Regression

<a id="saltelli-marivoet1990"></a>
<a id="saltellimarivoet1990"></a>
**Saltelli & Marivoet (1990)** → [method page](methods/regression.md)
Saltelli, A., & Marivoet, J. "Non-parametric statistics in sensitivity analysis for model output: A comparison of selected techniques." *Computational Statistics & Data Analysis* 9(1), 55–64.
SRC, SRRC, PCC, PRCC: regression-based sensitivity measures. SRC² approximates $S_i$ for linear models; rank-transformed versions (SRRC, PRCC) capture monotonic nonlinearity.

---

## Surrogate

<a id="xiu-karniadakis2002"></a>
<a id="xiukarniadakis2002"></a>
**Xiu & Karniadakis (2002)** → [method page](methods/surrogate.md#polynomial-chaos-expansion)
Xiu, D., & Karniadakis, G. E. "The Wiener-Askey polynomial chaos for stochastic differential equations." *SIAM Journal on Scientific Computing* 24(2), 619–644.
Generalized polynomial chaos: match orthogonal polynomial families to input distributions (Legendre for uniform, Hermite for Gaussian). Sobol' indices are analytic from the expansion coefficients.

<a id="blatman-sudret2011"></a>
<a id="blatmansudret2011"></a>
**Blatman & Sudret (2011)** → [method page](methods/surrogate.md#polynomial-chaos-expansion)
Blatman, G., & Sudret, B. "Adaptive sparse polynomial chaos expansion based on least angle regression." *Journal of Computational Physics* 230(6), 2345–2367.
Sparse PCE via LARS: automatic basis selection when the full polynomial basis has more terms than data points. Makes PCE practical in moderate-to-high dimensions.

<a id="li2001"></a>
<a id="li2002"></a>
**Li et al. (2001)** → [method page](methods/surrogate.md#hdmr)
Li, G., Rosenthal, C., & Rabitz, H. "High dimensional model representations." *Journal of Physical Chemistry A* 105(33), 7765–7777.
HDMR: decompose $f$ into a hierarchy of component functions (constant, univariate, bivariate, ...). Cut-HDMR evaluates components along cuts through a reference point.

<a id="constantine2015"></a>
**Constantine (2015)** → [method page](methods/surrogate.md#active-subspaces)
Constantine, P. G. *Active Subspaces: Emerging Ideas for Dimension Reduction in Parameter Studies.* SIAM, Philadelphia.
The monograph. Eigendecomposition of $C = \mathbb{E}[\nabla f \nabla f^T]$ reveals the directions in input space where the function varies most. Practical algorithms for gradient estimation, subspace identification, and dimension reduction.

---

## Game-theoretic

<a id="song2016"></a>
<a id="songnelsonstaum2016"></a>
**Song, Nelson & Staum (2016)** → [method page](methods/game-theoretic.md)
Song, E., Nelson, B. L., & Staum, J. "Shapley effects for global sensitivity analysis: Theory and computation." *SIAM/ASA Journal on Uncertainty Quantification* 4(1), 1060–1083.
Shapley values applied to sensitivity analysis. Unlike Sobol' indices, Shapley effects always sum to $\operatorname{Var}(Y)$ and handle correlated inputs without ambiguity.

---

## Experimental design

<a id="fisher1925"></a>
**Fisher (1925)** → [method page](methods/experimental-design.md#anova)
Fisher, R. A. *Statistical Methods for Research Workers.* Oliver & Boyd, Edinburgh.
The origin of analysis of variance. Two-way and higher-order ANOVA partition total variation into main effects, interactions, and error.

<a id="brennan2001"></a>
**Brennan (2001)** → [method page](methods/experimental-design.md#g-theory)
Brennan, R. L. *Generalizability Theory.* Springer, New York.
The standard reference for G-theory. Extends classical reliability theory to multiple sources of measurement error (facets). D-studies optimize measurement designs for target reliability.

<a id="hickernell1998"></a>
**Hickernell (1998)** → [method page](methods/experimental-design.md#discrepancy)
Hickernell, F. J. "A generalized discrepancy and quadrature error bound." *Mathematics of Computation* 67(221), 299–322.
L2-star discrepancy and its connection to quasi-Monte Carlo integration error via the Koksma–Hlawka inequality.

<a id="box1978"></a>
<a id="boxhunterhunter1978"></a>
**Box, Hunter & Hunter (1978)** → [method page](methods/experimental-design.md#fractional-factorial)
Box, G. E. P., Hunter, W. G., & Hunter, J. S. *Statistics for Experimenters.* Wiley, New York.
The textbook for designed experiments. Fractional factorial designs, confounding, resolution, and the analysis of main effects and interactions.

---

## General references

<a id="saltelli2008"></a>
**Saltelli et al. (2008)**
Saltelli, A., Ratto, M., Andres, T., Campolongo, F., Cariboni, J., Gatelli, D., Saisana, M., & Tarantola, S. *Global Sensitivity Analysis: The Primer.* Wiley, Chichester.
The comprehensive textbook. Covers all variance-based, screening, and moment-independent methods with worked examples and practical guidance. The standard reference for the field.

<a id="iooss2015"></a>
**Iooss & Lemaître (2015)**
Iooss, B., & Lemaître, P. "A review on global sensitivity analysis methods." In *Uncertainty Management in Simulation-Optimization of Complex Systems*, Operations Research/Computer Science Interfaces Series 59, 101–122. Springer.
Survey of the full landscape: variance-based, screening, moment-independent, metamodel-based, and derivative-based methods with a decision flowchart for method selection.
