# salib-cli

[![crates.io](https://img.shields.io/crates/v/salib-cli.svg)](https://crates.io/crates/salib-cli)
[![license](https://img.shields.io/crates/l/salib-cli.svg)](https://github.com/antimeme-ai/salib)

Command-line interface for [salib](https://crates.io/crates/salib).

```
cargo install salib-cli
```

## Subcommands

```
salib sample <problem.yaml> --sampler=<sobol|lhs|saltelli|morris|fast> --n=<N> --seed=<s>
salib run    <experiment.yaml>
salib analyze <samples.parquet> <outputs.parquet> --estimator=<saltelli2010|jansen|...>
```

`sample` generates a design matrix from a problem definition.
`run` evaluates a model over a sample matrix.
`analyze` computes sensitivity indices from sample/output pairs.

All operations are reproducible: the same seed produces the same results.

## License

MIT OR Apache-2.0, at your option.
