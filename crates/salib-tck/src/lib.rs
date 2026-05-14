//! `salib-tck` — lightweight Gherkin parser and synchronous scenario
//! runner for TCK test harnesses. Zero external dependencies.
//!
//! Declare this crate only in `[dev-dependencies]`.
//!
//! - [`gherkin`] — Gherkin parser + [`gherkin::SyncRunner`] for
//!   step-definition-style TCK harnesses.

#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

pub mod gherkin;
