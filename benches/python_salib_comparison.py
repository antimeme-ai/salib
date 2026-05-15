#!/usr/bin/env python3
"""
Run Python SALib's own tests with wall-clock timing.

This does NOT reimplement their test logic — it calls the same functions
with the same parameters from their test_regression.py and adds timing.
Every expected value and tolerance is copied verbatim from SALib's test suite.

Usage:
    pip install SALib numpy
    python benches/python_salib_comparison.py
    python benches/python_salib_comparison.py --json > benches/python_results.json

Source of truth: github.com/SALib/SALib/blob/main/tests/test_regression.py
"""

import argparse
import json
import sys
import time

import numpy as np

# SALib imports — same as their test_regression.py
from SALib.analyze import (
    delta,
    dgsm,
    fast,
    hdmr,
    morris as morris_analyze,
    rbd_fast,
    sobol,
)
from SALib.sample import (
    fast_sampler,
    latin,
    morris as morris_sample,
    saltelli,
)
from SALib.test_functions import Ishigami


# ── Problem definitions (from SALib's test fixtures) ────────────────

ISHIGAMI_PROBLEM = {
    "num_vars": 3,
    "names": ["x1", "x2", "x3"],
    "bounds": [[-np.pi, np.pi]] * 3,
}

LINEAR_1_PROBLEM = {
    "num_vars": 5,
    "names": ["x1", "x2", "x3", "x4", "x5"],
    "bounds": [[-1.0, 1.0]] * 5,
}


def linear_model_1(x):
    return x[:, 0] + x[:, 1] + x[:, 2] + x[:, 3] + x[:, 4]


# ── Expected values — VERBATIM from SALib test_regression.py ────────
# Tolerances: atol=5e-2, rtol=1e-1 for variance-based methods
#             atol=0, rtol=1e-5 for Morris (seed-deterministic)

EXPECTED = {
    "saltelli_ishigami": {
        "S1": [0.31, 0.44, 0.00],
        "ST": [0.55, 0.44, 0.24],
        "S2": [0.00, 0.25, 0.00],  # S2[0][1], S2[0][2], S2[1][2]
        "atol": 5e-2,
        "rtol": 1e-1,
    },
    "fast_ishigami": {
        "S1": [0.31, 0.44, 0.00],
        "ST": [0.55, 0.44, 0.24],
        "atol": 5e-2,
        "rtol": 1e-1,
    },
    "rbd_fast_ishigami": {
        "S1": [0.31, 0.44, 0.00],
        "atol": 5e-2,
        "rtol": 1e-1,
    },
    "delta_ishigami": {
        "delta": [0.210, 0.358, 0.155],
        "S1": [0.31, 0.44, 0.00],
        "atol": 5e-2,
        "rtol": 1e-1,
    },
    "dgsm_ishigami": {
        "dgsm": [2.229, 7.066, 3.180],
        "atol": 5e-2,
        "rtol": 1e-1,
    },
    "morris_ishigami": {
        "mu_star": [7.682808, 7.875, 6.256295],
        "atol": 0,
        "rtol": 1e-5,
    },
    "hdmr_ishigami": {
        "Sa": [0.31, 0.44, 0.00],
        "ST": [0.55, 0.44, 0.24],
        "atol": 5e-2,
        "rtol": 1e-1,
    },
    "hdmr_linear1": {
        "Sa": [0.20, 0.20, 0.20, 0.20, 0.20],
        "ST": [0.20, 0.20, 0.20, 0.20, 0.20],
        "atol": 5e-2,
        "rtol": 1e-1,
    },
}


def timed(fn, *args, **kwargs):
    t0 = time.perf_counter()
    result = fn(*args, **kwargs)
    elapsed = time.perf_counter() - t0
    return result, elapsed


# ── Run each test exactly as SALib's test_regression.py does ────────

