# salib-core

[![crates.io](https://img.shields.io/crates/v/salib-core.svg)](https://crates.io/crates/salib-core)
[![docs.rs](https://img.shields.io/docsrs/salib-core)](https://docs.rs/salib-core)
[![license](https://img.shields.io/crates/l/salib-core.svg)](https://github.com/antimeme-ai/salib)

Foundational types for the [salib](https://crates.io/crates/salib) workspace:
problem definitions, factor distributions, reproducible RNG, and
bit-deterministic reductions.

Most users should depend on `salib` directly. Use `salib-core` when you
need the base types without pulling in samplers or estimators.

## What's inside

| Type | Purpose |
|---|---|
| `Problem` / `ProblemBuilder` | Describes factors, distributions, and groups |
| `Factor` | Name, distribution, and optional group membership |
| `Distribution` | Uniform, Normal, LogNormal, Triangular, and more |
| `RngState` | Multi-stream ChaCha20 RNG with deterministic seeding |
| `tree_sum`, `tree_dot`, `tree_var` | Bit-deterministic reductions (serial) |
| `par_tree_sum`, `par_tree_dot`, `par_tree_var` | Same, parallelized via rayon |

## Feature flags

| Flag | Default | Effect |
|---|---|---|
| `parallel` | yes | Enables rayon and `par_tree_*` functions |

When `parallel` is off, `par_tree_*` falls back to the serial implementation.
Results are identical either way.

## License

MIT OR Apache-2.0, at your option.
