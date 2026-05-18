#!/usr/bin/env python3
"""
Generate docs/benchmarks.md from Criterion results.

Reads: target/criterion/*/new/estimates.json
Writes: docs/benchmarks.md

All numbers flow from data. No manual entry.
"""

import json
import os

REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CRITERION_BASE = os.path.join(REPO, "target", "criterion")
OUTPUT = os.path.join(REPO, "docs", "benchmarks.md")


def load_rust_median(group, param):
    path = os.path.join(CRITERION_BASE, group, str(param), "new", "estimates.json")
    if not os.path.isfile(path):
        return None
    with open(path) as f:
        d = json.load(f)
    return d["median"]["point_estimate"]  # nanoseconds


def fmt_time(ns):
    us = ns / 1000
    if us < 1000:
        return f"{us:.0f} µs" if us >= 10 else f"{us:.1f} µs"
    ms = us / 1000
    return f"{ms:.2f} ms" if ms < 10 else f"{ms:.0f} ms"


# (doc_name, criterion_group, N_values, includes_fn_eval)
METHODS = [
    ("Sobol (Saltelli 2010)", "saltelli2010_ishigami", [1024, 4096, 8192, 16384], True),
    ("Jansen",                "jansen_ishigami",        [1024, 4096, 8192],        True),
    ("Janon",                 "janon_ishigami",          [1024, 4096, 8192],        True),
    ("Sobol on Sobol' G (8D)", "saltelli2010_sobol_g",  [1024, 4096, 8192],        True),
    ("FAST / eFAST",          "fast_ishigami",           [1025, 4097, 8193],        True),
    ("Morris",                "morris_ishigami",         [10, 20, 50],              True),
    ("RBD-FAST",              "rbd_fast_ishigami",       [1024, 4096, 8192],        False),
    ("Borgonovo $\\delta$",   "borgonovo_ishigami",      [1024, 4096, 8192],        False),
    ("PAWN",                  "pawn_ishigami",           [1024, 4096, 8192],        False),
    ("DGSM",                  "dgsm_ishigami",           [1024, 4096, 8192],        False),
    ("Regression (SRC/SRRC/PCC/PRCC)", "regression_ishigami", [1024, 4096, 8192],  False),
]

SAMPLING = [
    ("Saltelli matrix",  "sampling_saltelli", [1024, 4096, 16384]),
    ("Morris trajectories", "sampling_morris", [10, 50, 100]),
]


def main():
    lines = []
    w = lines.append

    w("# Benchmarks")
    w("")
    w("Criterion benchmarks for every analysis method on the Ishigami function ($d = 3$), plus sampling.")
    w("")
    w("## Methodology")
    w("")
    w("[Criterion](https://github.com/bheisler/criterion.rs) 0.5, 100 samples per benchmark, automatic warmup. Statistic: **median**. Machine: Apple Silicon.")
    w("")
    w("**Test function:** Ishigami $f(x) = \\sin(x_1) + 7\\sin^2(x_2) + 0.1 x_3^4 \\sin(x_1)$ with $x_i \\in [-\\pi, \\pi]$.")
    w("")
    w("**What is timed:** Methods marked [fn] evaluate the model function inside the timed loop (sampling + analysis are interleaved in the estimator). All other methods take pre-computed $(X, Y)$ and time the analysis step only.")
    w("")

    # ── Analysis benchmarks ──
    w("## Analysis")
    w("")
    w("| Method | $N$ | Time |")
    w("|---|---|---|")
    for doc_name, group, ns, has_fn in METHODS:
        tag = " [fn]" if has_fn else ""
        for n in ns:
            rust_ns = load_rust_median(group, n)
            if rust_ns is None:
                continue
            w(f"| {doc_name}{tag} | {n} | {fmt_time(rust_ns)} |")
    w("")

    w("**[fn]** = benchmark includes Ishigami evaluation inside the timed loop. For these methods, the reported time is analysis + function evaluation. Ishigami is trivial (~3 ns/eval); for expensive models, function evaluation dominates and the analysis overhead shown here becomes negligible.")
    w("")

    # ── Morris note ──
    w("Morris $N$ is trajectory count $r$; total evaluations are $r \\times (d + 1)$.")
    w("")
    w("FAST $N$ values are odd (required by the algorithm).")
    w("")

    # ── Sampling benchmarks ──
    w("## Sampling")
    w("")
    w("| Method | $N$ | Time |")
    w("|---|---|---|")
    for doc_name, group, ns in SAMPLING:
        for n in ns:
            rust_ns = load_rust_median(group, n)
            if rust_ns is None:
                continue
            w(f"| {doc_name} | {n} | {fmt_time(rust_ns)} |")
    w("")

    w("Saltelli $N$ is base sample size; the matrix has $N \\times (d + 2)$ rows. Morris $N$ is trajectory count.")
    w("")

    # ── Reproducing ──
    w("## Reproducing")
    w("")
    w("```bash")
    w("cargo bench --manifest-path crates/salib/Cargo.toml")
    w("")
    w("# Regenerate this document from Criterion results")
    w("python benches/generate_benchmark_doc.py")
    w("```")
    w("")

    doc = "\n".join(lines)
    with open(OUTPUT, "w") as f:
        f.write(doc)
    print(f"Wrote {OUTPUT}")
    print(f"  {len(lines)} lines")


if __name__ == "__main__":
    main()
