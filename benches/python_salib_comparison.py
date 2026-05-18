#!/usr/bin/env python3
"""
Benchmark Python SALib and validate correctness.

Two modes:
  --validate   Run SALib's regression tests (correctness check).
  --bench      Time analysis methods at multiple N (performance comparison).
  --json       Output JSON instead of table (combinable with either mode).

Correctness tests replicate SALib's own test_regression.py exactly:
same samplers, same seeds, same expected values, same tolerances.

Timing benchmarks measure analysis time only (not sampling), at N values
matching the Rust Criterion benchmarks, with K=20 repetitions per method.

Usage:
    pip install SALib numpy
    python benches/python_salib_comparison.py --validate
    python benches/python_salib_comparison.py --bench
    python benches/python_salib_comparison.py --bench --json

SALib version tested: 1.5.2
Source of truth: github.com/SALib/SALib/blob/main/tests/test_regression.py
"""

import argparse
import json
import statistics
import sys
import time
import warnings

import numpy as np
from numpy.testing import assert_allclose

warnings.filterwarnings("ignore", category=DeprecationWarning)
warnings.filterwarnings("ignore", category=UserWarning)

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
    finite_diff,
    latin,
    saltelli,
)
from SALib.sample.morris import sample as morris_sampler
from SALib.test_functions import Ishigami
from SALib.util import handle_seed


ISHIGAMI_PROBLEM = {
    "num_vars": 3,
    "names": ["x1", "x2", "x3"],
    "bounds": [[-np.pi, np.pi]] * 3,
}


# ── Correctness validation ───────────────────────────────────────────
# Each test replicates SALib's test_regression.py exactly:
# same sampler, same seed mechanism, same expected values, same tolerances.


def validate_sobol():
    """test_regression_sobol: saltelli sampler, no explicit seed."""
    X = saltelli.sample(ISHIGAMI_PROBLEM, 10000, calc_second_order=True)
    Y = Ishigami.evaluate(X)
    Si = sobol.analyze(
        ISHIGAMI_PROBLEM, Y,
        calc_second_order=True, conf_level=0.95, print_to_console=False,
    )
    assert_allclose(Si["S1"], [0.31, 0.44, 0.00], atol=5e-2, rtol=1e-1)
    assert_allclose(Si["ST"], [0.55, 0.44, 0.24], atol=5e-2, rtol=1e-1)
    return Si


def validate_fast():
    """test_regression_fast: fast_sampler, no explicit seed."""
    X = fast_sampler.sample(ISHIGAMI_PROBLEM, 10000)
    Y = Ishigami.evaluate(X)
    Si = fast.analyze(ISHIGAMI_PROBLEM, Y, print_to_console=False)
    assert_allclose(Si["S1"], [0.31, 0.44, 0.00], atol=5e-2, rtol=1e-1)
    assert_allclose(Si["ST"], [0.55, 0.44, 0.24], atol=5e-2, rtol=1e-1)
    return Si


def validate_rbd_fast():
    """test_regression_rbd_fast: latin sampler, no explicit seed."""
    X = latin.sample(ISHIGAMI_PROBLEM, 10000)
    Y = Ishigami.evaluate(X)
    Si = rbd_fast.analyze(ISHIGAMI_PROBLEM, X, Y, print_to_console=False)
    assert_allclose(Si["S1"], [0.31, 0.44, 0.00], atol=5e-2, rtol=1e-1)
    return Si


def validate_delta():
    """test_regression_delta: latin sampler, no explicit seed.
    SALib 1.5.2 uses key 'delta'; main branch renamed to 'delta_raw'."""
    X = latin.sample(ISHIGAMI_PROBLEM, 10000)
    Y = Ishigami.evaluate(X)
    Si = delta.analyze(
        ISHIGAMI_PROBLEM, X, Y,
        num_resamples=10, conf_level=0.95, print_to_console=False,
    )
    delta_key = "delta_raw" if "delta_raw" in Si else "delta"
    assert_allclose(Si[delta_key], [0.210, 0.358, 0.155], atol=5e-2, rtol=1e-1)
    assert_allclose(Si["S1"], [0.31, 0.44, 0.00], atol=5e-2, rtol=1e-1)
    return Si


