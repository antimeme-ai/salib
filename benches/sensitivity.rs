#![allow(clippy::expect_used)]

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::f64::consts::PI;

use salib::estimators::{
    estimate_borgonovo_delta, estimate_dgsm, estimate_fast, estimate_janon, estimate_jansen,
    estimate_morris_effects, estimate_pawn, estimate_rbd_fast, estimate_regression_indices,
    estimate_saltelli2010, finite_difference_gradients, FdKind,
};
use salib::samplers::{
    build_fast_design, build_morris_trajectories, build_saltelli_matrix, LhsSampler, Sampler,
    SobolSampler,
};
use salib::*;

// ── Test functions ──────────────────────────────────────────────────

fn ishigami(x: &[f64]) -> f64 {
    x[0].sin() + 7.0 * x[1].sin().powi(2) + 0.1 * x[2].powi(4) * x[0].sin()
}

fn sobol_g(x: &[f64]) -> f64 {
    let a = [0.0, 1.0, 4.5, 9.0, 99.0, 99.0, 99.0, 99.0];
    x.iter()
        .zip(a.iter())
        .map(|(&xi, &ai)| ((4.0 * xi - 2.0).abs() + ai) / (1.0 + ai))
        .product()
}

fn problem_ishigami() -> Problem {
    ProblemBuilder::new()
        .factor("x1", Distribution::Uniform { lo: -PI, hi: PI })
        .factor("x2", Distribution::Uniform { lo: -PI, hi: PI })
        .factor("x3", Distribution::Uniform { lo: -PI, hi: PI })
        .build()
        .expect("valid problem")
}

fn problem_sobol_g() -> Problem {
    let mut b = ProblemBuilder::new();
    for i in 0..8 {
        b = b.factor(
            &format!("x{}", i + 1),
            Distribution::Uniform { lo: 0.0, hi: 1.0 },
        );
    }
    b.build().expect("valid problem")
}

// ── Saltelli / Sobol' estimators ────────────────────────────────────

fn bench_saltelli2010(c: &mut Criterion) {
    let mut group = c.benchmark_group("saltelli2010_ishigami");
    for n in [1024, 4096, 8192, 16384] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            let problem = problem_ishigami();
            let mut rng = RngState::from_seed([0u8; 32]);
            let sampler = SobolSampler::minimal(2 * problem.dim());
            let saltelli =
                build_saltelli_matrix(&sampler, n, false, &mut rng).expect("valid matrix");
            b.iter(|| estimate_saltelli2010(&saltelli, ishigami));
        });
    }
    group.finish();
}

fn bench_jansen(c: &mut Criterion) {
    let mut group = c.benchmark_group("jansen_ishigami");
    for n in [1024, 4096, 8192] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            let mut rng = RngState::from_seed([0u8; 32]);
            let problem = problem_ishigami();
            let sampler = SobolSampler::minimal(2 * problem.dim());
            let saltelli =
                build_saltelli_matrix(&sampler, n, false, &mut rng).expect("valid matrix");
            b.iter(|| estimate_jansen(&saltelli, ishigami));
        });
    }
    group.finish();
}

fn bench_janon(c: &mut Criterion) {
    let mut group = c.benchmark_group("janon_ishigami");
    for n in [1024, 4096, 8192] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            let mut rng = RngState::from_seed([0u8; 32]);
            let problem = problem_ishigami();
            let sampler = SobolSampler::minimal(2 * problem.dim());
            let saltelli =
                build_saltelli_matrix(&sampler, n, false, &mut rng).expect("valid matrix");
            b.iter(|| estimate_janon(&saltelli, ishigami));
        });
    }
    group.finish();
}

// ── Sobol' on Sobol' G function (8D) ───────────────────────────────

fn bench_saltelli2010_sobol_g(c: &mut Criterion) {
    let mut group = c.benchmark_group("saltelli2010_sobol_g");
    for n in [1024, 4096, 8192] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            let problem = problem_sobol_g();
            let mut rng = RngState::from_seed([0u8; 32]);
            let sampler = SobolSampler::minimal(2 * problem.dim());
            let saltelli =
                build_saltelli_matrix(&sampler, n, false, &mut rng).expect("valid matrix");
            b.iter(|| estimate_saltelli2010(&saltelli, sobol_g));
        });
    }
    group.finish();
}

// ── Morris ──────────────────────────────────────────────────────────

fn bench_morris(c: &mut Criterion) {
    let mut group = c.benchmark_group("morris_ishigami");
    for r in [10, 20, 50] {
        group.bench_with_input(BenchmarkId::from_parameter(r), &r, |b, &r| {
            let mut rng = RngState::from_seed([0u8; 32]);
            let trajectories =
                build_morris_trajectories(3, r, 4, &mut rng).expect("valid trajectories");
            b.iter(|| estimate_morris_effects(&trajectories, ishigami));
        });
    }
    group.finish();
}

// ── FAST / eFAST ────────────────────────────────────────────────────

