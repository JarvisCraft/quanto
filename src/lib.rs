//! Basic components for Rjacraft server development.
//!
//! # Features
#![doc = document_features::document_features!()]
//!

#[cfg(feature = "macros")]
/// Executes a `Quanto` expression using [global syncronous runtime][`runtime::global::sync_rt()`].
#[macro_export]
macro_rules! execute {
    ($($token:tt)*) => {
        $crate::__macro_support::quanto_macros::execute!($crate::__macro_support, $($token)*)
    };
}

#[cfg(feature = "macros")]
/// Makes a function a Quanto main function.
pub use quanto_macros::main;
pub use quanto_runtime as runtime;

#[cfg(feature = "macros")]
#[doc(hidden)]
pub mod __macro_support {
    pub use quanto_macros;
    pub use quanto_runtime;
}
