# salib-tck

[![crates.io](https://img.shields.io/crates/v/salib-tck.svg)](https://crates.io/crates/salib-tck)
[![docs.rs](https://img.shields.io/docsrs/salib-tck)](https://docs.rs/salib-tck)
[![license](https://img.shields.io/crates/l/salib-tck.svg)](https://github.com/antimeme-ai/salib)

Lightweight Gherkin parser and synchronous scenario runner for TCK
(Technology Compatibility Kit) test harnesses. Zero external dependencies.

This is an internal testing crate for the
[salib](https://crates.io/crates/salib) workspace. It parses `.feature`
files and runs Gherkin scenarios against Rust step implementations.
Application code does not need this crate.

## What's inside

| Type | Purpose |
|---|---|
| `parse_feature` | Parses a `.feature` file into a `Feature` AST |
| `SyncRunner<W>` | Registers step handlers and runs scenarios against a world state `W` |
| `RunReport` | Collects pass/fail/skip outcomes; `assert_all_passed()` for test assertions |
| `Feature`, `Scenario`, `Step` | AST types for Gherkin documents |

Supports `Given` / `When` / `Then` / `And` steps, `Scenario Outline`
with `Examples` tables, `Background` sections, and `@tag` annotations.

## License

MIT OR Apache-2.0, at your option.
