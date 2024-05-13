//! Basic components for Rjacraft server development.
//!
//! # Features
#![doc = document_features::document_features!()]
//!

#[cfg(feature = "macros")]
pub use quanto_macros::{execute, main};
pub use quanto_runtime as runtime;
