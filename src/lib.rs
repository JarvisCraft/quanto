//! Basic components for Rjacraft server development.
//!
//! # Features
#![doc = document_features::document_features!()]
//!

#[cfg(feature = "macros")]
/// Executes a `Quanto` expression using [global syncronous runtime][`runtime::global::sync_rt()`].
pub use quanto_macros::execute;
#[cfg(feature = "macros")]
/// Makes a function a Quanto main function.
pub use quanto_macros::main;
pub use quanto_runtime as runtime;
