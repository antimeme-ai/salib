//! `salib-cli` — operator-facing command surface for salib.
//!
//! Library half of the `salib` binary; the binary entry point at
//! `src/main.rs` is a thin clap-driven wrapper.
//!
//! # Subcommands
//!
//! - `salib sample <problem.yaml> --sampler=<sobol|lhs|saltelli|morris|fast> --n=<N> --seed=<s>`
//! - `salib run <experiment.yaml>`
//! - `salib analyze <samples.parquet> <outputs.parquet> --estimator=<saltelli2010|jansen|...>`

#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]