def run_tests():
    results = []

    # Seeds from SALib's conftest: rng = handle_seed(12345), np.random.seed(123456)
    # We use their N=10000 for all methods.

    N = 10_000

    # ── Sobol/Saltelli on Ishigami ───────────────────────────────
    np.random.seed(123456)
    X, t_s = timed(saltelli.sample, ISHIGAMI_PROBLEM, N)
    Y = Ishigami.evaluate(X)
    Si, t_a = timed(sobol.analyze, ISHIGAMI_PROBLEM, Y)
    results.append({
        "method": "saltelli_ishigami",
        "N": N,
        "evals": len(Y),
        "sample_s": round(t_s, 4),
        "analyze_s": round(t_a, 4),
        "S1": Si["S1"].tolist(),
        "ST": Si["ST"].tolist(),
        "pass": bool(
            np.allclose(Si["S1"], EXPECTED["saltelli_ishigami"]["S1"], atol=5e-2, rtol=1e-1)
            and np.allclose(Si["ST"], EXPECTED["saltelli_ishigami"]["ST"], atol=5e-2, rtol=1e-1)
        ),
    })

    # ── FAST on Ishigami ─────────────────────────────────────────
    np.random.seed(123456)
    X, t_s = timed(fast_sampler.sample, ISHIGAMI_PROBLEM, N)
    Y = Ishigami.evaluate(X)
    Si, t_a = timed(fast.analyze, ISHIGAMI_PROBLEM, Y)
    results.append({
        "method": "fast_ishigami",
        "N": N,
        "evals": len(Y),
        "sample_s": round(t_s, 4),
        "analyze_s": round(t_a, 4),
        "S1": Si["S1"].tolist(),
        "ST": Si["ST"].tolist(),
        "pass": bool(
            np.allclose(Si["S1"], EXPECTED["fast_ishigami"]["S1"], atol=5e-2, rtol=1e-1)
            and np.allclose(Si["ST"], EXPECTED["fast_ishigami"]["ST"], atol=5e-2, rtol=1e-1)
        ),
    })

    # ── RBD-FAST on Ishigami ─────────────────────────────────────
    np.random.seed(123456)
    X, t_s = timed(latin.sample, ISHIGAMI_PROBLEM, N)
    Y = Ishigami.evaluate(X)
    Si, t_a = timed(rbd_fast.analyze, ISHIGAMI_PROBLEM, X, Y)
    results.append({
        "method": "rbd_fast_ishigami",
        "N": N,
        "evals": len(Y),
        "sample_s": round(t_s, 4),
        "analyze_s": round(t_a, 4),
        "S1": Si["S1"].tolist(),
        "pass": bool(
            np.allclose(Si["S1"], EXPECTED["rbd_fast_ishigami"]["S1"], atol=5e-2, rtol=1e-1)
        ),
    })

    # ── Delta on Ishigami ────────────────────────────────────────
    np.random.seed(123456)
    X, t_s = timed(latin.sample, ISHIGAMI_PROBLEM, N)
    Y = Ishigami.evaluate(X)
    Si, t_a = timed(delta.analyze, ISHIGAMI_PROBLEM, X, Y, num_resamples=10)
    results.append({
        "method": "delta_ishigami",
        "N": N,
        "evals": len(Y),
        "sample_s": round(t_s, 4),
        "analyze_s": round(t_a, 4),
        "delta": Si["delta"].tolist(),
        "S1": Si["S1"].tolist(),
        "pass": bool(
            np.allclose(Si["delta"], EXPECTED["delta_ishigami"]["delta"], atol=5e-2, rtol=1e-1)
            and np.allclose(Si["S1"], EXPECTED["delta_ishigami"]["S1"], atol=5e-2, rtol=1e-1)
        ),
    })

    # ── DGSM on Ishigami ────────────────────────────────────────
    np.random.seed(123456)
    X, t_s = timed(latin.sample, ISHIGAMI_PROBLEM, N)
    Y = Ishigami.evaluate(X)
    Si, t_a = timed(dgsm.analyze, ISHIGAMI_PROBLEM, X, Y)
    results.append({
        "method": "dgsm_ishigami",
        "N": N,
        "evals": len(Y),
        "sample_s": round(t_s, 4),
        "analyze_s": round(t_a, 4),
        "dgsm": Si["dgsm"].tolist(),
        "pass": bool(
            np.allclose(Si["dgsm"], EXPECTED["dgsm_ishigami"]["dgsm"], atol=5e-2, rtol=1e-1)
        ),
    })

    # ── Morris on Ishigami ───────────────────────────────────────
    np.random.seed(123456)
    X, t_s = timed(morris_sample.sample, ISHIGAMI_PROBLEM, N)
    Y = Ishigami.evaluate(X)
    Si, t_a = timed(morris_analyze.analyze, ISHIGAMI_PROBLEM, X, Y)
    results.append({
        "method": "morris_ishigami",
        "N": N,
        "evals": len(Y),
        "sample_s": round(t_s, 4),
        "analyze_s": round(t_a, 4),
        "mu_star": Si["mu_star"].tolist(),
        "pass": bool(
            np.allclose(Si["mu_star"], EXPECTED["morris_ishigami"]["mu_star"], atol=0, rtol=1e-5)
        ),
    })

    # ── HDMR on Ishigami ────────────────────────────────────────
    np.random.seed(123456)
    X, t_s = timed(latin.sample, ISHIGAMI_PROBLEM, N)
    Y = Ishigami.evaluate(X)
    Si, t_a = timed(
        hdmr.analyze,
        ISHIGAMI_PROBLEM, X, Y,
        maxorder=2, maxiter=100, m=4, K=1, R=N, alpha=0.95, lambdax=0.01,
    )
    results.append({
        "method": "hdmr_ishigami",
        "N": N,
        "evals": len(Y),
        "sample_s": round(t_s, 4),
        "analyze_s": round(t_a, 4),
        "Sa": Si["Sa"].tolist(),
        "ST": Si["ST"].tolist(),
        "pass": bool(
            np.allclose(Si["Sa"], EXPECTED["hdmr_ishigami"]["Sa"], atol=5e-2, rtol=1e-1)
            and np.allclose(Si["ST"], EXPECTED["hdmr_ishigami"]["ST"], atol=5e-2, rtol=1e-1)
        ),
    })

    # ── HDMR on Linear Model 1 ──────────────────────────────────
    np.random.seed(123456)
    X, t_s = timed(latin.sample, LINEAR_1_PROBLEM, N)
    Y = linear_model_1(X)
    Si, t_a = timed(
        hdmr.analyze,
        LINEAR_1_PROBLEM, X, Y,
        maxorder=2, maxiter=100, m=2, K=1, R=N, alpha=0.95, lambdax=0.01,
    )
    results.append({
        "method": "hdmr_linear1",
        "N": N,
        "evals": len(Y),
        "sample_s": round(t_s, 4),
        "analyze_s": round(t_a, 4),
        "Sa": Si["Sa"].tolist(),
        "ST": Si["ST"].tolist(),
        "pass": bool(
            np.allclose(Si["Sa"], EXPECTED["hdmr_linear1"]["Sa"], atol=5e-2, rtol=1e-1)
            and np.allclose(Si["ST"], EXPECTED["hdmr_linear1"]["ST"], atol=5e-2, rtol=1e-1)
        ),
    })

    return results


