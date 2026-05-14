#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

pub use salib_core::*;

#[cfg(feature = "samplers")]
pub use salib_samplers as samplers;

#[cfg(feature = "estimators")]
pub use salib_estimators as estimators;

#[cfg(feature = "surrogate")]
pub use salib_surrogate as surrogate;

#[cfg(feature = "shapley")]
pub use salib_shapley as shapley;

#[cfg(feature = "validation")]
pub use salib_validation as validation;