def validate_dgsm():
    """test_regression_dgsm: finite_diff sampler (NOT latin), no explicit seed."""
    X = finite_diff.sample(ISHIGAMI_PROBLEM, 10000, delta=0.001)
    Y = Ishigami.evaluate(X)
    Si = dgsm.analyze(
        ISHIGAMI_PROBLEM, X, Y,
        conf_level=0.95, print_to_console=False,
    )
    assert_allclose(Si["dgsm"], [2.229, 7.066, 3.180], atol=5e-2, rtol=1e-1)
    return Si


def validate_morris():
    """test_regression_morris_vanilla: handle_seed(12345) + np.random.seed(123456).
    Morris is the only method that requires the dual-seed fixture."""
    rng = handle_seed(12345)
    np.random.seed(123456)
    X = morris_sampler(
        ISHIGAMI_PROBLEM, 10000,
        num_levels=4, optimal_trajectories=None, seed=rng,
    )
    Y = Ishigami.evaluate(X)
    Si = morris_analyze.analyze(
        ISHIGAMI_PROBLEM, X, Y,
        conf_level=0.95, print_to_console=False, num_levels=4, seed=rng,
    )
    assert_allclose(Si["mu_star"], [7.682808, 7.875, 6.256295], atol=0, rtol=1e-5)
    return Si


def validate_hdmr_ishigami():
    """test_regression_hdmr_ishigami: latin sampler, no explicit seed."""
    X = latin.sample(ISHIGAMI_PROBLEM, 10000)
    Y = Ishigami.evaluate(X)
    Si = hdmr.analyze(
        ISHIGAMI_PROBLEM, X, Y,
        maxorder=2, maxiter=100, m=4, K=1, R=10000,
        alpha=0.95, lambdax=0.01, print_to_console=False,
    )
    n = ISHIGAMI_PROBLEM["num_vars"]
    assert_allclose(Si["Sa"][:n], [0.31, 0.44, 0.00], atol=5e-2, rtol=1e-1)
    assert_allclose(Si["ST"][:n], [0.55, 0.44, 0.24], atol=5e-2, rtol=1e-1)
    return Si


VALIDATION_TESTS = [
    ("sobol", validate_sobol),
    ("fast", validate_fast),
    ("rbd_fast", validate_rbd_fast),
    ("delta", validate_delta),
    ("dgsm", validate_dgsm),
    ("morris", validate_morris),
    ("hdmr", validate_hdmr_ishigami),
]


def run_validate():
    results = []
    for name, fn in VALIDATION_TESTS:
        t0 = time.perf_counter()
        try:
            fn()
            elapsed = time.perf_counter() - t0
            results.append({"method": name, "pass": True, "time_s": round(elapsed, 4)})
        except AssertionError as e:
            elapsed = time.perf_counter() - t0
            results.append({
                "method": name, "pass": False,
                "time_s": round(elapsed, 4), "error": str(e),
            })
    return results


# ── Timing benchmarks ────────────────────────────────────────────────
# Measure analysis time only. Sampling and model evaluation happen once
# outside the timing loop. K repetitions, report median/min/max.
#
# N values match the Rust Criterion benchmarks: 1024, 4096, 8192.
# We test only methods that overlap between Rust salib and Python SALib.

BENCH_NS = [1024, 4096, 8192]
BENCH_K = 20


def bench_sobol(N, K):
    X = saltelli.sample(ISHIGAMI_PROBLEM, N, calc_second_order=False)
    Y = Ishigami.evaluate(X)
    times = []
    for _ in range(K):
        t0 = time.perf_counter()
        sobol.analyze(ISHIGAMI_PROBLEM, Y, calc_second_order=False, print_to_console=False)
        times.append(time.perf_counter() - t0)
    return times


def bench_fast(N, K):
    X = fast_sampler.sample(ISHIGAMI_PROBLEM, N)
    Y = Ishigami.evaluate(X)
    times = []
    for _ in range(K):
        t0 = time.perf_counter()
        fast.analyze(ISHIGAMI_PROBLEM, Y, print_to_console=False)
        times.append(time.perf_counter() - t0)
    return times