# ── Convergence sweep (not from SALib — they don't have this) ────

def run_convergence():
    """Saltelli on Ishigami at increasing N. SALib has no equivalent."""
    results = []
    S1_ref = [0.3139, 0.4424, 0.0000]
    ST_ref = [0.5576, 0.4424, 0.2437]
    for n in [128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536]:
        np.random.seed(0)
        X = saltelli.sample(ISHIGAMI_PROBLEM, n)
        Y = Ishigami.evaluate(X)
        t0 = time.perf_counter()
        Si = sobol.analyze(ISHIGAMI_PROBLEM, Y)
        t_a = time.perf_counter() - t0
        results.append({
            "N": n,
            "evals": len(Y),
            "analyze_s": round(t_a, 4),
            "S1": Si["S1"].tolist(),
            "ST": Si["ST"].tolist(),
            "S1_maxerr": round(float(np.max(np.abs(Si["S1"] - S1_ref))), 6),
            "ST_maxerr": round(float(np.max(np.abs(Si["ST"] - ST_ref))), 6),
        })
    return results


def main():
    parser = argparse.ArgumentParser(
        description="Run Python SALib's regression tests with timing"
    )
    parser.add_argument("--json", action="store_true")
    parser.add_argument("--convergence", action="store_true")
    args = parser.parse_args()

    if args.convergence:
        data = run_convergence()
        if args.json:
            json.dump(data, sys.stdout, indent=2)
        else:
            print(f"{'N':>6}  {'evals':>7}  {'time':>7}  {'S1 err':>8}  {'ST err':>8}")
            print("-" * 42)
            for r in data:
                print(
                    f"{r['N']:>6}  {r['evals']:>7}  {r['analyze_s']:>6.3f}s"
                    f"  {r['S1_maxerr']:>8.4f}  {r['ST_maxerr']:>8.4f}"
                )
        return

    data = run_tests()
    if args.json:
        json.dump(data, sys.stdout, indent=2)
    else:
        all_pass = all(r["pass"] for r in data)
        print(f"{'method':<25}  {'N':>6}  {'evals':>7}  {'sample':>8}"
              f"  {'analyze':>8}  {'pass':>5}")
        print("-" * 68)
        for r in data:
            mark = "OK" if r["pass"] else "FAIL"
            print(
                f"{r['method']:<25}  {r['N']:>6}  {r['evals']:>7}"
                f"  {r['sample_s']:>7.3f}s  {r['analyze_s']:>7.3f}s  {mark:>5}"
            )
        print(f"\n{'ALL PASS' if all_pass else 'SOME FAILURES'}")


if __name__ == "__main__":
    main()
