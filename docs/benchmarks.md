# Benchmarks

Criterion benchmarks for every analysis method on the Ishigami function ($d = 3$), plus sampling.

## Methodology

[Criterion](https://github.com/bheisler/criterion.rs) 0.5, 100 samples per benchmark, automatic warmup. Statistic: **median**. Machine: Apple Silicon.

**Test function:** Ishigami $f(x) = \sin(x_1) + 7\sin^2(x_2) + 0.1 x_3^4 \sin(x_1)$ with $x_i \in [-\pi, \pi]$.

**What is timed:** Methods marked [fn] evaluate the model function inside the timed loop (sampling + analysis are interleaved in the estimator). All other methods take pre-computed $(X, Y)$ and time the analysis step only.

## Analysis

| Method | $N$ | Time |
|---|---|---|
| Sobol (Saltelli 2010) [fn] | 1024 | 28 µs |
| Sobol (Saltelli 2010) [fn] | 4096 | 116 µs |
| Sobol (Saltelli 2010) [fn] | 8192 | 239 µs |
| Sobol (Saltelli 2010) [fn] | 16384 | 482 µs |
| Jansen [fn] | 1024 | 24 µs |
| Jansen [fn] | 4096 | 101 µs |
| Jansen [fn] | 8192 | 220 µs |
| Janon [fn] | 1024 | 28 µs |
| Janon [fn] | 4096 | 114 µs |
| Janon [fn] | 8192 | 260 µs |
| Sobol on Sobol' G (8D) [fn] | 1024 | 60 µs |
| Sobol on Sobol' G (8D) [fn] | 4096 | 249 µs |
| Sobol on Sobol' G (8D) [fn] | 8192 | 495 µs |
| FAST / eFAST [fn] | 1025 | 57 µs |
| FAST / eFAST [fn] | 4097 | 280 µs |
| FAST / eFAST [fn] | 8193 | 642 µs |
| Morris [fn] | 10 | 0.7 µs |
| Morris [fn] | 20 | 1.0 µs |
| Morris [fn] | 50 | 2.2 µs |
| RBD-FAST | 1024 | 41 µs |
| RBD-FAST | 4096 | 205 µs |
| RBD-FAST | 8192 | 455 µs |
| Borgonovo $\delta$ | 1024 | 793 µs |
| Borgonovo $\delta$ | 4096 | 3.27 ms |
| Borgonovo $\delta$ | 8192 | 6.78 ms |
| PAWN | 1024 | 108 µs |
| PAWN | 4096 | 497 µs |
| PAWN | 8192 | 1.15 ms |
| DGSM | 1024 | 2.3 µs |
| DGSM | 4096 | 11 µs |
| DGSM | 8192 | 29 µs |
| Regression (SRC/SRRC/PCC/PRCC) | 1024 | 307 µs |
| Regression (SRC/SRRC/PCC/PRCC) | 4096 | 1.11 ms |
| Regression (SRC/SRRC/PCC/PRCC) | 8192 | 1.78 ms |

**[fn]** = benchmark includes Ishigami evaluation inside the timed loop. For these methods, the reported time is analysis + function evaluation. Ishigami is trivial (~3 ns/eval); for expensive models, function evaluation dominates and the analysis overhead shown here becomes negligible.

Morris $N$ is trajectory count $r$; total evaluations are $r \times (d + 1)$.

FAST $N$ values are odd (required by the algorithm).

## Sampling

| Method | $N$ | Time |
|---|---|---|
| Saltelli matrix | 1024 | 10 µs |
| Saltelli matrix | 4096 | 56 µs |
| Saltelli matrix | 16384 | 249 µs |
| Morris trajectories | 10 | 0.9 µs |
| Morris trajectories | 50 | 3.7 µs |
| Morris trajectories | 100 | 7.4 µs |

Saltelli $N$ is base sample size; the matrix has $N \times (d + 2)$ rows. Morris $N$ is trajectory count.

## Reproducing

```bash
cargo bench --manifest-path crates/salib/Cargo.toml

# Regenerate this document from Criterion results
python benches/generate_benchmark_doc.py
```