def bench_rbd_fast(N, K):
    X = latin.sample(ISHIGAMI_PROBLEM, N)
    Y = Ishigami.evaluate(X)
    times = []
    for _ in range(K):
        t0 = time.perf_counter()
        rbd_fast.analyze(ISHIGAMI_PROBLEM, X, Y, print_to_console=False)
        times.append(time.perf_counter() - t0)
    return times


def bench_delta(N, K):
    X = latin.sample(ISHIGAMI_PROBLEM, N)
    Y = Ishigami.evaluate(X)
    times = []
    for _ in range(K):
        t0 = time.perf_counter()
        delta.analyze(
            ISHIGAMI_PROBLEM, X, Y,
            num_resamples=10, conf_level=0.95, print_to_console=False,
        )
        times.append(time.perf_counter() - t0)
    return times


def bench_dgsm(N, K):
    X = finite_diff.sample(ISHIGAMI_PROBLEM, N, delta=0.001)
    Y = Ishigami.evaluate(X)
    times = []
    for _ in range(K):
        t0 = time.perf_counter()
        dgsm.analyze(ISHIGAMI_PROBLEM, X, Y, conf_level=0.95, print_to_console=False)
        times.append(time.perf_counter() - t0)
    return times


def bench_morris(N, K):
    rng = handle_seed(12345)
    np.random.seed(123456)
    X = morris_sampler(
        ISHIGAMI_PROBLEM, N,
        num_levels=4, optimal_trajectories=None, seed=rng,
    )
    Y = Ishigami.evaluate(X)
    times = []
    for _ in range(K):
        rng_analyze = handle_seed(12345)
        t0 = time.perf_counter()
        morris_analyze.analyze(
            ISHIGAMI_PROBLEM, X, Y,
            conf_level=0.95, print_to_console=False, num_levels=4, seed=rng_analyze,
        )
        times.append(time.perf_counter() - t0)
    return times


BENCH_METHODS = [
    ("sobol", bench_sobol),
    ("fast", bench_fast),
    ("rbd_fast", bench_rbd_fast),
    ("delta", bench_delta),
    ("dgsm", bench_dgsm),
    ("morris", bench_morris),
]


def run_bench(ns, k):
    results = []
    for name, fn in BENCH_METHODS:
        for n in ns:
            times = fn(n, k)
            results.append({
                "method": name,
                "N": n,
                "K": k,
                "median_s": round(statistics.median(times), 6),
                "min_s": round(min(times), 6),
                "max_s": round(max(times), 6),
                "mean_s": round(statistics.mean(times), 6),
            })
    return results


# ── CLI ──────────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(
        description="Benchmark and validate Python SALib"
    )
    parser.add_argument("--validate", action="store_true",
                        help="Run SALib regression tests (correctness)")
    parser.add_argument("--bench", action="store_true",
                        help="Time analysis methods at multiple N")
    parser.add_argument("--json", action="store_true",
                        help="Output JSON instead of table")
    parser.add_argument("--reps", type=int, default=BENCH_K,
                        help=f"Repetitions per benchmark (default {BENCH_K})")
    args = parser.parse_args()

    if not args.validate and not args.bench:
        parser.print_help()
        sys.exit(1)

    if args.validate:
        data = run_validate()
        if args.json:
            json.dump(data, sys.stdout, indent=2)
            print()
        else:
            all_pass = all(r["pass"] for r in data)
            print(f"{'method':<12} {'time':>8} {'result':>6}")
            print("-" * 30)
            for r in data:
                mark = "OK" if r["pass"] else "FAIL"
                print(f"{r['method']:<12} {r['time_s']:>7.3f}s {mark:>6}")
            print(f"\n{'ALL PASS' if all_pass else 'SOME FAILURES'}")
            if not all_pass:
                sys.exit(1)

    if args.bench:
        data = run_bench(BENCH_NS, args.reps)
        if args.json:
            json.dump(data, sys.stdout, indent=2)
            print()
        else:
            print(f"{'method':<12} {'N':>6} {'median':>10} {'min':>10} {'max':>10}  (K={args.reps})")
            print("-" * 56)
            for r in data:
                print(
                    f"{r['method']:<12} {r['N']:>6}"
                    f" {r['median_s']*1000:>9.3f}ms"
                    f" {r['min_s']*1000:>9.3f}ms"
                    f" {r['max_s']*1000:>9.3f}ms"
                )


if __name__ == "__main__":
    main()