fn bench_fast(c: &mut Criterion) {
    let mut group = c.benchmark_group("fast_ishigami");
    for n in [1025, 4097, 8193] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            let mut rng = RngState::from_seed([0u8; 32]);
            let design = build_fast_design(3, n, 4, &mut rng).expect("valid design");
            b.iter(|| estimate_fast(&design, ishigami));
        });
    }
    group.finish();
}

// ── RBD-FAST ────────────────────────────────────────────────────────

fn bench_rbd_fast(c: &mut Criterion) {
    let mut group = c.benchmark_group("rbd_fast_ishigami");
    for n in [1024, 4096, 8192] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            let mut rng = RngState::from_seed([0u8; 32]);
            let sampler = LhsSampler::classic(3);
            let x = sampler.unit_sample(n, &mut rng);
            let y: Vec<f64> = (0..n)
                .map(|i| ishigami(x.row(i).as_slice().expect("contiguous")))
                .collect();
            b.iter(|| estimate_rbd_fast(x.view(), &y, 6));
        });
    }
    group.finish();
}

// ── Borgonovo δ ─────────────────────────────────────────────────────

fn bench_borgonovo(c: &mut Criterion) {
    let mut group = c.benchmark_group("borgonovo_ishigami");
    for n in [1024, 4096, 8192] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            let mut rng = RngState::from_seed([0u8; 32]);
            let sampler = LhsSampler::classic(3);
            let x = sampler.unit_sample(n, &mut rng);
            let y: Vec<f64> = (0..n)
                .map(|i| ishigami(x.row(i).as_slice().expect("contiguous")))
                .collect();
            b.iter(|| estimate_borgonovo_delta(x.view(), &y));
        });
    }
    group.finish();
}

// ── PAWN ────────────────────────────────────────────────────────────

fn bench_pawn(c: &mut Criterion) {
    let mut group = c.benchmark_group("pawn_ishigami");
    for n in [1024, 4096, 8192] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            let mut rng = RngState::from_seed([0u8; 32]);
            let sampler = LhsSampler::classic(3);
            let x = sampler.unit_sample(n, &mut rng);
            let y: Vec<f64> = (0..n)
                .map(|i| ishigami(x.row(i).as_slice().expect("contiguous")))
                .collect();
            b.iter(|| estimate_pawn(x.view(), &y, 10));
        });
    }
    group.finish();
}

// ── DGSM ────────────────────────────────────────────────────────────

fn bench_dgsm(c: &mut Criterion) {
    let mut group = c.benchmark_group("dgsm_ishigami");
    for n in [1024, 4096, 8192] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            let mut rng = RngState::from_seed([0u8; 32]);
            let sampler = LhsSampler::classic(3);
            let x = sampler.unit_sample(n, &mut rng);
            let y: Vec<f64> = (0..n)
                .map(|i| ishigami(x.row(i).as_slice().expect("contiguous")))
                .collect();
            let grads = finite_difference_gradients(x.view(), 1e-6, FdKind::Central, ishigami);
            // Poincaré constant for Uniform[-π, π]: (2π)² / π² = 4.0
            let poincare = vec![4.0; 3];
            let var_y = tree_var(&y);
            b.iter(|| estimate_dgsm(grads.view(), &poincare, var_y));
        });
    }
    group.finish();
}

// ── Regression (SRC/SRRC/PCC/PRCC) ─────────────────────────────────

fn bench_regression(c: &mut Criterion) {
    let mut group = c.benchmark_group("regression_ishigami");
    for n in [1024, 4096, 8192] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            let mut rng = RngState::from_seed([0u8; 32]);
            let sampler = LhsSampler::classic(3);
            let x = sampler.unit_sample(n, &mut rng);
            let y: Vec<f64> = (0..n)
                .map(|i| ishigami(x.row(i).as_slice().expect("contiguous")))
                .collect();
            b.iter(|| estimate_regression_indices(x.view(), &y));
        });
    }
    group.finish();
}

// ── Sampling benchmarks ─────────────────────────────────────────────

fn bench_sampling_saltelli(c: &mut Criterion) {
    let mut group = c.benchmark_group("sampling_saltelli");
    for n in [1024, 4096, 16384] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            let sampler = SobolSampler::minimal(6);
            b.iter(|| {
                let mut rng = RngState::from_seed([0u8; 32]);
                build_saltelli_matrix(&sampler, n, false, &mut rng).expect("valid")
            });
        });
    }
    group.finish();
}

fn bench_sampling_morris(c: &mut Criterion) {
    let mut group = c.benchmark_group("sampling_morris");
    for r in [10, 50, 100] {
        group.bench_with_input(BenchmarkId::from_parameter(r), &r, |b, &r| {
            b.iter(|| {
                let mut rng = RngState::from_seed([0u8; 32]);
                build_morris_trajectories(3, r, 4, &mut rng).expect("valid")
            });
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_saltelli2010,
    bench_jansen,
    bench_janon,
    bench_saltelli2010_sobol_g,
    bench_morris,
    bench_fast,
    bench_rbd_fast,
    bench_borgonovo,
    bench_pawn,
    bench_dgsm,
    bench_regression,
    bench_sampling_saltelli,
    bench_sampling_morris,
);
criterion_main!(benches);
