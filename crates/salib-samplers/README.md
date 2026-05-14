# salib-samplers

[![crates.io](https://img.shields.io/crates/v/salib-samplers.svg)](https://crates.io/crates/salib-samplers)
[![docs.rs](https://img.shields.io/docsrs/salib-samplers)](https://docs.rs/salib-samplers)
[![license](https://img.shields.io/crates/l/salib-samplers.svg)](https://github.com/antimeme-ai/salib)

Sampling designs for global sensitivity analysis. Part of the
[salib](https://crates.io/crates/salib) workspace.

Most users should depend on `salib` with the `samplers` feature (on by
default). Use `salib-samplers` directly when you need sampling without
the estimator dependency tree.

## Designs

| Design | Constructor | Reference |
|---|---|---|
| Latin Hypercube | `LhsSampler::classic`, `::centered` | McKay et al. (1979) |
| Sobol' QMC | `SobolSampler::standard` | Sobol' (1967); Joe & Kuo (2010) |
| Saltelli (A/B/A_Bi) | `build_saltelli_matrix` | Saltelli (2002) |
| Grouped Saltelli | `build_grouped_saltelli_matrix` | Saltelli et al. (2004) |
| Morris trajectories | `build_morris_trajectories` | Morris (1991) |
| Grouped Morris | `build_grouped_morris_trajectories` | Campolongo et al. (2007) |
| FAST / eFAST | `build_fast_design` | Cukier et al. (1973) |
| Owen scrambled | `build_owen_matrix` | Owen (1998) |
| Plackett-Burman | `build_plackett_burman` | Plackett & Burman (1946) |
| Iman-Conover | `iman_conover_transform` | Iman & Conover (1982) |

All samplers implement the `Sampler` trait, which provides `unit_sample`
(draws on [0, 1]^d) and `config_hash` (SHA-256 of the sampler configuration
for cache keying).

## Feature flags

| Flag | Default | Effect |
|---|---|---|
| `serde` | no | `Serialize`/`Deserialize` on output matrix types |

## License

MIT OR Apache-2.0, at your option.
